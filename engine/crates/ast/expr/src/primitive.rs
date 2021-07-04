use lexer::{Token};
use variants::UInt;

pub enum Primitive {
    Ident { value: String, token: Vec<Token> },
    Number { value: UInt, token: Vec<Token> },
}
