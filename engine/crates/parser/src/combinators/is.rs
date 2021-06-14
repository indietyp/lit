use lexer::{Kind, Pair};

macro_rules! create_is {
    ($name:tt, $( $pattern:pat )|+) => {
        simple_combinator!(is, $name, $($pattern)|*);
    };
}

create_is!(lparen, Kind::Paren(Pair::Left));
create_is!(rparen, Kind::Paren(Pair::Right));
create_is!(lbrace, Kind::Brace(Pair::Left));
create_is!(rbrace, Kind::Brace(Pair::Right));

create_is!(into, Kind::Into);
create_is!(ellipsis, Kind::Ellipsis);
create_is!(comma, Kind::Comma);
create_is!(assign, Kind::Assign);

create_is!(semicolon, Kind::Semicolon);
create_is!(newline, Kind::Newline);
