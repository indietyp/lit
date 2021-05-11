use crate::ast::func::filesystem::Directory;
use crate::ast::node::Node;
use crate::ast::polluted::PollutedNode;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FunctionImport {
    module: ModuleName,
    alias: Option<FunctionName>,
}

sum_type! {
    #[derive(Debug, Clone)]
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

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct ModuleName(String);
#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct FunctionName(String);
#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct FunctionAlias(String);
#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct FunctionQualName(String);

impl FunctionQualName {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone)]
pub struct ModuleMap(pub HashMap<ModuleName, ModuleContext>);
#[derive(Debug, Clone)]
pub struct ModuleContext(HashMap<FunctionName, FunctionContext>);

impl ModuleMap {
    pub fn new() -> Self {
        ModuleMap(HashMap::new())
    }

    pub fn insert(&mut self, key: ModuleName, value: ModuleContext) -> Option<ModuleContext> {
        self.0.insert(key, value)
    }
}

impl ModuleMap {
    fn resolve() {}

    pub fn from(self, directory: Directory) -> ModuleMap {
        // step 1) parse all modules
        // -> create a preliminary map
        // step 2) resolve recursively
        // step 3) check if collision in ModuleName
        // step 4) insert "ourselves" as _.

        // The directory is always prefixed with fs::,
        // while all others are looking into the /lib/ folder

        // let mut preliminary = HashMap::new();

        todo!()
    }
}
