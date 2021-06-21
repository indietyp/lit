pub struct Assign {
    pub kind: Vec<Kind>,

    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>
}