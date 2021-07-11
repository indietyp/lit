use crate::combinators::is::{is_assign, is_comma, is_ident, is_lparen, is_number, is_rparen};
use crate::utils::{to_ident, to_uint};
use combine::error::Info::Format;
use combine::parser::combinator::no_partial;
use combine::{sep_by, unexpected_any, value, Parser, Stream};

use fnc::{BoundCall, Call, Func};
use hir::Hir;
use itertools::{Either, Itertools};
use lexer::Token;

// TODO
// decl and calling functions

// parser for function calling statements
// parse lhs := ident(args, )
pub(crate) fn fnc_call<Input>() -> impl Parser<Input, Output = Hir, PartialState = ()>
where
    Input: Stream<Token = Token>,
    Input::Error: Sized,
{
    let combinator = (
        is_ident(),
        is_assign(),
        is_ident(),
        is_lparen(),
        sep_by::<Vec<_>, _, _, _>(
            choice!(
                is_ident().map(to_ident), //
                is_number().map(to_uint)
            ),
            is_comma(),
        ),
        is_rparen(),
    )
        .then(|(lhs, _, ident, _, args, end)| {
            let bound_lno = lhs.lno.end_at(&end.lno);
            let call_lno = ident.lno.end_at(&end.lno);

            // convert into primitives and correctly collect all the errors
            let lhs = to_ident(lhs);
            let ident = to_ident(ident);
            let (arg_errors, args): (Vec<_>, Vec<_>) =
                args.into_iter().partition_map(|value| match value {
                    Ok(v) => Either::Right(v),
                    Err(v) => Either::Left(v),
                });

            let arg_errors = arg_errors.into_iter().fold1(|a, b| a + b);

            collect!(exc | lhs, ident);

            if let Some(errors) = arg_errors {
                exc += errors;
            }

            if !exc.is_empty() {
                return unexpected_any(Format(exc)).right();
            }

            let hir = Hir::Func(Func::BoundCall(BoundCall {
                lno: bound_lno,
                lhs: lhs.unwrap(),
                rhs: Call {
                    lno: call_lno,

                    ident: ident.unwrap(),
                    args,
                },
            }));

            value(hir).left()
        });

    no_partial(combinator)
}

//region Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::disp::CompactRepresentation;

    #[test]
    fn parser_fnc_call_no_args() {
        let stream = crate::stream::LexerStream::new("x := y()");
        let parsed = fnc_call().parse(stream);

        assert!(parsed.is_ok());

        let (hir, stream) = parsed.unwrap();

        assert!(stream.is_exhausted());
        assert_eq!("FncCall@[0:0->0:8]: x := y()", hir.compact(None));
    }

    #[test]
    fn parser_fnc_call_sane_eq() {
        let stream = crate::stream::LexerStream::new("x = y()");
        let parsed = fnc_call().parse(stream);

        assert!(parsed.is_ok());

        let (hir, stream) = parsed.unwrap();

        assert!(stream.is_exhausted());
        assert_eq!("FncCall@[0:0->0:7]: x := y()", hir.compact(None));
    }

    #[test]
    fn parser_fnc_call_number_args() {
        let stream = crate::stream::LexerStream::new("x := y(1, 2, 3, 4, 5, 6)");
        let parsed = fnc_call().parse(stream);

        assert!(parsed.is_ok());

        let (hir, stream) = parsed.unwrap();

        assert!(stream.is_exhausted());
        assert_eq!(
            "FncCall@[0:0->0:24]: x := y(1, 2, 3, 4, 5, 6)",
            hir.compact(None)
        );
    }

    #[test]
    fn parser_fnc_call_var_args() {
        let stream = crate::stream::LexerStream::new("x := y(a, b, c, d, e)");
        let parsed = fnc_call().parse(stream);

        assert!(parsed.is_ok());

        let (hir, stream) = parsed.unwrap();

        assert!(stream.is_exhausted());
        assert_eq!(
            "FncCall@[0:0->0:21]: x := y(a, b, c, d, e)",
            hir.compact(None)
        );
    }

    #[test]
    fn parser_fnc_call_primitive_args() {
        let stream = crate::stream::LexerStream::new("x := y(1, b, 3, d, 5)");
        let parsed = fnc_call().parse(stream);

        assert!(parsed.is_ok());

        let (hir, stream) = parsed.unwrap();

        assert!(stream.is_exhausted());
        assert_eq!(
            "FncCall@[0:0->0:21]: x := y(1, b, 3, d, 5)",
            hir.compact(None)
        );
    }
}
//endregion
