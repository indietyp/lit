use crate::ast::context::CompileContext;
use crate::ast::node::Node;
use crate::build::Builder;
use crate::types::LineNo;
use indoc::indoc;

fn expand_assign_to_ident(lno: LineNo, context: &CompileContext, lhs: &Node, rhs: &Node) -> Node {
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
    // this will reset the context counter
    Builder::parse_and_compile2(instruction.as_str(), *context, Some(lno))
}

fn expand_assign_to_zero(lno: LineNo, context: &CompileContext, lhs: &Node) -> Node {
    let lhs = match lhs.clone() {
        Node::Ident(m) => m,
        _ => unreachable!(),
    };

    let instruction = format!(
        indoc! {"
        LOOP {lhs} DO
            {lhs} := {lhs} - 1
        END
    "},
        lhs = lhs
    );

    Builder::parse_and_compile2(instruction.as_str(), *context, Some(lno))
}

fn expand_assign_to_value(lno: LineNo, context: &CompileContext, lhs: &Node, rhs: &Node) -> Node {
    let lhs = match lhs.clone() {
        Node::Ident(m) => m,
        _ => unreachable!(),
    };

    let rhs = match rhs.clone() {
        Node::NaturalNumber(n) => n,
        _ => unreachable!(),
    };

    let instruction = format!(
        indoc! {"
        {lhs} := 0
        {lhs} := {lhs} + {rhs}
    "},
        lhs = lhs,
        rhs = rhs.to_string()
    );

    Builder::parse_and_compile2(instruction.as_str(), *context, Some(lno))
}
