use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Keyword {
    While,
    Loop,
    End,
    Fn,
    Decl,
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Keyword::While => "‘while‘",
            Keyword::Loop => "‘loop‘",
            Keyword::End => "‘end‘",
            Keyword::Fn => "‘fn‘",
            Keyword::Decl => "‘decl‘",
        })
    }
}
