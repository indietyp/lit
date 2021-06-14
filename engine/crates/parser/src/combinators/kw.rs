use lexer::{Keyword, Kind};

macro_rules! create_kw {
    ($name:tt, $( $pattern:pat )|+) => {
        simple_combinator!(kw, $name, $($pattern)|*);
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
