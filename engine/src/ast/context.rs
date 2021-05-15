use std::collections::HashMap;

use crate::ast::expr::Expr;
use crate::ast::func;
use crate::ast::func::fs::Directory;
use crate::ast::func::modmap::ModuleMap;
use crate::ast::func::types::FunctionQualName;
use crate::ast::module::{FuncDecl, Module};
use crate::errors::{Error, StdResult};
use crate::flags::CompileFlags;
use either::Either;

#[derive(Debug, Clone)]
pub struct CompileContext {
    counter: usize,
    inline_counter: HashMap<FunctionQualName, usize>,

    pub module: Vec<String>,

    pub fs: Directory,
    pub flags: CompileFlags,
    pub modules: ModuleMap,
}

impl CompileContext {
    pub fn new(main: Module, flags: CompileFlags, fs: Option<Directory>) -> StdResult<Self> {
        let ctx = CompileContext {
            counter: 0,
            flags,
            fs: fs.unwrap_or_default(),
            modules: ModuleMap::from(main, fs.unwrap_or_default())?,
            inline_counter: HashMap::new(),
            module: vec!["fs".to_string(), "main".to_string()],
        };

        Ok(ctx)
    }

    pub fn incr(&mut self) -> usize {
        let cur = self.counter;
        self.counter += 1;

        cur
    }

    pub fn incr_inline(&mut self, func: FunctionQualName) -> usize {
        let mut cur = self.inline_counter.get(&func).cloned().unwrap_or(0);
        cur += 1;
        self.inline_counter.insert(func, cur);

        cur.clone()
    }

    pub fn prefix(&self, func: FuncDecl, ident: Either<String, Expr>) -> StdResult<String> {
        let ident = ident.either(
            |f| Ok(f),
            |g| match g {
                Expr::Ident(m) => Ok(m),
                _ => Err(vec![Error::new_from_msg(
                    None,
                    format!("CompileContext::prefix expected Ident, got {:?}", g).as_str(),
                )]),
            },
        )?;

        let ident = func.get_ident()?;

        todo!()
    }

    pub fn change(&self) -> Self {
        todo!()
    }
}
