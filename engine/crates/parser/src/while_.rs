use combine::parser::token::Token as CombineToken;
use combine::{token, Stream};
use lexer::{Keyword, Kind, Token};

fn parse_while<Input>(c: Input::Token) -> CombineToken<Input>
where
    Input: Stream,
    Input::Token: PartialOrd,
{
    (token(Kind::Keyword(Keyword::While)), token(Kind::Ident), token(Kind::Keyword(Keyword::Do)));
}
