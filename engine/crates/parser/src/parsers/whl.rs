// This module is called whl instead of while because while is a reserved keyword

use crate::combinators::comp::comp_ne;
use crate::combinators::is::{is_ident, is_number};
use crate::combinators::kw::{kw_do_, kw_end, kw_while};
use crate::combinators::trivia::sep;
use crate::parsers::terms::terms;
use crate::utils::{to_comp_verb, to_ident, to_uint};
use combine::parser::token::Token as CombineToken;
use combine::{look_ahead, satisfy, token, Parser, Stream};
use ctrl::Control;
use expr::{Comp, Expr, Primitive};
use hir::Hir;
use lexer::{Keyword, Kind, Token};
use variants::{Errors, UInt};

// parse: WHILE <ident> != <uint> DO <terms> END
// return: HIR
pub(crate) fn whl<Input>() -> impl Parser<Input, Output = Hir>
where
    Input: Stream<Token = Token>,
    Input::Error: Sized,
{
    (
        kw_while(),
        is_ident(),
        comp_ne(),
        (
            look_ahead(is_number()),
            satisfy(|token| match token.kind {
                Kind::Number(number) => number.is_zero(),
                _ => false,
            }),
        )
            .map(|(token, _)| token),
        kw_do_(),
        sep(),
        terms(),
        kw_end(),
        sep(),
    )
        .map(|(start, ident, comp, number, _, _, terms, end, _)| {
            Hir::Control(Control::While {
                lno: start.lno.end_at(&end.lno),
                comp: Box::new(Hir::Expr(Expr::Comp(Comp {
                    token: vec![ident.clone(), comp.clone(), number.clone()],
                    lhs: to_ident(ident)?,
                    verb: to_comp_verb(comp)?,
                    rhs: to_uint(number)?,
                }))),
                terms: Box::new(terms),
            })
        })
}
