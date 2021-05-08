use num_bigint::BigUint;
#[cfg(feature = "cli")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::ast::context::CompileContext;
use crate::ast::control::Control;
use crate::ast::macros::Macro;
use crate::ast::node::Node;
use crate::ast::variant::UInt;
use crate::ast::verbs::{ComparisonVerb, OperatorVerb};
use crate::errors::{Error, ErrorVariant};
use crate::flags::CompilationFlags;
use crate::utils::private_identifier;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "cli", derive(JsonSchema))]
pub enum PollutedNode {
    Pure(Node),
    Macro(Macro),
    NoOp,

    Control(Control<PollutedNode>),
}

// [REWORK] the match mess into a separate file and functions
impl PollutedNode {
    fn expand_merge_errors(maybe: &[Result<Node, Vec<Error>>]) -> Vec<Error> {
        maybe
            .iter()
            .filter(|f| f.is_err())
            .flat_map(|f| f.clone().unwrap_err())
            .collect()
    }

    pub fn expand(&self, context: &mut CompileContext) -> Result<Node, Vec<Error>> {
        let result = match self {
            // Control Nodes
            PollutedNode::Control(Control::Terms(t)) => Node::Control(Control::Terms({
                let terms: Vec<Result<Node, Vec<Error>>> =
                    t.iter().map(|term| term.expand(context)).collect();

                let errors = PollutedNode::expand_merge_errors(&terms);
                if !errors.is_empty() {
                    return Err(errors);
                }

                let res: Vec<Node> = terms.iter().map(|t| t.clone().ok().unwrap()).collect();
                Ok::<Vec<Node>, Vec<Error>>(res)
            }?)),
            PollutedNode::Control(Control::Loop { lno, ident, terms }) => {
                // instead of instantly returning we collect the errors of both and then return if there
                // are any, this way the end user knows a lot more.
                let maybe_ident = ident.expand(context);
                let maybe_terms = terms.expand(context);

                let maybe = vec![maybe_ident.clone(), maybe_terms.clone()];
                let errors = PollutedNode::expand_merge_errors(&maybe);

                if !errors.is_empty() {
                    return Err(errors);
                }

                if context.flags.contains(CompilationFlags::LOOP) {
                    Node::Control(Control::Loop {
                        lno: *lno,
                        ident: Box::new(maybe_ident.unwrap()),
                        terms: Box::new(maybe_terms.unwrap()),
                    })
                } else if context.flags.contains(CompilationFlags::WHILE) {
                    // rewrite as WHILE
                    let tmp1 = private_identifier(context);

                    Node::Control(Control::Terms(vec![
                        Macro::AssignToIdent {
                            lno: *lno,
                            lhs: Box::new(Node::Ident(tmp1.clone())),
                            rhs: Box::new(maybe_ident.unwrap()),
                        }
                        .expand(context)?,
                        Node::Control(Control::While {
                            lno: *lno,
                            comp: Box::new(Node::Comparison {
                                lhs: Box::new(Node::Ident(tmp1.clone())),
                                verb: ComparisonVerb::NotEqual,
                                rhs: Box::new(Node::NaturalNumber(UInt(BigUint::from(
                                    0u8,
                                )))),
                            }),
                            terms: Box::new(Node::Control(Control::Terms(vec![
                                maybe_terms.unwrap(),
                                Node::Assign {
                                    lno: *lno,
                                    lhs: Box::new(Node::Ident(tmp1.clone())),
                                    rhs: Box::new(Node::BinaryOp {
                                        lhs: Box::new(Node::Ident(tmp1)),
                                        verb: OperatorVerb::Minus,
                                        rhs: Box::new(Node::NaturalNumber(UInt(
                                            BigUint::from(1u8),
                                        ))),
                                    }),
                                },
                            ]))),
                        }),
                    ]))
                } else {
                    return Err(vec![Error::new(
                        *lno,
                        ErrorVariant::Message(String::from(
                            "Cannot use LOOP if LOOP and WHILE are not enabled!",
                        )),
                    )]);
                }
            }

            PollutedNode::Control(Control::While { lno, comp, terms }) => {
                let maybe_comp = comp.expand(context);
                let maybe_terms = terms.expand(context);

                let maybe = vec![maybe_comp.clone(), maybe_terms.clone()];
                let mut errors = PollutedNode::expand_merge_errors(&maybe);
                if !context.flags.contains(CompilationFlags::WHILE) {
                    errors.push(Error::new(
                        *lno,
                        ErrorVariant::Message(String::from("Cannot replicate WHILE in LOOP mode!")),
                    ));
                }

                if !errors.is_empty() {
                    return Err(errors);
                }

                Node::Control(Control::While {
                    lno: *lno,
                    comp: Box::new(maybe_comp.unwrap()),
                    terms: Box::new(maybe_terms.unwrap()),
                })
            }
            PollutedNode::NoOp => Node::Control(Control::Terms(vec![])),
            PollutedNode::Pure(n) => n.clone(),
            PollutedNode::Macro(m) => m.expand(context)?,
        };

        Ok(result)
    }
}
