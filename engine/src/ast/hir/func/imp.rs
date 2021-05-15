#[cfg(feature = "cli")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use either::Either;

use crate::ast::expr::Expr;
use crate::types::LineNo;

#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "cli", derive(JsonSchema))]
pub struct ImpWildcard {}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "cli", derive(JsonSchema))]
pub struct ImpFunc {
    pub ident: Box<Expr>,
    pub alias: Option<Box<Expr>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "cli", derive(JsonSchema))]
pub struct Imp {
    pub lno: LineNo,

    pub path: Vec<Expr>,
    pub funcs: Either<Vec<ImpFunc>, ImpWildcard>,
}
