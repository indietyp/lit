use crate::combinators::is::{is_assign, is_ident, is_number};
use crate::combinators::op::{op_minus, op_plus};
use crate::combinators::trivia::sep;
use crate::utils::{to_binop_verb, to_ident, to_uint};
use combine::{choice, Parser, Stream};
use expr::binop::BinOpVerb;
use expr::{Assign, BinOp, Expr, Primitive};
use hir::Hir;
use lexer::Token;

pub(crate) fn assign<Input>() -> impl Parser<Input, Output = Hir>
where
    Input: Stream<Token = Token>,
    Input::Error: Sized,
{
    (
        is_ident(),
        is_assign(),
        is_ident(),
        choice([op_plus(), op_minus()]),
        is_number(),
        sep(),
    )
        .map(|(ident, _, lhs, op, rhs)| {
            Hir::Expr(Expr::Assign(Assign {
                lno: ident.lno.end_at(&rhs.lno),

                lhs: to_ident(ident)?,
                rhs: Box::new(Expr::BinOp(BinOp {
                    lno: lhs.lno.end_at(&rhs.lno),

                    lhs: to_ident(lhs)?,
                    verb: to_binop_verb(op)?,
                    rhs: to_uint(rhs)?,
                })),
            }))
        })
}
