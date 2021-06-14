use combine::{ParseError, Parser, satisfy, skip_many, Stream};

use lexer::Token;

pub fn trivia<Input>() -> impl Parser<Input, Output = Token, PartialState = ()>
where
    Input: Stream<Token = Token>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let f: fn(Token) -> bool = |token| token.kind.is_trivia();
    satisfy(f).expected("trivia")
}

pub fn many_trivia<Input>() -> impl Parser<Input, Output = ()>
where
    Input: Stream<Token = Token>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    skip_many(trivia()).expected("trivia")
}
