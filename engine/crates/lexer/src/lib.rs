#[macro_use]
extern crate bitflags;

use std::convert::TryFrom;

use combine::stream::{RangeStream, ResetStream};
use combine::{ParseError, Positioned, RangeStreamOnce, Stream, StreamOnce};
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
mod err;
mod kind;
mod kw;
mod op;
mod pair;

#[derive(Clone)]
pub struct Lexer<'a> {
    lexer: logos::Lexer<'a, Kind>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: Kind::lexer(input),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.lexer.next()?;
        let content = self.lexer.slice();

        let range = {
            let Span { start, end } = self.lexer.span();

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
