use crate::types::LineNo;
use serde::{Deserialize, Serialize};

#[cfg(feature = "cli")]
use schemars::JsonSchema;

// Control Structures have in their body potentially
// polluted information, these need to changed/unpolluted via
// macro expansion
#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "cli", derive(JsonSchema))]
pub enum Control<TNode> {
    Terms(Vec<TNode>),
    Loop {
        lno: LineNo,
        ident: Box<TNode>,
        terms: Box<TNode>,
    },
    While {
        lno: LineNo,
        comp: Box<TNode>,
        terms: Box<TNode>,
    },
}
