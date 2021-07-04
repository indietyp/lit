use crate::{Expr, Primitive};
use lexer::{Kind, Token};
use std::convert::TryFrom;
use variants::err::{ErrorKind, ErrorKindUnsupported};
use variants::Error;

// We use our own enum instead of the one from the lexer
// to divorce the lexer and AST
pub enum CompVerb {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
}

impl From<lexer::Comp> for CompVerb {
    fn from(value: lexer::Comp) -> Self {
        match value {
            lexer::Comp::Equal => Self::Equal,
            lexer::Comp::NotEqual => Self::NotEqual,
            lexer::Comp::GreaterThan => Self::GreaterThan,
            lexer::Comp::GreaterEqual => Self::GreaterThanEqual,
            lexer::Comp::LessThan => Self::LessThan,
            lexer::Comp::LessEqual => Self::LessThanEqual,
        }
    }
}

impl TryFrom<lexer::Token> for CompVerb {
    type Error = Error;

    fn try_from(value: lexer::Token) -> Result<Self, Self::Error> {
        match value.kind {
            Kind::Comp(comp) => Ok(comp.into()),
            _ => Err(Error::new_from_kind(
                Some(value.lno),
                ErrorKind::Unsupported(ErrorKindUnsupported::Comp),
            )),
        }
    }
}

pub struct Comp {
    pub token: Vec<Token>,

    pub lhs: Primitive,
    pub verb: CompVerb,
    pub rhs: Primitive,
}
