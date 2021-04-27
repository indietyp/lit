use crate::ast::control::Control;
use crate::ast::macro_::Macro;
use crate::ast::node::Node;
use crate::ast::node::Node::Ident;
use crate::ast::verbs::{ComparisonVerb, OperatorVerb};
use crate::flags::CompilationFlags;
use crate::utils::private_random_identifier;
use num_bigint::BigUint;
use std::io::empty;

#[derive(Debug, Clone)]
pub enum PollutedNode {
    Pure(Node),
    Macro(Macro),
    NoOp,

    Control(Control<PollutedNode>),
}

impl PollutedNode {
    pub fn expand(&self, flags: CompilationFlags) -> Node {
        match self {
            // Control Nodes
            PollutedNode::Control(Control::Terms(t)) => Node::Control(Control::Terms(
                t.iter().map(|term| term.expand(flags)).collect(),
            )),
            PollutedNode::Control(Control::Loop { lno, ident, terms }) => {
                if flags.contains(CompilationFlags::LOOP) {
                    Node::Control(Control::Loop {
                        lno: lno.clone(),
                        ident: Box::new(ident.expand(flags)),
                        terms: Box::new(terms.expand(flags)),
                    })
                } else if flags.contains(CompilationFlags::WHILE) {
                    // rewrite as WHILE
                    let tmp1 = private_random_identifier();

                    Node::Control(Control::Terms(vec![
                        Macro::AssignToIdent {
                            lno: lno.clone(),
                            lhs: Box::new(Node::Ident(tmp1.clone())),
                            rhs: Box::new(ident.clone().expand(flags)),
                        }
                        .expand(flags),
                        Node::Control(Control::While {
                            lno: lno.clone(),
                            comp: Box::new(Node::Comparison {
                                lhs: Box::new(Node::Ident(tmp1.clone())),
                                verb: ComparisonVerb::NotEqual,
                                rhs: Box::new(Node::NaturalNumber(BigUint::from(0u8))),
                            }),
                            terms: Box::new(Node::Control(Control::Terms(vec![
                                terms.expand(flags),
                                Node::Assign {
                                    lno: lno.clone(),
                                    lhs: Box::new(Node::Ident(tmp1.clone())),
                                    rhs: Box::new(Node::BinaryOp {
                                        lhs: Box::new(Node::Ident(tmp1.clone())),
                                        verb: OperatorVerb::Minus,
                                        rhs: Box::new(Node::NaturalNumber(BigUint::from(1u8))),
                                    }),
                                },
                            ]))),
                        }),
                    ]))
                } else {
                    panic!("Cannot use LOOP if LOOP and WHILE are not enabled!")
                }
            }
            PollutedNode::Control(Control::While { lno, comp, terms }) => {
                assert!(
                    flags.contains(CompilationFlags::WHILE),
                    "Cannot replicate WHILE in LOOP mode!",
                );

                Node::Control(Control::While {
                    lno: lno.clone(),
                    comp: Box::new(comp.expand(flags)),
                    terms: Box::new(terms.expand(flags)),
                })
            }
            PollutedNode::NoOp => Node::Control(Control::Terms(vec![])),
            PollutedNode::Pure(n) => n.clone(),
            PollutedNode::Macro(m) => m.expand(flags),
        }
    }
}
