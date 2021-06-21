use crate::Expr;

// We use our own enum instead of the one from the lexer
// to divorce the lexer and AST and give the symbols additional
// meaning
pub enum BinOpVerb {
    Plus,
    Minus,
    Multiply,
}

pub struct BinOp {
    pub kind: Vec<Kind>,

    pub lhs: Box<Expr>,
    pub verb: BinOpVerb,
    pub rhs: Box<Expr>,
}
