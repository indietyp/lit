use indoc::indoc;
use num_bigint::BigUint;

use crate::ast::context::CompileContext;
use crate::ast::macros::MacroAssign;
use crate::ast::node::Node;
use crate::ast::polluted::PollutedNode;
use crate::ast::variant::UInt;
use crate::ast::verbs::OperatorVerb;
use crate::build::Builder;
use crate::errors::Error;
use crate::types::LineNo;
use crate::utils::private_identifier;

pub(crate) fn box_ident(ident: String) -> Box<PollutedNode> {
    Box::new(PollutedNode::Pure(Node::Ident(ident)))
}

// Macro expansion for x := y
pub(crate) fn expand_assign_to_ident(
    lno: LineNo,
    context: &CompileContext,
    lhs: &Node,
    rhs: &Node,
) -> Result<Node, Vec<Error>> {
    let lhs = match lhs.clone() {
        Node::Ident(m) => m,
        _ => unreachable!(),
    };
    let rhs = match rhs.clone() {
        Node::Ident(m) => m,
        _ => unreachable!(),
    };

    let instruction = format! { indoc! {"
        {} := {} + 0
        "}, lhs, rhs};

    // we loose line numbers here
    // this will reset the context counter
    Builder::ext_parse_and_compile(instruction.as_str(), *context, Some(lno))
}

// Macro expansion for x := 0
pub(crate) fn expand_assign_to_zero(
    lno: LineNo,
    context: &CompileContext,
    lhs: &Node,
) -> Result<Node, Vec<Error>> {
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

    Builder::ext_parse_and_compile(instruction.as_str(), *context, Some(lno))
}

// Macro expansion for x := n
pub(crate) fn expand_assign_to_value(
    lno: LineNo,
    context: &CompileContext,
    lhs: &Node,
    rhs: &Node,
) -> Result<Node, Vec<Error>> {
    let lhs = match lhs.clone() {
        Node::Ident(m) => m,
        _ => unreachable!(),
    };

    let rhs = match rhs.clone() {
        Node::NaturalNumber(UInt(n)) => n,
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

    Builder::ext_parse_and_compile(instruction.as_str(), *context, Some(lno))
}

// Macro expansion for x := y +/- z
fn expand_assign_to_ident_simple_ident(
    lno: LineNo,
    context: &CompileContext,
    x: String,
    y: String,
    op: OperatorVerb,
    z: String,
) -> Result<Node, Vec<Error>> {
    let instruction = format!(
        indoc! {"
        {x} := {y}
        LOOP {b} DO
            {x} := {x} {op} 1
        END
        "},
        x = x,
        y = y,
        op = op.display(),
        b = z
    );

    Builder::ext_parse_and_compile(instruction.as_str(), *context, Some(lno))
}

// Macro expansion for x := y * z
fn expand_assign_to_ident_mul_ident(
    lno: LineNo,
    context: &CompileContext,
    x: String,
    y: String,
    z: String,
) -> Result<Node, Vec<Error>> {
    let instruction = format!(
        indoc! {"
        {x} := 0
        LOOP {y} DO
            {x} := {x} + {z}
        END
        "},
        x = x,
        y = y,
        z = z
    );

    Builder::ext_parse_and_compile(instruction.as_str(), *context, Some(lno))
}

// Macro expansion for x := y (+|-|*) z
pub(crate) fn expand_assign_to_ident_binop_ident(
    lno: LineNo,
    context: &CompileContext,
    lhs: &Node,
    rhs: &MacroAssign,
) -> Result<Node, Vec<Error>> {
    let lhs = match lhs.clone() {
        Node::Ident(m) => m,
        _ => unreachable!(),
    };

    let binop_lhs = match *rhs.lhs.clone() {
        Node::Ident(m) => m,
        _ => unreachable!(),
    };

    let binop_rhs = match *rhs.rhs.clone() {
        Node::Ident(m) => m,
        _ => unreachable!(),
    };

    let binop_op = rhs.verb.clone();

    match binop_op {
        OperatorVerb::Multiply => {
            expand_assign_to_ident_mul_ident(lno, context, lhs, binop_lhs, binop_rhs)
        }
        OperatorVerb::Plus | OperatorVerb::Minus => {
            expand_assign_to_ident_simple_ident(lno, context, lhs, binop_lhs, binop_op, binop_rhs)
        }
    }
}

// Macro expansion for x := y * n
fn expand_assign_to_ident_mul_value(
    lno: LineNo,
    context: &mut CompileContext,
    x: String,
    y: String,
    n: BigUint,
) -> Result<Node, Vec<Error>> {
    let tmp = private_identifier(context);

    let instruction = format!(
        indoc! {"
        {tmp} := {n}
        {x} := {y} * {tmp}
        "},
        x = x,
        y = y,
        n = n.to_string(),
        tmp = tmp
    );

    Builder::ext_parse_and_compile(instruction.as_str(), *context, Some(lno))
}

// Macro expansion for x := y (*|...) n
pub(crate) fn expand_assign_to_ident_extbinop_value(
    lno: LineNo,
    context: &mut CompileContext,
    lhs: &Node,
    rhs: &MacroAssign,
) -> Result<Node, Vec<Error>> {
    let lhs = match lhs.clone() {
        Node::Ident(m) => m,
        _ => unreachable!(),
    };

    let binop_lhs = match *rhs.lhs.clone() {
        Node::Ident(m) => m,
        _ => unreachable!(),
    };

    let binop_rhs = match *rhs.rhs.clone() {
        Node::NaturalNumber(UInt(n)) => n,
        _ => unreachable!(),
    };

    let binop_op = rhs.verb.clone();

    match binop_op {
        OperatorVerb::Multiply => {
            expand_assign_to_ident_mul_value(lno, context, lhs, binop_lhs, binop_rhs)
        }
        _ => unreachable!(),
    }
}
