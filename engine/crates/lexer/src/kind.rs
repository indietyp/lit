use std::fmt;
use std::string::String;

use lazy_static::lazy_static;
use logos::{Lexer, Logos};
use regex::{Captures, Regex};

use variants::uint::UInt;

use crate::comp::Comp;
use crate::dir::{Directive, MacroModifier, Placeholder};
use crate::op::Op;
use crate::pair::Pair;
use crate::Keyword;

fn placeholder(lex: &mut Lexer<Kind>) -> Option<Directive> {
    lazy_static! {
        static ref TEMPLATE_REGEX: Result<Regex, regex::Error> =
            Regex::new(r"([%$])([_a-z])\.([0-9]+)");
    }

    let slice = lex.slice();

    let regex = TEMPLATE_REGEX.as_ref().ok()?;
    let captures: Captures = regex.captures(slice)?;

    let scope = captures.get(1)?.as_str();
    let type_ = captures.get(2)?.as_str();
    let number = captures.get(3)?.as_str().parse::<u32>().ok()?;

    let ident = match scope {
        "%" => match type_ {
            "i" => Some(Placeholder::Ident(number)),
            "a" => Some(Placeholder::Atom(number)),
            "v" => Some(Placeholder::Value(number)),
            "e" => Some(Placeholder::Expr(number)),
            "t" => Some(Placeholder::Terms(number)),
            "o" => Some(Placeholder::Op(number)),
            "c" => Some(Placeholder::Comp(number)),
            "_" => Some(Placeholder::Any(number)),
            _ => None,
        },
        "$" => match type_ {
            "i" => Some(Placeholder::TempIdent(number)),
            _ => None,
        },
        _ => None,
    }?;

    Some(Directive::Placeholder(ident))
}

fn macro_start(lex: &mut Lexer<Kind>) -> Option<Directive> {
    lazy_static! {
        static ref MACRO_START_REGEX: Result<Regex, regex::Error> =
            Regex::new(r"@macro(?:/(?P<flags>[i]*))?");
    }

    let slice = lex.slice();
    let regex = MACRO_START_REGEX.as_ref().ok()?;
    let captures: Captures = regex.captures(slice)?;

    let flags = captures.name("flags");
    if flags.is_none() {
        return Some(Directive::Macro(MacroModifier::empty()));
    }

    let flags = flags.unwrap();
    let modifiers = flags
        .as_str()
        .chars()
        .into_iter()
        .map(|char| match char {
            'i' => MacroModifier::CaseInsensitive,
            _ => MacroModifier::empty(),
        })
        .fold(MacroModifier::empty(), |a, b| a | b);

    Some(Directive::Macro(modifiers))
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
    #[token("+", | _ | Op::Plus)]
    #[token("-", | _ | Op::Minus)]
    #[token("*", | _ | Op::Star)]
    #[token("/", | _ | Op::Slash)]
    Op(Op),

    #[token("...")]
    Ellipsis,

    #[regex("[_a-zA-Z][_a-zA-Z0-9]*", |lex| String::from(lex.slice()))]
    Ident(String),

    #[regex("[0-9]+", | v | v.slice().parse())]
    Number(UInt),

    #[token("while", ignore(case) callback = |_| Keyword::While)]
    #[token("loop", ignore(case) callback = |_| Keyword::Loop)]
    #[token("fn", ignore(case) callback = |_| Keyword::Fn)]
    #[token("decl", ignore(case) callback = |_| Keyword::Decl)]
    #[token("end", ignore(case) callback = |_| Keyword::End)]
    Keyword(Keyword),

    #[regex(r"@macro(/[i]*)?", macro_start)]
    #[token("@sub", | _ | Directive::Sub)]
    #[token("@end", | _ | Directive::End)]
    #[token("@if", | _ | Directive::If)]
    #[token("@else", | _ | Directive::Else)]
    #[regex(r"%[iavetco_]\.[0-9]+", placeholder)]
    #[regex(r"\$(i)\.[0-9]+", placeholder)]
    Directive(Directive),

    #[token("=")]
    #[token(":=")]
    Assign,

    #[token("->")]
    Into,

    #[token(",")]
    Comma,

    #[token("(", | _ | Pair::Left)]
    #[token(")", | _ | Pair::Right)]
    Paren(Pair),

    #[token("{", | _ | Pair::Left)]
    #[token("}", | _ | Pair::Right)]
    Brace(Pair),

    #[token("==", | _ | Comp::Equal)]
    #[token("!=", | _ | Comp::NotEqual)]
    #[token(">", | _ | Comp::GreaterThan)]
    #[token(">=", | _ | Comp::GreaterEqual)]
    #[token("<", | _ | Comp::LessThan)]
    #[token("<=", | _ | Comp::LessEqual)]
    Comp(Comp),

    #[token("#", line_comment)]
    #[token("###", block_comment)]
    Comment,

    #[regex(r"[ \t\f]+")]
    Whitespace,

    #[regex(r";?\n")]
    Separator,

    #[error]
    Error,
}

