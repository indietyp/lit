mod comp;
mod lower;

use crate::ast::context::CompileContext;

use crate::ast::expr::Expr;
use crate::ast::hir::Hir;
use crate::ast::verbs::OperatorVerb;

use crate::ast::hir::macros::comp::lower_cond;
use crate::ast::hir::macros::lower::{
    lower_assign_to_ident, lower_assign_to_ident_binop_ident, lower_assign_to_ident_extbinop_value,
    lower_assign_to_value, lower_assign_to_zero,
};
use crate::errors::StdResult;
use crate::types::LineNo;
use serde::{Deserialize, Serialize};

#[cfg(feature = "cli")]
use schemars::JsonSchema;

// This is a shorthand for the Node::Assign,
// I would love to make this one go away, but I have no idea how.
#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "cli", derive(JsonSchema))]
pub struct MacroAssign {
    pub lhs: Box<Expr>,
    pub verb: OperatorVerb,
    pub rhs: Box<Expr>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "cli", derive(JsonSchema))]
pub enum Macro {
    AssignToIdent {
        lno: LineNo,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    AssignToZero {
        lno: LineNo,
        lhs: Box<Expr>,
    },
    AssignToValue {
        lno: LineNo,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    AssignToIdentBinOpIdent {
        lno: LineNo,
        lhs: Box<Expr>,
        rhs: MacroAssign,
    },
    AssignToIdentExtBinOpValue {
        lno: LineNo,
        lhs: Box<Expr>,
        rhs: MacroAssign,
    },
    Conditional {
        lno: LineNo,
        comp: Box<Expr>,
        if_terms: Box<Hir>,
        else_terms: Box<Option<Hir>>,
    },
}

impl Macro {
    pub fn lower(&self, context: &mut CompileContext) -> StdResult<Expr> {
        match self {
            Macro::AssignToIdent { lno, lhs, rhs } => {
                lower_assign_to_ident(*lno, context, lhs, rhs)
            }
            Macro::AssignToZero { lno, lhs } => lower_assign_to_zero(*lno, context, lhs),
            Macro::AssignToValue { lno, lhs, rhs } => {
                lower_assign_to_value(*lno, context, lhs, rhs)
            }
            Macro::AssignToIdentBinOpIdent { lno, lhs, rhs } => {
                lower_assign_to_ident_binop_ident(*lno, context, lhs, rhs)
            }
            Macro::AssignToIdentExtBinOpValue { lno, lhs, rhs } => {
                lower_assign_to_ident_extbinop_value(*lno, context, lhs, rhs)
            }
            Macro::Conditional {
                lno,
                comp,
                if_terms,
                else_terms,
            } => lower_cond(*lno, context, comp, if_terms, else_terms),
        }
    }
}
