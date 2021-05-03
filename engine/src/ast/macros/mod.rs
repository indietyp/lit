mod comp;
mod expand;

use num_bigint::BigUint;

use crate::ast::context::CompileContext;
use crate::ast::control::Control;
use crate::ast::node::Node;
use crate::ast::polluted::PollutedNode;
use crate::ast::verbs::OperatorVerb;

use crate::ast::macros::expand::{
    expand_assign_to_ident, expand_assign_to_ident_binop_ident,
    expand_assign_to_ident_extbinop_value, expand_assign_to_value, expand_assign_to_zero,
    expand_comp, expand_if,
};
use crate::types::LineNo;
use crate::utils::private_identifier;
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
    AssignToOpIdent {
        lno: LineNo,
        lhs: Box<Node>,
        rhs: MacroAssign,
    },
    AssignToOpExtValue {
        lno: LineNo,
        lhs: Box<Node>,
        rhs: MacroAssign,
    },
    If {
        lno: LineNo,
        comp: Box<Node>,
        terms: Box<PollutedNode>,
    },
    IfElse {
        lno: LineNo,
        comp: Box<Node>,
        if_terms: Box<PollutedNode>,
        else_terms: Box<PollutedNode>,
    },
}

impl Macro {
    pub fn expand(&self, context: &mut CompileContext) -> Node {
        match self {
            Macro::AssignToIdent { lno, lhs, rhs } => {
                expand_assign_to_ident(lno.clone(), context, lhs, rhs)
            }
            Macro::AssignToZero { lno, lhs } => expand_assign_to_zero(lno.clone(), context, lhs),
            Macro::AssignToValue { lno, lhs, rhs } => {
                expand_assign_to_value(lno.clone(), context, lhs, rhs)
            }
            Macro::AssignToOpIdent { lno, lhs, rhs } => {
                expand_assign_to_ident_binop_ident(lno.clone(), context, lhs, rhs)
            }
            Macro::AssignToOpExtValue { lno, lhs, rhs } => {
                expand_assign_to_ident_extbinop_value(lno.clone(), context, lhs, rhs)
            }
            Macro::If { lno, comp, terms } => expand_if(lno.clone(), context, comp, terms),
            Macro::IfElse {
                lno,
                comp,
                if_terms,
                else_terms,
            } => expand_comp(lno.clone(), context, comp, if_terms, else_terms),
        }
    }
}
