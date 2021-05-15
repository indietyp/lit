#[cfg(feature = "cli")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::ast::expr::Expr;
use crate::ast::hir::Hir;
use crate::ast::utils::unwrap_ident;
use crate::errors::StdResult;
use crate::types::LineNo;
use crate::utils::check_errors;

#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "cli", derive(JsonSchema))]
pub struct FuncDecl {
    pub lno: LineNo,

    pub ident: Box<Expr>,
    pub params: Vec<Expr>,
    pub ret: Box<Expr>,

    pub terms: Box<Hir>,
}

impl FuncDecl {
    pub fn get_ident(&self) -> StdResult<String> {
        unwrap_ident(Some(self.lno), *self.ident, |expr| {
            format!("Expected ident to be Expr::Ident, got {}", expr.to_string())
        })
    }

    pub fn get_ret(&self) -> StdResult<String> {
        unwrap_ident(Some(self.lno), *self.ret, |expr| {
            format!(
                "Expected ret to be Expr::Ident, got {}",
                self.ident.to_string()
            )
        })
    }

    pub fn get_params(&self) -> StdResult<Vec<String>> {
        let params: Vec<_> = self
            .params
            .into_iter()
            .enumerate()
            .map(|(idx, expr)| {
                unwrap_ident(Some(self.lno), expr, |expr| {
                    format!(
                        "Expected param {} to be Expr::Ident, got {}",
                        idx,
                        expr.to_string()
                    )
                })
            })
            .collect();

        check_errors(&params)
    }
}
