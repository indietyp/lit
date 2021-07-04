use crate::{Expr, Primitive};
use lexer::Token;
use variants::LineNo;

pub struct Assign {
    pub lno: LineNo,

    pub lhs: Primitive,
    pub rhs: Box<Expr>,
}
