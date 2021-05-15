#[cfg(feature = "cli")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::ast::expr::Expr;
use crate::ast::hir::Hir;
use crate::errors::{Error, ErrorCode, StdResult};
use crate::types::LineNo;
use crate::utils::check_errors;
use either::Either;

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
    fn unwrap_ident(&self, val: Expr, fmt_err: impl Fn(&Expr) -> String) -> StdResult<String> {
        match val {
            Expr::Ident(m) => Ok(m),
            _ => Err(vec![Error::new_from_code(
                Some(self.lno),
                ErrorCode::UnexpectedExprType {
                    message: fmt_err(&val),
                    expected: String::from("Ident"),
                    got: val.to_string(),
                },
            )]),
        }
    }

    pub fn get_ident(&self) -> StdResult<String> {
        self.unwrap_ident(*self.ident, |expr| {
            format!("Expected ident to be Expr::Ident, got {}", expr.to_string())
        })
    }

    pub fn get_ret(&self) -> StdResult<String> {
        self.unwrap_ident(*self.ret, |expr| {
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
                self.unwrap_ident(expr, |expr| {
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

#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "cli", derive(JsonSchema))]
pub struct ImpWildcard {}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "cli", derive(JsonSchema))]
pub struct ImpFunc {
    pub ident: Box<Expr>,
    pub alias: Option<Box<Expr>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "cli", derive(JsonSchema))]
pub struct Imp {
    pub lno: LineNo,

    pub path: Vec<Expr>,
    pub funcs: Either<Vec<ImpFunc>, ImpWildcard>,
}

// Used as a container for all Module related codes, this contains:
// - imports (imp)
// - declarations (decl)
// - code (code)
// Code is replaced with NoOp if it is loaded via an import
#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "cli", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Module {
    // Import corresponds to
    // from path1::path2::path3 import (ident1 as alias1, ident2)
    // on top of the file.
    pub imp: Vec<Imp>,
    // Decl corresponds to
    // fn ident(arg1, arg2, arg3, ...) -> ret decl
    //  terms
    // end
    // always after imports
    pub decl: Vec<FuncDecl>,

    // actual code that is run on execution
    pub code: Hir,
}
