use crate::ast::context::CompileContext;
use crate::ast::expr::Expr;
use crate::ast::hir::func::inline::Inline;
use crate::ast::hir::func::structs::funcname::FuncName;
use crate::ast::hir::func::{utils, FuncCall};
use crate::errors::StdResult;
use crate::types::LineNo;

pub fn lower_call(
    context: &mut CompileContext,
    lno: LineNo,
    lhs: Expr,
    rhs: FuncCall,
) -> StdResult<Expr> {
    let module = context.get_current_frame().clone().module;

    let module_ctx = context.modules.get(&module).map_or(
        Err(utils::could_not_find_module(Some(lno), &module)),
        |ctx| Ok(ctx),
    )?;

    let func_name: FuncName = rhs.get_ident()?.into();
    let func_ctx = module_ctx.get(&func_name).map_or(
        Err(utils::could_not_find_function(
            Some(lno),
            &module,
            &func_name,
        )),
        |f| Ok(f),
    )?;

    let expr = func_ctx.inline(context, &module)?;

    todo!()
}
