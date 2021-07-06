// This module is called whl instead of while because while is a reserved keyword

use crate::combinators::comp::comp_ne;
use crate::combinators::is::{is_ident, is_number};
use crate::combinators::kw::{kw_do, kw_end, kw_while};
use crate::parsers::terms::block;
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
        block(),
    )
        .then(|(start, ident, comp, number, block)| {
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
                lno: start.lno.end_at(&block.lno),

                comp: Comp {
                    lno: ident.lno.end_at(&number.lno),

                    lhs: lhs.unwrap(),
                    verb: verb.unwrap(),
                    rhs: rhs.unwrap(),
                },
                terms: Box::new(block.terms),
            });

            value(ctrl).left()
        });

    no_partial(combinator)
}

//region Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::disp::CompactRepresentation;
    use indoc::indoc;

    #[test]
    fn parser_whl() {
        let stream = crate::stream::LexerStream::new("WHILE x != 0 DO; ... ;END");
        let parsed = whl().parse(stream);

        assert!(parsed.is_ok());

        let (hir, stream) = parsed.unwrap();
        assert!(stream.is_exhausted());

        assert!(matches!(hir, Hir::Control(Control::While { .. })))
    }

    #[test]
    fn parser_nested_whl() {
        let stream =
            crate::stream::LexerStream::new("WHILE x != 0 DO; WHILE y != 0 DO; ... ; END; END");
        let parsed = whl().parse(stream);

        assert!(parsed.is_ok());

        assert_eq!(
            indoc!(
                "
                While@[0:0->0:48]:
                  Comp: x != 0
                  Terms:
                    While@[0:17->0:43]:
                      Comp: y != 0
                      Terms:
                        NoOp\n\n"
            ),
            parsed.unwrap().0.compact(None)
        )
    }

    #[test]
    fn parser_nested_whl_lp() {
        let stream = crate::stream::LexerStream::new(indoc!(
            "\
            WHILE x != 0 DO;
                LOOP y DO
                END
            END"
        ));
        let parsed = whl().parse(stream);

        assert!(parsed.is_ok());

        assert_eq!(
            indoc!(
                "
                While@[0:0->3:3]:
                  Comp: x != 0
                  Terms:
                    Loop@[1:4->2:7]:
                      Ident: y
                      Terms:
                        NoOp\n\n"
            ),
            parsed.unwrap().0.compact(None)
        )
    }
}
//endregion
