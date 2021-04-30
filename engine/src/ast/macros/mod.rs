use num_bigint::BigUint;

use crate::ast::context::CompileContext;
use crate::ast::control::Control;
use crate::ast::node::Node;
use crate::ast::polluted::PollutedNode;
use crate::ast::verbs::OperatorVerb;

use crate::types::LineNo;
use crate::utils::private_identifier;

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
        lno: LineNo,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    AssignToZero {
        lno: LineNo,
        lhs: Box<Node>,
    },
    AssignToValue {
        lno: LineNo,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    AssignToOpIdent {
        lno: LineNo,
        lhs: Box<Node>,
        rhs: MacroAssign,
    },
    AssignToOpExtIdent {
        lno: LineNo,
        lhs: Box<Node>,
        rhs: MacroAssign,
    },
    AssignToOpExtValue {
        lno: LineNo,
        lhs: Box<Node>,
        rhs: MacroAssign,
    },
    If {
        lno: LineNo, // this is tuple/range - if has a body
        comp: Box<Node>,
        terms: Box<PollutedNode>,
    },
    IfElse {
        lno: LineNo,
        comp: Box<Node>,
        if_terms: Box<PollutedNode>,
        else_terms: Box<PollutedNode>,
    },
}

impl Macro {
    fn expand_assign_to_value(&self, ident: String, value: u32, lno: LineNo) -> Macro {
        Macro::AssignToValue {
            lno,
            lhs: Box::new(Node::Ident(ident)),
            rhs: Box::new(Node::NaturalNumber(BigUint::from(value))),
        }
    }

