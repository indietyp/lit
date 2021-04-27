use crate::ast::control::Control;
use crate::ast::node::Node;
use crate::eval::comp::ComparisonExec;
use crate::eval::exec::Exec;
use crate::eval::traits::Executable;
use crate::eval::types::{ChangeSet, Variables};

pub struct WhileExec {
    comp: ComparisonExec,
    terms: Box<Exec>,

    check: bool,
    exhausted: bool, // continue
}

impl WhileExec {
    pub fn step(&mut self, locals: &mut Variables) -> Option<(usize, ChangeSet)> {
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
            self.terms = Box::new(self.terms.renew());
            self.check = true;

            return self.step(locals);
        }

        value
    }

    pub fn new(node: Node) -> Self {
        match node {
            Node::Control(Control::While {
                comp,
                terms,
                lno: _,
            }) => WhileExec {
                comp: ComparisonExec::new(*comp.clone()),
                terms: Box::new(Exec::new(*terms.clone())),
                check: true,
                exhausted: false,
            },
            _ => unreachable!(),
        }
    }

    pub fn renew(&self) -> Self {
        WhileExec {
            comp: self.comp.renew(),
            terms: Box::new(self.terms.renew()),

            check: true,
            exhausted: false,
        }
    }
}
