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

create_is!(ident, Kind::Ident(_));
create_is!(number, Kind::Number(_));

//region Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::combinators::check_single_kind;

    #[test]
    fn combine_paren() {
        check_single_kind("(", is_lparen);
        check_single_kind(")", is_rparen);
    }

    #[test]
    fn combine_brace() {
        check_single_kind("{", is_lbrace);
        check_single_kind("}", is_rbrace);
    }

    #[test]
    fn combine_symbols() {
        check_single_kind("->", is_into);
        check_single_kind("...", is_ellipsis);
        check_single_kind(",", is_comma);

        check_single_kind(":=", is_assign);
        check_single_kind("=", is_assign);
    }

    #[test]
    fn combine_sep() {
        check_single_kind(";", is_semicolon);
        check_single_kind("\n", is_newline)
    }

    #[test]
    fn combine_primitive() {
        check_single_kind("abc", is_ident);
        check_single_kind("123", is_number);
    }
}
//endregion
