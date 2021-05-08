use num_bigint::BigUint;
use serde::{Deserialize, Serialize};

#[cfg(feature = "cli")]
use schemars::gen::SchemaGenerator;
#[cfg(feature = "cli")]
use schemars::schema::{InstanceType, Schema, SchemaObject};
#[cfg(feature = "cli")]
use schemars::JsonSchema;
use std::cmp::Ordering;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UInt(pub BigUint);

NewtypeDeref! {() pub struct UInt(pub BigUint); }
NewtypeDerefMut! {() pub struct UInt(pub BigUint); }
NewtypeFrom! {() pub struct UInt(pub BigUint); }

#[cfg(feature = "cli")]
impl JsonSchema for UInt {
    fn schema_name() -> String {
        "Unsigned Integer".to_string()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        SchemaObject {
            instance_type: Some(InstanceType::Integer.into()),
            format: Some(String::from("uint")),
            ..Default::default()
        }
        .into()
    }
}

impl PartialEq for UInt {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn ne(&self, other: &Self) -> bool {
        self.0 != other.0
    }
}
impl Eq for UInt {}

impl Ord for UInt {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for UInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
