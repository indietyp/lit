// TODO: consider using the token instead of the lno

use crate::LineNo;
use std::error;
use std::fmt;
use std::ops::Add;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ErrorKindUnsupported {
    BinOp,
    Comp,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ErrorKindInvalidToken {
    expected: String,
    got: String,
}

impl ErrorKindInvalidToken {
    pub fn new(expected: String, got: String) -> Self {
        Self { expected, got }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorKind {
    Unsupported(ErrorKindUnsupported),
    InvalidToken(ErrorKindInvalidToken),
}

#[derive(Debug, Clone)]
pub struct Error {
    pub lno: Option<LineNo>,

    pub kind: ErrorKind,
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{:#?}", self))
    }
}

impl Error {
    pub fn new_from_kind(lno: Option<LineNo>, kind: ErrorKind) -> Self {
        Self { lno, kind }
    }
}

#[derive(Debug, Clone)]
pub struct Errors(Vec<Error>);

impl Errors {
    pub fn new() -> Self {
        Errors(vec![])
    }

    pub fn push(&mut self, error: Error) {
        self.0.push(error)
    }

    fn concat(&self, slice: Self) -> Self {
        Self([&self.0[..], &slice.0[..]].concat())
    }
}

impl Add for Errors {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.concat(rhs)
    }
}

impl From<Error> for Errors {
    fn from(error: Error) -> Self {
        Errors(vec![error])
    }
}
