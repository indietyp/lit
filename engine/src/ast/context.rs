use std::collections::HashMap;

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

type CallStack = Vec<Frame>;

#[derive(Debug, Clone)]
pub struct Frame {
    pub caller: Option<FuncQualName>,
    pub module: ModuleName,

    pub context: CompileLocalContext,
}

impl Default for Frame {
    fn default() -> Self {
        Self {
            caller: None,
            module: ModuleName::main(),
            context: CompileLocalContext::default(),
        }
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
        };

        Ok(ctx)
    }
}

impl CompileContext {
    // "dive" into the callstack, by creating a new frame for the function provided
    pub fn dive<T>(
        &mut self,
        caller: FuncQualName,
        module: ModuleName,
        func: impl Fn(&mut CompileContext, &CallStack, &mut CompileLocalContext) -> StdResult<T>,
    ) -> StdResult<T> {
        let mut frame = Frame {
            caller: Some(caller),
            module,
            context: CompileLocalContext::default(),
        };

        self.stack.push(frame);
        let ret = func(self, &self.stack, &mut frame.context);
        self.stack.pop();

        ret
    }

    pub fn get_current_frame(&self) -> &Frame {
        let default = Frame::default();

        self.stack.last().unwrap_or(&default)
    }

    // function is because the stack should not be mutable.
    pub fn get_stack(&self) -> &CallStack {
        &self.stack
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

    // fn prefix(
    //     &mut self,
    //     module: Option<ModuleName>,
    //     func: FuncDecl,
    //     ident: Either<String, Expr>,
    // ) -> StdResult<String> {
    //     let module = module.unwrap_or(ModuleName::main());
    //
    //     let ident = ident.either(
    //         |f| Ok(f),
    //         |g| match g {
    //             Expr::Ident(m) => Ok(m),
    //             _ => Err(vec![Error::new_from_msg(
    //                 None,
    //                 format!("CompileContext::prefix expected Ident, got {:?}", g).as_str(),
    //             )]),
    //         },
    //     )?;
    //
    //     let func_name = func.get_ident()?;
    //     let qual_name: FuncQualName = (module.clone(), func_name.into()).into();
    //
    //     let name = format!(
    //         "_{}_{}_{}__{}",
    //         module.join("_"),
    //         func_name,
    //         self.incr_inline(qual_name),
    //         ident
    //     );
    //
    //     Ok(name)
    // }
}
