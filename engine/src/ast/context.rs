use std::collections::HashMap;

use crate::ast::func;
use crate::ast::func::types::{FunctionQualName, ModuleContext, ModuleMap, ModuleName};
use crate::flags::CompilationFlags;

#[derive(Debug, Clone)]
pub struct CompileContext {
    counter: usize,
    pub flags: CompilationFlags,
    pub fs: func::filesystem::Directory,
    pub modules: ModuleMap,
    pub inline_counter: HashMap<FunctionQualName, usize>,
}

impl CompileContext {
    pub fn new(flags: CompilationFlags, fs: Option<func::filesystem::Directory>) -> Self {
        CompileContext {
            counter: 0,
            flags,
            fs: fs.unwrap_or_default(),
            modules: ModuleMap::new(),
            inline_counter: HashMap::new(),
        }
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
