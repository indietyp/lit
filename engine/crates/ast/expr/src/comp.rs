use crate::{Expr, Primitive};
use lexer::{Kind, Token};
use std::convert::TryFrom;
use variants::err::{ErrorKind, ErrorKindUnsupported};
use variants::{Error, LineNo};

// We use our own enum instead of the one from the lexer
// to divorce the lexer and AST
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
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

impl ToString for CompVerb {
    fn to_string(&self) -> String {
        match self {
            CompVerb::Equal => "==".to_string(),
            CompVerb::NotEqual => "!=".to_string(),
            CompVerb::GreaterThan => ">".to_string(),
            CompVerb::GreaterThanEqual => ">=".to_string(),
            CompVerb::LessThan => "<".to_string(),
            CompVerb::LessThanEqual => "<=".to_string(),
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

#[derive(Debug, Clone)]
pub struct Comp {
    pub lno: LineNo,

    pub lhs: Primitive,
    pub verb: CompVerb,
    pub rhs: Primitive,
}
