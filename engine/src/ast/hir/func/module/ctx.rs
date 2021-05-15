use std::collections::HashMap;

use crate::ast::hir::func::types::{FunctionContext, FunctionName};

pub type ModuleContextHashMap = HashMap<FunctionName, FunctionContext>;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleContext(pub ModuleContextHashMap);

NewtypeDeref! {() pub struct ModuleContext(pub ModuleContextHashMap); }
NewtypeDerefMut! {() pub struct ModuleContext(pub ModuleContextHashMap); }

impl ModuleContext {
    pub fn new() -> Self {
        ModuleContext(HashMap::new())
    }

    pub fn insert(&mut self, key: FunctionName, value: FunctionContext) -> Option<FunctionContext> {
        self.0.insert(key, value)
    }
}
