use std::fmt;
use std::string::String;

use lazy_static::lazy_static;
use logos::{Lexer, Logos};
use regex::{Captures, Regex};

use variants::uint::UInt;

use crate::comp::Comp;
use crate::dir::{Directive, MacroModifier, Placeholder, PlaceholderVariant};
use crate::op::Op;
use crate::pair::Pair;
use crate::Keyword;

fn placeholder(lex: &mut Lexer<Kind>) -> Option<Directive> {
    lazy_static! {
        static ref TEMPLATE_REGEX: Result<Regex, regex::Error> =
            Regex::new(r"([%$])([0-9]+)\.([_a-z]+)");
    }

    let slice = lex.slice();

    let regex = TEMPLATE_REGEX.as_ref().ok()?;
    let captures: Captures = regex.captures(slice)?;

    let scope = captures.get(1)?.as_str();
    let index = captures.get(2)?.as_str().parse::<u32>().ok()?;
    let type_ = captures.get(3)?.as_str();

    let variant = type_
        .split("")
        .into_iter()
        .map(|t| match t {
            "i" => PlaceholderVariant::IDENT,
            "n" => PlaceholderVariant::NUMBER,
            "p" => PlaceholderVariant::PRIMITIVE,
            "e" => PlaceholderVariant::EXPR,
            "b" => PlaceholderVariant::BLOCK,
            "t" => PlaceholderVariant::TERMS,
            "c" => PlaceholderVariant::COMP,
            "o" => PlaceholderVariant::OP,
            "_" => PlaceholderVariant::ANY,
            _ => PlaceholderVariant::NONE,
        })
        .fold(PlaceholderVariant::NONE, |a, b| a | b);

    let ident = match scope {
        "%" => Some(Placeholder::Match { variant, index }),
        "$" => Some(Placeholder::Sub { variant, index }),
        _ => None,
    }?;

    Some(Directive::Placeholder(ident))
}

fn macro_start(lex: &mut Lexer<Kind>) -> Option<Directive> {
    lazy_static! {
        static ref MACRO_START_REGEX: Result<Regex, regex::Error> =
            Regex::new(r"@macro(?:/(?P<flags>[i]*))?(?:/(?P<priority>[0-9]*))?");
    }

    let slice = lex.slice();
    let regex = MACRO_START_REGEX.as_ref().ok()?;
    let captures: Captures = regex.captures(slice)?;

    let modifier = captures
        .name("flags")
        .map(|flag| {
            flag.as_str()
                .chars()
                .into_iter()
                .map(|char| match char {
                    'i' => MacroModifier::CASE_INSENSITIVE,
                    _ => MacroModifier::NONE,
                })
                .fold(MacroModifier::NONE, |a, b| a | b)
        })
        .unwrap_or(MacroModifier::NONE);
    let priority = captures
        .name("priority")
        .map(|priority| {
            priority
                .as_str() //
                .parse::<u32>()
                .unwrap_or(0)
        })
        .unwrap_or(0);

    Some(Directive::Macro { modifier, priority })
}

fn line_comment(lex: &mut Lexer<Kind>) -> Option<()> {
    let len = lex.remainder().find('\n');
    if len.is_none() {
        // means the comment is at the end of the line
        lex.bump(lex.remainder().len());
        return Some(());
    }

    let len = len?;
    lex.bump(len); // do not include \n as \n is a separator

    Some(())
}

fn block_comment(lex: &mut Lexer<Kind>) -> Option<()> {
    let len = lex.remainder().find("###")?;
    lex.bump(len + 3); // include the length of ###

    Some(())
}

#[derive(Debug, Clone, PartialEq, Logos)]
pub enum Kind {
    #[regex("[_a-zA-Z][_a-zA-Z0-9]*", callback = |lex| String::from(lex.slice()))]
    Ident(String),

    #[regex("[a-zA-Z_]+(::[a-zA-Z_]+)+", callback = |lex| lex.slice().split("::").map(String::from).collect::<Vec<_>>())]
    Path(Vec<String>),

    #[regex("[0-9]+", | v | v.slice().parse())]
    Number(UInt),

    #[token("while", ignore(case) callback = |_| Keyword::While)]
    #[token("loop", ignore(case) callback = |_| Keyword::Loop)]
    #[token("fn", ignore(case) callback = |_| Keyword::Fn)]
    #[token("do", ignore(case) callback = |_| Keyword::Do)]
    #[token("decl", ignore(case) callback = |_| Keyword::Decl)]
    #[token("end", ignore(case) callback = |_| Keyword::End)]
    #[token("import", ignore(case) callback = |_| Keyword::Import)]
    #[token("from", ignore(case) callback = |_| Keyword::From)]
    #[token("as", ignore(case) callback = |_| Keyword::As)]
    #[token("macro", ignore(case) callback = |_| Keyword::Macro)]
    Keyword(Keyword),

