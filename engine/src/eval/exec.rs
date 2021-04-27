// This is the best solution I came up with how traits are implemented and I am dying inside. lol
// I know that I should use traits to implement this, but I have major problems dealing with those
// I am getting a vtable exception with every way I tried to implement this.

use crate::ast::control::Control;
use crate::ast::node::Node;
use crate::eval::assign::AssignExec;
use crate::eval::loop_::LoopExec;
use crate::eval::terms::TermsExec;
use crate::eval::traits::Executable;
use crate::eval::types::{ExecutionResult, Variables};
use crate::eval::while_::WhileExec;

pub enum Exec {
    Assign(AssignExec),
    Terms(TermsExec),
    While(WhileExec),
    Loop(LoopExec),
}

impl Exec {
    pub fn step(&mut self, locals: &mut Variables) -> Option<ExecutionResult> {
        match self {
            Exec::Assign(exec) => exec.step(locals),
            Exec::Terms(exec) => exec.step(locals),
            Exec::While(exec) => exec.step(locals),
            Exec::Loop(exec) => exec.step(locals),
        }
    }

    pub fn new(node: Node) -> Self {
        match node {
            Node::Ident(_)
            | Node::NaturalNumber(_)
            | Node::Comparison { .. }
            | Node::BinaryOp { .. } => panic!(
                "Cannot create direct executable from Ident, NaturalNumber, BinaryOp or Comparison"
            ),
            Node::Assign { .. } => Exec::Assign(AssignExec::new(node.clone())),
            Node::Control(Control::While { .. }) => Exec::While(WhileExec::new(node.clone())),
            Node::Control(Control::Terms(_)) => Exec::Terms(TermsExec::new(node.clone())),
            Node::Control(Control::Loop { .. }) => Exec::Loop(LoopExec::new(node.clone())),
        }
    }

    pub fn renew(&self) -> Self {
        match self {
            Exec::Assign(exec) => Exec::Assign(exec.renew()),
            Exec::Terms(exec) => Exec::Terms(exec.renew()),
            Exec::While(exec) => Exec::While(exec.renew()),
            Exec::Loop(exec) => Exec::Loop(exec.renew()),
        }
    }
}
