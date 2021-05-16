#[cfg(feature = "cli")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::ast::context::CompileContext;
use crate::ast::expr::Expr;
use crate::ast::hir::func::lower::lower_call;
use crate::ast::hir::func::utils::unwrap_ident;
use crate::errors::StdResult;
use crate::types::LineNo;
use structs::modname::ModuleName;

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
        unwrap_ident(None, *self.ident, |expr| {
            format!("Function call expected ident, got {}", expr.to_string())
        })
    }
}

// TODO: Func recursion detection on expand
// TODO: on inline check if argument count is correct
// TODO: outline, import and inline and name collision detection.
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
    pub fn lower(
        &self,
        context: &mut CompileContext,
        module: Option<ModuleName>,
    ) -> StdResult<Expr> {
        match self {
            Func::Call { lno, lhs, rhs } => lower_call(
                context,
                module.unwrap_or(ModuleName::main()),
                *lno,
                *lhs.clone(),
                rhs.clone(),
            ),
        }
    }
}
