use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Keyword {
    While,
    Loop,
    Do,
    End,
    Fn,
    Decl,
    Import,
    From,
    As,
    Macro,
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Keyword::While => "‘while‘",
            Keyword::Loop => "‘loop‘",
            Keyword::Do => "‘do‘",
            Keyword::End => "‘end‘",
            Keyword::Fn => "‘fn‘",
            Keyword::Decl => "‘decl‘",
            Keyword::Import => "‘import‘",
            Keyword::From => "‘from‘",
            Keyword::As => "‘as‘",
            Keyword::Macro => "‘macro‘",
        })
    }
}
