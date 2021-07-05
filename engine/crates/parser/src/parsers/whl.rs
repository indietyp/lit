// This module is called whl instead of while because while is a reserved keyword

use crate::combinators::comp::comp_ne;
use crate::combinators::is::{is_ident, is_number};
use crate::combinators::kw::{kw_do_, kw_end, kw_while};
use crate::combinators::trivia::sep;
use crate::parsers::terms::terms;
use crate::utils::{to_comp_verb, to_ident, to_uint};
use combine::error::Info::Format;
use combine::parser::combinator::no_partial;
use combine::{unexpected_any, value, Parser, Stream};
use ctrl::Control;
use expr::Comp;
use hir::Hir;
use lexer::{Kind, Token};
use num_traits::Zero;
use variants::Errors;

// parse: WHILE <ident> != <uint> DO <terms> END
// return: HIR
pub(crate) fn whl<Input>() -> impl Parser<Input, Output = Hir, PartialState = ()>
where
    Input: Stream<Token = Token>,
    Input::Error: Sized,
{
    let combinator = (
        kw_while(),
        is_ident(),
        comp_ne(),
        is_number().then(|token| match &token.kind {
            Kind::Number(number) if number.is_zero() => value(token).left(),
            _ => unexpected_any("number")
                .message("Number is not a 0")
                .right(),
        }),
        kw_do_(),
        sep(),
        terms(true),
        kw_end(),
    )
        .then(|(start, ident, comp, number, _, _, terms, end)| {
            let lhs = to_ident(ident.clone());
            let verb = to_comp_verb(comp);
            let rhs = to_uint(number.clone());

            let mut errors = Errors::new();
            if let Err(err) = &lhs {
                errors += err.clone();
            }
            if let Err(err) = &verb {
                errors += err.clone();
            }
            if let Err(err) = &rhs {
                errors += err.clone();
            }

            if !errors.is_empty() {
                // TODO: we somehow need to return the error!
                return unexpected_any(Format(errors)).right();
            }

            let ctrl = Hir::Control(Control::While {
                lno: start.lno.end_at(&end.lno),

                comp: Comp {
                    lno: ident.lno.end_at(&number.lno),

                    lhs: lhs.unwrap(),
                    verb: verb.unwrap(),
                    rhs: rhs.unwrap(),
                },
                terms: Box::new(terms),
            });

            return value(ctrl).left();
        });

    no_partial(combinator)
}
