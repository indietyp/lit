extern crate pest;

#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate enum_display_derive;

use crate::ast::Node::Comparison;
use crate::ast::{ComparisonVerb, Control, Macro, Node, OperatorVerb, PollutedNode};
use num_bigint::BigUint;
use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;
use std::fs::read_to_string;
use std::str::FromStr;

mod ast;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct LoopParser;

fn build_pure_ast_from_expression(pair: Pair<Rule>) -> Node {
    // this code will be called if there i
    match pair.as_rule() {
        // Terminal Encoding
        Rule::IDENT => Node::Ident(String::from(pair.as_str())),
        Rule::VALUE => Node::NaturalNumber(BigUint::from_str(pair.as_str()).unwrap()),

        // Comparison
        Rule::compEqual
        | Rule::compNotEqual
        | Rule::compGreaterThan
        | Rule::compGreaterEqual
        | Rule::compLessThan
        | Rule::compLessEqual => {
            let mut pair = pair.into_inner();

            let lhs = Box::new(build_pure_ast_from_expression(pair.next().unwrap()));
            let verb = pair.next().unwrap().as_str();
            let rhs = Box::new(build_pure_ast_from_expression(pair.next().unwrap()));

            Node::Comparison {
                lhs,
                verb: match verb {
                    "=" | "==" => ComparisonVerb::Equal,
                    "!=" => ComparisonVerb::NotEqual,
                    ">" => ComparisonVerb::GreaterThan,
                    ">=" => ComparisonVerb::GreaterThanEqual,
                    "<" => ComparisonVerb::LessThan,
                    "<=" => ComparisonVerb::LessThanEqual,
                    _ => panic!("Currently do not support comparison operator {}.", verb),
                },
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
                verb: match verb {
                    "+" => OperatorVerb::Plus,
                    "-" => OperatorVerb::Minus,
                    "*" => OperatorVerb::Multiply,
                    _ => panic!("Currently do not support specified operator {}", verb),
                },
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
        _ => panic!("You should not reach this, {:#?}", pair.as_rule()),
    }
}

fn build_ast_from_expression(pair: Pair<Rule>) -> PollutedNode {
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
                rhs: Box::new(build_pure_ast_from_expression(pair.next().unwrap())),
            })
        }
        Rule::macroAssignToIdentExtOpValue => {
            let mut pair = pair.into_inner();

            PollutedNode::Macro(Macro::AssignToOpValue {
                lhs: Box::new(build_pure_ast_from_expression(pair.next().unwrap())),
                rhs: Box::new(build_pure_ast_from_expression(pair.next().unwrap())),
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
        _ => PollutedNode::ASTNode(build_pure_ast_from_expression(pair)),
    }
}

fn parse(source: &str) -> Result<Vec<PollutedNode>, Error<Rule>> {
    let mut ast = vec![];

    let pairs = LoopParser::parse(Rule::grammar, source)?;
    for pair in pairs {
        ast.push(build_ast_from_expression(pair));
    }

    Ok(ast)
}

fn main() {
    let unparsed = read_to_string("example.loop").expect("Cannot read example file");
    let polluted = parse(&unparsed).expect("Unsuccessful Parse");
    println!("{:#?}", polluted)
}
