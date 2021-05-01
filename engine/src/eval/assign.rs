use crate::ast::node::Node;
use crate::eval::op::BinaryOpExec;
use crate::eval::types::{ChangeSet, Variables};
use crate::types::LineNo;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct AssignExec {
    lhs: String, // always an ident
    rhs: BinaryOpExec,

    lno: LineNo,
    exhausted: bool,
}

impl AssignExec {
    pub fn step(&mut self, locals: &mut Variables) -> Option<(usize, ChangeSet)> {
        if self.exhausted {
            return None;
        }

        let value = self.rhs.exec(locals);
        locals.insert(self.lhs.clone(), value);
        self.exhausted = true;

        Some((self.lno.0, vec![self.lhs.clone()]))
    }

    pub fn new(node: Node) -> Self {
        match node {
            Node::Assign { lhs, rhs, lno } => AssignExec {
                lhs: match *lhs {
                    Node::Ident(m) => m,
                    _ => unreachable!(),
                },
                rhs: BinaryOpExec::new(*rhs.clone()),
                lno,
                exhausted: false,
            },
            _ => unreachable!(),
        }
    }

    pub fn renew(&self) -> Self {
        AssignExec {
            lhs: self.lhs.clone(),
            rhs: self.rhs.renew(),
            lno: self.lno,
            exhausted: false,
        }
    }
}
