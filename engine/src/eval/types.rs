use num_bigint::BigUint;
use std::collections::HashMap;

pub type Variables = HashMap<String, BigUint>;
pub type ChangeSet = Vec<String>;
pub type ExecutionResult = (usize, ChangeSet);
