use crate::ast::context::CompileContext;
use crate::ast::control::Control;
use crate::ast::macros::MacroAssign;
use crate::ast::node::Node;
use crate::ast::polluted::PollutedNode;
use crate::ast::verbs::{ComparisonVerb, OperatorVerb};
use crate::build::Builder;
use crate::types::LineNo;
use crate::utils::private_identifier;
use indoc::indoc;
use num_bigint::BigUint;
use num_traits::Zero;

fn box_ident(ident: String) -> Box<PollutedNode> {
    Box::new(PollutedNode::Pure(Node::Ident(ident)))
}

// Macro expansion for x := y
pub(crate) fn expand_assign_to_ident(
    lno: LineNo,
    context: &CompileContext,
    lhs: &Node,
    rhs: &Node,
) -> Node {
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
    Builder::parse_and_compile2(instruction.as_str(), *context, Some(lno))
}

// Macro expansion for x := 0
pub(crate) fn expand_assign_to_zero(lno: LineNo, context: &CompileContext, lhs: &Node) -> Node {
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

// Macro expansion for x := n
pub(crate) fn expand_assign_to_value(
    lno: LineNo,
    context: &CompileContext,
    lhs: &Node,
    rhs: &Node,
) -> Node {
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

// Macro expansion for x := y +/- z
fn expand_assign_to_ident_simple_ident(
    lno: LineNo,
    context: &CompileContext,
    x: String,
    y: String,
    op: OperatorVerb,
    z: String,
) -> Node {
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

    Builder::parse_and_compile2(instruction.as_str(), *context, Some(lno))
}

// Macro expansion for x := y * z
fn expand_assign_to_ident_mul_ident(
    lno: LineNo,
    context: &CompileContext,
    x: String,
    y: String,
    z: String,
) -> Node {
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

    Builder::parse_and_compile2(instruction.as_str(), *context, Some(lno))
}

// Macro expansion for x := y (+|-|*) z
pub(crate) fn expand_assign_to_ident_binop_ident(
    lno: LineNo,
    context: &CompileContext,
    lhs: &Node,
    rhs: &MacroAssign,
) -> Node {
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
) -> Node {
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

    Builder::parse_and_compile2(instruction.as_str(), *context, Some(lno))
}

// Macro expansion for x := y (*|...) n
pub(crate) fn expand_assign_to_ident_extbinop_value(
    lno: LineNo,
    context: &mut CompileContext,
    lhs: &Node,
    rhs: &MacroAssign,
) -> Node {
    let lhs = match lhs.clone() {
        Node::Ident(m) => m,
        _ => unreachable!(),
    };

    let binop_lhs = match *rhs.lhs.clone() {
        Node::Ident(m) => m,
        _ => unreachable!(),
    };

    let binop_rhs = match *rhs.rhs.clone() {
        Node::NaturalNumber(n) => n,
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

fn expand_if_not_zero(
    lno: LineNo,
    context: &mut CompileContext,
    ident: String,
    terms: &PollutedNode,
) -> Node {
    let tmp = private_identifier(context);

    let instruction = format!(
        indoc! {"
        LOOP {ident} DO
            {tmp} := 1
        END
        "},
        ident = ident,
        tmp = tmp
    );

    let is_not_zero = Builder::parse_and_compile2(instruction.as_str(), *context, Some(lno));

    // We need to build the body manually
    let body = PollutedNode::Control(Control::Loop {
        lno,
        ident: box_ident(tmp),
        terms: Box::new(terms.clone()),
    })
    .expand(context);

    Node::Control(Control::Terms(vec![is_not_zero, body]))
}

// Macro Expansion for IF ... THEN ... END
// currently we only support IF x != 0 THEN ... END
pub(crate) fn expand_if(
    lno: LineNo,
    context: &mut CompileContext,
    comp: &Node,
    terms: &PollutedNode,
) -> Node {
    let (comp_lhs, comp_verb, comp_rhs) = match comp {
        Node::Comparison { lhs, verb, rhs } => (
            match *lhs.clone() {
                Node::Ident(m) => m,
                _ => unreachable!(),
            },
            verb,
            match *rhs.clone() {
                Node::NaturalNumber(n) => n,
                _ => unreachable!(),
            },
        ),
        _ => unreachable!(),
    };

    match (comp_verb, comp_rhs) {
        (ComparisonVerb::NotEqual, rhs) if BigUint::zero().eq(&rhs) => {
            expand_if_not_zero(lno, context, comp_lhs, terms)
        }
        _ => unreachable!(),
    }
}

#[derive(new)]
struct IfElseComparison {
    lhs: String,
    verb: ComparisonVerb,
    rhs: String,
}

// Macro Expansion for IF x > y THEN ... ELSE ... END
fn expand_if_else_gt(
    lno: LineNo,
    context: &mut CompileContext,
    comp: IfElseComparison,
    if_terms: &PollutedNode,
    else_terms: &PollutedNode,
) -> Node {
    let x = comp.lhs;
    let y = comp.rhs;

    let tmp1 = private_identifier(context);
    let tmp2 = private_identifier(context);
    let tmp3 = private_identifier(context);

    let instruction = format!(
        indoc! {"
        {_1} := {x} - {y}
        {_2} := 0
        {_3} := 1

        LOOP {_1} DO
            {_2} := 1
            {_3} := 0
        END
        "},
        _1 = tmp1,
        _2 = tmp2,
        _3 = tmp3,
        x = x,
        y = y
    );

    let is_greater_than = Builder::parse_and_compile2(instruction.as_str(), *context, Some(lno));

    let if_body = PollutedNode::Control(Control::Loop {
        lno,
        ident: box_ident(tmp2),
        terms: Box::new(if_terms.clone()),
    })
    .expand(context);

    let else_body = PollutedNode::Control(Control::Loop {
        lno,
        ident: box_ident(tmp3),
        terms: Box::new(else_terms.clone()),
    })
    .expand(context);

    Node::Control(Control::Terms(vec![is_greater_than, if_body, else_body]))
}

// Macro Expansion IF x (>) y THEN ... ELSE ... END
pub(crate) fn expand_if_else(
    lno: LineNo,
    context: &mut CompileContext,
    comp: &Node,
    if_terms: &PollutedNode,
    else_terms: &PollutedNode,
) -> Node {
    let (comp_lhs, comp_verb, comp_rhs) = match comp {
        Node::Comparison { lhs, verb, rhs } => (
            match *lhs.clone() {
                Node::Ident(m) => m,
                _ => unreachable!(),
            },
            verb,
            match *rhs.clone() {
                Node::Ident(m) => m,
                _ => unreachable!(),
            },
        ),
        _ => unreachable!(),
    };

    match comp_verb {
        ComparisonVerb::GreaterThan => expand_if_else_gt(
            lno,
            context,
            IfElseComparison::new(comp_lhs, comp_verb.clone(), comp_rhs),
            if_terms,
            else_terms,
        ),
        _ => unreachable!(),
    }
}
