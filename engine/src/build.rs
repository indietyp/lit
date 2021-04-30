use core::option::Option::Some;
use std::str::FromStr;

use num_bigint::BigUint;
use pest::error::Error;
use pest::iterators::{Pair, Pairs};

use crate::ast::context::CompileContext;
use crate::ast::control::Control;
use crate::ast::macros::{Macro, MacroAssign};
use crate::ast::node::Node;
use crate::ast::polluted::PollutedNode;
use crate::ast::verbs::{ComparisonVerb, OperatorVerb};
use crate::eval::exec::Exec;
use crate::flags::CompilationFlags;
use crate::pest::Parser;
use crate::runtime::Runtime;
use crate::types::LineNo;
use crate::LoopParser;
use crate::Rule;

pub struct Builder {}

impl Builder {
    fn build_pure(pair: Pair<Rule>) -> Node {
        let span = pair.as_span();
        let lno: LineNo = (span.start_pos().line_col().0, span.end_pos().line_col().1);

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
                    lno,
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
        let span = pair.as_span();
        let lno: LineNo = (span.start_pos().line_col().0, span.end_pos().line_col().1);

        match pair.as_rule() {
            // Macros
            Rule::macroAssignToIdent => {
                let mut pair = pair.into_inner();

                PollutedNode::Macro(Macro::AssignToIdent {
                    lno,
                    lhs: Box::new(Builder::build_pure(pair.next().unwrap())),
                    rhs: Box::new(Builder::build_pure(pair.next().unwrap())),
                })
            }
            Rule::macroAssignToZero => {
                let mut pair = pair.into_inner();

                PollutedNode::Macro(Macro::AssignToZero {
                    lno,
                    lhs: Box::new(Builder::build_pure(pair.next().unwrap())),
                })
            }
            Rule::macroAssignToValue => {
                let mut pair = pair.into_inner();

                PollutedNode::Macro(Macro::AssignToValue {
                    lno,
                    lhs: Box::new(Builder::build_pure(pair.next().unwrap())),
                    rhs: Box::new(Builder::build_pure(pair.next().unwrap())),
                })
            }
            Rule::macroAssignToIdentOpIdent => {
                let mut pair = pair.into_inner();

                PollutedNode::Macro(Macro::AssignToOpIdent {
                    lno,
                    lhs: Box::new(Builder::build_pure(pair.next().unwrap())),
                    rhs: Builder::build_macro_assign(&mut pair),
                })
            }
            Rule::macroAssignToIdentExtOpIdent => {
                let mut pairs = pair.into_inner();

                PollutedNode::Macro(Macro::AssignToOpExtIdent {
                    lno,
                    lhs: Box::new(Builder::build_pure(pairs.next().unwrap())),
                    rhs: Builder::build_macro_assign(&mut pairs),
                })
            }
            Rule::macroAssignToIdentExtOpValue => {
                let mut pair = pair.into_inner();

                PollutedNode::Macro(Macro::AssignToOpExtValue {
                    lno,
                    lhs: Box::new(Builder::build_pure(pair.next().unwrap())),
                    rhs: Builder::build_macro_assign(&mut pair),
                })
            }
            Rule::macroIf => {
                let mut pair = pair.into_inner();

                PollutedNode::Macro(Macro::If {
                    lno,
                    comp: Box::new(Builder::build_pure(pair.next().unwrap())),
                    terms: Box::new(Builder::build(pair.next().unwrap())),
                })
            }
            Rule::macroIfElse => {
                let mut pair = pair.into_inner();

                PollutedNode::Macro(Macro::IfElse {
                    lno,
                    comp: Box::new(Builder::build_pure(pair.next().unwrap())),
                    if_terms: Box::new(Builder::build(pair.next().unwrap())),
                    else_terms: Box::new(Builder::build(pair.next().unwrap())),
                })
            }

            // Control Structures
            Rule::loop_ => {
                let mut pair = pair.into_inner();

                PollutedNode::Control(Control::Loop {
                    lno,
                    ident: Box::new(Builder::build(pair.next().unwrap())),
                    terms: Box::new(Builder::build(pair.next().unwrap())),
                })
            }
            Rule::while_ => {
                let mut pair = pair.into_inner();

                PollutedNode::Control(Control::While {
                    lno,
                    comp: Box::new(Builder::build(pair.next().unwrap())),
                    terms: Box::new(Builder::build(pair.next().unwrap())),
                })
            }
            Rule::terms => {
                let mut pair = pair.into_inner();
                let mut terms = vec![];

                for term in pair {
                    terms.push(Builder::build(term))
                }

                PollutedNode::Control(Control::Terms(terms))
            }
            Rule::EOI => PollutedNode::NoOp,
            _ => PollutedNode::Pure(Builder::build_pure(pair)),
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

    pub fn compile(ast: &mut Vec<PollutedNode>, flags: Option<CompilationFlags>) -> Node {
        let wrapped = PollutedNode::Control(Control::Terms(ast.clone()));

        let mut context = CompileContext::new(flags.unwrap_or_default());
        // TODO: if flags are new, display then set the new lno
        wrapped.expand(&mut context).flatten()
    }

    pub fn eval(ast: Node) -> Runtime {
        Runtime::new(Exec::new(ast), None)
    }

    pub fn parse_and_compile(source: &str, flags: Option<CompilationFlags>) -> Node {
        Builder::compile(&mut Builder::parse(source).unwrap(), flags)
    }

    pub fn all(source: &str, flags: Option<CompilationFlags>) -> Runtime {
        Builder::eval(Builder::parse_and_compile(source, flags))
    }
}
