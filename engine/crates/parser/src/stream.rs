use combine::stream::{ResetStream, StreamErrorFor};
use combine::{ParseError, Positioned, RangeStreamOnce, StreamOnce};

use combine::easy::{Error, Errors};
use combine::error::{StringStreamError, UnexpectedParse};
use lexer::{Lexer, Token};

pub(crate) type Position = usize;
pub(crate) type Range = Vec<Token>;

#[derive(Debug, Clone)]
pub struct LexerStream {
    pos: Position,
    pub(crate) tokens: Range,
}

impl LexerStream {
    pub fn new(input: &str) -> Self {
        Lexer::new(input).into()
    }

    pub(crate) fn new_from_lexer(lexer: Lexer) -> Self {
        lexer.into()
    }
}

impl<'a> From<Lexer<'a>> for LexerStream {
    fn from(lexer: Lexer<'a>) -> Self {
        Self {
            pos: 0,
            // while tokenization needs to know trivia we do not.
            // This means we're constructing a lossy stream
            tokens: lexer.filter(|token| !token.kind.is_trivia()).collect(),
        }
    }
}

impl StreamOnce for LexerStream {
    type Token = Token;
    type Range = Range;
    type Position = Position;
    type Error = Errors<Self::Token, Self::Range, Self::Position>;

    fn uncons(&mut self) -> Result<Self::Token, StreamErrorFor<Self>> {
        let token = self
            .tokens
            .get(self.pos)
            .map_or_else(|| Err(Error::end_of_input()), |value| Ok(value))?;

        self.pos += 1;

        Ok(token.clone())
    }
}

impl Positioned for LexerStream {
    fn position(&self) -> Self::Position {
        self.pos
    }
}

impl ResetStream for LexerStream {
    type Checkpoint = Self::Position;

    fn checkpoint(&self) -> Self::Checkpoint {
        self.pos
    }

    fn reset(&mut self, checkpoint: Self::Checkpoint) -> Result<(), Self::Error> {
        self.pos = checkpoint;

        Ok(())
    }
}

impl RangeStreamOnce for LexerStream {
    fn uncons_range(&mut self, size: usize) -> Result<Self::Range, StreamErrorFor<Self>> {
        if self.position() + size >= self.tokens.len() {
            Err(Error::end_of_input())
        } else {
            Ok(self.tokens.as_slice()[self.pos..self.pos + size].to_vec())
        }
    }

    fn uncons_while<F>(&mut self, mut predicate: F) -> Result<Self::Range, StreamErrorFor<Self>>
    where
        F: FnMut(Self::Token) -> bool,
    {
        Ok(self
            .tokens
            .clone()
            .into_iter()
            .take_while(|value| predicate(value.clone()))
            .collect::<Vec<_>>())
    }

    fn distance(&self, end: &Self::Checkpoint) -> usize {
        end - self.pos
    }

    fn range(&self) -> Self::Range {
        self.tokens.as_slice()[self.pos..].to_vec()
    }
}
