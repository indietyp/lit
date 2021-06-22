use lexer::{Kind, Op, Token};
use variants::err::{ErrorKind, ErrorKindUnsupported};
use variants::Error;

use crate::Expr;
use std::convert::TryFrom;

// We use our own enum instead of the one from the lexer
// to divorce the lexer and AST and give the symbols additional
// meaning
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum BinOpVerb {
    Plus,
    Minus,
    Multiply,
}

impl TryFrom<lexer::Op> for BinOpVerb {
    type Error = Error;

    fn try_from(value: lexer::Op) -> Result<Self, Error> {
        match value {
            Op::Plus => Ok(Self::Plus),
            Op::Minus => Ok(Self::Minus),
            Op::Star => Ok(Self::Multiply),

            Op::Slash => Err(Error::new_from_kind(
                None,
                ErrorKind::Unsupported(ErrorKindUnsupported::BinOp),
            )),
        }
    }
}

impl TryFrom<lexer::Token> for BinOpVerb {
    type Error = Error;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value.kind {
            Kind::Op(Op::Slash) => Err(Error::new_from_kind(
                Some(value.lno),
                ErrorKind::Unsupported(ErrorKindUnsupported::BinOp),
            )),
            Kind::Op(value) => Self::try_from(value),
            _ => Err(Error::new_from_kind(
                Some(value.lno),
                ErrorKind::Unsupported(ErrorKindUnsupported::BinOp),
            )),
        }
    }
}

pub struct BinOp {
    pub kind: Vec<Kind>,

    pub lhs: Box<Expr>,
    pub verb: BinOpVerb,
    pub rhs: Box<Expr>,
}

//region Tests
#[cfg(test)]
mod tests {
    use super::*;
    use lexer::Lexer;
    use std::convert::TryInto;

    fn find_kind(input: &str) -> Token {
        let mut lexer = Lexer::new(input);
        lexer.next().unwrap()
    }

    fn token_matches_verb(input: &str, should_be: BinOpVerb) {
        let token = find_kind(input);
        let verb: Result<BinOpVerb, _> = token.try_into();
        let verb = verb.unwrap();

        assert_eq!(verb, should_be);
    }

    fn token_matches_err(input: &str, should_be: ErrorKind) {
        let token = find_kind(input);
        let verb: Result<BinOpVerb, _> = token.try_into();
        let err = verb.unwrap_err();

        assert_eq!(err.kind, should_be);
    }

    #[test]
    fn binop_from_token() {
        token_matches_verb("+", BinOpVerb::Plus);
        token_matches_verb("-", BinOpVerb::Minus);
        token_matches_verb("*", BinOpVerb::Multiply);

        token_matches_err("/", ErrorKind::Unsupported(ErrorKindUnsupported::BinOp));
        token_matches_err(":=", ErrorKind::Unsupported(ErrorKindUnsupported::BinOp));
    }

    #[test]
    fn binop_from_op() {
        let result: Result<BinOpVerb, _> = Op::Slash.try_into();
        let error = result.unwrap_err();

        assert_eq!(
            error.kind,
            ErrorKind::Unsupported(ErrorKindUnsupported::BinOp)
        )
    }
}
//endregion
