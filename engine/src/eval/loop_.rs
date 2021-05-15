use num_bigint::BigUint;

use crate::ast::control::Control;
use crate::ast::expr::Expr;
use crate::ast::variant::UInt;
use crate::eval::exec::Exec;
use crate::eval::types::{ChangeSet, Variables};
use crate::types::LineNo;
use num_traits::Zero;
#[cfg(feature = "cli")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::ops::AddAssign;

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "cli", derive(JsonSchema))]
pub struct LoopExec {
    lno: LineNo,
    ident: String,
    terms: Box<Exec>,

    init: bool,

    cur: UInt,
    iters: UInt,
}

impl LoopExec {
    pub fn step(&mut self, locals: &mut Variables) -> Option<(usize, ChangeSet)> {
        // if not init copy the iteration count into our state
        if !self.init {
            // count setting our own value as a step -> for introspection;
            self.iters = UInt(
                locals
                    .get(self.ident.as_str())
                    .unwrap_or(&BigUint::zero())
                    .clone(),
            );
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

    pub fn new(node: Expr) -> Self {
        match node {
            Expr::Control(Control::Loop { lno, ident, terms }) => LoopExec {
                ident: match *ident {
                    Expr::Ident(m) => m,
                    _ => unreachable!(),
                },
                terms: Box::new(Exec::new(*terms)),
                lno,
                init: false,
                cur: UInt(BigUint::zero()),
                iters: UInt(BigUint::zero()),
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
            cur: UInt(BigUint::zero()),
            iters: UInt(BigUint::zero()),
        }
    }
}
