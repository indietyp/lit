#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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
