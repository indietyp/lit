use crate::stream::LexerStream;
use combine::Parser;
use lexer::Token;

#[macro_use]
pub(crate) mod macros;

pub(crate) mod is;
pub(crate) mod kw;
pub(crate) mod trivia;

#[cfg(test)]
fn check_single_kind<T: Parser<LexerStream, Output = Token, PartialState = ()>>(
    input: &str,
    combinator: fn() -> T,
) {
    let stream = LexerStream::new(input);

    let result = combinator().parse(stream);
    if let Err(error) = result {
        println!("{:?}", error)
    }
    assert!(result.is_ok())
}