    #[token("+", | _ | Op::Plus)]
    #[token("-", | _ | Op::Minus)]
    #[token("*", | _ | Op::Star)]
    #[token("/", | _ | Op::Slash)]
    Op(Op),

    #[token("==", | _ | Comp::Equal)]
    #[token("!=", | _ | Comp::NotEqual)]
    #[token(">", | _ | Comp::GreaterThan)]
    #[token(">=", | _ | Comp::GreaterEqual)]
    #[token("<", | _ | Comp::LessThan)]
    #[token("<=", | _ | Comp::LessEqual)]
    Comp(Comp),

    #[regex(r"@macro(/[i]*)?(/[0-9]*)?", macro_start)]
    #[token("@sub", | _ | Directive::Sub)]
    #[token("@end", | _ | Directive::End)]
    #[token("@if", | _ | Directive::If)]
    #[token("@else", | _ | Directive::Else)]
    #[token("@sep", | _ | Directive::Sep)]
    #[regex(r"%[0-9]+\.[inpebtco_]+", placeholder)]
    #[regex(r"\$[0-9]+\.[i]+", placeholder)]
    Directive(Directive),

    #[token("=")]
    #[token(":=")]
    Assign,

    #[token("(", | _ | Pair::Left)]
    #[token(")", | _ | Pair::Right)]
    Paren(Pair),

    #[token("{", | _ | Pair::Left)]
    #[token("}", | _ | Pair::Right)]
    Brace(Pair),

    #[token("->")]
    Into,

    #[token("...")]
    Ellipsis,

    #[token(",")]
    Comma,

    #[token("#", line_comment)]
    #[token("###", block_comment)]
    Comment,

    #[regex(r"[ \t\f]+")]
    Whitespace,

    #[regex(r";")]
    Semicolon,

    #[token("\n")]
    Newline,

    #[error]
    Error,
}

impl Kind {
    pub fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Whitespace => "␠".into(),
                Self::Ident(m) => format!("ident<‘{}‘>", m),
                Self::Comment => "comment".into(),
                Self::Op(op) => format!("{}", op),
                Self::Ellipsis => "‘...‘".into(),
                Self::Number(n) => format!("number<‘{}‘>", n),
                Self::Keyword(keyword) => format!("{}", keyword),
                Self::Directive(directive) => format!("{}", directive),
                Self::Assign => "‘:=‘".into(),
                Self::Comp(comp) => format!("{}", comp),
                Self::Semicolon => "‘;‘\n".into(),
                Self::Newline => "␊\n".into(),
                Self::Into => "‘->‘".into(),
                Self::Comma => "‘,‘".into(),
                Self::Paren(Pair::Left) => "‘(‘".into(),
                Self::Paren(Pair::Right) => "‘)‘".into(),
                Self::Brace(Pair::Left) => "‘{‘".into(),
                Self::Brace(Pair::Right) => "‘}‘".into(),
                Self::Error => "an unrecognized token".into(),
                Self::Path(_) => "path".into(),
            }
        )
    }
}

//region Tests
#[cfg(test)]
mod tests {
    use crate::Lexer;

    use super::*;
    use std::fs::read_to_string;

    fn check_single_kind(input: &str, kind: Kind) {
        let mut lexer = Lexer::new(input);

        let token = lexer.next().unwrap();
        assert_eq!(token.kind, kind);
        assert_eq!(token.content, input)
    }

    #[test]
    fn lex_op() {
        check_single_kind("+", Kind::Op(Op::Plus));
        check_single_kind("-", Kind::Op(Op::Minus));
        check_single_kind("*", Kind::Op(Op::Star));
        check_single_kind("/", Kind::Op(Op::Slash))
    }

    #[test]
    fn lex_ellipsis() {
        check_single_kind("...", Kind::Ellipsis);
    }

    #[test]
    fn lex_ident() {
        check_single_kind("abc", Kind::Ident(String::from("abc")))
    }

    #[test]
    fn lex_path() {
        let path: Vec<String> = vec!["abc".into(), "ccd".into()];
        check_single_kind("abc::ccd", Kind::Path(path))
    }

    #[test]
    fn lex_number() {
        check_single_kind("123", Kind::Number(UInt::from(123u8)))
    }

    #[test]
    fn lex_kw() {
        check_single_kind("while", Kind::Keyword(Keyword::While));
        check_single_kind("wHiLe", Kind::Keyword(Keyword::While));
        check_single_kind("loop", Kind::Keyword(Keyword::Loop));
        check_single_kind("LOOP", Kind::Keyword(Keyword::Loop));
        check_single_kind("end", Kind::Keyword(Keyword::End));
        check_single_kind("fn", Kind::Keyword(Keyword::Fn));
    }

