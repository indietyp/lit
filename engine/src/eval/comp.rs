use num_bigint::BigUint;
use num_traits::Zero;
#[cfg(feature = "cli")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::ast::node::Node;
use crate::ast::variant::UInt;
use crate::ast::verbs::ComparisonVerb;
use crate::eval::types::Variables;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "cli", derive(JsonSchema))]
enum ComparisonSide {
    Ident(String),
    NaturalNumber(UInt),
}

#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "cli", derive(JsonSchema))]
pub struct ComparisonExec {
    lhs: ComparisonSide,
    verb: ComparisonVerb,
    rhs: ComparisonSide,
}

impl ComparisonExec {
    pub fn new(node: Node) -> Self {
        match node {
            Node::Comparison { lhs, verb, rhs } => ComparisonExec {
                lhs: match *lhs {
                    Node::Ident(m) => ComparisonSide::Ident(m),
                    Node::NaturalNumber(m) => ComparisonSide::NaturalNumber(m),
                    _ => unreachable!(),
                },
                verb,
                rhs: match *rhs {
                    Node::Ident(m) => ComparisonSide::Ident(m),
                    Node::NaturalNumber(m) => ComparisonSide::NaturalNumber(m),
                    _ => unreachable!(),
                },
            },
            _ => unreachable!(),
        }
    }

    pub fn renew(&self) -> Self {
        ComparisonExec {
            lhs: self.lhs.clone(),
            verb: self.verb.clone(),
            rhs: self.rhs.clone(),
        }
    }
}

impl ComparisonExec {
    pub fn exec(&self, locals: &Variables) -> bool {
        let lhs = match self.lhs.clone() {
            ComparisonSide::Ident(i) => locals
                .get(i.as_str())
                .cloned()
                .unwrap_or_else(BigUint::zero),
            ComparisonSide::NaturalNumber(n) => n.0,
        };
        let rhs = match self.rhs.clone() {
            ComparisonSide::Ident(i) => locals
                .get(i.as_str())
                .cloned()
                .unwrap_or_else(BigUint::zero),
            ComparisonSide::NaturalNumber(n) => n.0,
        };

        match self.verb {
            ComparisonVerb::Equal => lhs.eq(&rhs),
            ComparisonVerb::NotEqual => lhs.ne(&rhs),
            ComparisonVerb::GreaterThan => lhs.gt(&rhs),
            ComparisonVerb::GreaterThanEqual => lhs.ge(&rhs),
            ComparisonVerb::LessThan => lhs.lt(&rhs),
            ComparisonVerb::LessThanEqual => rhs.le(&rhs),
        }
    }
}
