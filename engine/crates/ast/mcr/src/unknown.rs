use lexer::{Kind, Token};

#[derive(Debug, Clone)]
pub enum Unknown {
    Token(Token),
    Tokens(Vec<Token>),
}