    #[test]
    fn lex_directive() {
        check_single_kind(
            "@macro",
            Kind::Directive(Directive::Macro {
                modifier: MacroModifier::empty(),
                priority: 0,
            }),
        );
        check_single_kind(
            "@macro/i",
            Kind::Directive(Directive::Macro {
                modifier: MacroModifier::CASE_INSENSITIVE,
                priority: 0,
            }),
        );
        check_single_kind(
            "@macro/i/10",
            Kind::Directive(Directive::Macro {
                modifier: MacroModifier::CASE_INSENSITIVE,
                priority: 10,
            }),
        );

        check_single_kind(
            "@macro//10",
            Kind::Directive(Directive::Macro {
                modifier: MacroModifier::NONE,
                priority: 10,
            }),
        );

        check_single_kind(
            "@macro/10",
            Kind::Directive(Directive::Macro {
                modifier: MacroModifier::NONE,
                priority: 10,
            }),
        );

        check_single_kind("@sub", Kind::Directive(Directive::Sub));
        check_single_kind("@end", Kind::Directive(Directive::End));

        check_single_kind("@if", Kind::Directive(Directive::If));
        check_single_kind("@else", Kind::Directive(Directive::Else));

        check_single_kind("@sep", Kind::Directive(Directive::Sep));

        check_single_kind(
            "%1.i",
            Kind::Directive(Directive::Placeholder(Placeholder::Match {
                index: 1,
                variant: PlaceholderVariant::IDENT,
            })),
        );
        check_single_kind(
            "%1.n",
            Kind::Directive(Directive::Placeholder(Placeholder::Match {
                index: 1,
                variant: PlaceholderVariant::NUMBER,
            })),
        );
        check_single_kind(
            "%1.p",
            Kind::Directive(Directive::Placeholder(Placeholder::Match {
                index: 1,
                variant: PlaceholderVariant::PRIMITIVE,
            })),
        );
        check_single_kind(
            "%1.in",
            Kind::Directive(Directive::Placeholder(Placeholder::Match {
                index: 1,
                variant: PlaceholderVariant::PRIMITIVE,
            })),
        );
        check_single_kind(
            "%1.e",
            Kind::Directive(Directive::Placeholder(Placeholder::Match {
                index: 1,
                variant: PlaceholderVariant::EXPR,
            })),
        );
        check_single_kind(
            "%1.b",
            Kind::Directive(Directive::Placeholder(Placeholder::Match {
                index: 1,
                variant: PlaceholderVariant::BLOCK,
            })),
        );
        check_single_kind(
            "%1.t",
            Kind::Directive(Directive::Placeholder(Placeholder::Match {
                index: 1,
                variant: PlaceholderVariant::TERMS,
            })),
        );
        check_single_kind(
            "%1.c",
            Kind::Directive(Directive::Placeholder(Placeholder::Match {
                index: 1,
                variant: PlaceholderVariant::COMP,
            })),
        );
        check_single_kind(
            "%1.o",
            Kind::Directive(Directive::Placeholder(Placeholder::Match {
                index: 1,
                variant: PlaceholderVariant::OP,
            })),
        );
        check_single_kind(
            "%1._",
            Kind::Directive(Directive::Placeholder(Placeholder::Match {
                index: 1,
                variant: PlaceholderVariant::ANY,
            })),
        );
        check_single_kind(
            "$1.i",
            Kind::Directive(Directive::Placeholder(Placeholder::Sub {
                index: 1,
                variant: PlaceholderVariant::IDENT,
            })),
        );
    }

    #[test]
    fn lex_assign() {
        check_single_kind("=", Kind::Assign);
        check_single_kind(":=", Kind::Assign);
    }

    #[test]
    fn lex_comp() {
        check_single_kind("==", Kind::Comp(Comp::Equal));
        check_single_kind("!=", Kind::Comp(Comp::NotEqual));
        check_single_kind(">", Kind::Comp(Comp::GreaterThan));
        check_single_kind(">=", Kind::Comp(Comp::GreaterEqual));
        check_single_kind("<", Kind::Comp(Comp::LessThan));
        check_single_kind("<=", Kind::Comp(Comp::LessEqual));
    }

    #[test]
    fn lex_arrow() {
        check_single_kind("->", Kind::Into);
    }

    #[test]
    fn lex_comment() {
        check_single_kind(
            "### comment ## intermediate # \n comment ###",
            Kind::Comment,
        );
        check_single_kind("# comment", Kind::Comment);

        // check if \n is marked as a separator
        let mut lexer = Lexer::new("# comment\n");

        let token = lexer.next().unwrap();
        assert_eq!(token.kind, Kind::Comment);
        assert_eq!(token.content, "# comment");

        let token = lexer.next().unwrap();
        assert_eq!(token.kind, Kind::Newline);
        assert_eq!(token.content, "\n");
    }

    #[test]
    fn lex_whitespace() {
        check_single_kind("     \t    ", Kind::Whitespace);
    }

    #[test]
    fn lex_sep() {
        check_single_kind(";", Kind::Semicolon);
        check_single_kind("\n", Kind::Newline);
    }
}
//endregion
