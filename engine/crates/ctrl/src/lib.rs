// TODO: this might need to move into variants

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "schema")]
use schemars::JsonSchema;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
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
