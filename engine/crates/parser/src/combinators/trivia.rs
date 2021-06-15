use combine::{satisfy, skip_many, ParseError, Parser, Stream};

use lexer::Token;

pub fn single_trivia<Input>() -> impl Parser<Input, Output = ()>
where
    Input: Stream<Token = Token>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let f: fn(Token) -> bool = |token| token.kind.is_trivia();
    satisfy(f).expected("trivia").map(|v: Input::Token| ())
}

pub fn trivia<Input>() -> impl Parser<Input, Output = ()>
where
    Input: Stream<Token = Token>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    skip_many(single_trivia()).expected("trivia")
}
