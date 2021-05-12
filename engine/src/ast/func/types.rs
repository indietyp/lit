use crate::ast::func::filesystem::Directory;
use crate::ast::module::Module;
use crate::ast::node::Node;
use crate::ast::polluted::PollutedNode;
use crate::build::Builder;
use crate::errors::Error;
use crate::utils::check_errors;
use itertools::Itertools;
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
    fn parse(directory: Directory) -> Result<HashMap<Vec<String>, Module>, Vec<Error>> {
        let mut preliminary: HashMap<_, _> = directory.walk().collect();
        let mut files = preliminary
            .iter()
            .map(|(k, v)| (k, Builder::parse(v.as_str(), None)));

        let results: Vec<_> = files
            .clone()
            .map(|(k, v)| v.map_err(|err| vec![Error::new_from_parse(err)]))
            .collect();
        check_errors(&results)?;

        // we know these are all save thanks to check_errors
        Ok(files.map(|(k, v)| (k.clone(), v.unwrap())).collect())
    }
    fn resolve() {}

    pub fn from(self, directory: Directory) -> Result<ModuleMap, Vec<Error>> {
        // step 1) parse all modules - DONE
        // -> create a preliminary map
        // -> parse results
        // step 2) create an import map
        // step 3) resolve recursively
        //  --> check if collision in ModuleName
        // step 5) insert "ourselves" as main.

        // The directory is always prefixed with fs::,
        // while all others are looking into the /lib/ folder
        let mut modules = Self::parse(directory);

        todo!()
    }
}
