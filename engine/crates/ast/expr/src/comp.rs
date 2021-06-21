use crate::Expr;

// We use our own enum instead of the one from the lexer
// to divorce the lexer and AST
pub enum CompVerb {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
}

pub struct Comp {
    pub kind: Vec<Kind>,

    pub lhs: Box<Expr>,
    pub verb: CompVerb,
    pub rhs: Box<Expr>,
}
