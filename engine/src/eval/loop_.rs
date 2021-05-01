use num_bigint::BigUint;

use crate::ast::control::Control;
use crate::ast::node::Node;
use crate::eval::exec::Exec;
use crate::eval::types::{ChangeSet, Variables};
use crate::types::LineNo;
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use std::ops::AddAssign;

#[derive(Serialize, Deserialize)]
pub struct LoopExec {
    lno: LineNo,
    ident: String,
    terms: Box<Exec>,

    init: bool,

    cur: BigUint,
    iters: BigUint,
}

impl LoopExec {
    pub fn step(&mut self, locals: &mut Variables) -> Option<(usize, ChangeSet)> {
        // if not init copy the iteration count into our state
        if !self.init {
            // count setting our own value as a step -> for introspection;
            self.iters = locals
                .get(self.ident.as_str())
                .unwrap_or(&BigUint::zero())
                .clone();
            self.init = true;

            return Some((self.lno.0, vec![String::from("<internal variable>")]));
        }

        // means we have run our course
        if self.cur >= self.iters {
            return None;
        }

        let result = self.terms.step(locals);
        if result.is_none() {
            self.terms = Box::new(self.terms.renew());
            self.cur.add_assign(1u32);

            return self.step(locals);
        }

        result
    }

    pub fn new(node: Node) -> Self {
        match node {
            Node::Control(Control::Loop { lno, ident, terms }) => LoopExec {
                ident: match *ident {
                    Node::Ident(m) => m,
                    _ => unreachable!(),
                },
                terms: Box::new(Exec::new(*terms)),
                lno,
                init: false,
                cur: BigUint::zero(),
                iters: BigUint::zero(),
            },
            _ => unreachable!(),
        }
    }

    pub fn renew(&self) -> Self {
        LoopExec {
            lno: self.lno,
            ident: self.ident.clone(),
            terms: Box::new(self.terms.renew()),

            init: false,
            cur: BigUint::zero(),
            iters: BigUint::zero(),
        }
    }
}
