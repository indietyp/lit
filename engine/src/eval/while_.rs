use crate::ast::control::Control;
use crate::ast::node::Node;
use crate::eval::comp::ComparisonExec;
use crate::eval::traits::Executable;
use crate::eval::types::{ChangeSet, Variables};

pub struct WhileExec {
    comp: Box<ComparisonExec>,
    terms: Box<dyn Executable>,

    check: bool,
    exhausted: bool, // continue
}

impl Executable for WhileExec {
    fn step(&mut self, locals: &mut Variables) -> Option<(usize, ChangeSet)> {
        // A) check if true or false if check is true, set check to false
        // A.1) set exhausted if check is false and return None
        // B) exhaust current terms
        // C) if exhausted set check to true and re-step
        if self.check {
            self.exhausted = !self.comp.exec(locals)
        }

        if self.exhausted {
            return None;
        }

        let value = self.terms.step(locals);
        if value.is_none() {
            self.terms = self.terms.renew();
            self.check = true;

            return self.step(locals);
        }

        value
    }

    fn new(node: Node) -> Self {
        match node {
            Node::Control(Control::While {
                comp,
                terms,
                lno: _,
            }) => WhileExec {
                comp: Box::new(ComparisonExec::new(*comp.clone())),
                terms: terms.clone().executable(),
                check: true,
                exhausted: false,
            },
            _ => unreachable!(),
        }
    }

    fn renew(&self) -> Box<dyn Executable> {
        Box::new(WhileExec {
            comp: self.comp.renew(),
            terms: self.terms.renew(),
            check: true,
            exhausted: false,
        })
    }
}
