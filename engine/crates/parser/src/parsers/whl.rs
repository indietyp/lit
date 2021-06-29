// This module is called whl instead of while because while is a reserved keyword

use crate::combinators::kw::{kw_end, kw_while, kw_do_};
use crate::combinators::comp::{comp_ne};
use crate::combinators::trivia::sep;
use combine::parser::token::Token as CombineToken;
use combine::{token, Parser, Stream};
use hir::Hir;
use lexer::{Keyword, Kind, Token};
use variants::Errors;

// Would be like:
// parse: WHILE IDENT != VALUE DO terms END
// return: HIR of While

fn parse_while(input: &str) -> Result<Hir, Errors> {
    let (_, ident, _, number, _, _, terms, _, _, _) = (
        kw_while(),
        is_ident(),
        comp_ne(),
        is_number().satisfy(|token| match token.kind {
            Kind::Number(m) => m == BigInt::zero(),
            _ => false,
        }),
        kw_do_(),
        sep(),
        terms(),
        kw_end(),
        sep(),
    )
        .parse(input);

    todo!()
}
