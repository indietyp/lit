mod expand;

use num_bigint::BigUint;
#[cfg(feature = "cli")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::ast::context::CompileContext;
use crate::ast::control::Control;
use crate::ast::func::Func;
use crate::ast::macros::Macro;
use crate::ast::node::Node;
use crate::ast::polluted::expand::{expand_loop, expand_terms, expand_while};
use crate::ast::variant::UInt;
use crate::ast::verbs::{ComparisonVerb, OperatorVerb};
use crate::errors::{Error, ErrorVariant};
use crate::flags::CompilationFlags;
use crate::utils::private_identifier;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "cli", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub enum PollutedNode {
    Pure(Node),
    Macro(Macro),
    Function(Func),

    NoOp,

    Control(Control<PollutedNode>),
}

// [REWORK] the match mess into a separate file and functions
impl PollutedNode {
    pub fn expand(&self, context: &mut CompileContext) -> Result<Node, Vec<Error>> {
        let result = match self {
            // Control Nodes
            PollutedNode::Control(Control::Terms(t)) => expand_terms(context, t)?,
            PollutedNode::Control(Control::Loop { lno, ident, terms }) => {
                expand_loop(context, *lno, ident, terms)?
            }
            PollutedNode::Control(Control::While { lno, comp, terms }) => {
                expand_while(context, *lno, comp, terms)?
            }
            PollutedNode::NoOp => Node::Control(Control::Terms(vec![])),
            PollutedNode::Pure(n) => n.clone(),
            PollutedNode::Macro(m) => m.expand(context)?,
            _ => panic!("Not implemented yet!"),
        };

        Ok(result)
    }
}
