use lexer::{Comp, Kind};

macro_rules! create_comp {
    ($name:tt, $( $pattern:pat )|+) => {
        simple_combinator!(comp, $name, $($pattern)|*);
    };
}

create_comp!(eq, Kind::Comp(Comp::Equal));
create_comp!(ne, Kind::Comp(Comp::NotEqual));
create_comp!(gt, Kind::Comp(Comp::GreaterThan));
create_comp!(ge, Kind::Comp(Comp::GreaterEqual));
create_comp!(lt, Kind::Comp(Comp::LessThan));
create_comp!(le, Kind::Comp(Comp::LessEqual));

//region Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::combinators::check_single_kind;

    #[test]
    fn combine_op() {
        check_single_kind("==", comp_eq);
        check_single_kind("!=", comp_ne);
        check_single_kind(">", comp_gt);
        check_single_kind(">=", comp_ge);
        check_single_kind("<", comp_lt);
        check_single_kind("<=", comp_le);
    }
}
//endregion
