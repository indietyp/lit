// We do not yet say what exactly they are, we just assume that they are
// an operator, now what they do. Could be used for unary or something else.

use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Op {
    Plus,
    Minus,
    Star,
    Slash,
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Plus => "+",
            Self::Minus => "-",
            Self::Star => "*",
            Self::Slash => "/",
        })
    }
}
