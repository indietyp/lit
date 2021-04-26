use std::fmt::Display;
use std::ops::Deref;

use num_bigint::BigUint;

use lit::random_identifier;

use crate::ast::Node::{BinaryOp, Ident, NaturalNumber};

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
}

#[derive(Debug, Clone)]
pub struct MacroBinaryAssignOperation {
    pub lhs: Box<Node>,
    pub verb: OperatorVerb,
    pub rhs: Box<Node>,
}

#[derive(Debug, Clone)]
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
        rhs: MacroBinaryAssignOperation,
    },
    AssignToOpExtIdent {
        lhs: Box<Node>,
        rhs: MacroBinaryAssignOperation,
    },
    AssignToOpExtValue {
        lhs: Box<Node>,
        rhs: MacroBinaryAssignOperation,
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
    pub fn purify(&self) -> Node {
        match self {
            Macro::AssignToIdent { lhs, rhs } => Node::Assign {
                lhs: lhs.clone(),
                rhs: Box::new(BinaryOp {
                    verb: OperatorVerb::Plus,
                    lhs: rhs.clone(),
                    rhs: Box::new(Node::NaturalNumber(BigUint::from(0u32))),
                }),
            },
            Macro::AssignToZero { lhs } => Node::Control(Control::Loop {
                ident: lhs.clone(),
                terms: Box::new(Node::Control(Control::Terms(vec![Node::Assign {
                    lhs: lhs.clone(),
                    rhs: Box::new(BinaryOp {
                        verb: OperatorVerb::Minus,
                        lhs: lhs.clone(),
                        rhs: Box::new(Node::NaturalNumber(BigUint::from(1u32))),
                    }),
                }]))),
            }),
            Macro::AssignToValue { lhs, rhs } => Node::Control(Control::Terms(vec![
                Macro::AssignToZero { lhs: lhs.clone() }.purify(),
                Node::Assign {
                    lhs: lhs.clone(),
                    rhs: Box::new(BinaryOp {
                        lhs: lhs.clone(),
                        rhs: rhs.clone(),
                        verb: OperatorVerb::Plus,
                    }),
                },
            ])),
            Macro::AssignToOpIdent { lhs, rhs } => Node::Control(Control::Terms(vec![
                Macro::AssignToIdent {
                    lhs: lhs.clone(),
                    rhs: rhs.lhs.clone(),
                }
                .purify(),
                Node::Control(Control::Loop {
                    ident: rhs.rhs.clone(),
                    terms: Box::new(Node::Control(Control::Terms(vec![Node::Assign {
                        lhs: lhs.clone(),
                        rhs: Box::new(BinaryOp {
                            lhs: lhs.clone(),
                            rhs: Box::new(Node::NaturalNumber(BigUint::from(1u32))),
                            verb: rhs.verb.clone(),
                        }),
                    }]))),
                }),
            ])),
            Macro::AssignToOpExtIdent { lhs, rhs } => Node::Control(Control::Terms(vec![
                Macro::AssignToZero { lhs: lhs.clone() }.purify(),
                Node::Control(Control::Loop {
                    ident: rhs.lhs.clone(),
                    terms: Box::new(Node::Control(Control::Terms(vec![
                        Macro::AssignToOpIdent {
                            lhs: lhs.clone(),
                            rhs: MacroBinaryAssignOperation {
                                lhs: lhs.clone(),
                                verb: OperatorVerb::Plus,
                                rhs: rhs.rhs.clone(),
                            },
                        }
                        .purify(),
                    ]))),
                }),
            ])),
            Macro::AssignToOpExtValue { lhs, rhs } => {
                let mut tmp = random_identifier();
                Node::Control(Control::Terms(vec![
                    Macro::AssignToValue {
                        lhs: Box::new(Node::Ident(tmp.clone())),
                        rhs: rhs.rhs.clone(),
                    }
                    .purify(),
                    Macro::AssignToOpExtIdent {
                        lhs: lhs.clone(),
                        rhs: MacroBinaryAssignOperation {
                            lhs: rhs.lhs.clone(),
                            verb: rhs.verb.clone(),
                            rhs: Box::new(Node::Ident(tmp.clone())),
                        },
                    }
                    .purify(),
                ]))
            }
            Macro::If { comp, terms } => {
                let mut tmp = random_identifier();

                Node::Control(Control::Terms(vec![
                    Node::Control(Control::Loop {
                        ident: match *comp.clone() {
                            Node::Comparison { lhs, verb, rhs } => lhs.clone(),
                            _ => panic!("Unexpected argument for identifier."),
                        },
                        terms: Box::new(Node::Control(Control::Terms(vec![
                            Macro::AssignToValue {
                                lhs: Box::new(Node::Ident(tmp.clone())),
                                rhs: Box::new(Node::NaturalNumber(BigUint::from(1u32))),
                            }
                            .purify(),
                        ]))),
                    }),
                    Node::Control(Control::Loop {
                        ident: Box::new(Node::Ident(tmp.clone())),
                        terms: Box::new(terms.clone().purify()),
                    }),
                ]))
            }
            Macro::IfElse {
                comp,
                if_terms,
                else_terms,
            } => {
                let mut tmp1 = random_identifier();
                let mut tmp2 = random_identifier();
                let mut tmp3 = random_identifier();

                // TODO: implement other things than > ?
                Node::Control(Control::Terms(vec![
                    Macro::AssignToOpIdent {
                        lhs: Box::new(Node::Ident(tmp1.clone())),
                        rhs: MacroBinaryAssignOperation {
                            lhs: match *comp.clone() {
                                Node::Comparison {
                                    lhs,
                                    rhs: _,
                                    verb: _,
                                } => lhs,
                                _ => panic!(
                                    "Comparison for IF ... THEN ... ELSE needs to be comparison!"
                                ),
                            },
                            verb: OperatorVerb::Minus,
                            rhs: match *comp.clone() {
                                Node::Comparison {
                                    lhs: _,
                                    rhs,
                                    verb: _,
                                } => rhs,
                                _ => panic!(
                                    "Comparison for IF ... THEN ... ELSE needs to be comparison!"
                                ),
                            },
                        },
                    }
                    .purify(),
                    Macro::AssignToZero {
                        lhs: Box::new(Node::Ident(tmp2.clone())),
                    }
                    .purify(),
                    Macro::AssignToValue {
                        lhs: Box::new(Node::Ident(tmp3.clone())),
                        rhs: Box::new(Node::NaturalNumber(BigUint::from(1u32))),
                    }
                    .purify(),
                    Node::Control(Control::Loop {
                        ident: Box::new(Node::Ident(tmp1.clone())),
                        terms: Box::new(Node::Control(Control::Terms(vec![
                            Macro::AssignToValue {
                                lhs: Box::new(Node::Ident(tmp2.clone())),
                                rhs: Box::new(Node::NaturalNumber(BigUint::from(1u32))),
                            }
                            .purify(),
                            Macro::AssignToZero {
                                lhs: Box::new(Node::Ident(tmp3.clone())),
                            }
                            .purify(),
                        ]))),
                    }),
                    Node::Control(Control::Loop {
                        ident: Box::new(Node::Ident(tmp2.clone())),
                        terms: Box::new(if_terms.purify()),
                    }),
                    Node::Control(Control::Loop {
                        ident: Box::new(Node::Ident(tmp3.clone())),
                        terms: Box::new(else_terms.purify()),
                    }),
                ]))
            }
            _ => panic!("Macro currently not implemented"),
        }
    }
}

