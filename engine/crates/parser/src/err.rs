use crate::stream::{Position, Range};
use crate::Token;
use combine::error::{StreamError, StringStreamError, Tracked};
use combine::ParseError;

pub struct LexerStreamError<'a> {
    pub errors: Vec<StringStreamError>,
    pub position: Position,
    pub expected: Option<StringStreamError>,
}

impl<'a> ParseError<Token<'a>, Range<'a>, Position> for LexerStreamError<'a> {
    type StreamError = StringStreamError;

    fn empty(position: usize) -> Self {
        Self {
            position,
            errors: vec![],
            expected: None,
        }
    }

    fn set_position(&mut self, position: usize) {
        self.position = position;
    }

    fn add(&mut self, err: Self::StreamError) {
        self.errors.push(err)
    }

    fn set_expected<F>(self_: &mut Tracked<Self>, info: Self::StreamError, f: F)
    where
        F: FnOnce(&mut Tracked<Self>),
    {
        if self_.info.is_none() {
            f(self_)
        }

        self_.info = info;
    }

    fn is_unexpected_end_of_input(&self) -> bool {
        if let Some(expected) = self.expected {
            matches!(expected, StringStreamError::Eoi)
        } else {
            false
        }
    }

    fn into_other<T>(self) -> T
    where
        T: ParseError<Token<'a>, Range<'a>, Position>,
    {
        let mut other = T::empty(self.position);
        for err in self.errors {
            other.add(err);
        }
        let mut tracked = Tracked::from(&other);
        T::set_expected(&mut tracked, self.expected, |_| {});

        other
    }
}
