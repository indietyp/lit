use lexer::Token;
use variants::UInt;

#[derive(Debug, Clone)]
pub enum Primitive {
    Ident { value: String, token: Vec<Token> },
    Number { value: UInt, token: Vec<Token> },
}
