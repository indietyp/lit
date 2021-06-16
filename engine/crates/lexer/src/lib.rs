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

#[derive(Clone)]
pub struct Lexer<'a> {
    lexer: logos::Lexer<'a, Kind>,

    rel_row: TextSize,
    rel_col: TextSize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: Kind::lexer(input),

            rel_row: TextSize::from(0),
            rel_col: TextSize::from(0),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.lexer.next()?;
        let content = self.lexer.slice();

        let range = {
            let Span { start, end } = self.lexer.span();

            let start = TextSize::try_from(start).unwrap();
            let end = TextSize::try_from(end).unwrap();

            TextRange::new(start, end)
        };

        let item = Some(Self::Item {
            kind: kind.clone(),
            content: content.to_string(),

            span: range,

            row: TextRange::new(self.rel_row, self.rel_row),
            col: TextRange::new(range.start() - self.rel_col, range.end() - self.rel_col),
        });

        if matches!(kind, Kind::Newline) {
            // increase the line number manually
            self.rel_row += TextSize::from(1);
            // increase the col to the current character + 1, so that the first item on the next line
            // has the value 0
            self.rel_col += range.start() + TextSize::from(1);
        }

        item
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: Kind,
    pub content: String,

    pub span: TextRange,

    pub row: TextRange,
    pub col: TextRange,
}
