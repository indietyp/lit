use crate::parsers::terms::terms;
use combine::parser::repeat::take_until;
use combine::{attempt, Parser, Stream};
use hir::Hir;
use lexer::Token;
use mcr::Unknown;

pub(crate) fn unknowns<Input>() -> impl Parser<Input, Output = Hir>
where
    Input: Stream<Token = Token>,
    Input::Error: Sized,
{
    take_until(attempt(terms(false))).map(|val| Hir::Unknown(Unknown(val)))
}
