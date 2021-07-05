use crate::combinators::trivia::sep;
use crate::parsers::lp::lp;
use crate::parsers::noop::noop;
use crate::parsers::unknown::unknowns;
use crate::parsers::whl::whl;
use combine::parser::combinator::no_partial;
use combine::{many, optional, Parser, Stream};
use ctrl::Control;
use hir::Hir;
use lexer::Token;

fn term<Input>(unknown: bool) -> impl Parser<Input, Output = Hir, PartialState = ()>
where
    Input: Stream<Token = Token>,
    Input::Error: Sized,
{
    let combinator = whl().or(lp().or(if unknown {
        noop().or(unknowns()).left()
    } else {
        noop().right()
    }));

    no_partial(combinator)
}

fn terms_<Input>(unknown: bool) -> impl Parser<Input, Output = Hir, PartialState = ()>
where
    Input: Stream<Token = Token>,
    Input::Error: Sized,
{
    let combinator = optional(
        (
            term(unknown),
            many::<Vec<_>, _, _>((sep(), term(unknown)).map(|(_, term)| term)),
        )
            .map(|(term, terms)| vec![term].into_iter().chain(terms).collect()),
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
