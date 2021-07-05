use expr::Primitive;
use lexer::Token;
use variants::LineNo;

#[derive(Debug, Clone)]
pub struct Call {
    pub token: Token,

    pub ident: Primitive,
}

#[derive(Debug, Clone)]
pub struct BoundCall {
    pub lno: LineNo,

    pub lhs: Primitive,
    pub rhs: Call,
}
