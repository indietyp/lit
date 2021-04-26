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
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    AssignToZero {
        lhs: Box<Node>,
    },
    AssignToValue {
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    AssignToOpIdent {
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    AssignToOpValue {
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    If {
        comp: Box<Node>,
        terms: Box<PollutedNode>,
    },
    IfElse {
        comp: Box<Node>,
        if_terms: Box<PollutedNode>,
        else_terms: Box<PollutedNode>,
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
pub enum Node {
    // Smallest Units
    Ident(String),
    NaturalNumber(BigUint),
    Terms(Vec<Node>),

    // Assignment and Expressions
    Comparison {
        verb: ComparisonVerb,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    BinaryOp {
        verb: OperatorVerb,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    Assign {
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    Control(Control<Node>),
}

#[derive(Debug)]
pub enum PollutedNode {
    ASTNode(Node),
    Macro(Macro),
    NoOp,

    Control(Control<PollutedNode>),
}
