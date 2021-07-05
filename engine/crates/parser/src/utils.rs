use crate::Token;
use expr::binop::BinOpVerb;
use expr::comp::CompVerb;
use expr::Primitive;
use lexer::Kind;
use std::convert::TryFrom;
use variants::err::{ErrorKind, ErrorKindInvalidToken};
use variants::{Error, Errors};

pub(crate) fn to_ident(token: Token) -> Result<Primitive, Errors> {
    match token.kind {
        Kind::Ident(ident) => Ok(Primitive::Ident {
            value: ident,
            token: vec![token],
        }),
        _ => Err(Errors::from(Error::new_from_kind(
            Some(token.lno),
            ErrorKind::InvalidToken(ErrorKindInvalidToken::new(
                stringify!(Kind::Ident).into(),
                stringify!(token.kind).into(),
            )),
        ))),
    }
}

pub(crate) fn to_comp_verb(token: Token) -> Result<CompVerb, Errors> {
    match token.kind {
        Kind::Comp(comp) => Ok(comp.into()),
        _ => Err(Errors::from(Error::new_from_kind(
            Some(token.lno),
            ErrorKind::InvalidToken(ErrorKindInvalidToken::new(
                stringify!(Kind::Comp).into(),
                stringify!(token.kind).into(),
            )),
        ))),
    }
}

pub(crate) fn to_binop_verb(token: Token) -> Result<BinOpVerb, Errors> {
    match token.kind {
        Kind::Op(op) => Ok(BinOpVerb::try_from(op)?),
        _ => Err(Errors::from(Error::new_from_kind(
            Some(token.lno),
            ErrorKind::InvalidToken(ErrorKindInvalidToken::new(
                stringify!(Kind::Op).into(),
                stringify!(token.kind).into(),
            )),
        ))),
    }
}

pub(crate) fn to_uint(token: Token) -> Result<Primitive, Errors> {
    match token.kind {
        Kind::Number(ident) => Ok(Primitive::Number {
            value: ident,
            token: vec![token],
        }),
        _ => Err(Errors::from(Error::new_from_kind(
            Some(token.lno),
            ErrorKind::InvalidToken(ErrorKindInvalidToken::new(
                stringify!(Kind::Number).into(),
                stringify!(token.kind).into(),
            )),
        ))),
    }
}
