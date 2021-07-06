use combine::{attempt, optional, satisfy, skip_many, ParseError, Parser, Stream};

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

pub fn single_sep<Input>() -> impl Parser<Input, Output = ()>
where
    Input: Stream<Token = Token>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let newline: fn(Token) -> bool = |token| matches!(token.kind, ::lexer::Kind::Newline);
    let semicolon: fn(Token) -> bool = |token| matches!(token.kind, ::lexer::Kind::Semicolon);

    choice!(
        attempt((satisfy(semicolon), satisfy(newline))).map(|_| ()),
        attempt(satisfy(semicolon)).map(|_| ()),
        attempt(satisfy(newline)).map(|_| ())
    )
    .expected("separator")
}

pub fn sep<Input>() -> impl Parser<Input, Output = ()>
where
    Input: Stream<Token = Token>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    skip_many(single_sep()).expected("sep")
}
