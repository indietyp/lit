use pest::error::Error;
use pest_consume::Parser;
use serde::{Deserialize, Serialize};

use crate::ast::context::CompileContext;

use crate::ast::expr::Expr;
use crate::ast::hir::func;

use crate::ast::module::Module;
use crate::errors;
use crate::errors::StdResult;
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
        LoopParser::grammar(pair)
    }

    pub fn compile(
        module: &mut Module,
        flags: Option<CompileFlags>,
        fs: Option<func::fs::Directory>,
    ) -> StdResult<Expr> {
        let mut context = CompileContext::new(module.clone(), flags.unwrap_or_default(), fs)?;

        Builder::ext_compile(module, &mut context)
    }

    pub(crate) fn ext_compile(
        module: &mut Module,
        context: &mut CompileContext,
    ) -> Result<Expr, Vec<errors::Error>> {
        let hir = module.code.clone();
        let expanded = hir.lower(context)?.verify(context)?.flatten();

        Ok(expanded)
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
        fs: Option<func::fs::Directory>,
    ) -> StdResult<Expr> {
        Builder::compile(
            &mut Builder::parse(source, None)
                .map_err(|err| vec![errors::Error::new_from_parse(err)])?,
            flags,
            fs,
        )
    }

    // parse_and_compile2 is an internal compile that also uses CompileContext
    pub(crate) fn ext_parse_and_compile(
        source: &str,
        context: &mut CompileContext,
        lno: Option<LineNo>,
    ) -> StdResult<Expr> {
        Builder::ext_compile(
            &mut Builder::parse(source, lno)
                .map_err(|err| vec![errors::Error::new_from_parse(err)])?,
            context,
        )
    }

    pub fn all(
        source: &str,
        flags: Option<CompileFlags>,
        fs: Option<func::fs::Directory>,
    ) -> Result<Runtime, Vec<errors::Error>> {
        Ok(Builder::eval(Builder::parse_and_compile(
            source, flags, fs,
        )?))
    }

    // ext_all is mostly used for tests only
    pub fn ext_all(
        source: &str,
        flags: Option<CompileFlags>,
        locals: Option<Variables>,
        fs: Option<func::fs::Directory>,
    ) -> Result<Runtime, Vec<errors::Error>> {
        Ok(Builder::ext_eval(
            Builder::parse_and_compile(source, flags, fs)?,
            locals,
        ))
    }
}
