use crate::ast::context::CompileContext;
use crate::ast::control::Control;
use crate::ast::macros::expand::box_ident;
use crate::ast::node::Node;
use crate::ast::polluted::PollutedNode;
use crate::ast::verbs::ComparisonVerb;
use crate::build::Builder;
use crate::types::LineNo;
use crate::utils::private_identifier;
use core::option::Option;
use core::option::Option::Some;
use either::Either;
use indoc::indoc;
use num_bigint::BigUint;
use num_traits::Zero;
use std::ops::Add;

fn if_else_body(
    lno: LineNo,
    context: &mut CompileContext,
    terms: &mut Vec<Node>,
    if_ident: String,
    if_terms: &PollutedNode,
    else_ident: String,
    else_terms: &Option<PollutedNode>,
) {
    // We need to build the body manually
    let if_body = PollutedNode::Control(Control::Loop {
        lno,
        ident: box_ident(if_ident),
        terms: Box::new(if_terms.clone()),
    })
    .expand(context);
    terms.push(if_body);

    if else_terms.is_some() {
        let else_body = PollutedNode::Control(Control::Loop {
            lno,
            ident: box_ident(else_ident),
            terms: Box::new(else_terms.clone().unwrap().clone()),
        })
        .expand(context);

        terms.push(else_body);
    }
}

#[derive(new, Clone)]
struct Comparison {
    lhs: Either<BigUint, String>,
    verb: ComparisonVerb,
    rhs: Either<BigUint, String>,
}

