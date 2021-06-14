use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Comp {
    Equal,
    NotEqual,

    GreaterThan,
    GreaterEqual,

    LessThan,
    LessEqual,
}

impl fmt::Display for Comp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Comp::Equal => "==",
            Comp::NotEqual => "!=",
            Comp::GreaterThan => ">",
            Comp::GreaterEqual => ">=",
            Comp::LessThan => "<",
            Comp::LessEqual => "<=",
        })
    }
}

impl From<&str> for Comp {
    fn from(value: &str) -> Self {
        match value {
            "==" => Self::Equal,
            "=" => Self::Equal,
            "!=" => Self::NotEqual,
            ">" => Self::GreaterThan,
            ">=" => Self::GreaterEqual,
            "<" => Self::LessThan,
            "<=" => Self::LessEqual,
            _ => panic!("Unsupported value"),
        }
    }
}

impl From<String> for Comp {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}
