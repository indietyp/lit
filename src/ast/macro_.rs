use crate::ast::control::Control;
use crate::ast::node::Node;
use crate::ast::polluted::PollutedNode;
use crate::ast::verbs::OperatorVerb;
use crate::utils::private_random_identifier;
use num_bigint::BigUint;

// This is a shorthand for the Node::Assign,
// I would love to make this one go away, but I have no idea how.
#[derive(Debug, Clone)]
pub struct MacroAssign {
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
        rhs: MacroAssign,
    },
    AssignToOpExtIdent {
        lhs: Box<Node>,
        rhs: MacroAssign,
    },
    AssignToOpExtValue {
        lhs: Box<Node>,
        rhs: MacroAssign,
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
                rhs: Box::new(Node::BinaryOp {
                    verb: OperatorVerb::Plus,
                    lhs: rhs.clone(),
                    rhs: Box::new(Node::NaturalNumber(BigUint::from(0u32))),
                }),
            },
            Macro::AssignToZero { lhs } => Node::Control(Control::Loop {
                ident: lhs.clone(),
                terms: Box::new(Node::Control(Control::Terms(vec![Node::Assign {
                    lhs: lhs.clone(),
                    rhs: Box::new(Node::BinaryOp {
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
                    rhs: Box::new(Node::BinaryOp {
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
                        rhs: Box::new(Node::BinaryOp {
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
                            rhs: MacroAssign {
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
                let tmp = private_random_identifier();

                Node::Control(Control::Terms(vec![
                    Macro::AssignToValue {
                        lhs: Box::new(Node::Ident(tmp.clone())),
                        rhs: rhs.rhs.clone(),
                    }
                    .purify(),
                    Macro::AssignToOpExtIdent {
                        lhs: lhs.clone(),
                        rhs: MacroAssign {
                            lhs: rhs.lhs.clone(),
                            verb: rhs.verb.clone(),
                            rhs: Box::new(Node::Ident(tmp.clone())),
                        },
                    }
                    .purify(),
                ]))
            }
            Macro::If { comp, terms } => {
                let tmp = private_random_identifier();

                Node::Control(Control::Terms(vec![
                    Node::Control(Control::Loop {
                        ident: match *comp.clone() {
                            Node::Comparison {
                                lhs,
                                verb: _,
                                rhs: _,
                            } => lhs.clone(),
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
                let tmp1 = private_random_identifier();
                let tmp2 = private_random_identifier();
                let tmp3 = private_random_identifier();

                // TODO: implement other things than > ?
                Node::Control(Control::Terms(vec![
                    Macro::AssignToOpIdent {
                        lhs: Box::new(Node::Ident(tmp1.clone())),
                        rhs: MacroAssign {
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
        }
    }
}
