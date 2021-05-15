use std::collections::HashMap;

use crate::ast::hir::func::types::{FuncContext, FunctionName};

pub type ModuleContextHashMap = HashMap<FunctionName, FuncContext>;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleContext(pub ModuleContextHashMap);

NewtypeDeref! {() pub struct ModuleContext(pub ModuleContextHashMap); }
NewtypeDerefMut! {() pub struct ModuleContext(pub ModuleContextHashMap); }

impl ModuleContext {
    pub fn new() -> Self {
        ModuleContext(HashMap::new())
    }

    pub fn insert(&mut self, key: FunctionName, value: FuncContext) -> Option<FuncContext> {
        self.0.insert(key, value)
    }
}