// Macro expansion for IF x != THEN ... ELSE ... END
fn expand_comp_not_zero(
    lno: LineNo,
    context: &mut CompileContext,
    initial: Option<Vec<String>>,
    comp: Comparison,
    if_terms: &PollutedNode,
    else_terms: &Option<PollutedNode>,
) -> Node {
    let mut instructions = initial.unwrap_or_default();

    let ident = {
        if comp.lhs.is_left() {
            let tmp = private_identifier(context);

            instructions.push(format!(
                "{_3} := {value}",
                _3 = tmp,
                value = comp.lhs.left().unwrap()
            ));
            tmp
        } else {
            comp.lhs.right().unwrap()
        }
    };

    let tmp1 = private_identifier(context);
    let tmp2 = private_identifier(context);
    let stmt = format!(
        indoc! {"
        {_1} := 0
        {_2} := 1

        LOOP {ident} DO
            {_1} := 1
            {_2} := 0
        END
        "},
        ident = ident,
        _1 = tmp1,
        _2 = tmp2
    );
    instructions.push(stmt);

    let mut terms = vec![];
    let is_not_zero =
        Builder::parse_and_compile2(instructions.join("\n").as_str(), *context, Some(lno));
    terms.push(is_not_zero);

    if_else_body(lno, context, &mut terms, tmp1, if_terms, tmp2, else_terms);

    Node::Control(Control::Terms(terms))
}

// Macro Expansion for IF x > y THEN ... ELSE ... END
fn expand_comp_gt(
    lno: LineNo,
    context: &mut CompileContext,
    initial: Option<Vec<String>>,
    comp: Comparison,
    if_terms: &PollutedNode,
    else_terms: &Option<PollutedNode>,
) -> Node {
    let mut instructions: Vec<String> = initial.unwrap_or_default();

    // if the value of y is a number, implicity convert it to a variable when expanding
    let x = {
        if comp.lhs.is_left() {
            let tmp = private_identifier(context);
            instructions.push(format!(
                "{_4} := {value}",
                _4 = tmp,
                value = comp.lhs.left().unwrap()
            ));
            tmp
        } else {
            comp.lhs.right().unwrap()
        }
    };

    // if the value of y is a number implicitly convert it to a variable when expanding
    let y = {
        if comp.rhs.is_left() {
            let tmp = private_identifier(context);
            instructions.push(format!(
                "{_5} := {value}",
                _5 = tmp,
                value = comp.rhs.left().unwrap()
            ));
            tmp
        } else {
            comp.rhs.right().unwrap()
        }
    };

    let tmp1 = private_identifier(context);
    let tmp2 = private_identifier(context);
    let tmp3 = private_identifier(context);

    let stmt = format!(
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
    instructions.push(stmt);

    // assemble the different terms
    let mut terms = vec![];

    let is_greater_than =
        Builder::parse_and_compile2(instructions.join("\n").as_str(), *context, Some(lno));
    terms.push(is_greater_than);

    if_else_body(lno, context, &mut terms, tmp2, if_terms, tmp3, else_terms);

    Node::Control(Control::Terms(terms))
}

// Macro Expansion for IF x >= y THEN ... ELSE ... END
// can be simplified into IF x + 1 >= y THEN ... ELSE ... END
fn expand_comp_gte(
    lno: LineNo,
    context: &mut CompileContext,
    initial: Option<Vec<String>>,
    comp: Comparison,
    if_terms: &PollutedNode,
    else_terms: &Option<PollutedNode>,
) -> Node {
    let mut instructions = initial.clone().unwrap_or_default();
    let mut comp = comp.clone();

    // if this is the case mutate x >= y into
    // x + 1 > y
    if comp.lhs.is_left() {
        // if x is a number this is super easy
        comp.lhs = Either::Left(comp.lhs.left().map(|lhs| lhs.add(1u8)).unwrap());
    } else {
        // if x is an identifier create a new instruction that just adds one to a
        // new variable and mutate comp.rhs
        let tmp = private_identifier(context);
        instructions.push(format!(
            "{_1} := {x} + 1",
            _1 = tmp,
            x = comp.lhs.clone().right().unwrap()
        ));
        comp.rhs = Either::Right(tmp);
    }

    return expand_comp_gt(
        lno,
        context,
        Some(instructions),
        comp.clone(),
        if_terms,
        else_terms,
    );
}

// Macro Expansion for IF x < y THEN ... ELSE ... END
fn expand_comp_lt(
    lno: LineNo,
    context: &mut CompileContext,
    initial: Option<Vec<String>>,
    comp: Comparison,
    if_terms: &PollutedNode,
    else_terms: &Option<PollutedNode>,
) -> Node {
    return expand_comp_gt(
        lno,
        context,
        initial,
        // just switch rhs and lhs
        Comparison::new(comp.rhs, ComparisonVerb::GreaterThan, comp.lhs),
        if_terms,
        else_terms,
    );
}

fn expand_comp_lte(
    lno: LineNo,
    context: &mut CompileContext,
    initial: Option<Vec<String>>,
    comp: Comparison,
    if_terms: &PollutedNode,
    else_terms: &Option<PollutedNode>,
) -> Node {
    return expand_comp_gte(
        lno,
        context,
        initial,
        Comparison::new(comp.rhs, ComparisonVerb::GreaterThanEqual, comp.lhs),
        if_terms,
        else_terms,
    );
}

// Macro Expansion IF x (> | < | >= | <=) y THEN ... ELSE ... END
//                 IF x != 0 THEN ... ELSE ... END
pub(crate) fn expand_cond(
    lno: LineNo,
    context: &mut CompileContext,
    comp: &Node,
    if_terms: &PollutedNode,
    else_terms: &Option<PollutedNode>,
) -> Node {
    let zero = BigUint::zero();
    let (comp_lhs, comp_verb, comp_rhs) = match comp {
        Node::Comparison { lhs, verb, rhs } => (
            match *lhs.clone() {
                Node::Ident(m) => Either::Right(m),
                Node::NaturalNumber(m) => Either::Left(m),
                _ => unreachable!(),
            },
            verb,
            match *rhs.clone() {
                Node::Ident(m) => Either::Right(m),
                Node::NaturalNumber(m) => Either::Left(m),
                _ => unreachable!(),
            },
        ),
        _ => unreachable!(),
    };

    let comp = Comparison::new(comp_lhs, comp_verb.clone(), comp_rhs.clone());
    match comp_verb {
        ComparisonVerb::GreaterThan => {
            expand_comp_gt(lno, context, None, comp, if_terms, else_terms)
        }
        ComparisonVerb::GreaterThanEqual => {
            expand_comp_gte(lno, context, None, comp, if_terms, else_terms)
        }
        ComparisonVerb::LessThan => expand_comp_lt(lno, context, None, comp, if_terms, else_terms),
        ComparisonVerb::LessThanEqual => {
            expand_comp_lte(lno, context, None, comp, if_terms, else_terms)
        }
        ComparisonVerb::NotEqual if comp_rhs.left().eq(&Some(zero)) => {
            expand_comp_not_zero(lno, context, None, comp, if_terms, else_terms)
        }
        _ => unreachable!(),
    }
}
