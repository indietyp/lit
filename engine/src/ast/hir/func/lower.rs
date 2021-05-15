use crate::ast::context::CompileContext;
use crate::ast::expr::Expr;
use crate::ast::hir::func::types::{FunctionContext, FunctionName, FunctionQualName, ModuleName};
use crate::ast::hir::func::FuncCall;
use crate::errors::{Error, ErrorCode, StdResult};
use crate::types::LineNo;
use std::collections::HashSet;

pub fn lower_call(
    context: &mut CompileContext,
    module: ModuleName,
    lno: LineNo,
    lhs: Expr,
    rhs: FuncCall,
    history: Option<HashSet<FunctionQualName>>,
) -> StdResult<Expr> {
    let history = history.unwrap_or_default();
    let module_ctx = context.modules.get(&module).map_or(
        Err(vec![Error::new_from_code(
            Some(lno),
            ErrorCode::CouldNotFindModule {
                module: module.join("::"),
            },
        )]),
        |ctx| Ok(ctx),
    )?;

    let func_name: FunctionName = rhs.get_ident()?.into();
    let func_ctx = module_ctx.get(&func_name).map_or(
        Err(vec![Error::new_from_code(
            Some(lno),
            ErrorCode::CouldNotFindFunction {
                module: module.join("::"),
                func: *func_name,
            },
        )]),
        |f| Ok(f),
    )?;

    match func_ctx {
        FunctionContext::Import(_) => {
            // refer to the actual function output, to inline or not :o
        }
        FunctionContext::Func(_) => {
            // we need to inline
        }
        FunctionContext::Inline(_) => {
            // we can already use the inline and just
            // assign the variables
        }
    }

    todo!()
}
