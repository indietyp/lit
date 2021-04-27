use num_bigint::BigUint;

use crate::ast::control::Control;
use crate::ast::verbs::{ComparisonVerb, OperatorVerb};
use crate::eval::assign::AssignExec;
use crate::eval::loop_::LoopExec;
use crate::eval::terms::TermsExec;
use crate::eval::traits::Executable;
use crate::eval::while_::WhileExec;
use crate::types::LineNo;

// Note(bmahmoud): in the future we could also support unary expressions?
#[derive(Debug, Clone)]
pub enum Node {
    // Smallest Units
    Ident(String),
    NaturalNumber(BigUint),

    // Assignment and Expressions
    Comparison {
        lhs: Box<Node>,
        verb: ComparisonVerb,
        rhs: Box<Node>,
    },
    BinaryOp {
        lhs: Box<Node>,
        verb: OperatorVerb,
        rhs: Box<Node>,
    },
    Assign {
        lno: LineNo,
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
                        Node::Control(Control::Loop { lno, ident, terms }) => {
                            vec![Node::Control(Control::Loop {
                                lno: *lno,
                                ident: ident.clone(),
                                terms: Box::new(terms.flatten()),
                            })]
                        }
                        Node::Control(Control::While { lno, comp, terms }) => {
                            vec![Node::Control(Control::While {
                                lno: *lno,
                                comp: comp.clone(),
                                terms: Box::new(terms.flatten()),
                            })]
                        }
                        _ => vec![node.clone()],
                    })
                    .collect(),
            )),
            Node::Control(Control::Loop { lno, ident, terms }) => Node::Control(Control::Loop {
                lno: *lno,
                ident: ident.clone(),
                terms: Box::new(terms.flatten()),
            }),
            Node::Control(Control::While { lno, comp, terms }) => Node::Control(Control::While {
                lno: *lno,
                comp: comp.clone(),
                terms: Box::new(terms.flatten()),
            }),
            _ => self.clone(),
        }
    }

    /* Display human friendly representation */
    pub fn display(&self, indent: u8, cur: Option<u8>) -> String {
        let cur = cur.or(Some(0));
        let prefix = " ".repeat((indent * cur.unwrap()) as usize);

        match self {
            Node::Ident(s) => s.clone(),
            Node::NaturalNumber(n) => n.to_string(),
            Node::Comparison { lhs, verb, rhs } => format!(
                "{} {} {}",
                lhs.display(indent, cur),
                verb.display(),
                rhs.display(indent, cur)
            ),
            Node::BinaryOp { lhs, verb, rhs } => format!(
                "{} {} {}",
                lhs.display(indent, cur),
                verb.display(),
                rhs.display(indent, cur)
            ),
            Node::Assign { lno: _, lhs, rhs } => format!(
                "{}{} := {}",
                prefix,
                lhs.display(indent, cur),
                rhs.display(indent, cur)
            ),
            Node::Control(Control::Terms(terms)) => terms
                .iter()
                .map(|term| term.display(indent, cur))
                .collect::<Vec<String>>()
                .join("\n"),
            Node::Control(Control::Loop {
                lno: _,
                ident,
                terms,
            }) => format!(
                "{prefix}LOOP {} DO\n{}\n{prefix}END",
                ident.display(indent, cur),
                terms.display(indent, cur.map(|c| c + 1)),
                prefix = prefix
            ),
            Node::Control(Control::While {
                lno: _,
                comp,
                terms,
            }) => format!(
                "{prefix}WHILE {} DO\n{}\n{prefix}END",
                comp.display(indent, cur),
                terms.display(indent, cur.map(|c| c + 1)),
                prefix = prefix
            ),
        }
    }
}
