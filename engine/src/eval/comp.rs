use num_bigint::BigUint;

use crate::ast::node::Node;

use crate::ast::verbs::ComparisonVerb;

use crate::eval::types::Variables;
use num_traits::Zero;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
enum ComparisonSide {
    Ident(String),
    NaturalNumber(BigUint),
}

#[derive(Clone, Serialize, Deserialize)]
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
            ComparisonSide::NaturalNumber(n) => n,
        };
        let rhs = match self.rhs.clone() {
            ComparisonSide::Ident(i) => locals
                .get(i.as_str())
                .cloned()
                .unwrap_or_else(BigUint::zero),
            ComparisonSide::NaturalNumber(n) => n,
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