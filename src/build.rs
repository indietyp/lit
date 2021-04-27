use core::option::Option::Some;
use std::str::FromStr;

use num_bigint::BigUint;
use pest::iterators::{Pair, Pairs};

use crate::ast::control::Control;
use crate::ast::macro_::{Macro, MacroAssign};
use crate::ast::node::Node;
use crate::ast::polluted::PollutedNode;
use crate::ast::verbs::{ComparisonVerb, OperatorVerb};
use crate::LoopParser;
use crate::Rule;
use pest::error::Error;

pub struct Builder {}

impl Builder {
    fn build_pure(pair: Pair<Rule>) -> Node {
        match pair.as_rule() {
            // Terminal Encoding
            Rule::IDENT => Node::Ident(String::from(pair.as_str())),
            Rule::VALUE => Node::NaturalNumber(BigUint::from_str(pair.as_str()).unwrap()),

            // Comparison
            Rule::compEqual
            | Rule::compNotEqual
            | Rule::compGreaterThan
            | Rule::compGreaterThanIdent
            | Rule::compGreaterEqual
            | Rule::compLessThan
            | Rule::compLessEqual => {
                let mut pair = pair.into_inner();

                let lhs = Box::new(Builder::build_pure(pair.next().unwrap()));
                let verb = pair.next().unwrap().as_str();
                let rhs = Box::new(Builder::build_pure(pair.next().unwrap()));

                Node::Comparison {
                    lhs,
                    verb: ComparisonVerb::from(verb),
                    rhs,
                }
            }

            // Binary Operator
            Rule::binaryOp => {
                let mut pair = pair.into_inner();
                let lhs = Box::new(Builder::build_pure(pair.next().unwrap()));
                let verb = pair.next().unwrap().as_str();
                let rhs = Box::new(Builder::build_pure(pair.next().unwrap()));

                Node::BinaryOp {
                    lhs,
                    verb: OperatorVerb::from(verb),
                    rhs,
                }
            }

            // Core Language Features
            Rule::assign => {
                let mut pair = pair.into_inner();

                Node::Assign {
                    lhs: Box::new(Builder::build_pure(pair.next().unwrap())),
                    rhs: Box::new(Builder::build_pure(pair.next().unwrap())),
                }
            }
            _ => panic!(
                "Unprocessable state {:#?} found! Panic exit.",
                pair.as_rule()
            ),
        }
    }

    fn build_macro_assign(pairs: &mut Pairs<Rule>) -> MacroAssign {
        MacroAssign {
            lhs: Box::new(Builder::build_pure(pairs.next().unwrap())),
            verb: OperatorVerb::from(pairs.next().unwrap().as_str()),
            rhs: Box::new(Builder::build_pure(pairs.next().unwrap())),
        }
    }

    fn build(pair: Pair<Rule>) -> PollutedNode {
        match pair.as_rule() {
            // Macros
            Rule::macroAssignToIdent => {
                let mut pair = pair.into_inner();

                PollutedNode::Macro(Macro::AssignToIdent {
                    lhs: Box::new(Builder::build_pure(pair.next().unwrap())),
                    rhs: Box::new(Builder::build_pure(pair.next().unwrap())),
                })
            }
            Rule::macroAssignToZero => {
                let mut pair = pair.into_inner();

                PollutedNode::Macro(Macro::AssignToZero {
                    lhs: Box::new(Builder::build_pure(pair.next().unwrap())),
                })
            }
            Rule::macroAssignToValue => {
                let mut pair = pair.into_inner();

                PollutedNode::Macro(Macro::AssignToValue {
                    lhs: Box::new(Builder::build_pure(pair.next().unwrap())),
                    rhs: Box::new(Builder::build_pure(pair.next().unwrap())),
                })
            }
            Rule::macroAssignToIdentOpIdent => {
                let mut pair = pair.into_inner();

                PollutedNode::Macro(Macro::AssignToOpIdent {
                    lhs: Box::new(Builder::build_pure(pair.next().unwrap())),
                    rhs: Builder::build_macro_assign(pairs),
                })
            }
            Rule::macroAssignToIdentExtOpIdent => {
                let mut pair = pair.into_inner();

                PollutedNode::Macro(Macro::AssignToOpExtIdent {
                    lhs: Box::new(Builder::build_pure(pair.next().unwrap())),
                    rhs: Builder::build_macro_assign(pairs),
                })
            }
            Rule::macroAssignToIdentExtOpValue => {
                let mut pair = pair.into_inner();

                PollutedNode::Macro(Macro::AssignToOpExtValue {
                    lhs: Box::new(build_pure_ast_from_expression(pair.next().unwrap())),
                    rhs: Builder::build_macro_assign(&mut pair),
                })
            }
            Rule::macroIf => {
                let mut pair = pair.into_inner();

                PollutedNode::Macro(Macro::If {
                    comp: Box::new(build_pure_ast_from_expression(pair.next().unwrap())),
                    terms: Box::new(build_ast_from_expression(pair.next().unwrap())),
                })
            }
            Rule::macroIfElse => {
                let mut pair = pair.into_inner();

                PollutedNode::Macro(Macro::IfElse {
                    comp: Box::new(build_pure_ast_from_expression(pair.next().unwrap())),
                    if_terms: Box::new(build_ast_from_expression(pair.next().unwrap())),
                    else_terms: Box::new(build_ast_from_expression(pair.next().unwrap())),
                })
            }

            // Control Structures
            Rule::loop_ => {
                let mut pair = pair.into_inner();

                PollutedNode::Control(Control::Loop {
                    ident: Box::new(build_ast_from_expression(pair.next().unwrap())),
                    terms: Box::new(build_ast_from_expression(pair.next().unwrap())),
                })
            }
            Rule::while_ => {
                let mut pair = pair.into_inner();

                PollutedNode::Control(Control::While {
                    comp: Box::new(build_ast_from_expression(pair.next().unwrap())),
                    terms: Box::new(build_ast_from_expression(pair.next().unwrap())),
                })
            }
            Rule::terms => {
                let mut pair = pair.into_inner();
                let mut terms = vec![];

                while let Some(term) = pair.next() {
                    terms.push(build_ast_from_expression(term))
                }

                PollutedNode::Control(Control::Terms(terms))
            }
            Rule::EOI => PollutedNode::NoOp,
            _ => PollutedNode::Pure(build_pure_ast_from_expression(pair)),
        }
    }

    pub fn parse(source: &str) -> Result<Vec<PollutedNode>, Error<Rule>> {
        let mut ast = vec![];

        let pairs = LoopParser::parse(Rule::grammar, source)?;
        for pair in pairs {
            ast.push(Builder::build(pair));
        }

        Ok(ast)
    }

    pub fn purify(ast: &mut Vec<PollutedNode>) -> Node {
        let wrapped = PollutedNode::Control(Control::Terms(ast.clone()));

        wrapped.purify().flatten()
    }
}
