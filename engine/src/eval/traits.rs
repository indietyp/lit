use crate::ast::node::Node;
use crate::eval::types::{ExecutionResult, Variables};

pub trait Executable {
    fn step(&mut self, locals: &mut Variables) -> Option<ExecutionResult>;
    fn new(node: Node) -> Self
    where
        Self: Sized;

    fn reset(&mut self);
}
