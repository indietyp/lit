use crate::ast::{
    ComparisonVerb, Control, Macro, MacroBinaryAssignOperation, Node, OperatorVerb, PollutedNode,
};
use crate::Rule;
use core::option::Option::Some;
use num_bigint::BigUint;
use pest::iterators::Pair;
use std::str::FromStr;

fn build_pure_ast_from_expression(pair: Pair<Rule>) -> Node {
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

            let lhs = Box::new(build_pure_ast_from_expression(pair.next().unwrap()));
            let verb = pair.next().unwrap().as_str();
            let rhs = Box::new(build_pure_ast_from_expression(pair.next().unwrap()));

            Node::Comparison {
                lhs,
                verb: ComparisonVerb::from(verb),
                rhs,
            }
        }

        // Binary Operator
        Rule::binaryOp => {
            let mut pair = pair.into_inner();
            let lhs = Box::new(build_pure_ast_from_expression(pair.next().unwrap()));
            let verb = pair.next().unwrap().as_str();
            let rhs = Box::new(build_pure_ast_from_expression(pair.next().unwrap()));

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
                lhs: Box::new(build_pure_ast_from_expression(pair.next().unwrap())),
                rhs: Box::new(build_pure_ast_from_expression(pair.next().unwrap())),
            }
        }
        _ => panic!(
            "Unprocessable state {:#?} found! Panic exit.",
            pair.as_rule()
        ),
    }
}

pub fn build_ast_from_expression(pair: Pair<Rule>) -> PollutedNode {
    match pair.as_rule() {
        // Macros
        Rule::macroAssignToIdent => {
            let mut pair = pair.into_inner();

            PollutedNode::Macro(Macro::AssignToIdent {
                lhs: Box::new(build_pure_ast_from_expression(pair.next().unwrap())),
                rhs: Box::new(build_pure_ast_from_expression(pair.next().unwrap())),
            })
        }
        Rule::macroAssignToZero => {
            let mut pair = pair.into_inner();

            PollutedNode::Macro(Macro::AssignToZero {
                lhs: Box::new(build_pure_ast_from_expression(pair.next().unwrap())),
            })
        }
        Rule::macroAssignToValue => {
            let mut pair = pair.into_inner();

            PollutedNode::Macro(Macro::AssignToValue {
                lhs: Box::new(build_pure_ast_from_expression(pair.next().unwrap())),
                rhs: Box::new(build_pure_ast_from_expression(pair.next().unwrap())),
            })
        }
        Rule::macroAssignToIdentOpIdent => {
            let mut pair = pair.into_inner();

            PollutedNode::Macro(Macro::AssignToOpIdent {
                lhs: Box::new(build_pure_ast_from_expression(pair.next().unwrap())),
                rhs: MacroBinaryAssignOperation {
                    lhs: Box::new(build_pure_ast_from_expression(pair.next().unwrap())),
                    verb: OperatorVerb::from(pair.next().unwrap().as_str()),
                    rhs: Box::new(build_pure_ast_from_expression(pair.next().unwrap())),
                },
            })
        }

        Rule::macroAssignToIdentExtOpIdent => {
            let mut pair = pair.into_inner();

            PollutedNode::Macro(Macro::AssignToOpExtIdent {
                lhs: Box::new(build_pure_ast_from_expression(pair.next().unwrap())),
                rhs: MacroBinaryAssignOperation {
                    lhs: Box::new(build_pure_ast_from_expression(pair.next().unwrap())),
                    verb: OperatorVerb::from(pair.next().unwrap().as_str()),
                    rhs: Box::new(build_pure_ast_from_expression(pair.next().unwrap())),
                },
            })
        }
        Rule::macroAssignToIdentExtOpValue => {
            let mut pair = pair.into_inner();

            PollutedNode::Macro(Macro::AssignToOpExtValue {
                lhs: Box::new(build_pure_ast_from_expression(pair.next().unwrap())),
                rhs: MacroBinaryAssignOperation {
                    lhs: Box::new(build_pure_ast_from_expression(pair.next().unwrap())),
                    verb: OperatorVerb::from(pair.next().unwrap().as_str()),
                    rhs: Box::new(build_pure_ast_from_expression(pair.next().unwrap())),
                },
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
