use pest::error::Error;

use serde::{Deserialize, Serialize};

use crate::ast::context::CompileContext;
use crate::ast::control::Control;

use crate::ast::node::Node;
use crate::ast::polluted::PollutedNode;
use crate::ast::verbs::{ComparisonVerb, OperatorVerb};
use crate::eval::exec::Exec;
use crate::flags::CompilationFlags;

use crate::eval::types::Variables;
use crate::parser::Rule;
use crate::parser::{LoopParser, ParseSettings};
use crate::pest_consume::Parser;
use crate::runtime::Runtime;
use crate::types::LineNo;

#[derive(Serialize, Deserialize)]
pub struct Builder {}

impl Builder {
    pub fn parse(
        source: &str,
        lno_overwrite: Option<LineNo>,
    ) -> Result<Vec<PollutedNode>, Error<Rule>> {
        let settings = ParseSettings::new(lno_overwrite);
        let pairs = LoopParser::parse_with_userdata(Rule::grammar, source, &settings)?;

        let pair = pairs.single()?;
        Ok(vec![LoopParser::grammar(pair)?.left().unwrap()])
    }

    pub fn compile(ast: &mut Vec<PollutedNode>, flags: Option<CompilationFlags>) -> Node {
        Builder::compile2(ast, CompileContext::new(flags.unwrap_or_default()))
    }

    pub(crate) fn compile2(ast: &mut Vec<PollutedNode>, context: CompileContext) -> Node {
        let wrapped = PollutedNode::Control(Control::Terms(ast.clone()));
        let mut context = context.clone();

        // TODO: if flags are new, display then set the new lno
        wrapped.expand(&mut context).flatten()
    }

    pub fn eval(ast: Node) -> Runtime {
        Builder::eval2(ast, None)
    }

    fn eval2(ast: Node, locals: Option<Variables>) -> Runtime {
        Runtime::new(Exec::new(ast), locals)
    }

    pub fn parse_and_compile(source: &str, flags: Option<CompilationFlags>) -> Node {
        Builder::compile(&mut Builder::parse(source, None).unwrap(), flags)
    }

    // parse_and_compile2 is an internal compile that also uses CompileContext
    pub(crate) fn parse_and_compile2(
        source: &str,
        context: CompileContext,
        lno: Option<LineNo>,
    ) -> Node {
        Builder::compile2(&mut Builder::parse(source, lno).unwrap(), context)
    }

    pub fn all(source: &str, flags: Option<CompilationFlags>) -> Runtime {
        Builder::eval(Builder::parse_and_compile(source, flags))
    }

    // all2 has more options than all (used mostly for tests)
    pub(crate) fn all2(
        source: &str,
        flags: Option<CompilationFlags>,
        locals: Option<Variables>,
    ) -> Runtime {
        Builder::eval2(Builder::parse_and_compile(source, flags), locals)
    }
}
