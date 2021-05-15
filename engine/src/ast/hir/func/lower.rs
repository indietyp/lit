use crate::ast::context::CompileContext;
use crate::ast::expr::Expr;
use crate::ast::hir::func::types::ModuleName;
use crate::ast::hir::func::FuncCall;
use crate::errors::StdResult;
use crate::types::LineNo;

pub fn lower_call(
    context: &mut CompileContext,
    module: ModuleName,
    lno: LineNo,
    lhs: Expr,
    rhs: FuncCall,
) -> StdResult<Expr> {
    todo!()
}