    pub fn expand(&self, context: &mut CompileContext) -> Node {
        match self {
            Macro::AssignToIdent { lno, lhs, rhs } => Node::Assign {
                lno: *lno,
                lhs: lhs.clone(),
                rhs: Box::new(Node::BinaryOp {
                    verb: OperatorVerb::Plus,
                    lhs: rhs.clone(),
                    rhs: Box::new(Node::NaturalNumber(BigUint::from(0u32))),
                }),
            },
            Macro::AssignToZero { lno, lhs } => PollutedNode::Control(Control::Loop {
                lno: *lno,
                ident: Box::new(PollutedNode::Pure(*lhs.clone())),
                terms: Box::new(PollutedNode::Control(Control::Terms(vec![
                    PollutedNode::Pure(Node::Assign {
                        lno: *lno,
                        lhs: lhs.clone(),
                        rhs: Box::new(Node::BinaryOp {
                            verb: OperatorVerb::Minus,
                            lhs: lhs.clone(),
                            rhs: Box::new(Node::NaturalNumber(BigUint::from(1u32))),
                        }),
                    }),
                ]))),
            })
            .expand(context),
            Macro::AssignToValue { lno, lhs, rhs } => Node::Control(Control::Terms(vec![
                Macro::AssignToZero {
                    lno: *lno,
                    lhs: lhs.clone(),
                }
                .expand(context),
                Node::Assign {
                    lno: *lno,
                    lhs: lhs.clone(),
                    rhs: Box::new(Node::BinaryOp {
                        lhs: lhs.clone(),
                        rhs: rhs.clone(),
                        verb: OperatorVerb::Plus,
                    }),
                },
            ])),
            Macro::AssignToOpIdent { lno, lhs, rhs } => Node::Control(Control::Terms(vec![
                Macro::AssignToIdent {
                    lno: *lno,
                    lhs: lhs.clone(),
                    rhs: rhs.lhs.clone(),
                }
                .expand(context),
                PollutedNode::Control(Control::Loop {
                    lno: *lno,
                    ident: Box::new(PollutedNode::Pure(*rhs.rhs.clone())),
                    terms: Box::new(PollutedNode::Control(Control::Terms(vec![
                        PollutedNode::Pure(Node::Assign {
                            lno: *lno,
                            lhs: lhs.clone(),
                            rhs: Box::new(Node::BinaryOp {
                                lhs: lhs.clone(),
                                rhs: Box::new(Node::NaturalNumber(BigUint::from(1u32))),
                                verb: rhs.verb.clone(),
                            }),
                        }),
                    ]))),
                })
                .expand(context),
            ])),
            Macro::AssignToOpExtIdent { lno, lhs, rhs } => Node::Control(Control::Terms(vec![
                Macro::AssignToZero {
                    lno: *lno,
                    lhs: lhs.clone(),
                }
                .expand(context),
                PollutedNode::Control(Control::Loop {
                    lno: *lno,
                    ident: Box::new(PollutedNode::Pure(*rhs.lhs.clone())),
                    terms: Box::new(PollutedNode::Control(Control::Terms(vec![
                        PollutedNode::Macro(Macro::AssignToOpIdent {
                            lno: *lno,
                            lhs: lhs.clone(),
                            rhs: MacroAssign {
                                lhs: lhs.clone(),
                                verb: OperatorVerb::Plus,
                                rhs: rhs.rhs.clone(),
                            },
                        }),
                    ]))),
                })
                .expand(context),
            ])),
            Macro::AssignToOpExtValue { lno, lhs, rhs } => {
                let tmp = private_identifier(context);

                Node::Control(Control::Terms(vec![
                    Macro::AssignToValue {
                        lno: *lno,
                        lhs: Box::new(Node::Ident(tmp.clone())),
                        rhs: rhs.rhs.clone(),
                    }
                    .expand(context),
                    Macro::AssignToOpExtIdent {
                        lno: *lno,
                        lhs: lhs.clone(),
                        rhs: MacroAssign {
                            lhs: rhs.lhs.clone(),
                            verb: rhs.verb.clone(),
                            rhs: Box::new(Node::Ident(tmp)),
                        },
                    }
                    .expand(context),
                ]))
            }
            Macro::If { lno, comp, terms } => {
                let tmp = private_identifier(context);

                Node::Control(Control::Terms(vec![
                    PollutedNode::Control(Control::Loop {
                        lno: *lno,
                        ident: match *comp.clone() {
                            Node::Comparison {
                                lhs,
                                verb: _,
                                rhs: _,
                            } => Box::new(PollutedNode::Pure(*lhs)),
                            _ => panic!("Unexpected argument for identifier."),
                        },
                        terms: Box::new(PollutedNode::Control(Control::Terms(vec![
                            PollutedNode::Macro(self.expand_assign_to_value(tmp.clone(), 1, *lno)),
                        ]))),
                    })
                    .expand(context),
                    PollutedNode::Control(Control::Loop {
                        lno: *lno,
                        ident: Box::new(PollutedNode::Pure(Node::Ident(tmp))),
                        terms: terms.clone(),
                    })
                    .expand(context),
                ]))
            }
            Macro::IfElse {
                lno,
                comp,
                if_terms,
                else_terms,
            } => {
                let tmp1 = private_identifier(context);
                let tmp2 = private_identifier(context);
                let tmp3 = private_identifier(context);

                // TODO: implement other things than > ?
                Node::Control(Control::Terms(vec![
                    Macro::AssignToOpIdent {
                        lno: *lno,
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
                    .expand(context),
                    Macro::AssignToZero {
                        lno: *lno,
                        lhs: Box::new(Node::Ident(tmp2.clone())),
                    }
                    .expand(context),
                    self.expand_assign_to_value(tmp3.clone(), 1, *lno)
                        .expand(context),
                    PollutedNode::Control(Control::Loop {
                        lno: *lno,
                        ident: Box::new(PollutedNode::Pure(Node::Ident(tmp1))),
                        terms: Box::new(PollutedNode::Control(Control::Terms(vec![
                            PollutedNode::Macro(self.expand_assign_to_value(tmp2.clone(), 1, *lno)),
                            PollutedNode::Macro(Macro::AssignToZero {
                                lno: *lno,
                                lhs: Box::new(Node::Ident(tmp3.clone())),
                            }),
                        ]))),
                    })
                    .expand(context),
                    PollutedNode::Control(Control::Loop {
                        lno: *lno,
                        ident: Box::new(PollutedNode::Pure(Node::Ident(tmp2))),
                        terms: if_terms.clone(),
                    })
                    .expand(context),
                    PollutedNode::Control(Control::Loop {
                        lno: *lno,
                        ident: Box::new(PollutedNode::Pure(Node::Ident(tmp3))),
                        terms: else_terms.clone(),
                    })
                    .expand(context),
                ]))
            }
        }
    }
}
