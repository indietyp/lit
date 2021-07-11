use expr::Primitive;

use variants::LineNo;

#[derive(Debug, Clone)]
pub struct Call {
    pub lno: LineNo,

    pub ident: Primitive,
    pub args: Vec<Primitive>,
}

#[derive(Debug, Clone)]
pub struct BoundCall {
    pub lno: LineNo,

    pub lhs: Primitive,
    pub rhs: Call,
}
