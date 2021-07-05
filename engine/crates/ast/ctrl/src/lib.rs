#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{Comp, Expr, Primitive};
#[cfg(feature = "schema")]
use schemars::JsonSchema;
use variants::LineNo;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum Control<Type, Primitive, Comp> {
    Terms {
        terms: Vec<Type>,
    },
    Loop {
        lno: LineNo,

        ident: Primitive,
        terms: Box<Type>,
    },
    While {
        lno: LineNo,

        comp: Comp,
        terms: Box<Type>,
    },
}