impl Kind {
    pub fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment | Self::Separator)
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Whitespace => "_".into(),
                Self::Ident(m) => format!("ident<‘{}‘>", m),
                Self::Comment => "comment".into(),
                Self::Op(op) => format!("{}", op),
                Self::Ellipsis => "‘...‘".into(),
                Self::Number(n) => format!("number<‘{}‘>", n),
                Self::Keyword(keyword) => format!("{}", keyword),
                Self::Directive(directive) => format!("{}", directive),
                Self::Assign => "‘:=‘".into(),
                Self::Comp(comp) => format!("{}", comp),
                Self::Separator => "sep\n".into(),
                Self::Into => "‘->‘".into(),
                Self::Comma => "‘,‘".into(),
                Self::Paren(Pair::Left) => "‘(‘".into(),
                Self::Paren(Pair::Right) => "‘)‘".into(),
                Self::Brace(Pair::Left) => "‘{‘".into(),
                Self::Brace(Pair::Right) => "‘}‘".into(),
                Self::Error => "an unrecognized token".into(),
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
            Kind::Directive(Directive::Macro(MacroModifier::empty())),
        );
        check_single_kind(
            "@macro/i",
            Kind::Directive(Directive::Macro(MacroModifier::CaseInsensitive)),
        );

        check_single_kind("@sub", Kind::Directive(Directive::Sub));
        check_single_kind("@end", Kind::Directive(Directive::End));

        check_single_kind("@if", Kind::Directive(Directive::If));
        check_single_kind("@else", Kind::Directive(Directive::Else));

        check_single_kind(
            "%i.1",
            Kind::Directive(Directive::Placeholder(Placeholder::Ident(1))),
        );
        check_single_kind(
            "%a.1",
            Kind::Directive(Directive::Placeholder(Placeholder::Atom(1))),
        );
        check_single_kind(
            "%v.1",
            Kind::Directive(Directive::Placeholder(Placeholder::Value(1))),
        );
        check_single_kind(
            "%e.1",
            Kind::Directive(Directive::Placeholder(Placeholder::Expr(1))),
        );
        check_single_kind(
            "%t.1",
            Kind::Directive(Directive::Placeholder(Placeholder::Terms(1))),
        );
        check_single_kind(
            "%c.1",
            Kind::Directive(Directive::Placeholder(Placeholder::Comp(1))),
        );
        check_single_kind(
            "%o.1",
            Kind::Directive(Directive::Placeholder(Placeholder::Op(1))),
        );
        check_single_kind(
            "%_.1",
            Kind::Directive(Directive::Placeholder(Placeholder::Any(1))),
        );
        check_single_kind(
            "$i.1",
            Kind::Directive(Directive::Placeholder(Placeholder::TempIdent(1))),
        );
    }

    #[test]
    fn lex_assign() {
        check_single_kind(":=", Kind::Assign);
    }

    #[test]
    fn lex_comp() {
        check_single_kind("=", Kind::Comp(Comp::Equal));
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
        assert_eq!(token.kind, Kind::Separator);
        assert_eq!(token.content, "\n");
    }

    #[test]
    fn lex_whitespace() {
        check_single_kind("     \t    ", Kind::Whitespace);
    }

    #[test]
    fn lex_sep() {
        check_single_kind(";\n", Kind::Separator);
        check_single_kind("\n", Kind::Separator);
    }
}
//endregion
