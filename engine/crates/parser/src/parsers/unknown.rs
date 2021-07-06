use combine::{satisfy, Parser, Stream};
use hir::Hir;
use lexer::{Keyword, Kind, Token};
use mcr::Unknown;

pub(crate) fn unknown<Input>() -> impl Parser<Input, Output = Hir>
where
    Input: Stream<Token = Token>,
    Input::Error: Sized,
{
    // Do and End are hard keywords, which should never be used in an unknown context
    satisfy(|token: Token| {
        !matches!(token.kind, Kind::Keyword(Keyword::End))
            && !matches!(token.kind, Kind::Keyword(Keyword::Do))
    })
    .map(|value| Hir::Unknown(Unknown::Token(value)))
    .expected("unknown")
}
