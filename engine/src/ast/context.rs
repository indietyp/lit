use crate::ast::module::filesystem;
use crate::flags::CompilationFlags;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

pub type ModuleName = String;
pub type FunctionName = String;
pub type FunctionAlias = String;
pub type ModuleContext = HashMap<FunctionName, FunctionContext>;

pub struct FunctionImport {
    module: ModuleName,
    alias: Option<FunctionName>,
}

sum_type! {
    #[derive(Debug, Clone, PartialEq)]
    pub enum FunctionContext {
        // This means it is an import
        Import(FunctionImport),

        /// This means it is still a function,
        /// which needs to be inlined
        Func(PollutedNode),

        /// This means it is already inlined
        /// and can be used
        Inline(Node)
    }
}

#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub struct CompileContext {
    counter: usize,
    pub flags: CompilationFlags,
    pub fs: filesystem::Directory,
    pub modules: HashMap<ModuleName, ModuleContext>,
}

impl CompileContext {
    pub fn new(flags: CompilationFlags) -> Self {
        CompileContext { counter: 0, flags }
    }

    pub fn incr(&mut self) -> usize {
        let cur = self.counter;
        self.counter += 1;

        cur
    }
}
