// TODO
// (should return which op is used -> token kind?)
use lexer::{Kind, Op};

macro_rules! create_op {
    ($name:tt, $( $pattern:pat )|+) => {
        simple_combinator!(op, $name, $($pattern)|*);
    };
}

create_op!(plus, Kind::Op(Op::Plus));
create_op!(minus, Kind::Op(Op::Minus));
create_op!(slash, Kind::Op(Op::Slash));
create_op!(star, Kind::Op(Op::Star));

//region Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::combinators::check_single_kind;

    #[test]
    fn combine_op() {
        check_single_kind("+", op_plus);
        check_single_kind("-", op_minus);
        check_single_kind("*", op_star);
        check_single_kind("/", op_slash);
    }
}
//endregion
