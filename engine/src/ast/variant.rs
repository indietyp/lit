use num_bigint::BigUint;
use serde::{Deserialize, Serialize};

#[cfg(feature = "cli")]
use schemars::gen::SchemaGenerator;
#[cfg(feature = "cli")]
use schemars::schema::{InstanceType, Schema, SchemaObject};
#[cfg(feature = "cli")]
use schemars::JsonSchema;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "cli", derive(JsonSchema))]
pub struct UInt(pub BigUint);

NewtypeDeref! {() pub struct UInt(pub BigUint); }
NewtypeDerefMut! {() pub struct UInt(pub BigUint); }
NewtypeFrom! {() pub struct UInt(pub BigUint); }

#[cfg(feature = "cli")]
impl JsonSchema for UInt {
    fn schema_name() -> String {
        return "UInt".to_string();
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        SchemaObject {
            instance_type: Some(InstanceType::Number.into()),
            ..Default::default()
        }
        .into()
    }
}
