use crate::combinators::trivia::sep;
use crate::parsers::lp::lp;
use crate::parsers::noop::noop;
use crate::parsers::whl::whl;
use combine::{choice, many, optional, Parser, Stream};
use hir::Hir;
use lexer::Token;
use variants::Errors;

pub(crate) fn term<Input>() -> impl Parser<Input, Output = Hir>
where
    Input: Stream<Token = Token>,
    Input::Error: Sized,
{
    choice([whl(), lp(), noop()])
}

pub(crate) fn terms<Input>() -> impl Parser<Input, Output = Hir>
where
    Input: Stream<Token = Token>,
    Input::Error: Sized,
{
    optional(
        (
            term(),
            many::<Vec<_>, _, _>((sep(), term()).map(|(_, term)| term)),
        )
            .map(|(term, terms)| vec![term].into_iter().chain(terms).collect_vec()),
    )
    .map(|terms| match terms {
        None => Hir::NoOp,
        Some(terms) => Hir::Control(Control::Terms { terms }),
    })
}
