use std::fmt::Binary;
use std::ops::{Add, Sub};

use num_bigint::BigUint;

use crate::ast::node::Node;
use crate::ast::verbs::OperatorVerb;
use crate::eval::traits::Executable;
use crate::eval::types::{ExecutionResult, Variables};
use num_traits::Zero;

pub struct BinaryOpExec {
    lhs: String,
    verb: OperatorVerb,
    rhs: BigUint,
}

impl Executable for BinaryOpExec {
    fn step(&mut self, locals: &mut Variables) -> Option<ExecutionResult> {
        // You CANNOT step into a BinaryOp, you can only exec()
        panic!("BinaryOpExec cannot step, use exec instead!");
    }

    fn new(node: Node) -> Self {
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

    fn renew(&self) -> Box<BinaryOpExec> {
        Box::new(BinaryOpExec {
            lhs: self.lhs.clone(),
            verb: self.verb.clone(),
            rhs: self.rhs.clone(),
        })
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
