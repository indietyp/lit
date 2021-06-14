#[macro_use]
extern crate bitflags;

use std::convert::TryFrom;

use logos::{Logos, Span};
use text_size::{TextRange, TextSize};

pub use crate::comp::Comp;
pub use crate::dir::{Directive, MacroModifier, Placeholder};
pub use crate::kind::Kind;
pub use crate::kw::Keyword;
pub use crate::op::Op;
pub use crate::pair::Pair;

mod comp;
mod dir;
mod kind;
mod kw;
mod op;
mod pair;

pub struct Lexer<'a>(logos::Lexer<'a, Kind>);

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self(Kind::lexer(input))
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.0.next()?;
        let content = self.0.slice();

        let range = {
            let Span { start, end } = self.0.span();

            let start = TextSize::try_from(start).unwrap();
            let end = TextSize::try_from(end).unwrap();

            TextRange::new(start, end)
        };

        Some(Self::Item {
            kind,
            content,
            range,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a> {
    pub kind: Kind,
    pub content: &'a str,
    pub range: TextRange,
}
