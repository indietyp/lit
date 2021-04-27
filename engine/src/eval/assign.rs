use crate::ast::node::Node;
use crate::eval::op::BinaryOpExec;
use crate::eval::traits::Executable;
use crate::eval::types::{ChangeSet, Variables};
use crate::types::LineNo;

pub struct AssignExec {
    lhs: String, // always an ident
    rhs: BinaryOpExec,

    lno: LineNo,
    exhausted: bool,
}

impl Executable for AssignExec {
    fn step(&mut self, locals: &mut Variables) -> Option<(usize, ChangeSet)> {
        if self.exhausted {
            return None;
        }

        let value = self.rhs.exec(locals);
        locals.insert(self.lhs.clone(), value);

        Some((self.lno.0, vec![self.lhs.clone()]))
    }

    fn new(node: Node) -> Self {
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

    fn reset(&mut self) {
        self.rhs.reset();
        self.exhausted = false;
    }
}
