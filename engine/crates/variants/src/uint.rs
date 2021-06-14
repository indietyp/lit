use num_bigint::{BigUint, ParseBigIntError};
use num_traits::{One, Zero};

#[cfg(feature = "schema")]
use schemars::gen::SchemaGenerator;
#[cfg(feature = "schema")]
use schemars::schema::{InstanceType, Schema, SchemaObject};
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "schema")]
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UInt(pub BigUint);

NewtypeDeref! {() pub struct UInt(pub BigUint); }
NewtypeDerefMut! {() pub struct UInt(pub BigUint); }
NewtypeFrom! {() pub struct UInt(pub BigUint); }

// a + b
NewtypeAdd! {() pub struct UInt(pub BigUint); }
NewtypeAdd! {(BigUint) pub struct UInt(pub BigUint); }

// a - b
NewtypeSub! {() pub struct UInt(pub BigUint); }
NewtypeSub! {(BigUint) pub struct UInt(pub BigUint); }

// a * b
NewtypeMul! {() pub struct UInt(pub BigUint); }
NewtypeMul! {(BigUint) pub struct UInt(pub BigUint); }

// a / b
NewtypeDiv! {() pub struct UInt(pub BigUint); }
NewtypeDiv! {(BigUint) pub struct UInt(pub BigUint); }

#[cfg(feature = "schema")]
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
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl Eq for UInt {}

impl Ord for UInt {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for UInt {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Zero for UInt {
    fn zero() -> Self {
        Self(BigUint::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl One for UInt {
    fn one() -> Self {
        Self(BigUint::one())
    }
}

impl Hash for UInt {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl FromStr for UInt {
    type Err = ParseBigIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        BigUint::from_str(s).map(Self)
    }
}
