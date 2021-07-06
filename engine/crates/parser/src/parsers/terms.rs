use crate::combinators::trivia::sep;
use crate::parsers::lp::lp;
use crate::parsers::noop::noop;
use crate::parsers::unknown::unknowns;
use crate::parsers::whl::whl;
use combine::parser::combinator::no_partial;
use combine::parser::repeat::repeat_until;
use combine::{any, attempt, many, optional, satisfy, sep_by, Parser, Stream};
use ctrl::Control;
use hir::Hir;
use lexer::{Keyword, Kind, Token};
use mcr::Unknown;

fn term<Input>(unknown: bool) -> impl Parser<Input, Output = Hir, PartialState = ()>
where
    Input: Stream<Token = Token>,
    Input::Error: Sized,
{
    let combinator = attempt(whl()).or(attempt(lp()).or(if unknown {
        attempt(noop())
            .or(attempt(
                satisfy(|token: Token| !matches!(token.kind, Kind::Keyword(Keyword::End)))
                    .map(|value| Hir::Unknown(Unknown(vec![value]))),
            ))
            .left()
        // noop().left()
    } else {
        attempt(noop()).right()
    }));

    no_partial(combinator)
}

fn terms_<Input>(unknown: bool) -> impl Parser<Input, Output = Hir, PartialState = ()>
where
    Input: Stream<Token = Token>,
    Input::Error: Sized,
{
    // TODO: this must be done better
    let combinator = optional(
        (
            term(unknown),
            optional(sep()),
            many::<Vec<_>, _, _>((sep(), term(unknown)).map(|(_, term)| term)),
        )
            .map(|(term, _, terms)| vec![term].into_iter().chain(terms).collect()),
    )
    .map(|terms| match terms {
        None => Hir::NoOp,
        Some(terms) => Hir::Control(Control::Terms { terms }),
    });

    no_partial(combinator)
}

parser! {
    pub(crate) fn terms[Input](unknown: bool)(Input) -> Hir
    where [Input: Stream<Token = Token>]
    {
        terms_(*unknown)
    }
}
