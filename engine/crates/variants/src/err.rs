// TODO: consider using the token instead of the lno

use crate::LineNo;
use std::error;
use std::fmt;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ErrorKindUnsupported {
    BinOp,
    Comp,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ErrorKind {
    Unsupported(ErrorKindUnsupported),
}

#[derive(Debug, Clone, Copy)]
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
