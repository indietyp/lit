// This module is called whl instead of while because while is a reserved keyword

use combine::parser::token::Token as CombineToken;
use combine::{token, Stream};
use lexer::{Keyword, Kind, Token};

// Would be like:
// parse: WHILE IDENT != VALUE DO terms END
// return: HIR of While

fn parse_while(input: &str) {
    (
        kw_while(),
        is_ident(),
        is_ne(),
        is_number().satisfy(|token| match token.kind {
            Kind::Number(m) => m == BigInt::zero(),
            _ => false,
        }),
        kw_do(),
        terms(),
        kw_end(),
    )
        .parse(input)
}
