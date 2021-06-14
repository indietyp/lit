use combine::parser::token::Token as CombineToken;
use combine::{token, Stream};
use lexer::{Keyword, Kind, Token};

// Would be like:
// parse: WHILE IDENT != VALUE DO terms END
// return: HIR of While
