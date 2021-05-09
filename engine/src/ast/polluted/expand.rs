use crate::ast::context::CompileContext;
use crate::ast::control::Control;
use crate::ast::macros::Macro;
use crate::ast::node::Node;
use crate::ast::polluted::PollutedNode;
use crate::ast::variant::UInt;
use crate::ast::verbs::{ComparisonVerb, OperatorVerb};
use crate::errors::{Error, ErrorVariant};
use crate::flags::CompilationFlags;
use crate::types::LineNo;
use crate::utils::private_identifier;
use either::Either;
use itertools::Itertools;
use num_traits::{One, Zero};

fn check_errors(maybe: &[Result<Node, Vec<Error>>]) -> Result<Vec<Node>, Vec<Error>> {
    let (ok, err): (Vec<_>, Vec<_>) = maybe.iter().partition_map(|r| match r {
        Ok(r) => Either::Left(r.clone()),
        Err(r) => Either::Right(r.clone()),
    });
    let err: Vec<_> = err.iter().flat_map(|f| f.clone()).collect_vec();

    if !err.is_empty() {
        Err(err)
    } else {
        Ok(ok)
    }
}

pub(crate) fn expand_terms(
    context: &mut CompileContext,
    terms: &[PollutedNode],
) -> Result<Node, Vec<Error>> {
    let maybe: Vec<_> = terms.iter().map(|term| term.expand(context)).collect();
    let nodes = check_errors(&maybe)?;

    Ok(Node::Control(Control::Terms(nodes)))
}

pub(crate) fn expand_loop(
    context: &mut CompileContext,
    lno: LineNo,
    ident: &PollutedNode,
    terms: &PollutedNode,
) -> Result<Node, Vec<Error>> {
    let maybe_ident = ident.expand(context);
    let maybe_terms = terms.expand(context);

    let mut maybe = vec![maybe_ident, maybe_terms];
    if !context.flags.intersects(CompilationFlags::LOOP_AND_WHILE) {
        maybe.push(Err(vec![Error::new(
            lno,
            ErrorVariant::Message(String::from(
                "Cannot use LOOP if LOOP and WHILE are not enabled!",
            )),
        )]))
    }

    let error_free = check_errors(&maybe)?;
    let (ident, terms) = match error_free.as_slice() {
        [ident, terms] => (ident, terms),
        [ident, terms, ..] => (ident, terms),
        &_ => unreachable!(),
    };

    let node = if context.flags.contains(CompilationFlags::LOOP) {
        Node::Control(Control::Loop {
            lno,
            ident: Box::new(ident.clone()),
            terms: Box::new(terms.clone()),
        })
    } else if context.flags.contains(CompilationFlags::WHILE) {
        // This rewrites the LOOP into WHILE
        // LOOP x DO
        //  ...
        // END
        // is converted to:
        // _1 := x
        // WHILE _1 != 0 DO
        //  ...
        //  _1 := _1 - 1
        // END

        let tmp = Box::new(Node::Ident(private_identifier(context)));

        Node::Control(Control::Terms(vec![
            Macro::AssignToIdent {
                lno,
                lhs: tmp.clone(),
                rhs: Box::new(ident.clone()),
            }
            .expand(context)?,
            Node::Control(Control::While {
                lno,
                comp: Box::new(Node::Comparison {
                    lhs: tmp.clone(),
                    verb: ComparisonVerb::NotEqual,
                    rhs: Box::new(Node::NaturalNumber(UInt::zero())),
                }),
                terms: Box::new(Node::Control(Control::Terms(vec![
                    terms.clone(),
                    Node::Assign {
                        lno,
                        lhs: tmp.clone(),
                        rhs: Box::new(Node::BinaryOp {
                            lhs: tmp,
                            verb: OperatorVerb::Minus,
                            rhs: Box::new(Node::NaturalNumber(UInt::one())),
                        }),
                    },
                ]))),
            }),
        ]))
    } else {
        unreachable!()
    };

    Ok(node)
}

pub(crate) fn expand_while(
    context: &mut CompileContext,
    lno: LineNo,
    comp: &PollutedNode,
    terms: &PollutedNode,
) -> Result<Node, Vec<Error>> {
    let maybe_comp = comp.expand(context);
    let maybe_terms = terms.expand(context);

    let mut maybe = vec![maybe_comp, maybe_terms];
    if !context.flags.contains(CompilationFlags::WHILE) {
        maybe.push(Err(vec![Error::new(
            lno,
            ErrorVariant::Message(String::from("Cannot replicate WHILE in LOOP mode!")),
        )]))
    }

    // always only two if succeeds (context check adds error, which will always get unwrapped)
    let error_free = check_errors(&maybe)?;
    let (comp, terms) = match error_free.as_slice() {
        [comp, terms] => (comp, terms),
        [comp, terms, ..] => (comp, terms),
        &_ => unreachable!(),
    };

    let node = Node::Control(Control::While {
        lno,
        comp: Box::new(comp.clone()),
        terms: Box::new(terms.clone()),
    });

    Ok(node)
}
