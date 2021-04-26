use num_bigint::BigUint;

pub enum ComparisonVerb {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterEqual,
    LessThan,
    LessThanEqual,
}

pub enum OperatorVerb {
    Plus,
    Minus,
    Multiply,
}

// TODO: UnaryExpression?
pub enum ASTNode {
    // Smallest Units
    Identifier(String),
    NaturalNumber(BigUint),
    Terms(Vec<ASTNode>),

    // Assignment and Expressions
    BinaryOperation {
        verb: OperatorVerb,
        lhs: Box<ASTNode>,
        rhs: Box<ASTNode>,
    },
    Assignment {
        lhs: Box<ASTNode>,
        rhs: Box<ASTNode>,
    },

    // Control Structures
    Loop {
        ident: Box<ASTNode>,
        terms: Box<ASTNode>,
    },
    While {
        comp: Box<ASTNode>,
        terms: Box<ASTNode>,
    },
}

pub enum Macro {
    AssignToVariable {
        lhs: Box<ASTNode>,
        rhs: Box<ASTNode>,
    },
    AssignToZero {
        lhs: Box<ASTNode>,
        rhs: Box<ASTNode>,
    },
    AssignToValue {
        lhs: Box<ASTNode>,
        rhs: Box<ASTNode>,
    },
    AssignToOpVariables {
        lhs: Box<ASTNode>,
        rhs: Box<ASTNode>,
    },
    AssignToOpValue {
        lhs: Box<ASTNode>,
        rhs: Box<ASTNode>,
    },
}

pub enum PollutedASTNode {
    ASTNode(ASTNode),
    Macro(Macro),
}
