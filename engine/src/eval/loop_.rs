use num_bigint::BigUint;

use crate::ast::control::Control;
use crate::ast::node::Node;
use crate::eval::traits::Executable;
use crate::eval::types::{ChangeSet, Variables};
use crate::types::LineNo;

pub struct LoopExec {
    lno: LineNo,
    ident: String,
    terms: Box<dyn Executable>,

    init: bool,

    cur: BigUint,
    iters: BigUint,
}

impl Executable for LoopExec {
    fn step(&mut self, locals: &mut Variables) -> Option<(usize, ChangeSet)> {
        // if not init copy the iteration count into our state
        if !self.init {
            // count setting our own value as a step -> for introspection;
            self.iters = locals
                .get(self.ident.as_str())
                .unwrap_or(&BigUint::zero())
                .clone();

            return Some((self.lno.0, vec![String::from("<internal variable>")]));
        }

        // means we have run our course
        if self.cur >= self.iters {
            return None;
        }

        let result = self.terms.step(locals);
        if result.is_none() {
            self.terms = self.terms.renew();
            self.cur += 1;

            return self.step(locals);
        }

        return result;
    }

    fn new(node: Node) -> Self {
        match node {
            Node::Control(Control::Loop { lno, ident, terms }) => LoopExec {
                ident: match ident {
                    Node::Ident(m) => m,
                    _ => unreachable!(),
                },
                terms: terms.executable(),
                lno,
                init: false,
                cur: BigUint::zero(),
                iters: BigUint::zero(),
            },
            _ => unreachable!(),
        }
    }

    fn renew(&self) -> Self {
        LoopExec {
            ident: self.ident.clone(),
            terms: self.terms.renew(),
            lno: self.lno,
            init: false,
            cur: BigUint::zero(),
            iters: BigUint::zero(),
        }
    }
}
