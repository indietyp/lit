use either::Either;
use itertools::Itertools;

use crate::ast::context::CompileContext;
use crate::ast::expr::Expr;
use crate::ast::hir::Hir;
use crate::errors::{Error, ErrorCode, StdResult, StrictModeViolation};
use crate::flags::CompileFlags;
use crate::types::LineNo;

pub fn priv_ident(context: &mut CompileContext) -> String {
    let mut id = String::new();
    id.push('_');
    id.push_str(context.incr().to_string().as_str());

    id
}

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn check_errors<T: Clone>(maybe: &[Result<T, Vec<Error>>]) -> Result<Vec<T>, Vec<Error>> {
    let (ok, err): (Vec<_>, Vec<_>) = maybe.iter().partition_map(|r| match r {
        Ok(r) => Either::Left(r.clone()),
        Err(r) => Either::Right(r.clone()),
    });
    let err: Vec<_> = err.iter().flat_map(|f| f.clone()).collect_vec();

    if !err.is_empty() {
        Err(err)
    } else {
        Ok(ok)
    }
}

pub(crate) fn box_hir_ident(ident: String) -> Box<Hir> {
    Box::new(Hir::Expr(Expr::Ident(ident)))
}

pub(crate) fn box_expr_ident(ident: String) -> Box<Expr> {
    Box::new(Expr::Ident(ident))
}

/// Utility function to check if a violation is triggered,
/// if that is the case error out, if everything is fine just return nothing.
/// This is so that we can easily use ?
pub(crate) fn check_strict_flag(
    lno: Option<LineNo>,
    context: &mut CompileContext,
    flag: CompileFlags,
    violation: StrictModeViolation,
) -> StdResult<()> {
    if context.flags.contains(flag) {
        Err(vec![Error::new_from_code(
            lno,
            ErrorCode::StrictModeViolation { violation },
        )])
    } else {
        Ok(())
    }
}
