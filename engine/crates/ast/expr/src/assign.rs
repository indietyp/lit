use crate::{Expr, Primitive};
use lexer::Kind;

pub struct Assign {
    pub kind: Vec<Kind>,

    pub lhs: Primitive,
    pub rhs: Box<Expr>,
}
