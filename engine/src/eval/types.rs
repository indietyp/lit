use std::collections::HashMap;

use num_bigint::BigUint;

pub type Variables = HashMap<String, BigUint>;
pub type ChangeSet = Vec<String>;
pub type ExecutionResult = (usize, ChangeSet);
