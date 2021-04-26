use num_bigint::BigUint;
use std::fmt::Display;

#[derive(Debug)]
pub enum ComparisonVerb {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
}

#[derive(Debug)]
pub enum OperatorVerb {
    Plus,
    Minus,
    Multiply,
}

#[derive(Debug)]
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

// Control Structures have in their body potentially
// polluted information, these need to changed/unpolluted via
// macro expansion
#[derive(Debug)]
pub enum Control<Node> {
    Terms(Vec<Node>),
    Loop { ident: Box<Node>, terms: Box<Node> },
    While { comp: Box<Node>, terms: Box<Node> },
}

// TODO: UnaryExpression?
#[derive(Debug)]
pub enum ASTNode {
    // Smallest Units
    Ident(String),
    NaturalNumber(BigUint),
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
    Control(Control<ASTNode>),
}

#[derive(Debug)]
pub enum PollutedASTNode {
    ASTNode(ASTNode),
    Macro(Macro),
    NoOp,

    Control(Control<PollutedASTNode>),
}
