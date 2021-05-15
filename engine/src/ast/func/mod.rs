pub mod fs;
pub mod modctx;
pub mod modmap;
pub mod types;

use crate::ast::expr::Expr;
use crate::types::LineNo;
#[cfg(feature = "cli")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "cli", derive(JsonSchema))]
pub struct FuncCall {
    pub ident: Box<Expr>,
    pub args: Vec<Expr>,
}

// TODO: Func recursion detection on expand
// TODO: on inline check if argument count is correct
// TODO: outline, import and inline and name collision detection.
#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "cli", derive(JsonSchema))]
pub enum Func {
    // Call corresponds to:
    // lhs := func(arg1, arg2, arg3, ...)
    Call {
        lno: LineNo,

        lhs: Box<Expr>,
        rhs: FuncCall,
    },
}
