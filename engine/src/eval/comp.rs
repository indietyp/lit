use num_bigint::BigUint;
use num_traits::Zero;
#[cfg(feature = "cli")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::ast::expr::Expr;
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
    pub fn new(node: Expr) -> Self {
        match node {
            Expr::Comparison { lhs, verb, rhs } => ComparisonExec {
                lhs: match *lhs {
                    Expr::Ident(m) => ComparisonSide::Ident(m),
                    Expr::NaturalNumber(m) => ComparisonSide::NaturalNumber(m),
                    _ => unreachable!(),
                },
                verb,
                rhs: match *rhs {
                    Expr::Ident(m) => ComparisonSide::Ident(m),
                    Expr::NaturalNumber(m) => ComparisonSide::NaturalNumber(m),
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
            ComparisonSide::NaturalNumber(UInt(n)) => n,
        };
        let rhs = match self.rhs.clone() {
            ComparisonSide::Ident(i) => locals
                .get(i.as_str())
                .cloned()
                .unwrap_or_else(BigUint::zero),
            ComparisonSide::NaturalNumber(UInt(n)) => n,
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
