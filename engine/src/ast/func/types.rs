use crate::ast::func::filesystem::Directory;
use crate::ast::module::{FuncDecl, ImpFunc, ImpWildcard, Module};
use crate::ast::node::Node;
use crate::ast::polluted::PollutedNode;
use crate::build::Builder;
use crate::errors::Error;
use crate::errors::ErrorCode;
use crate::utils::check_errors;
use either::Either;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FunctionImport {
    module: ModuleName,
    ident: FunctionName,
}

sum_type! {
    #[derive(Debug, Clone)]
    pub enum FunctionContext {
        // This means it is an import
        Import(FunctionImport),

        /// This means it is still a function,
        /// which needs to be inlined
        Func(FuncDecl),

        /// This means it is already inlined
        /// and can be used
        Inline(Node),
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct ModuleName(Vec<String>);
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

type ModuleHashMap = HashMap<ModuleName, ModuleContext>;
#[derive(Debug, Clone)]
pub struct ModuleMap(pub ModuleHashMap);

type ModuleContextHashMap = HashMap<FunctionName, FunctionContext>;
#[derive(Debug, Clone)]
pub struct ModuleContext(pub ModuleContextHashMap);

impl ModuleMap {
    pub fn new() -> Self {
        ModuleMap(HashMap::new())
    }

    pub fn insert(&mut self, key: ModuleName, value: ModuleContext) -> Option<ModuleContext> {
        self.0.insert(key, value)
    }
}

impl ModuleContext {
    pub fn new() -> Self {
        ModuleContext(HashMap::new())
    }

    pub fn insert(&mut self, key: FunctionName, value: FunctionContext) -> Option<FunctionContext> {
        self.0.insert(key, value)
    }
}

impl ModuleMap {
    /// Parses a directory of modules into their AST representation,
    /// strips their module body as a normalization step. They are only used as function/import referer.
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
        Ok(files
            .map(|(k, v)| (k.clone(), v.unwrap()))
            .map(|(k, v)| {
                let mut module = v.clone();
                module.code = PollutedNode::NoOp;

                (k, module)
            })
            .collect())
    }

    fn search_single(
        from: (&Vec<String>, &Module),
        to: (&Vec<String>, &Module),
        target: &ImpFunc,
        modules: &HashMap<Vec<String>, Module>,
    ) -> Result<ModuleContextHashMap, Vec<Error>> {
        // This means this is a single import.
        let mut imports = HashMap::new();

        // try if we have a function of that name
        let decl: Vec<_> = to.1.decl.iter().filter(|f| match *f.ident.clone() {
            Node::Ident(m) => m,
            _ => unreachable!()
        } == match *target.ident.clone() {
            Node::Ident(m) => m,
            _ => unreachable!()
        }).collect();

        if !decl.is_empty() {
            imports.insert(
                match *target.alias.unwrap_or(target.clone().ident).clone() {
                    Node::Ident(m) => m,
                    _ => unreachable!(),
                }
                .into(),
                FunctionContext::Import(FunctionImport {
                    module: to.0.clone().into(),
                    ident: match *target.clone().ident {
                        Node::Ident(m) => m,
                        _ => unreachable!(),
                    }
                    .into(),
                }),
            );
            return Ok(imports);
        }

        // try if we have a named import of that name
        let imp: Vec<_> =
            to.1.imp
                .iter()
                .filter(|imp| imp.funcs.is_left())
                .flat_map(|imp| {
                    imp.funcs.unwrap_left().iter().map(|func| {
                        (
                            imp.path
                                .iter()
                                .map(|p| match p {
                                    Node::Ident(m) => m.clone(),
                                    _ => unreachable!(),
                                })
                                .collect_vec(),
                            func.clone(),
                        )
                    })
                })
                .filter(|(module, func)| match *func.alias.unwrap_or(func.clone().ident).clone() {
                    Node::Ident(m) => m,
                    _ => unreachable!()
                } == match *target.ident.clone() {
                    Node::Ident(m) => m,
                    _ => unreachable!()
                })
                .collect();

        todo!()
    }

    fn search(
        from: (&Vec<String>, &Module),
        to: (&Vec<String>, &Module),
        target: &Either<ImpFunc, ImpWildcard>,
        modules: &HashMap<Vec<String>, Module>,
    ) -> Result<HashMap<Vec<String>, FunctionImport>, Vec<Error>> {
        // wildcard means to add everything we have in the module
        // how to avoid searching twice?
        let mut imports: ModuleContextHashMap = HashMap::new();

        match target {
            Either::Left(func) => {
                Self::search_single(from, to, func, modules);
            }
            Either::Right(_) => {
                // import everything
            }
        }

        todo!()
    }

    /// Creates an import map, this means it will follow and resolve all imports to their destination.
    /// A imports b from C, which imports b from D -> A will point to b
    /// things that are not under the fs namespace are searched under lib and added lazily to the ModuleMap
    fn context(
        key: &Vec<String>,
        modules: &mut HashMap<Vec<String>, Module>,
    ) -> Result<ModuleContext, Vec<Error>> {
        let mut ctx = ModuleContext::new();

        // get the current module
        let module = match modules.get(key) {
            None => Err(vec![Error::new_from_code(
                None,
                ErrorCode::CouldNotFindModule,
            )]),
            Some(m) => Ok(m),
        }?;

        // import all functions
        for decl in module.decl {
            let key = match *decl.ident.clone() {
                Node::Ident(i) => i,
                _ => unreachable!(),
            };

            ctx.insert(key.into(), FunctionContext::Func(decl));
        }

        // resolve all functions
        // wildcard special?

        Ok(ctx)
    }

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
        let mut modules = Self::parse(directory)?;

        // prefix initial modules with the fs prefix
        modules = modules
            .iter()
            .map(|(k, v)| {
                let mut k_new = k.clone();
                k_new.insert(0, "fs".to_string());

                (k_new, v.clone())
            })
            .collect();

        todo!()
    }
}

impl Into<ModuleName> for Vec<String> {
    fn into(self) -> ModuleName {
        ModuleName(self)
    }
}
impl Into<FunctionName> for String {
    fn into(self) -> FunctionName {
        FunctionName(self)
    }
}
