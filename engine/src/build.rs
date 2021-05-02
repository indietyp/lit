use std::str::FromStr;
use wasm_bindgen::prelude::*;

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
use crate::runtime::{JavaScriptRuntime, Runtime};
use crate::types::LineNo;
use crate::LoopParser;
use crate::Rule;
use js_sys::Map;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize)]
pub struct Builder {}

impl Builder {
    fn build_pure(pair: Pair<Rule>, lno_overwrite: Option<LineNo>) -> Node {
        let span = pair.as_span();
        let lno: LineNo =
            lno_overwrite.unwrap_or((span.start_pos().line_col().0, span.end_pos().line_col().1));

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

                let lhs = Box::new(Builder::build_pure(pair.next().unwrap(), lno_overwrite));
                let verb = pair.next().unwrap().as_str();
                let rhs = Box::new(Builder::build_pure(pair.next().unwrap(), lno_overwrite));

                Node::Comparison {
                    lhs,
                    verb: ComparisonVerb::from(verb),
                    rhs,
                }
            }

            // Binary Operator
            Rule::binaryOp => {
                let mut pair = pair.into_inner();
                let lhs = Box::new(Builder::build_pure(pair.next().unwrap(), lno_overwrite));
                let verb = pair.next().unwrap().as_str();
                let rhs = Box::new(Builder::build_pure(pair.next().unwrap(), lno_overwrite));

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
                    lhs: Box::new(Builder::build_pure(pair.next().unwrap(), lno_overwrite)),
                    rhs: Box::new(Builder::build_pure(pair.next().unwrap(), lno_overwrite)),
                }
            }
            _ => panic!(
                "Unprocessable state {:#?} found! Panic exit.",
                pair.as_rule()
            ),
        }
    }

    fn build_macro_assign(lno_overwrite: Option<LineNo>, pairs: &mut Pairs<Rule>) -> MacroAssign {
        MacroAssign {
            lhs: Box::new(Builder::build_pure(pairs.next().unwrap(), lno_overwrite)),
            verb: OperatorVerb::from(pairs.next().unwrap().as_str()),
            rhs: Box::new(Builder::build_pure(pairs.next().unwrap(), lno_overwrite)),
        }
    }

    fn build(pair: Pair<Rule>, lno_overwrite: Option<LineNo>) -> PollutedNode {
        let span = pair.as_span();
        let lno: LineNo =
            lno_overwrite.unwrap_or((span.start_pos().line_col().0, span.end_pos().line_col().1));

        match pair.as_rule() {
            // Macros
            Rule::macroAssignToIdent => {
                let mut pair = pair.into_inner();

                PollutedNode::Macro(Macro::AssignToIdent {
                    lno,
                    lhs: Box::new(Builder::build_pure(pair.next().unwrap(), lno_overwrite)),
                    rhs: Box::new(Builder::build_pure(pair.next().unwrap(), lno_overwrite)),
                })
            }
            Rule::macroAssignToZero => {
                let mut pair = pair.into_inner();

                PollutedNode::Macro(Macro::AssignToZero {
                    lno,
                    lhs: Box::new(Builder::build_pure(pair.next().unwrap(), lno_overwrite)),
                })
            }
            Rule::macroAssignToValue => {
                let mut pair = pair.into_inner();

                PollutedNode::Macro(Macro::AssignToValue {
                    lno,
                    lhs: Box::new(Builder::build_pure(pair.next().unwrap(), lno_overwrite)),
                    rhs: Box::new(Builder::build_pure(pair.next().unwrap(), lno_overwrite)),
                })
            }
            Rule::macroAssignToIdentOpIdent => {
                let mut pair = pair.into_inner();

                PollutedNode::Macro(Macro::AssignToOpIdent {
                    lno,
                    lhs: Box::new(Builder::build_pure(pair.next().unwrap(), lno_overwrite)),
                    rhs: Builder::build_macro_assign(lno_overwrite, &mut pair),
                })
            }
            Rule::macroAssignToIdentExtOpValue => {
                let mut pair = pair.into_inner();

                PollutedNode::Macro(Macro::AssignToOpExtValue {
                    lno,
                    lhs: Box::new(Builder::build_pure(pair.next().unwrap(), lno_overwrite)),
                    rhs: Builder::build_macro_assign(lno_overwrite, &mut pair),
                })
            }
            Rule::macroIf => {
                let mut pair = pair.into_inner();

                PollutedNode::Macro(Macro::If {
                    lno,
                    comp: Box::new(Builder::build_pure(pair.next().unwrap(), lno_overwrite)),
                    terms: Box::new(Builder::build(pair.next().unwrap(), lno_overwrite)),
                })
            }
            Rule::macroIfElse => {
                let mut pair = pair.into_inner();

                PollutedNode::Macro(Macro::IfElse {
                    lno,
                    comp: Box::new(Builder::build_pure(pair.next().unwrap(), lno_overwrite)),
                    if_terms: Box::new(Builder::build(pair.next().unwrap(), lno_overwrite)),
                    else_terms: Box::new(Builder::build(pair.next().unwrap(), lno_overwrite)),
                })
            }

            // Control Structures
            Rule::loop_ => {
                let mut pair = pair.into_inner();

                PollutedNode::Control(Control::Loop {
                    lno,
                    ident: Box::new(Builder::build(pair.next().unwrap(), lno_overwrite)),
                    terms: Box::new(Builder::build(pair.next().unwrap(), lno_overwrite)),
                })
            }
            Rule::while_ => {
                let mut pair = pair.into_inner();

                PollutedNode::Control(Control::While {
                    lno,
                    comp: Box::new(Builder::build(pair.next().unwrap(), lno_overwrite)),
                    terms: Box::new(Builder::build(pair.next().unwrap(), lno_overwrite)),
                })
            }
            Rule::terms => {
                let mut pair = pair.into_inner();
                let mut terms = vec![];

                for term in pair {
                    terms.push(Builder::build(term, lno_overwrite))
                }

                PollutedNode::Control(Control::Terms(terms))
            }
            Rule::EOI => PollutedNode::NoOp,
            _ => PollutedNode::Pure(Builder::build_pure(pair, lno_overwrite)),
        }
    }

    pub fn parse(
        source: &str,
        lno_overwrite: Option<LineNo>,
    ) -> Result<Vec<PollutedNode>, Error<Rule>> {
        let mut ast = vec![];

        let pairs = LoopParser::parse(Rule::grammar, source)?;
        for pair in pairs {
            ast.push(Builder::build(pair, lno_overwrite));
        }

        Ok(ast)
    }

    pub fn compile(ast: &mut Vec<PollutedNode>, flags: Option<CompilationFlags>) -> Node {
        Builder::compile2(ast, CompileContext::new(flags.unwrap_or_default()))
    }

    pub fn eval(ast: Node) -> Runtime {
        Runtime::new(Exec::new(ast), None)
    }

    pub fn parse_and_compile(source: &str, flags: Option<CompilationFlags>) -> Node {
        Builder::compile(&mut Builder::parse(source, None).unwrap(), flags)
    }

    pub(crate) fn compile2(ast: &mut Vec<PollutedNode>, context: CompileContext) -> Node {
        let wrapped = PollutedNode::Control(Control::Terms(ast.clone()));
        let mut context = context.clone();

        // TODO: if flags are new, display then set the new lno
        wrapped.expand(&mut context).flatten()
    }

    // parse_and_compile2 is an internal compile that also uses CompileContext
    pub(crate) fn parse_and_compile2(
        source: &str,
        context: CompileContext,
        lno: Option<LineNo>,
    ) -> Node {
        Builder::compile2(&mut Builder::parse(source, lno).unwrap(), context)
    }

    pub fn all(source: &str, flags: Option<CompilationFlags>, offset: Option<usize>) -> Runtime {
        Builder::eval(Builder::parse_and_compile(source, flags))
    }
}

#[wasm_bindgen(js_name = Builder)]
#[derive(Serialize, Deserialize)]
pub struct JavaScriptBuilder {
    builder: Builder,
}

#[wasm_bindgen(js_class = Builder)]
impl JavaScriptBuilder {
    pub fn parse(source: &str) -> Result<JsValue, JsValue> {
        let result = Builder::parse(source, None);

        return if result.is_ok() {
            Ok(JsValue::from_serde(&result.ok().unwrap()).unwrap())
        } else {
            Err(JsValue::from_str(&format!("{}", result.err().unwrap())))
        };
    }

    pub fn compile(ast: &JsValue, flags: Option<CompilationFlags>) -> Result<JsValue, JsValue> {
        let mut ast: Vec<PollutedNode> = ast.into_serde().unwrap();
        let result = Builder::compile(&mut ast, flags);

        Ok(JsValue::from_serde(&result).unwrap())
    }

    pub fn eval(ast: &JsValue, locals: Map) -> Result<JavaScriptRuntime, JsValue> {
        JavaScriptRuntime::new(ast, locals)
    }
}
