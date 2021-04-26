use crate::ast::Node::BinaryOp;
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
}

#[derive(Debug)]
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
            _ => panic!("Currently do not support specified operator {}", verb)
        }
    }
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
        rhs_lhs: Box<Node>,
        rhs_verb: OperatorVerb,
        rhs_rhs: Box<Node>
    },
    AssignToOpExtIdent {
        lhs: Box<Node>,
        rhs_lhs: Box<Node>,
        rhs_verb: OperatorVerb,
        rhs_rhs: Box<Node>
    },
    AssignToOpExtValue {
        lhs: Box<Node>,
        rhs_lhs: Box<Node>,
        rhs_verb: OperatorVerb,
        rhs_rhs: Box<Node>
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

impl Macro {
    pub fn expand(&self) -> Node {
        match self {
            Macro::AssignToIdent => Node::Assign {
                lhs: self.lhs,
                rhs: Box::new(BinaryOp {
                    verb: OperatorVerb::Plus,
                    lhs: self.rhs,
                    rhs: Box::new(Node::NaturalNumber(BigUint::from(0))),
                }),
            },
            Macro::AssignToZero => Node::Control(Control::Loop {
                ident: self.lhs,
                terms: Box::new(Node::Control(Control::Terms(vec![Node::Assign {
                    lhs: self.lhs,
                    rhs: Box::new(BinaryOp {
                        verb: OperatorVerb::Minus,
                        lhs: self.lhs,
                        rhs: Box::new(Node::NaturalNumber(BigUint::from(1))),
                    }),
                }]))),
            }),
            Macro::AssignToValue => Node::Control(Control::Terms(vec![
                Macro::AssignToZero { lhs: self.lhs }.expand(),
                Node::Assign {
                    lhs: self.lhs,
                    rhs: Box::new(BinaryOp {
                        lhs: self.lhs,
                        rhs: self.rhs,
                        verb: OperatorVerb::Plus,
                    }),
                },
            ])),
            Macro::AssignToOpIdent => Node::Control(Control::Terms(vec![
                Macro::AssignToIdent {
                    lhs: self.lhs,
                    rhs: self.rhs.lhs,
                }
                .expand(),
                Node::Control(Control::Loop {
                    ident: self.rhs.rhs,
                    terms: Box::new(Node::Control(Control::Terms(vec![Node::Assign {
                        lhs: self.lhs,
                        rhs: Box::new(BinaryOp {
                            lhs: self.lhs,
                            rhs: Box::new(Node::NaturalNumber(BigUint::from(1))),
                            verb: self.rhs.verb,
                        }),
                    }]))),
                }),
            ])),
            Macro::AssignToOpIdent {

            }
        }
    }
}

// Control Structures have in their body potentially
// polluted information, these need to changed/unpolluted via
// macro expansion
#[derive(Debug)]
pub enum Control<TNode> {
    Terms(Vec<TNode>),
    Loop {
        ident: Box<TNode>,
        terms: Box<TNode>,
    },
    While {
        comp: Box<TNode>,
        terms: Box<TNode>,
    },
}

// TODO: UnaryExpression?
#[derive(Debug, Copy, Clone)]
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
    Pure(Node),
    Macro(Macro),
    NoOp,

    Control(Control<PollutedNode>),
}

impl PollutedNode {
    pub fn purify(&self) -> Node {
        match self {
            PollutedNode::Pure(n) => n.clone(),
            _ => panic!("Cannot Purify!"),
        }
    }
}

// TODO: flatten method
