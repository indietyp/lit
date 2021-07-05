use crate::combinators::is::is_ident;
use crate::combinators::kw::{kw_do_, kw_end, kw_loop};
use crate::combinators::trivia::sep;
use crate::parsers::terms::terms;
use crate::utils::to_ident;
use combine::error::Info::Format;
use combine::parser::combinator::no_partial;
use combine::{unexpected_any, value, Parser, Stream};
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
        kw_loop(),
        is_ident(),
        kw_do_(),
        sep(),
        terms(true),
        kw_end(),
    )
        .then(|(start, ident, _, _, terms, end)| {
            let ident = to_ident(ident);

            if let Err(err) = ident {
                return unexpected_any(Format(err)).right();
            }

            let hir = Hir::Control(Control::Loop {
                lno: start.lno.end_at(&end.lno),

                ident: ident.unwrap(),
                terms: Box::new(terms),
            });

            value(hir).left()
        });

    no_partial(combinator)
}

//region Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_lp() {
        println!("{:#?}", lp().parse("LOOP x DO; ;END").unwrap());
    }
}
//endregion
