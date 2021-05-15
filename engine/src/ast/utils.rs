use crate::ast::expr::Expr;
use crate::errors::{Error, ErrorCode, StdResult};
use crate::types::LineNo;

pub fn unwrap_ident(
    lno: Option<LineNo>,
    val: Expr,
    fmt_err: impl Fn(&Expr) -> String,
) -> StdResult<String> {
    match val {
        Expr::Ident(m) => Ok(m),
        _ => Err(vec![Error::new_from_code(
            lno,
            ErrorCode::UnexpectedExprType {
                message: fmt_err(&val),
                expected: String::from("Ident"),
                got: val.to_string(),
            },
        )]),
    }
}
