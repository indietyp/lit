use core::option::Option;
use core::option::Option::Some;
use std::ops::Add;

use either::Either;
use indoc::indoc;
use num_bigint::BigUint;
use num_traits::Zero;

use crate::ast::context::CompileContext;
use crate::ast::control::Control;
use crate::ast::macros::expand::box_ident;
use crate::ast::macros::Macro;
use crate::ast::node::Node;
use crate::ast::polluted::PollutedNode;
use crate::ast::variant::UInt;
use crate::ast::verbs::ComparisonVerb;
use crate::build::Builder;
use crate::errors::Error;
use crate::types::LineNo;
use crate::utils::private_identifier;

fn terms_are_ok(terms: Vec<Result<Node, Vec<Error>>>) -> Result<Vec<Node>, Vec<Error>> {
    let iter = terms.iter().clone();

    let erroneous = iter.clone().filter(|res| res.is_err());

    if erroneous.clone().count() > 0 {
        Err(erroneous
            .clone()
            .flat_map(|e| e.clone().err().unwrap())
            .collect())
    } else {
        Ok(iter.clone().map(|res| res.clone().ok().unwrap()).collect())
    }
}

fn if_else_body(
    lno: LineNo,
    context: &mut CompileContext,
    terms: &mut Vec<Result<Node, Vec<Error>>>,
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
            terms: Box::new(else_terms.clone().unwrap()),
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
) -> Result<Node, Vec<Error>> {
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
        Builder::ext_parse_and_compile(instructions.join("\n").as_str(), *context, Some(lno));
    terms.push(is_not_zero);

    if_else_body(lno, context, &mut terms, tmp1, if_terms, tmp2, else_terms);

    let res = Node::Control(Control::Terms(terms_are_ok(terms)?));
    Ok(res)
}

// Macro Expansion for IF x > y THEN ... ELSE ... END
fn expand_comp_gt(
    lno: LineNo,
    context: &mut CompileContext,
    initial: Option<Vec<String>>,
    comp: Comparison,
    if_terms: &PollutedNode,
    else_terms: &Option<PollutedNode>,
) -> Result<Node, Vec<Error>> {
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
        Builder::ext_parse_and_compile(instructions.join("\n").as_str(), *context, Some(lno));
    terms.push(is_greater_than);

    if_else_body(lno, context, &mut terms, tmp2, if_terms, tmp3, else_terms);

    let res = Node::Control(Control::Terms(terms_are_ok(terms)?));
    Ok(res)
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
) -> Result<Node, Vec<Error>> {
    let mut instructions = initial.unwrap_or_default();
    let mut comp = comp;

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
        comp.lhs = Either::Right(tmp);
    }

    expand_comp_gt(lno, context, Some(instructions), comp, if_terms, else_terms)
}

// Macro Expansion for IF x < y THEN ... ELSE ... END
fn expand_comp_lt(
    lno: LineNo,
    context: &mut CompileContext,
    initial: Option<Vec<String>>,
    comp: Comparison,
    if_terms: &PollutedNode,
    else_terms: &Option<PollutedNode>,
) -> Result<Node, Vec<Error>> {
    expand_comp_gt(
        lno,
        context,
        initial,
        // just switch rhs and lhs
        Comparison::new(comp.rhs, ComparisonVerb::GreaterThan, comp.lhs),
        if_terms,
        else_terms,
    )
}

// Macro Expansion for IF x <= y THEN ... ELSE ... END
fn expand_comp_lte(
    lno: LineNo,
    context: &mut CompileContext,
    initial: Option<Vec<String>>,
    comp: Comparison,
    if_terms: &PollutedNode,
    else_terms: &Option<PollutedNode>,
) -> Result<Node, Vec<Error>> {
    expand_comp_gte(
        lno,
        context,
        initial,
        Comparison::new(comp.rhs, ComparisonVerb::GreaterThanEqual, comp.lhs),
        if_terms,
        else_terms,
    )
}

// Macro Expansion for IF x == y THEN ... ELSE ... END
fn expand_comp_eq(
    lno: LineNo,
    context: &mut CompileContext,
    comp: Comparison,
    if_terms: &PollutedNode,
    else_terms: &Option<PollutedNode>,
) -> Result<Node, Vec<Error>> {
    // This one is a bit more complicated. Constructs equal through:
    // IF x >= y THEN
    //     IF x <= y THEN
    //         if_terms
    //     ELSE
    //         else_terms
    //     END
    // ELSE
    //     else_terms
    // END

    PollutedNode::Macro(Macro::Conditional {
        lno,
        comp: Box::new(Node::Comparison {
            lhs: Box::new(
                comp.lhs
                    .clone()
                    .either(|p0| Node::NaturalNumber(UInt(p0)), Node::Ident),
            ),
            verb: ComparisonVerb::GreaterThanEqual,
            rhs: Box::new(
                comp.rhs
                    .clone()
                    .either(|p0| Node::NaturalNumber(UInt(p0)), Node::Ident),
            ),
        }),
        if_terms: Box::new(PollutedNode::Macro(Macro::Conditional {
            lno,
            comp: Box::new(Node::Comparison {
                lhs: Box::new(
                    comp.lhs
                        .either(|p0| Node::NaturalNumber(UInt(p0)), Node::Ident),
                ),
                verb: ComparisonVerb::LessThanEqual,
                rhs: Box::new(
                    comp.rhs
                        .either(|p0| Node::NaturalNumber(UInt(p0)), Node::Ident),
                ),
            }),
            if_terms: Box::new(if_terms.clone()),
            else_terms: Box::new(else_terms.clone()),
        })),
        else_terms: Box::new(else_terms.clone()),
    })
    .expand(context)
}

// IF x != y is eq, but if_terms and else_terms are switched around,
// will set a default for ELSE if not given, as it is the body (empty instructions)
fn expand_comp_neq(
    lno: LineNo,
    context: &mut CompileContext,
    comp: Comparison,
    if_terms: &PollutedNode,
    else_terms: &Option<PollutedNode>,
) -> Result<Node, Vec<Error>> {
    let if_terms = Some(if_terms.clone());
    let else_terms = else_terms
        .clone()
        .unwrap_or_else(|| PollutedNode::Control(Control::Terms(vec![])));

    expand_comp_eq(lno, context, comp, &else_terms, &if_terms)
}

// Macro Expansion IF x (> | < | >= | <= | == | !=) y THEN ... ELSE ... END
//                 IF x != 0 THEN ... ELSE ... END
pub(crate) fn expand_cond(
    lno: LineNo,
    context: &mut CompileContext,
    comp: &Node,
    if_terms: &PollutedNode,
    else_terms: &Option<PollutedNode>,
) -> Result<Node, Vec<Error>> {
    let zero = BigUint::zero();
    let (comp_lhs, comp_verb, comp_rhs) = match comp {
        Node::Comparison { lhs, verb, rhs } => (
            match *lhs.clone() {
                Node::Ident(m) => Either::Right(m),
                Node::NaturalNumber(UInt(m)) => Either::Left(m),
                _ => unreachable!(),
            },
            verb,
            match *rhs.clone() {
                Node::Ident(m) => Either::Right(m),
                Node::NaturalNumber(UInt(m)) => Either::Left(m),
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
        ComparisonVerb::Equal => expand_comp_eq(lno, context, comp, if_terms, else_terms),
        ComparisonVerb::NotEqual => expand_comp_neq(lno, context, comp, if_terms, else_terms),
    }
}
