use pest::error::Error;
use pest_consume::Parser;
use serde::{Deserialize, Serialize};

use crate::ast::context::CompileContext;
use crate::ast::expr::Expr;
use crate::ast::hir::func;
use crate::ast::hir::Hir;
use crate::ast::module::Module;
use crate::errors;
use crate::eval::exec::Exec;
use crate::eval::types::Variables;
use crate::flags::CompileFlags;
use crate::parser::Rule;
use crate::parser::{LoopParser, ParseSettings};
use crate::runtime::Runtime;
use crate::types::LineNo;

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
        ast: &mut Hir,
        flags: Option<CompileFlags>,
        // fs can be used to specify additional files that can be used
        // at compile time, HashMap for "name: contents"
        fs: Option<func::fs::Directory>,
    ) -> Result<Expr, Vec<errors::Error>> {
        Builder::ext_compile(ast, CompileContext::new(flags.unwrap_or_default(), fs))
    }

    pub(crate) fn ext_compile(
        ast: &mut Hir,
        context: CompileContext,
    ) -> Result<Expr, Vec<errors::Error>> {
        todo!()

        // let wrapped = PollutedNode::Control(Control::Terms(ast.clone()));
        // let mut context = context;
        //
        // let expanded = wrapped
        //     .expand(&mut context)?
        //     .verify(&mut context)?
        //     .flatten();
        //
        // Ok(expanded)
    }

    pub fn eval(ast: Expr) -> Runtime {
        Builder::ext_eval(ast, None)
    }

    fn ext_eval(ast: Expr, locals: Option<Variables>) -> Runtime {
        Runtime::new(Exec::new(ast), locals)
    }

    pub fn parse_and_compile(
        source: &str,
        flags: Option<CompileFlags>,
    ) -> Result<Expr, Vec<errors::Error>> {
        todo!()
        // Builder::compile(
        //     &mut Builder::parse(source, None)
        //         .map_err(|err| vec![errors::Error::new_from_parse(err)])?,
        //     flags,
        // )
    }

    // parse_and_compile2 is an internal compile that also uses CompileContext
    pub(crate) fn ext_parse_and_compile(
        source: &str,
        context: CompileContext,
        lno: Option<LineNo>,
    ) -> Result<Expr, Vec<errors::Error>> {
        todo!()
        // Builder::ext_compile(
        //     &mut Builder::parse(source, lno)
        //         .map_err(|err| vec![errors::Error::new_from_parse(err)])?,
        //     context,
        // )
    }

    pub fn all(source: &str, flags: Option<CompileFlags>) -> Result<Runtime, Vec<errors::Error>> {
        Ok(Builder::eval(Builder::parse_and_compile(source, flags)?))
    }

    // all2 has more options than all (used mostly for tests)
    pub fn ext_all(
        source: &str,
        flags: Option<CompileFlags>,
        locals: Option<Variables>,
    ) -> Result<Runtime, Vec<errors::Error>> {
        Ok(Builder::ext_eval(
            Builder::parse_and_compile(source, flags)?,
            locals,
        ))
    }
}
