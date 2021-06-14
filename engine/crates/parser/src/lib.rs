#[macro_use]
extern crate combine;

use combinators::trivia::many_trivia;
use combine::parser::char::{letter, spaces};
use combine::{many1, ParseError, Parser, Stream};
use lexer::Token;

mod combinators;
pub(crate) mod err;
mod skip;
pub(crate) mod stream;
mod while_;

// fn parse(input: &str) -> Parser {
//     todo!()
// }

// `impl Parser` can be used to create reusable parsers with zero overhead
fn expr_<Input>() -> impl Parser<Input, Output = Expr>
where
    Input: Stream<Token = Token>,
    // Necessary due to rust-lang/rust#24159
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let word = many1(letter());

    // A parser which skips past whitespace.
    // Since we aren't interested in knowing that our expression parser
    // could have accepted additional whitespace between the tokens we also silence the error.
    let skip_spaces = || many_trivia().silent();

    //Creates a parser which parses a char and skips any trailing whitespace
    let lex_char = |c| char(c).skip(skip_spaces());

    let comma_list = sep_by(expr(), lex_char(','));
    let array = between(lex_char('['), lex_char(']'), comma_list);

    //We can use tuples to run several parsers in sequence
    //The resulting type is a tuple containing each parsers output
    let pair = (lex_char('('), expr(), lex_char(','), expr(), lex_char(')'))
        .map(|t| Expr::Pair(Box::new(t.1), Box::new(t.3)));

    choice((word.map(Expr::Id), array.map(Expr::Array), pair)).skip(skip_spaces())
}

parser! {
    fn expr[Input]()(Input) -> Expr
    where [Input: Stream<Token = Token>]
    {
        expr_()
    }
}
