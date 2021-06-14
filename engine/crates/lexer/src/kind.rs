use crate::comp::Comp;
use crate::dir::{Directive, MacroModifier, Placeholder};
use crate::op::Op;

use lazy_static::lazy_static;
use logos::{Lexer, Logos};
use regex::{Captures, Regex};
use std::{fmt, panic};
use variants::uint::UInt;

fn comp(lex: &mut Lexer<Kind>) -> std::thread::Result<Comp> {
    let slice = lex.slice();
    panic::catch_unwind(|| slice.into())
}

fn template(lex: &mut Lexer<Kind>) -> Option<Directive> {
    lazy_static! {
        static ref TEMPLATE_REGEX: Result<Regex, regex::Error> =
            Regex::new(r"(% | \$)([_a-z])\.([0-9])");
    }

    let slice = lex.slice();

    let regex = TEMPLATE_REGEX.as_ref().ok()?;
    let captures: Captures = regex.captures(slice)?;

    let scope = captures.get(0)?.as_str();
    let type_ = captures.get(1)?.as_str();
    let number: u32 = captures.get(2)?.as_str().parse().ok()?;

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
        return Some(Directive::MacroStart(MacroModifier::empty()));
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

    Some(Directive::MacroStart(modifiers))
}

#[derive(Debug, Clone, PartialEq, Logos)]
pub enum Kind {
    #[token("+", |_| Some(Op::Plus))]
    #[token("-", |_| Some(Op::Minus))]
    #[token("*", |_| Some(Op::Star))]
    #[token("/", |_| Some(Op::Slash))]
    Op(Op),

    #[token("...")]
    Ellipsis,

    #[regex("[_a-zA-Z][_a-zA-Z0-9]*")]
    Ident,

    #[regex("[0-9]+", |v| v.slice().parse())]
    Number(UInt),

    #[regex("[wH][hH][iI][lL][eE]")]
    WhileKw,

    #[regex("[lL][oO][oO][pP]")]
    LoopKw,

    #[regex(r"@macro(/[i]*)?", macro_start)]
    #[token("@sub", |_| Directive::SubStart)]
    #[token("@end", |_| Directive::End)]
    #[token("@if", |_| Directive::If)]
    #[token("@else", |_| Directive::Else)]
    #[regex(r"%(i | a | v | e | t | c | o | _)\.[0-9]+", template)]
    #[regex(r"\$(i)\.[0-9]+", template)]
    Directive(Directive),

    #[token(":=")]
    Assign,

    #[regex("(== | != | > | >= | <= | < | =)", comp)]
    Comp(Comp),

    #[regex("#.*")]
    #[regex("###[.\n]*###")]
    Comment,

    #[regex(r"[ \t\n\f]+")]
    Whitespace,

    #[regex(";?\n")]
    Separator,

    #[error]
    Error,
}

impl Kind {
    pub fn is_trivia(self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment | Self::Separator)
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Whitespace => "whitespace".into(),
                Self::Ident => "identifier".into(),
                Self::Comment => "comment".into(),
                Self::Op(op) => format!("{}", op),
                Self::Ellipsis => "...".into(),
                Self::Number(n) => "number".into(),
                Self::WhileKw => "while".into(),
                Self::LoopKw => "loop".into(),
                Self::Directive(directive) => format!("{}", directive),
                Self::Assign => ":=".into(),
                Self::Comp(comp) => format!("{}", comp),
                Self::Separator => "sep".into(),
                Self::Error => "an unrecognized token".into(),
            }
        )
    }
}

//region Tests
#[cfg(test)]
mod tests {
    use super::*;
}
//endregion
