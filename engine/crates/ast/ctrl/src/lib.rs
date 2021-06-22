// TODO: should this use kind?!

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "schema")]
use schemars::JsonSchema;
use variants::LineNo;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum Control<TNode> {
    Terms {
        lno: LineNo,

        terms: Vec<TNode>,
    },
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
