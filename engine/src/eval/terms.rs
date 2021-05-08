use crate::ast::control::Control;
use crate::ast::node::Node;
use crate::eval::exec::Exec;
use crate::eval::types::{ChangeSet, Variables};
#[cfg(feature = "cli")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "cli", derive(JsonSchema))]
pub struct TermsExec {
    terms: Vec<Exec>,

    ptr: usize,
}

impl TermsExec {
    pub fn step(&mut self, locals: &mut Variables) -> Option<(usize, ChangeSet)> {
        // try single one, until exhausted, then increment ptr and just re-call ourselves
        if self.ptr >= self.terms.len() {
            return None;
        }

        let term = self.terms.get_mut(self.ptr).unwrap();
        let result = term.step(locals);

        if result.is_none() {
            self.ptr += 1;
            return self.step(locals);
        }

        result
    }

    pub fn new(node: Node) -> Self {
        match node {
            Node::Control(Control::Terms(terms)) => TermsExec {
                terms: terms.iter().map(|term| Exec::new(term.clone())).collect(),
                ptr: 0,
            },
            _ => unreachable!(),
        }
    }

    pub fn renew(&self) -> Self {
        TermsExec {
            terms: self.terms.iter().map(|term| term.renew()).collect(),
            ptr: 0,
        }
    }
}
