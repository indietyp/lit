use crate::ast::module::filesystem;
use crate::ast::node::Node;
use crate::ast::polluted::PollutedNode;
use crate::flags::CompilationFlags;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

// TODO: pub into separate function
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
        Inline(Node),
    }
}

pub type ModuleName = String;
pub type FunctionName = String;
pub type FunctionAlias = String;
pub type FunctionQualName = String;
pub type ModuleContext = HashMap<FunctionName, FunctionContext>;

pub struct FunctionImport {
    module: ModuleName,
    alias: Option<FunctionName>,
}

#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub struct CompileContext {
    counter: usize,
    pub flags: CompilationFlags,
    pub fs: filesystem::Directory,
    pub modules: HashMap<ModuleName, ModuleContext>,
    pub inline_counter: HashMap<FunctionQualName, usize>,
}

impl CompileContext {
    pub fn new(flags: CompilationFlags, fs: Option<filesystem::Directory>) -> Self {
        CompileContext {
            counter: 0,
            flags,
            fs: fs.unwrap_or_default(),
            modules: HashMap::new(),
            inline_counter: HashMap::new(),
        }
    }

    pub fn incr(&mut self) -> usize {
        let cur = self.counter;
        self.counter += 1;

        cur
    }

    pub fn incr_inline(&mut self, func: FunctionQualName) -> usize {
        let mut cur = self.inline_counter.get(func.as_str()).cloned().unwrap_or(0);
        cur += 1;
        self.inline_counter.insert(func, cur);

        cur.clone()
    }
}
