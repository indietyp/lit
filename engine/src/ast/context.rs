use std::collections::HashMap;

use either::Either;

use crate::ast::expr::Expr;
use crate::ast::hir::func::decl::FuncDecl;
use crate::ast::hir::func::fs::Directory;
use crate::ast::hir::func::module::map::ModuleMap;
use crate::ast::hir::func::structs::modname::ModuleName;
use crate::ast::hir::func::structs::qualname::FuncQualName;
use crate::ast::module::Module;
use crate::errors::{Error, StdResult};
use crate::flags::CompileFlags;

#[derive(Debug, Clone)]
pub struct CompileLocalContext {}

impl Default for CompileLocalContext {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Debug, Clone)]
pub struct CompileContext {
    counter: usize,
    inline_counter: HashMap<FuncQualName, usize>,

    // implement a call stack of some sorts when lowering
    // .call() which also gives you a new CompileContext?
    // remove the locals stuff as soon as we're out of the callstack
    pub fs: Directory,
    pub flags: CompileFlags,
    pub modules: ModuleMap,

    stack: CallStack,
    stack_ctx: Option<CompileLocalContext>,
}

impl CompileContext {
    pub fn new(main: Module, flags: CompileFlags, fs: Option<Directory>) -> StdResult<Self> {
        let ctx = CompileContext {
            counter: 0,
            inline_counter: HashMap::new(),

            fs: fs.unwrap_or_default(),
            flags,

            modules: ModuleMap::from(main, fs.unwrap_or_default())?,
            stack: vec![],
            stack_ctx: None,
        };

        Ok(ctx)
    }
}

type CallStack = Vec<FuncQualName>;
impl CompileContext {
    pub fn call<T>(
        &mut self,
        qual: FuncQualName,
        func: impl Fn(&mut CompileContext, &CallStack, &mut CompileLocalContext) -> StdResult<T>,
    ) -> StdResult<T> {
        if self.stack_ctx.is_none() {
            self.stack_ctx = Some(CompileLocalContext::default())
        }

        self.stack.push(qual);
        let ret = func(self, &self.stack, &mut self.stack_ctx.unwrap_or_default());
        self.stack.pop();

        // clear the stack_ctx if we're empty
        if self.stack.is_empty() {
            self.stack_ctx = None
        }

        ret
    }

    pub fn get_current_frame(&self) {
        // self.stack.last()
    }
}

impl CompileContext {
    fn incr(&mut self) -> usize {
        let cur = self.counter;
        self.counter += 1;

        cur
    }

    fn incr_inline(&mut self, func: FuncQualName) -> usize {
        let mut cur = self.inline_counter.get(&func).cloned().unwrap_or(0);
        cur += 1;
        self.inline_counter.insert(func, cur);

        cur.clone()
    }

    fn prefix(
        &mut self,
        module: Option<ModuleName>,
        func: FuncDecl,
        ident: Either<String, Expr>,
    ) -> StdResult<String> {
        let module = module.unwrap_or(ModuleName::main());

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

        let func_name = func.get_ident()?;
        let qual_name: FuncQualName = (module.clone(), func_name.into()).into();

        let name = format!(
            "_{}_{}_{}__{}",
            module.join("_"),
            func_name,
            self.incr_inline(qual_name),
            ident
        );

        Ok(name)
    }
}
