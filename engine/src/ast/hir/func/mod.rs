#[cfg(feature = "cli")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::ast::context::CompileContext;
use crate::ast::expr::Expr;
use crate::ast::hir::func::lower::lower_call;
use crate::ast::hir::func::utils::unwrap_ident;
use crate::errors::{Error, ErrorCode, StdResult, StrictModeViolation};
use crate::flags::CompileFlags;
use crate::types::LineNo;
use crate::utils::check_strict_flag;

pub mod decl;
pub mod fs;
pub mod imp;
pub mod inline;
pub mod lower;
pub mod module;
pub mod structs;
pub mod utils;

#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "cli", derive(JsonSchema))]
pub struct FuncCall {
    pub ident: Box<Expr>,
    pub args: Vec<Expr>,
}

impl FuncCall {
    pub fn get_ident(&self) -> StdResult<String> {
        unwrap_ident(None, *self.ident.clone(), |expr| {
            format!("Function call expected ident, got {}", expr.to_string())
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "cli", derive(JsonSchema))]
pub enum Func {
    // Call corresponds to:
    // lhs := func(arg1, arg2, arg3, ...)
    Call {
        lno: LineNo,

        lhs: Box<Expr>,
        rhs: FuncCall,
    },
}

impl Func {
    pub fn lower(&self, context: &mut CompileContext) -> StdResult<Expr> {
        check_strict_flag(
            self.lno(),
            context,
            CompileFlags::STRCT_NO_FUNC,
            StrictModeViolation::FuncForbidden,
        )?;

        match self {
            Func::Call { lno, lhs, rhs } => lower_call(context, *lno, *lhs.clone(), rhs.clone()),
        }
    }

    /// Get the LineNo, Option<> is here explicitly allowed to have a compliant API to [`Macro`]
    /// and to allow easy further extension of code
    #[allow(clippy::unnecessary_wraps)]
    fn lno(&self) -> Option<LineNo> {
        match self {
            Func::Call { lno, .. } => Some(*lno),
        }
    }
}
