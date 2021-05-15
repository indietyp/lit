use num_traits::{One, Zero};

use crate::ast::context::CompileContext;
use crate::ast::control::Control;
use crate::ast::expr::Expr;
use crate::ast::hir::macros::Macro;
use crate::ast::hir::Hir;
use crate::ast::variant::UInt;
use crate::ast::verbs::{ComparisonVerb, OperatorVerb};
use crate::errors::{Error, ErrorVariant, StdResult};
use crate::flags::CompileFlags;
use crate::types::LineNo;
use crate::utils;
use crate::utils::private_identifier;

pub(crate) fn lower_terms(context: &mut CompileContext, terms: &[Hir]) -> StdResult<Expr> {
    let maybe: Vec<_> = terms.iter().map(|term| term.lower(context)).collect();
    let nodes = utils::check_errors(&maybe)?;

    Ok(Expr::Control(Control::Terms(nodes)))
}

pub(crate) fn lower_loop(
    context: &mut CompileContext,
    lno: LineNo,
    ident: &Hir,
    terms: &Hir,
) -> StdResult<Expr> {
    let maybe_ident = ident.lower(context);
    let maybe_terms = terms.lower(context);

    let mut maybe = vec![maybe_ident, maybe_terms];
    if !context.flags.intersects(CompileFlags::LOOP_AND_WHILE) {
        maybe.push(Err(vec![Error::new(
            lno,
            ErrorVariant::Message(String::from(
                "Cannot use LOOP if LOOP and WHILE are not enabled!",
            )),
        )]))
    }

    let error_free = utils::check_errors(&maybe)?;
    let (ident, terms) = match error_free.as_slice() {
        [ident, terms] => (ident, terms),
        [ident, terms, ..] => (ident, terms),
        &_ => unreachable!(),
    };

    let node = if context.flags.contains(CompileFlags::LOOP) {
        Expr::Control(Control::Loop {
            lno,
            ident: Box::new(ident.clone()),
            terms: Box::new(terms.clone()),
        })
    } else if context.flags.contains(CompileFlags::WHILE) {
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

        let tmp = Box::new(Expr::Ident(private_identifier(context)));

        Expr::Control(Control::Terms(vec![
            Macro::AssignToIdent {
                lno,
                lhs: tmp.clone(),
                rhs: Box::new(ident.clone()),
            }
            .lower(context)?,
            Expr::Control(Control::While {
                lno,
                comp: Box::new(Expr::Comparison {
                    lhs: tmp.clone(),
                    verb: ComparisonVerb::NotEqual,
                    rhs: Box::new(Expr::NaturalNumber(UInt::zero())),
                }),
                terms: Box::new(Expr::Control(Control::Terms(vec![
                    terms.clone(),
                    Expr::Assign {
                        lno,
                        lhs: tmp.clone(),
                        rhs: Box::new(Expr::BinaryOp {
                            lhs: tmp,
                            verb: OperatorVerb::Minus,
                            rhs: Box::new(Expr::NaturalNumber(UInt::one())),
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

pub(crate) fn lower_while(
    context: &mut CompileContext,
    lno: LineNo,
    comp: &Hir,
    terms: &Hir,
) -> StdResult<Expr> {
    let maybe_comp = comp.lower(context);
    let maybe_terms = terms.lower(context);

    let mut maybe = vec![maybe_comp, maybe_terms];
    if !context.flags.contains(CompileFlags::WHILE) {
        maybe.push(Err(vec![Error::new(
            lno,
            ErrorVariant::Message(String::from("Cannot replicate WHILE in LOOP mode!")),
        )]))
    }

    // always only two if succeeds (context check adds error, which will always get unwrapped)
    let error_free = utils::check_errors(&maybe)?;
    let (comp, terms) = match error_free.as_slice() {
        [comp, terms] => (comp, terms),
        [comp, terms, ..] => (comp, terms),
        &_ => unreachable!(),
    };

    let node = Expr::Control(Control::While {
        lno,
        comp: Box::new(comp.clone()),
        terms: Box::new(terms.clone()),
    });

    Ok(node)
}
