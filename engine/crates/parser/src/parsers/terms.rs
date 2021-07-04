use crate::parsers::lp::lp;
use crate::parsers::whl::whl;
use combine::{choice, many, Parser, Stream};
use ctrl::Control;
use hir::Hir;
use lexer::Token;
use variants::Errors;

pub(crate) fn terms<Input>() -> impl Parser<Input, Output = Hir>
where
    Input: Stream<Token = Token>,
    Input::Error: Sized,
{
    many(choice([whl(), lp()])).map(|many| Hir::Control(Control::Terms { terms: many }))
}
