use crate::{Expr, Primitive};
use variants::LineNo;

#[derive(Debug, Clone)]
pub struct Assign {
    pub lno: LineNo,

    pub lhs: Primitive,
    pub rhs: Box<Expr>,
}
