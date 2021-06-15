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
