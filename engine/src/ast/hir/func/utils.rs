use crate::ast::expr::Expr;
use crate::ast::hir::func::structs::funcname::FuncName;
use crate::ast::hir::func::structs::modname::ModuleName;
use crate::ast::hir::func::structs::qualname::FuncQualName;
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

pub fn could_not_find_module(lno: Option<LineNo>, module: &ModuleName) -> Vec<Error> {
    vec![Error::new_from_code(
        lno,
        ErrorCode::CouldNotFindModule {
            module: module.join("::"),
        },
    )]
}

pub fn could_not_find_function(
    lno: Option<LineNo>,
    module: &ModuleName,
    function: &FuncName,
) -> Vec<Error> {
    vec![Error::new_from_code(
        lno,
        ErrorCode::CouldNotFindFunction {
            module: module.join("::"),
            func: function.to_string(),
        },
    )]
}

pub fn prefix_ident(qual: &FuncQualName, count: &usize, ident: &String) -> String {
    format!("_{}_{}_{}", qual.func_smol(), count, m)
}
