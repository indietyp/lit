use lexer::{Keyword, Kind, Token};

macro_rules! create_kw {
    ($name:tt, $( $pattern:pat )|+) => {
        simple_combinator!(kw, $name, $($pattern)|*);
    };
}

create_kw!(while, Kind::Keyword(Keyword::While));
create_kw!(loop, Kind::Keyword(Keyword::Loop));
create_kw!(end, Kind::Keyword(Keyword::End));
create_kw!(fn, Kind::Keyword(Keyword::Fn));
create_kw!(decl, Kind::Keyword(Keyword::Decl));
create_kw!(import, Kind::Keyword(Keyword::Import));
create_kw!(from, Kind::Keyword(Keyword::From));
create_kw!(as, Kind::Keyword(Keyword::As));

// idk why, but this needs to be done separately
pub(crate) fn kw_do<Input>() -> impl ::combine::Parser<Input, Output = Token, PartialState = ()>
where
    Input: ::combine::Stream<Token = Token>,
    Input::Error: ::combine::ParseError<Input::Token, Input::Range, Input::Position>,
{
    let f: fn(::lexer::Token) -> bool =
        |token| ::std::matches!(token.kind, Kind::Keyword(Keyword::Do));
    ::combine::Parser::expected(::combine::satisfy(f), "do")
}

//region Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::combinators::check_single_kind;

    #[test]
    fn combine_primitive() {
        check_single_kind("while", kw_while);
        check_single_kind("loop", kw_loop);
        check_single_kind("end", kw_end);
        check_single_kind("do", kw_do);
    }

    #[test]
    fn combine_fn() {
        check_single_kind("fn", kw_fn);
        check_single_kind("decl", kw_decl);
    }

    #[test]
    fn combine_import() {
        check_single_kind("import", kw_import);
        check_single_kind("from", kw_from);
        check_single_kind("as", kw_as);
    }
}
//endregion
