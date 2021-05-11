use pest::error::Error;

use serde::{Deserialize, Serialize};

use crate::ast::context::CompileContext;
use crate::ast::control::Control;

use crate::ast::node::Node;
use crate::ast::polluted::PollutedNode;

use crate::eval::exec::Exec;
use crate::flags::CompilationFlags;

use crate::ast::module::{filesystem, Module};
use crate::errors;
use crate::eval::types::Variables;
use crate::parser::Rule;
use crate::parser::{LoopParser, ParseSettings};
use crate::runtime::Runtime;
use crate::types::LineNo;
use pest_consume::Parser;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Builder {}

impl Builder {
    pub fn parse(source: &str, lno_overwrite: Option<LineNo>) -> Result<Module, Error<Rule>> {
        let settings = ParseSettings::new(lno_overwrite);
        let pairs = LoopParser::parse_with_userdata(Rule::grammar, source, &settings)?;

        let pair = pairs.single()?;
        Ok(LoopParser::grammar(pair)?)
    }

    pub fn compile(
        ast: &mut PollutedNode,
        flags: Option<CompilationFlags>,
        // fs can be used to specify additional files that can be used
        // at compile time, HashMap for "name: contents"
        fs: Option<filesystem::Directory>,
    ) -> Result<Node, Vec<errors::Error>> {
        Builder::ext_compile(ast, CompileContext::new(flags.unwrap_or_default()))
    }

    pub(crate) fn ext_compile(
        ast: &mut PollutedNode,
        context: CompileContext,
    ) -> Result<Node, Vec<errors::Error>> {
        let wrapped = PollutedNode::Control(Control::Terms(ast.clone()));
        let mut context = context;

        let expanded = wrapped
            .expand(&mut context)?
            .verify(&mut context)?
            .flatten();

        Ok(expanded)
    }

    pub fn eval(ast: Node) -> Runtime {
        Builder::ext_eval(ast, None)
    }

    fn ext_eval(ast: Node, locals: Option<Variables>) -> Runtime {
        Runtime::new(Exec::new(ast), locals)
    }

    pub fn parse_and_compile(
        source: &str,
        flags: Option<CompilationFlags>,
    ) -> Result<Node, Vec<errors::Error>> {
        Builder::compile(
            &mut Builder::parse(source, None)
                .map_err(|err| vec![errors::Error::new_from_parse(err)])?,
            flags,
        )
    }

    // parse_and_compile2 is an internal compile that also uses CompileContext
    pub(crate) fn ext_parse_and_compile(
        source: &str,
        context: CompileContext,
        lno: Option<LineNo>,
    ) -> Result<Node, Vec<errors::Error>> {
        Builder::ext_compile(
            &mut Builder::parse(source, lno)
                .map_err(|err| vec![errors::Error::new_from_parse(err)])?,
            context,
        )
    }

    pub fn all(
        source: &str,
        flags: Option<CompilationFlags>,
    ) -> Result<Runtime, Vec<errors::Error>> {
        Ok(Builder::eval(Builder::parse_and_compile(source, flags)?))
    }

    // all2 has more options than all (used mostly for tests)
    pub fn ext_all(
        source: &str,
        flags: Option<CompilationFlags>,
        locals: Option<Variables>,
    ) -> Result<Runtime, Vec<errors::Error>> {
        Ok(Builder::ext_eval(
            Builder::parse_and_compile(source, flags)?,
            locals,
        ))
    }
}
