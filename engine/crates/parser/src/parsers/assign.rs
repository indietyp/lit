use crate::combinators::is::{is_assign, is_ident, is_number};
use crate::combinators::op::{op_minus, op_plus};

use crate::utils::{to_binop_verb, to_ident, to_uint};
use combine::error::Info::Format;
use combine::parser::combinator::no_partial;
use combine::{choice, unexpected_any, value, Parser, Stream};

use expr::{Assign, BinOp, Expr};
use hir::Hir;
use lexer::Token;
use variants::Errors;

pub(crate) fn assign<Input>() -> impl Parser<Input, Output = Hir>
where
    Input: Stream<Token = Token>,
    Input::Error: Sized,
{
    let combinator = (
        is_ident(),
        is_assign(),
        is_ident(),
        choice!(op_plus(), op_minus()),
        is_number(),
    )
        .then(|(ident, _, lhs, op, rhs)| {
            let res_ident = to_ident(ident.clone());

            let res_lhs = to_ident(lhs.clone());
            let res_verb = to_binop_verb(op);
            let res_rhs = to_uint(rhs.clone());

            let mut errors = Errors::new();
            if let Err(err) = &res_ident {
                errors += err.clone();
            }
            if let Err(err) = &res_lhs {
                errors += err.clone();
            }
            if let Err(err) = &res_verb {
                errors += err.clone();
            }
            if let Err(err) = &res_rhs {
                errors += err.clone();
            }

            if !errors.is_empty() {
                return unexpected_any(Format(errors)).right();
            }

            let hir = Hir::Expr(Expr::Assign(Assign {
                lno: ident.lno.end_at(&rhs.lno),

                lhs: res_ident.unwrap(),
                rhs: Box::new(Expr::BinOp(BinOp {
                    lno: lhs.lno.end_at(&rhs.lno),

                    lhs: res_lhs.unwrap(),
                    verb: res_verb.unwrap(),
                    rhs: res_rhs.unwrap(),
                })),
            }));

            value(hir).left()
        });

    no_partial(combinator)
}
