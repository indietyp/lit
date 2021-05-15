use std::ops::Add;

use num_bigint::BigUint;
use num_traits::{CheckedSub, Zero};
#[cfg(feature = "cli")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::ast::expr::Expr;
use crate::ast::variant::UInt;
use crate::ast::verbs::OperatorVerb;
use crate::eval::types::Variables;

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "cli", derive(JsonSchema))]
pub struct BinaryOpExec {
    lhs: String,
    verb: OperatorVerb,
    rhs: UInt,
}

impl BinaryOpExec {
    pub fn new(node: Expr) -> Self {
        // The type narrowing is done already elsewhere, not _super_ clean, but good enough
        match node {
            Expr::BinaryOp { lhs, verb, rhs } => BinaryOpExec {
                lhs: match *lhs {
                    Expr::Ident(m) => m,
                    _ => unreachable!(),
                },
                verb,
                rhs: match *rhs {
                    Expr::NaturalNumber(m) => m,
                    _ => unreachable!(),
                },
            },
            _ => unreachable!(),
        }
    }

    pub fn renew(&self) -> Self {
        BinaryOpExec {
            lhs: self.lhs.clone(),
            verb: self.verb.clone(),
            rhs: self.rhs.clone(),
        }
    }
}

impl BinaryOpExec {
    pub fn exec(&self, locals: &Variables) -> BigUint {
        let lhs = locals
            .get(self.lhs.as_str())
            .unwrap_or(&BigUint::zero())
            .clone();

        match self.verb {
            OperatorVerb::Plus => lhs.add(self.rhs.0.clone()),
            OperatorVerb::Minus => lhs.checked_sub(&self.rhs).unwrap_or_else(BigUint::zero),
            OperatorVerb::Multiply => panic!("You cannot multiply in LOOP/WHILE"),
        }
    }
}
