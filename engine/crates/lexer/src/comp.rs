#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Comp {
    Equal,
    NotEqual,

    GreaterThan,
    GreaterEqual,

    LessThan,
    LessEqual,
}

impl From<&str> for Comp {
    fn from(value: &str) -> Self {
        match value {
            "==" | "=" => Self::Equal,
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