// Control Structures have in their body potentially
// polluted information, these need to changed/unpolluted via
// macro expansion
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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

impl Node {
    pub fn flatten(&self) -> Node {
        match self {
            Node::Control(Control::Terms(terms)) => Node::Control(Control::Terms(
                terms
                    .iter()
                    .flat_map(|node| match node {
                        Node::Control(Control::Terms(t)) => t
                            .iter()
                            .flat_map(|term| {
                                let flat = term.flatten();

                                match flat {
                                    Node::Control(Control::Terms(t)) => t,
                                    _ => vec![flat],
                                }
                            })
                            .collect(),
                        Node::Control(Control::Loop { ident, terms }) => {
                            vec![Node::Control(Control::Loop {
                                ident: ident.clone(),
                                terms: Box::new(terms.flatten()),
                            })]
                        }
                        Node::Control(Control::While { comp, terms }) => {
                            vec![Node::Control(Control::While {
                                comp: comp.clone(),
                                terms: Box::new(terms.flatten()),
                            })]
                        }
                        _ => vec![node.clone()],
                    })
                    .collect(),
            )),
            Node::Control(Control::Loop { ident, terms }) => Node::Control(Control::Loop {
                ident: ident.clone(),
                terms: Box::new(terms.flatten()),
            }),
            Node::Control(Control::While { comp, terms }) => Node::Control(Control::While {
                comp: comp.clone(),
                terms: Box::new(terms.flatten()),
            }),
            _ => self.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum PollutedNode {
    Pure(Node),
    Macro(Macro),
    NoOp,

    Control(Control<PollutedNode>),
}

impl PollutedNode {
    pub fn purify(&self) -> Node {
        match self {
            // Control Nodes
            PollutedNode::Control(Control::Terms(t)) => {
                Node::Control(Control::Terms(t.iter().map(|term| term.purify()).collect()))
            }
            PollutedNode::Control(Control::Loop { ident, terms }) => Node::Control(Control::Loop {
                ident: Box::new(ident.purify()),
                terms: Box::new(terms.purify()),
            }),
            PollutedNode::Control(Control::While { comp, terms }) => {
                Node::Control(Control::While {
                    comp: Box::new(comp.purify()),
                    terms: Box::new(terms.purify()),
                })
            }
            PollutedNode::NoOp => Node::Control(Control::Terms(vec![])),
            PollutedNode::Pure(n) => n.clone(),
            PollutedNode::Macro(m) => m.purify(),
            _ => panic!("Cannot Purify!"),
        }
    }
}

// TODO: flatten method
