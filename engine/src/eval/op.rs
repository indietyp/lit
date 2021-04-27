use std::fmt::Binary;
use std::ops::{Add, Sub};

use num_bigint::BigUint;

use crate::ast::node::Node;
use crate::ast::verbs::OperatorVerb;
use crate::eval::traits::Executable;
use crate::eval::types::{ExecutionResult, Variables};
use num_traits::Zero;

#[derive(Clone)]
pub struct BinaryOpExec {
    lhs: String,
    verb: OperatorVerb,
    rhs: BigUint,
}

impl BinaryOpExec {
    pub fn new(node: Node) -> Self {
        // The type narrowing is done already elsewhere, not _super_ clean, but good enough
        match node {
            Node::BinaryOp { lhs, verb, rhs } => BinaryOpExec {
                lhs: match *lhs {
                    Node::Ident(m) => m,
                    _ => unreachable!(),
                },
                verb,
                rhs: match *rhs {
                    Node::NaturalNumber(m) => m,
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
            OperatorVerb::Plus => lhs.add(self.rhs.clone()),
            OperatorVerb::Minus => BigUint::zero().min(lhs.sub(self.rhs.clone())),
            OperatorVerb::Multiply => panic!("You cannot multiply in LOOP/WHILE"),
        }
    }
}
