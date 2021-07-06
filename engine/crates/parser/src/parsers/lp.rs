use crate::combinators::is::is_ident;
use crate::combinators::kw::{kw_do, kw_loop};
use crate::combinators::trivia::sep;
use crate::parsers::terms::block;
use crate::utils::to_ident;
use combine::error::Info::Format;
use combine::parser::combinator::no_partial;
use combine::{between, unexpected_any, value, Parser, Stream};
use ctrl::Control;

use hir::Hir;
use lexer::Token;

// parse LOOP <ident> DO <terms> END
// return HIR
pub(crate) fn lp<Input>() -> impl Parser<Input, Output = Hir, PartialState = ()>
where
    Input: Stream<Token = Token>,
    Input::Error: Sized,
{
    let combinator = (
        kw_loop(), //
        is_ident(),
        block(),
    )
        .then(|(start, ident, block)| {
            let ident = to_ident(ident);

            if let Err(err) = ident {
                // TODO: into proper format
                return unexpected_any(Format(err)).right();
            }

            let hir = Hir::Control(Control::Loop {
                lno: start.lno.end_at(&block.lno),

                ident: ident.unwrap(),
                terms: Box::new(block.terms),
            });

            value(hir).left()
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
    fn parser_lp() {
        let stream = crate::stream::LexerStream::new("LOOP x DO; ... ;END");
        let parsed = lp().parse(stream);

        assert!(parsed.is_ok());

        let (hir, stream) = parsed.unwrap();
        assert!(stream.is_exhausted());

        assert!(matches!(hir, Hir::Control(Control::Loop { .. })))
    }

    #[test]
    fn parser_inline_nested_lp() {
        let stream = crate::stream::LexerStream::new("LOOP x DO; LOOP y DO; ... ; END; END");
        let parsed = lp().parse(stream);

        assert!(parsed.is_ok());

        assert_eq!(
            indoc!(
                "
                Loop@[0:0->0:36]:
                  Ident: x
                  Terms:
                    Loop@[0:11->0:31]:
                      Ident: y
                      Terms:
                        NoOp\n\n"
            ),
            parsed.unwrap().0.compact(None)
        )
    }

    #[test]
    fn parser_nested_lp_with_unknowns() {
        let stream = crate::stream::LexerStream::new(indoc!(
            "\
            LOOP x DO
                LOOP y DO
                    WHILE WHILE WHILE WHILE
                END
            END"
        ));
        let parsed = lp().parse(stream);

        assert!(parsed.is_ok());

        assert_eq!(
            indoc!(
                "
                Loop@[0:0->4:3]:
                  Ident: x
                  Terms:
                    Loop@[1:4->3:7]:
                      Ident: y
                      Terms:
                        Unknown:
                          WHILE WHILE WHILE WHILE\n\n"
            ),
            parsed.unwrap().0.compact(None)
        )
    }
}
//endregion
