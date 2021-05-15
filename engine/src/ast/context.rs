use std::collections::HashMap;

use crate::ast::func;
use crate::ast::func::modmap::ModuleMap;
use crate::ast::func::types::FunctionQualName;
use crate::ast::module::Module;
use crate::errors::StdResult;
use crate::flags::CompileFlags;

#[derive(Debug, Clone)]
pub struct CompileContext {
    counter: usize,
    pub flags: CompileFlags,
    pub fs: func::fs::Directory,
    pub modules: ModuleMap,
    pub inline_counter: HashMap<FunctionQualName, usize>,
}

impl CompileContext {
    pub fn new(
        main: Module,
        flags: CompileFlags,
        fs: Option<func::fs::Directory>,
    ) -> StdResult<Self> {
        let ctx = CompileContext {
            counter: 0,
            flags,
            fs: fs.unwrap_or_default(),
            modules: ModuleMap::from(main, fs.unwrap_or_default())?,
            inline_counter: HashMap::new(),
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
}
