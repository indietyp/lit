use crate::ast::control::Control;
use crate::ast::node::Node;
use crate::ast::polluted::PollutedNode;
use crate::ast::verbs::OperatorVerb;
use crate::flags::CompilationFlags;
use crate::types::LineNo;
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
    fn expand_assign_to_value(
        &self,
        ident: String,
        value: u32,
        flags: CompilationFlags,
        lno: LineNo,
    ) -> Node {
        Macro::AssignToValue {
            lno: lno.clone(),
            lhs: Box::new(Node::Ident(ident.clone())),
            rhs: Box::new(Node::NaturalNumber(BigUint::from(value))),
        }
        .expand(flags)
    }

    pub fn expand(&self, flags: CompilationFlags) -> Node {
        match self {
            Macro::AssignToIdent { lno, lhs, rhs } => Node::Assign {
                lno: lno.clone(),
                lhs: lhs.clone(),
                rhs: Box::new(Node::BinaryOp {
                    verb: OperatorVerb::Plus,
                    lhs: rhs.clone(),
                    rhs: Box::new(Node::NaturalNumber(BigUint::from(0u32))),
                }),
            },
            Macro::AssignToZero { lno, lhs } => Node::Control(Control::Loop {
                lno: lno.clone(),
                ident: lhs.clone(),
                terms: Box::new(Node::Control(Control::Terms(vec![Node::Assign {
                    lno: lno.clone(),
                    lhs: lhs.clone(),
                    rhs: Box::new(Node::BinaryOp {
                        verb: OperatorVerb::Minus,
                        lhs: lhs.clone(),
                        rhs: Box::new(Node::NaturalNumber(BigUint::from(1u32))),
                    }),
                }]))),
            }),
            Macro::AssignToValue { lno, lhs, rhs } => Node::Control(Control::Terms(vec![
                Macro::AssignToZero {
                    lno: lno.clone(),
                    lhs: lhs.clone(),
                }
                .expand(flags),
                Node::Assign {
                    lno: lno.clone(),
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
                    lno: lno.clone(),
                    lhs: lhs.clone(),
                    rhs: rhs.lhs.clone(),
                }
                .expand(flags),
                Node::Control(Control::Loop {
                    lno: lno.clone(),
                    ident: rhs.rhs.clone(),
                    terms: Box::new(Node::Control(Control::Terms(vec![Node::Assign {
                        lno: lno.clone(),
                        lhs: lhs.clone(),
                        rhs: Box::new(Node::BinaryOp {
                            lhs: lhs.clone(),
                            rhs: Box::new(Node::NaturalNumber(BigUint::from(1u32))),
                            verb: rhs.verb.clone(),
                        }),
                    }]))),
                }),
            ])),
            Macro::AssignToOpExtIdent { lno, lhs, rhs } => Node::Control(Control::Terms(vec![
                Macro::AssignToZero {
                    lno: lno.clone(),
                    lhs: lhs.clone(),
                }
                .expand(flags),
                Node::Control(Control::Loop {
                    lno: lno.clone(),
                    ident: rhs.lhs.clone(),
                    terms: Box::new(Node::Control(Control::Terms(vec![
                        Macro::AssignToOpIdent {
                            lno: lno.clone(),
                            lhs: lhs.clone(),
                            rhs: MacroAssign {
                                lhs: lhs.clone(),
                                verb: OperatorVerb::Plus,
                                rhs: rhs.rhs.clone(),
                            },
                        }
                        .expand(flags),
                    ]))),
                }),
            ])),
            Macro::AssignToOpExtValue { lno, lhs, rhs } => {
                let tmp = private_random_identifier();

                Node::Control(Control::Terms(vec![
                    Macro::AssignToValue {
                        lno: lno.clone(),
                        lhs: Box::new(Node::Ident(tmp.clone())),
                        rhs: rhs.rhs.clone(),
                    }
                    .expand(flags),
                    Macro::AssignToOpExtIdent {
                        lno: lno.clone(),
                        lhs: lhs.clone(),
                        rhs: MacroAssign {
                            lhs: rhs.lhs.clone(),
                            verb: rhs.verb.clone(),
                            rhs: Box::new(Node::Ident(tmp.clone())),
                        },
                    }
                    .expand(flags),
                ]))
            }
            Macro::If { lno, comp, terms } => {
                let tmp = private_random_identifier();

                Node::Control(Control::Terms(vec![
                    Node::Control(Control::Loop {
                        lno: lno.clone(),
                        ident: match *comp.clone() {
                            Node::Comparison {
                                lhs,
                                verb: _,
                                rhs: _,
                            } => lhs.clone(),
                            _ => panic!("Unexpected argument for identifier."),
                        },
                        terms: Box::new(Node::Control(Control::Terms(vec![
                            self.expand_assign_to_value(tmp.clone(), 1, flags, lno.clone())
                        ]))),
                    }),
                    Node::Control(Control::Loop {
                        lno: lno.clone(),
                        ident: Box::new(Node::Ident(tmp.clone())),
                        terms: Box::new(terms.clone().expand(flags)),
                    }),
                ]))
            }
            Macro::IfElse {
                lno,
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
                        lno: lno.clone(),
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
                    .expand(flags),
                    Macro::AssignToZero {
                        lno: lno.clone(),
                        lhs: Box::new(Node::Ident(tmp2.clone())),
                    }
                    .expand(flags),
                    self.expand_assign_to_value(tmp3.clone(), 1, flags, lno.clone()),
                    Node::Control(Control::Loop {
                        lno: lno.clone(),
                        ident: Box::new(Node::Ident(tmp1.clone())),
                        terms: Box::new(Node::Control(Control::Terms(vec![
                            self.expand_assign_to_value(tmp2.clone(), 1, flags, lno.clone()),
                            Macro::AssignToZero {
                                lno: lno.clone(),
                                lhs: Box::new(Node::Ident(tmp3.clone())),
                            }
                            .expand(flags),
                        ]))),
                    }),
                    Node::Control(Control::Loop {
                        lno: lno.clone(),
                        ident: Box::new(Node::Ident(tmp2.clone())),
                        terms: Box::new(if_terms.expand(flags)),
                    }),
                    Node::Control(Control::Loop {
                        lno: lno.clone(),
                        ident: Box::new(Node::Ident(tmp3.clone())),
                        terms: Box::new(else_terms.expand(flags)),
                    }),
                ]))
            }
        }
    }
}
