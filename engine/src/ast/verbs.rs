use core::fmt;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

#[cfg(feature = "cli")]
use schemars::JsonSchema;

#[derive(Debug, Clone, Serialize, Deserialize, Hash, Ord, PartialOrd, Eq, PartialEq)]
#[cfg_attr(feature = "cli", derive(JsonSchema))]
pub enum ComparisonVerb {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
}

impl ComparisonVerb {
    pub fn from(verb: &str) -> Self {
        match verb {
            "=" | "==" => ComparisonVerb::Equal,
            "!=" => ComparisonVerb::NotEqual,
            ">" => ComparisonVerb::GreaterThan,
            ">=" => ComparisonVerb::GreaterThanEqual,
            "<" => ComparisonVerb::LessThan,
            "<=" => ComparisonVerb::LessThanEqual,
            _ => panic!("Currently do not support comparison operator {}.", verb),
        }
    }

    pub fn display(&self) -> String {
        String::from(match self {
            ComparisonVerb::Equal => "==",
            ComparisonVerb::NotEqual => "!=",
            ComparisonVerb::GreaterThan => ">",
            ComparisonVerb::GreaterThanEqual => ">=",
            ComparisonVerb::LessThan => "<",
            ComparisonVerb::LessThanEqual => "<=",
        })
    }
}

impl fmt::Display for ComparisonVerb {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, Ord, PartialOrd, Eq, PartialEq)]
#[cfg_attr(feature = "cli", derive(JsonSchema))]
pub enum OperatorVerb {
    Plus,
    Minus,
    Multiply,
}

impl OperatorVerb {
    pub fn from(verb: &str) -> Self {
        match verb {
            "+" => OperatorVerb::Plus,
            "-" => OperatorVerb::Minus,
            "*" => OperatorVerb::Multiply,
            _ => panic!("Currently do not support specified operator {}", verb),
        }
    }

    pub fn display(&self) -> String {
        String::from(match self {
            OperatorVerb::Plus => "+",
            OperatorVerb::Minus => "-",
            OperatorVerb::Multiply => "*",
        })
    }
}

impl fmt::Display for OperatorVerb {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display())
    }
}
