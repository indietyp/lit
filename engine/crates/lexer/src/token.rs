use crate::comp::Comp;
use crate::dir::{Directive, Placeholder};
use crate::op::Op;

use logos::{Lexer, Logos};
use regex::Regex;
use std::panic;

fn comp(lex: &mut Lexer<Token>) -> std::thread::Result<Comp> {
    let slice = lex.slice();
    panic::catch_unwind(|| slice.into())
}

fn template(lex: &mut Lexer<Token>) -> Option<Directive> {
    let slice = lex.slice();

    let regex = Regex::new(r"(% | \$)([_a-z])\.([0-9])").ok()?;
    let captures = regex.captures(slice)?;

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

    Some(Directive::Ident(ident))
}

#[derive(Debug, Copy, Clone, PartialEq, Logos)]
pub enum Token {
    #[token("+", |_| Some(Op::Plus))]
    #[token("-", |_| Some(Op::Minus))]
    #[token("*", |_| Some(Op::Star))]
    #[token("/", |_| Some(Op::Slash))]
    Op(Op),

    #[token("...")]
    Ellipsis,

    #[regex("[_a-zA-Z][_a-zA-Z0-9]*")]
    Ident,

    #[regex("[wH][hH][iI][lL][eE]")]
    WhileKw,

    #[regex("[lL][oO][oO][pP]")]
    LoopKw,

    #[token("@macro", |_| Directive::MacroStart)]
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
