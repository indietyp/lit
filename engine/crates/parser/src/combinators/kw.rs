use combine::{satisfy, ParseError, Parser, Stream};

use lexer::{Keyword, Kind};
use paste::paste;

macro_rules! create_kw {
    ($name:tt, $( $pattern:pat )|+) => {
        paste! {
            pub fn [<kw_ $name>]<Input>() -> impl Parser<Input, Output = ::lexer::Token, PartialState = ()>
            where
                Input: Stream<Token = ::lexer::Token>,
                Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
            {
                let f: fn(::lexer::Token) -> bool = |token| ::std::matches!(token.kind, $($pattern)|*);
                ::combine::satisfy(f).expected(::std::stringify!($name))
            }
        }
    };
}

create_kw!(while, Kind::Keyword(Keyword::While));
create_kw!(loop, Kind::Keyword(Keyword::Loop));
create_kw!(do, Kind::Keyword(Keyword::Do));
create_kw!(end, Kind::Keyword(Keyword::End));
create_kw!(fn, Kind::Keyword(Keyword::Fn));
create_kw!(decl, Kind::Keyword(Keyword::Decl));
create_kw!(import, Kind::Keyword(Keyword::Import));
create_kw!(from, Kind::Keyword(Keyword::From));
create_kw!(as, Kind::Keyword(Keyword::As));
