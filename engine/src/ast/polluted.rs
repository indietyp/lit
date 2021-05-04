use num_bigint::BigUint;

use crate::ast::context::CompileContext;
use crate::ast::control::Control;
use crate::ast::macros::Macro;
use crate::ast::node::Node;

use crate::ast::verbs::{ComparisonVerb, OperatorVerb};
use crate::errors::{Error, ErrorVariant};
use crate::flags::CompilationFlags;
use crate::utils::private_identifier;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PollutedNode {
    Pure(Node),
    Macro(Macro),
    NoOp,

    Control(Control<PollutedNode>),
}

impl PollutedNode {
    pub fn expand(&self, context: &mut CompileContext) -> Result<Node, Vec<Error>> {
        let result = match self {
            // Control Nodes
            PollutedNode::Control(Control::Terms(t)) => Node::Control(Control::Terms({
                let terms: Vec<Result<Node, Vec<Error>>> =
                    t.iter().map(|term| term.expand(context)).collect();

                let errors: Vec<Error> = terms
                    .clone()
                    .iter()
                    .filter(|f| f.is_err())
                    .flat_map(|f| f.clone().unwrap_err())
                    .collect();
                if errors.len() > 0 {
                    return Err(errors);
                }

                let res: Vec<Node> = terms
                    .clone()
                    .iter()
                    .map(|t| t.clone().ok().unwrap())
                    .collect();
                Ok::<Vec<Node>, Vec<Error>>(res)
            }?)),
            PollutedNode::Control(Control::Loop { lno, ident, terms }) => {
                if context.flags.contains(CompilationFlags::LOOP) {
                    Node::Control(Control::Loop {
                        lno: *lno,
                        ident: Box::new(ident.expand(context)?),
                        terms: Box::new(terms.expand(context)?),
                    })
                } else if context.flags.contains(CompilationFlags::WHILE) {
                    // rewrite as WHILE
                    let tmp1 = private_identifier(context);

                    Node::Control(Control::Terms(vec![
                        Macro::AssignToIdent {
                            lno: *lno,
                            lhs: Box::new(Node::Ident(tmp1.clone())),
                            rhs: Box::new(ident.clone().expand(context)?),
                        }
                        .expand(context)?,
                        Node::Control(Control::While {
                            lno: *lno,
                            comp: Box::new(Node::Comparison {
                                lhs: Box::new(Node::Ident(tmp1.clone())),
                                verb: ComparisonVerb::NotEqual,
                                rhs: Box::new(Node::NaturalNumber(BigUint::from(0u8))),
                            }),
                            terms: Box::new(Node::Control(Control::Terms(vec![
                                terms.expand(context)?,
                                Node::Assign {
                                    lno: *lno,
                                    lhs: Box::new(Node::Ident(tmp1.clone())),
                                    rhs: Box::new(Node::BinaryOp {
                                        lhs: Box::new(Node::Ident(tmp1)),
                                        verb: OperatorVerb::Minus,
                                        rhs: Box::new(Node::NaturalNumber(BigUint::from(1u8))),
                                    }),
                                },
                            ]))),
                        }),
                    ]))
                } else {
                    return Err(vec![Error::new(
                        lno.clone(),
                        ErrorVariant::Message(String::from(
                            "Cannot use LOOP if LOOP and WHILE are not enabled!",
                        )),
                    )]);
                }
            }

            PollutedNode::Control(Control::While { lno, comp, terms }) => {
                if !context.flags.contains(CompilationFlags::WHILE) {
                    return Err(vec![Error::new(
                        lno.clone(),
                        ErrorVariant::Message(String::from("Cannot replicate WHILE in LOOP mode!")),
                    )]);
                }

                Node::Control(Control::While {
                    lno: *lno,
                    comp: Box::new(comp.expand(context)?),
                    terms: Box::new(terms.expand(context)?),
                })
            }
            PollutedNode::NoOp => Node::Control(Control::Terms(vec![])),
            PollutedNode::Pure(n) => n.clone(),
            PollutedNode::Macro(m) => m.expand(context)?,
        };

        Ok(result)
    }
}
