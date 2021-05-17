#[cfg(feature = "cli")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use num_bigint::BigUint;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "cli", derive(JsonSchema))]
pub enum InternalAction {
    WhileComparison,
    LoopIteration,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "cli", derive(JsonSchema))]
pub enum ChangeLog {
    Ident(String),
    Internal(InternalAction),
}

pub type Variables = HashMap<String, BigUint>;
pub type ChangeSet = Vec<String>;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "cli", derive(JsonSchema))]
pub struct ExecutionResult(pub usize, pub Vec<ChangeLog>);
