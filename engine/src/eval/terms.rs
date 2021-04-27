use crate::ast::control::Control;
use crate::ast::node::Node;
use crate::eval::traits::Executable;
use crate::eval::types::{ChangeSet, Variables};

pub struct TermsExec {
    terms: Vec<Box<dyn Executable>>,

    ptr: usize,
}

impl Executable for TermsExec {
    fn step(&mut self, locals: &mut Variables) -> Option<(usize, ChangeSet)> {
        // try single one, until exhausted, then increment ptr and just re-call ourselves
        if self.ptr >= self.terms.len() {
            return None;
        }

        let mut term = self.terms.get(self.ptr).unwrap();
        let result = term.step(locals);

        if result.is_none() {
            self.ptr += 1;
            return self.step(locals);
        }

        result
    }

    fn new(node: Node) -> Self {
        match node {
            Node::Control(Control::Terms(terms)) => TermsExec {
                terms: terms
                    .iter()
                    .map(|term| &term.executable().clone())
                    .collect(),
                ptr: 0,
            },
            _ => unreachable!(),
        }
    }

    fn renew(&self) -> Box<dyn Executable> {
        Box::new(TermsExec {
            terms: self
                .terms
                .iter()
                .map(|term| term.renew())
                .collect(),
            ptr: 0,
        })
    }
}
