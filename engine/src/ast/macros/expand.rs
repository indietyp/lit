use crate::ast::context::CompileContext;
use crate::ast::node::Node;
use crate::build::Builder;
use crate::types::LineNo;
use indoc::indoc;

fn expand_assign_to_ident(lno: LineNo, lhs: &Node, rhs: &Node, context: &CompileContext) -> Node {
    let lhs = match lhs.clone() {
        Node::Ident(m) => m,
        _ => unreachable!(),
    };
    let rhs = match rhs.clone() {
        Node::Ident(m) => m,
        _ => unreachable!(),
    };

    let instruction = format! { indoc! {"
        {} := {} + 0;
    "}, lhs, rhs};

    // we loose line numbers here
    Builder::parse_and_compile(instruction.as_str(), Some(context.flags), Some(lno))
}
