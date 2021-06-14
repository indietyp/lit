use combine::stream::{ResetStream, StreamErrorFor};
use combine::{ParseError, Positioned, RangeStreamOnce, StreamOnce};

use crate::err::LexerStreamError;
use combine::error::StringStreamError;
use lexer::{Lexer, Token};

pub(crate) type Position = usize;
pub(crate) type Range<'a> = Vec<Token<'a>>;

pub struct LexerStream<'a> {
    lexer: Lexer<'a>,

    pos: Position,
    tokens: Range<'a>,
}

impl<'a> LexerStream<'a> {
    pub fn new(input: &str) -> Self {
        Lexer::new(input).into()
    }

    pub(crate) fn new_from_lexer(lexer: Lexer<'a>) -> Self {
        lexer.into()
    }
}

impl<'a, 'b> From<Lexer<'b>> for LexerStream<'a> {
    fn from(lexer: Lexer<'a>) -> Self {
        Self {
            lexer: lexer.clone(),
            pos: 0,
            tokens: lexer.collect_vec(),
        }
    }
}

impl<'a> StreamOnce for LexerStream<'a> {
    type Token = Token<'a>;
    type Range = Range<'a>;
    type Position = Position;
    type Error = LexerStreamError<'a>;

    fn uncons(&mut self) -> Result<Self::Token, StreamErrorFor<Self>> {
        let token = self.tokens.get(self.pos).map_or_else(
            Err(LexerStreamError::from_error(
                self.pos,
                StringStreamError::Eoi,
            )),
            |value| Ok(value),
        )?;

        self.pos += 1;

        Ok(token.clone())
    }
}

impl<'a> Positioned for LexerStream<'a> {
    fn position(&self) -> Self::Position {
        self.pos
    }
}

impl<'a> ResetStream for LexerStream<'a> {
    type Checkpoint = Self::Position;

    fn checkpoint(&self) -> Self::Checkpoint {
        self.pos
    }

    fn reset(&mut self, checkpoint: Self::Checkpoint) -> Result<(), Self::Error> {
        self.position = checkpoint;

        Ok(())
    }
}

impl<'a> RangeStreamOnce for LexerStream<'a> {
    fn uncons_range(&mut self, size: usize) -> Result<Self::Range, StreamErrorFor<Self>> {
        if self.position() + size >= self.tokens.len() {
            Err(())
        } else {
            Ok(self.tokens.as_slice().clone()[self.pos..self.pos + size].to_vec())
        }
    }

    fn uncons_while<F>(&mut self, predicate: F) -> Result<Self::Range, StreamErrorFor<Self>>
    where
        F: FnMut(Self::Token) -> bool,
    {
        Ok(self
            .tokens
            .into_iter()
            .take_while(|item| predicate(item.clone()))
            .into())
    }

    fn distance(&self, end: &Self::Checkpoint) -> usize {
        end - self.pos
    }

    fn range(&self) -> Self::Range {
        self.tokens.as_slice()[self.pos..].to_vec()
    }
}
