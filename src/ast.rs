use num_bigint::BigUint;

pub enum ComparisonVerb {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanEqual,
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
    Ident(String),
    UInt(BigUint),
    Terms(Vec<ASTNode>),

    // Assignment and Expressions
    Comparison {
        verb: ComparisonVerb,
        lhs: Box<ASTNode>,
        rhs: Box<ASTNode>,
    },
    BinaryOp {
        verb: OperatorVerb,
        lhs: Box<ASTNode>,
        rhs: Box<ASTNode>,
    },
    Assign {
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
    AssignToIdent {
        lhs: Box<ASTNode>,
        rhs: Box<ASTNode>,
    },
    AssignToZero {
        lhs: Box<ASTNode>,
    },
    AssignToValue {
        lhs: Box<ASTNode>,
        rhs: Box<ASTNode>,
    },
    AssignToOpIdent {
        lhs: Box<ASTNode>,
        rhs: Box<ASTNode>,
    },
    AssignToOpValue {
        lhs: Box<ASTNode>,
        rhs: Box<ASTNode>,
    },
    If {
        comp: Box<ASTNode>,
        terms: Box<ASTNode>,
    },
    IfElse {
        comp: Box<ASTNode>,
        if_terms: Box<ASTNode>,
        else_terms: Box<ASTNode>,
    },
}

pub enum PollutedASTNode {
    ASTNode(ASTNode),
    Macro(Macro),
}
