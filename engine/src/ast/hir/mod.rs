#[cfg(feature = "cli")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use func::Func;
use macros::Macro;

use crate::ast::context::CompileContext;
use crate::ast::control::Control;
use crate::ast::expr::Expr;
use crate::ast::hir::lower::{lower_loop, lower_terms, lower_while};
use crate::errors::StdResult;

pub mod func;
pub mod lower;
pub mod macros;

/// HIR = High-Level Intermediate Representation
#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "cli", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub enum Hir {
    Expr(Expr),
    Macro(Macro),
    Function(Func),

    NoOp,

    Control(Control<Hir>),
}

impl Hir {
    pub fn lower(&self, context: &mut CompileContext) -> StdResult<Expr> {
        let result = match self {
            Hir::Control(Control::Terms(t)) => lower_terms(context, t)?,
            Hir::Control(Control::Loop { lno, ident, terms }) => {
                lower_loop(context, *lno, ident, terms)?
            }
            Hir::Control(Control::While { lno, comp, terms }) => {
                lower_while(context, *lno, comp, terms)?
            }
            Hir::NoOp => Expr::Control(Control::Terms(vec![])),
            Hir::Expr(n) => n.clone(),
            Hir::Macro(m) => m.lower(context)?,
            Hir::Function(f) => f.lower(context)?,
            _ => panic!("Not implemented yet!"),
        };

        Ok(result)
    }
}
