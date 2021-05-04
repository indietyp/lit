mod comp;
mod expand;

use crate::ast::context::CompileContext;

use crate::ast::node::Node;
use crate::ast::polluted::PollutedNode;
use crate::ast::verbs::OperatorVerb;

use crate::ast::macros::comp::expand_cond;
use crate::ast::macros::expand::{
    expand_assign_to_ident, expand_assign_to_ident_binop_ident,
    expand_assign_to_ident_extbinop_value, expand_assign_to_value, expand_assign_to_zero,
};
use crate::errors::Error;
use crate::types::LineNo;
use serde::{Deserialize, Serialize};

// This is a shorthand for the Node::Assign,
// I would love to make this one go away, but I have no idea how.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacroAssign {
    pub lhs: Box<Node>,
    pub verb: OperatorVerb,
    pub rhs: Box<Node>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Macro {
    AssignToIdent {
        lno: LineNo,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    AssignToZero {
        lno: LineNo,
        lhs: Box<Node>,
    },
    AssignToValue {
        lno: LineNo,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    AssignToIdentBinOpIdent {
        lno: LineNo,
        lhs: Box<Node>,
        rhs: MacroAssign,
    },
    AssignToIdentExtBinOpValue {
        lno: LineNo,
        lhs: Box<Node>,
        rhs: MacroAssign,
    },
    Conditional {
        lno: LineNo,
        comp: Box<Node>,
        if_terms: Box<PollutedNode>,
        else_terms: Box<Option<PollutedNode>>,
    },
}

impl Macro {
    pub fn expand(&self, context: &mut CompileContext) -> Result<Node, Vec<Error>> {
        match self {
            Macro::AssignToIdent { lno, lhs, rhs } => {
                expand_assign_to_ident(lno.clone(), context, lhs, rhs)
            }
            Macro::AssignToZero { lno, lhs } => expand_assign_to_zero(lno.clone(), context, lhs),
            Macro::AssignToValue { lno, lhs, rhs } => {
                expand_assign_to_value(lno.clone(), context, lhs, rhs)
            }
            Macro::AssignToIdentBinOpIdent { lno, lhs, rhs } => {
                expand_assign_to_ident_binop_ident(lno.clone(), context, lhs, rhs)
            }
            Macro::AssignToIdentExtBinOpValue { lno, lhs, rhs } => {
                expand_assign_to_ident_extbinop_value(lno.clone(), context, lhs, rhs)
            }
            Macro::Conditional {
                lno,
                comp,
                if_terms,
                else_terms,
            } => expand_cond(lno.clone(), context, comp, if_terms, else_terms),
        }
    }
}
