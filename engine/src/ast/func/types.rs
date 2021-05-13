use crate::ast::func::filesystem::Directory;
use crate::ast::module::{FuncDecl, Imp, ImpFunc, ImpWildcard, Module};
use crate::ast::node::Node;
use crate::ast::polluted::PollutedNode;
use crate::build::Builder;
use crate::errors::ErrorCode;
use crate::errors::{Error, ErrorVariant};
use crate::utils::check_errors;
use either::Either;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::PathBuf;

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

        // we know these are all safe thanks to check_errors
        Ok(files
            .map(|(k, v)| (k.clone(), v.unwrap()))
            .map(|(k, v)| {
                let mut module = v.clone();
                module.code = PollutedNode::NoOp;

                (k, module)
            })
            .collect())
    }

    fn find_module(
        modules: &mut HashMap<Vec<String>, Module>,
        imp: Imp,
    ) -> Result<(Vec<String>, Module), Vec<Error>> {
        let module_name: Vec<_> = imp
            .path
            .iter()
            .map(|p| match p.clone() {
                Node::Ident(m) => m,
                _ => unreachable!(),
            })
            .collect();

        // fs = filesystem, we cannot fetch those and early skip them.
        // if the module is none, this indicates that it isn't loaded, or not found,
        // Should the module not start with fs (<- indicates that it is locally available)
        // then we search the ./lib directory for file. We do so by walking the tree and failing
        // early if it does not exist.
        // if nothing is found. We parse that module and add it to the modules to avoid unnecessary
        // re-parsing.
        let module = modules.get(&module_name);
        if module.is_none() && module_name.first().cloned() != Some("fs".to_string()) {
            let mut buffer = PathBuf::new();
            buffer.push("./lib");

            for p in module_name {
                buffer.push(p);

                if !buffer.exists() {
                    break;
                }
            }

            if buffer.is_file() {
                let contents =
                    read_to_string(buffer).map_err(|err| vec![Error::new_from_io(err)])?;
                let mut contents = Builder::parse(contents.as_str(), None)
                    .map_err(|err| Error::new_from_parse(err))?;
                // erase all code
                contents.code = PollutedNode::NoOp;

                modules.insert(module_name.clone(), contents);
            }
        }

        return if module.is_none() {
            Err(vec![Error::new_from_code(
                Some(imp.lno),
                ErrorCode::CouldNotFindModule {
                    module: module_name.join("::"),
                },
            )])
        } else {
            Ok((module_name, module.unwrap().clone()))
        };
    }

    /// This checks the history, and start point to check if we have a circular import somewhere
    /// Having one would mean that we cannot fully import and we need to report an error.
    fn catch_circular(
        from: (&Vec<String>, &Module),
        to: (&Vec<String>, &Module),
        history: Option<Vec<Vec<String>>>,
    ) -> Result<(), Vec<Error>> {
        let mut history = history.unwrap_or_default();
        history.insert(0, from.0.clone());

        let circular = history
            .iter()
            .enumerate()
            .filter(|(idx, h)| h.clone() == to.0)
            .next();

        if circular.is_some() {
            let (idx, circular) = circular.unwrap();
            let history: Vec<String> = history.iter().map(|f| f.join("::")).collect();
            let (prev, path) = history.split_at(idx);

            Err(vec![Error::new_from_code(
                None,
                ErrorCode::CircularImport {
                    message: format!(
                        "Found Circular Import, {} tried to import itself. ({})",
                        circular.join("::"),
                        [prev.join(" -> "), path.join(" -> ")].join(" | ")
                    ),
                    history,
                    origin: from.0.join("::"),
                },
            )])
        } else {
            Ok(())
        }
    }

    /// This is used to import via *
    /// fetches all functions and returns them.
    fn resolve_wildcard(
        from: (&Vec<String>, &Module),
        to: (&Vec<String>, &Module),
        modules: &mut HashMap<Vec<String>, Module>,
        history: Option<Vec<Vec<String>>>,
    ) -> Result<ModuleContextHashMap, Vec<Error>> {
        Self::catch_circular(from, to, history)?;

        let mut history = history.unwrap_or_default();
        history.push(to.0.clone());

        let mut imports: ModuleContextHashMap = HashMap::new();
        let mut errors: Vec<Error> = vec![];

        // add our functions declarations (if we have any)
        for func in to.1.decl {
            let name: FunctionName = match *func.ident.clone() {
                Node::Ident(m) => m,
                _ => unreachable!(),
            }
            .into();

            imports.insert(
                name.clone(),
                FunctionContext::Import(FunctionImport {
                    module: to.0.clone().into(),
                    ident: name,
                }),
            );
        }

        // add our imports (if we have any), either recursively calls ourselves
        // (with a guard in place to stop circular imports) or finds a single module.
        for imp in to.1.imp {
            let res = Self::find_module(modules, imp.clone());
            if res.is_err() {
                errors.extend(res.unwrap_err());
                continue;
            }
            let (module_name, module) = res.unwrap();

            let module_imports = match imp.funcs {
                Either::Left(remote) => {
                    let mut individual = vec![];
                    for import in remote {
                        let import = Self::resolve_imp(
                            from,
                            (&module_name, &module),
                            &import,
                            modules,
                            Some(history.clone()),
                        );
                        if import.is_err() {
                            errors.extend(import.err().unwrap());
                            continue;
                        }
                        let import = import.unwrap();
                        individual.push(import);
                    }

                    individual
                }
                Either::Right(_) => {
                    let module_imports = Self::resolve_wildcard(
                        from,
                        (&module_name, &module),
                        modules,
                        Some(history.clone()),
                    );

                    if module_imports.is_err() {
                        errors.extend(module_imports.err().unwrap());
                        vec![]
                    } else {
                        vec![module_imports.unwrap()]
                    }
                }
            };

            // tries to resolve them, if there is an error, append it and exit as late as possible
            for res in module_imports {
                for (name, context) in res {
                    if imports.get(&name).is_some() {
                        errors.push(Error::new_from_code(
                            Some(imp.lno),
                            ErrorCode::FunctionNameCollision {
                                module: to.0.clone().join("::"),
                                func: name.0,
                                count: None,
                            },
                        ));
                    } else {
                        imports.insert(name, context);
                    }
                }
            }
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(imports)
    }

    // Search for a single import,
    // this means the semantics of
    // FROM x::y::z IMPORT a as b
    // or
    // FROM x::y::z IMPORT (a as b, c, d as e)
    fn resolve_imp(
        from: (&Vec<String>, &Module),
        to: (&Vec<String>, &Module),
        target: &ImpFunc,
        modules: &mut HashMap<Vec<String>, Module>,
        history: Option<Vec<Vec<String>>>,
    ) -> Result<ModuleContextHashMap, Vec<Error>> {
        Self::catch_circular(from, to, history)?;

        // This means this is a single import.
        let mut imports = HashMap::new();
        let mut history = history.unwrap_or_default();
        history.push(to.0.clone());

        let mut errors = vec![];

        // try if we have a function of that name
        let decl: Vec<_> = to.1.decl.iter().filter(|f| match *f.ident.clone() {
            Node::Ident(m) => m,
            _ => unreachable!()
        } == match *target.ident.clone() {
            Node::Ident(m) => m,
            _ => unreachable!()
        }).collect();

        // if there is a declaration if the same name, then we can just use this.
        // This is always our end-state.
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

        // This flattens all imports. Every import is (module, vec<import>),
        // this flattens the result into (module, import), with that approach
        // we can also guarantee unique results.
        // We also eliminate the same import using .unique()
        let imp: Vec<_> =
            to.1.imp
                .iter()
                .filter(|imp| imp.funcs.is_left())
                .flat_map(|imp| {
                    imp.funcs.unwrap_left().iter().map(|func| {
                        (
                            imp.clone(),
                            func.clone(),
                        )
                    })
                })
                .filter(|(_, func)| match *func.alias.unwrap_or(func.clone().ident).clone() {
                    Node::Ident(m) => m,
                    _ => unreachable!()
                } == match *target.ident.clone() {
                    Node::Ident(m) => m,
                    _ => unreachable!()
                })
                .unique()
                .collect();

        // if there is an import matching our target alias/ident, then use that to find the correct thing.
        if !imp.is_empty() {
            let (imp, func) = imp.get(0).unwrap().clone();
            let (module_name, module) = Self::find_module(modules, imp)?;

            return Self::resolve_imp(
                from,
                (&module_name, &module),
                &func,
                modules,
                Some(history.clone()),
            );
        }

        // look in all wildcards if we find something, return that,
        // this means that we potentially search multiple times, but that should be fine
        // for now, definitely room for improvement
        let wildcards: Vec<_> = to.1.imp.iter().filter(|i| i.funcs.is_right()).collect();

        for wildcard in wildcards {
            let res = Self::find_module(modules, wildcard.clone());
            if res.is_err() {
                errors.extend(res.unwrap_err());
                continue;
            }
            let (module_name, module) = res.unwrap();

            let res = Self::resolve_wildcard(
                from,
                (&module_name, &module),
                modules,
                Some(history.clone()),
            );
            if res.is_err() {
                errors.extend(res.unwrap_err());
                continue;
            }
            let wildcard_context = res.unwrap();
            let func_name: FunctionName = match *target.ident.clone() {
                Node::Ident(m) => m,
                _ => unreachable!(),
            }
            .into();
            let func = wildcard_context.get(&func_name);

            if func.is_some() {
                imports.insert(
                    match *target.alias.unwrap_or(target.clone().ident).clone() {
                        Node::Ident(m) => m,
                        _ => unreachable!(),
                    }
                    .into(),
                    func.unwrap().clone(),
                );

                // early return, we do not need to look at the others
                return Ok(imports);
            }
        }

        // instead of immediately failing we try to be as late as possible, we know we're going
        // to fail, but wildcard also potentially adds new errors, so this is used to not overwrite them.
        errors.push(Error::new_from_code(
            None,
            ErrorCode::CouldNotFindFunction {
                module: to.0.join("::"),
                func: match *target.ident.clone() {
                    Node::Ident(m) => m,
                    _ => unreachable!(),
                },
            },
        ));

        Err(errors)
    }

    /// Creates an import map, this means it will follow and resolve all imports to their destination.
    /// A imports b from C, which imports b from D -> A will point to b
    /// things that are not under the fs namespace are searched under lib and added lazily to the ModuleMap
    fn resolve(
        from: (&Vec<String>, &Module),
        modules: &mut HashMap<Vec<String>, Module>,
    ) -> Result<ModuleContextHashMap, Vec<Error>> {
        // wildcard means to add everything we have in the module
        // how to avoid searching twice?
        let mut context: ModuleContextHashMap = HashMap::new();
        let mut errors = vec![];

        for imp in from.1.imp {
            let res = Self::find_module(modules, imp.clone());
            if res.is_err() {
                errors.extend(res.unwrap_err())
            }
            let (module_name, module) = res.unwrap();

            let results = match imp.funcs {
                Either::Left(funcs) => {
                    let mut results = vec![];
                    for func in funcs {
                        let res =
                            Self::resolve_imp(from, (&module_name, &module), &func, modules, None);
                        if res.is_err() {
                            errors.extend(res.unwrap_err());
                            continue;
                        }
                        let res = res.unwrap();
                        results.push(res);
                    }
                    results
                }
                Either::Right(_) => {
                    let res = Self::resolve_wildcard(from, (&module_name, &module), modules, None);
                    if res.is_err() {
                        errors.extend(res.unwrap_err());
                    }
                    vec![res.unwrap()]
                }
            };

            for res in results {
                let overlapping: Vec<_> = res.keys().filter(|k| context.contains_key(k)).collect();

                for collision in overlapping {
                    errors.push(Error::new_from_code(
                        Some(imp.lno),
                        ErrorCode::FunctionNameCollision {
                            module: from.0.join("::"),
                            func: collision.clone().0,
                            count: None,
                        },
                    ))
                }
                if !overlapping.is_empty() {
                    continue;
                }

                context.extend(res);
            }
        }

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(context)
        }
    }

    /// This is the first preliminary collision check, it checks if there are any colliding
    /// names (without wildcard import). The thorough check of wildcard violates happens at a
    /// later date, to be exact they happen in the resolution stage.
    fn basic_collision_check(modules: &HashMap<Vec<String>, Module>) -> Result<(), Vec<Error>> {
        let mut errors = vec![];

        for (name, module) in modules {
            let decl = module.clone().decl;
            let imp = module.clone().imp;

            // chain the declarations and import names together
            let duplicates: HashMap<_, _> = decl
                .iter()
                .map(|d| match *d.clone().ident {
                    Node::Ident(m) => m,
                    _ => unreachable!(),
                })
                .chain(
                    imp.iter()
                        // ignore wildcard imports, as we do not know yet if they can collide
                        .filter(|i| i.funcs.is_left())
                        .flat_map(|i| i.funcs.unwrap_left())
                        // first try to use the alias, if there is none use the real name
                        .map(|i| match *i.alias.unwrap_or(i.clone().ident).clone() {
                            Node::Ident(m) => m,
                            _ => unreachable!(),
                        }),
                )
                // use counts() from itertools to determine if there are more than 1 present
                .counts()
                .iter()
                .filter(|(k, v)| v.clone().clone() > 1)
                .collect();

            // if there are any duplicates add them to the error list.
            for (func, count) in duplicates {
                errors.push(Error::new(
                    (0, 0),
                    ErrorVariant::ErrorCode(ErrorCode::FunctionNameCollision {
                        module: name.clone().join("::"),
                        func: func.clone(),
                        count: Some(count.clone()),
                    }),
                ))
            }
        }

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(())
        }
    }

    pub fn from(main: Module, directory: Directory) -> Result<ModuleMap, Vec<Error>> {
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
        modules.insert(vec!["fs".to_string(), "main".to_string()], main);

        Self::basic_collision_check(&modules)?;

        // import and resolve everything properly
        let mut errors = vec![];
        let mut map = ModuleMap::new();

        for (name, module) in modules.clone() {
            let res = Self::resolve((&name, &module), &mut modules);
            if res.is_err() {
                errors.extend(res.unwrap_err());
            }
            let context = ModuleContext(res.unwrap());
            map.insert(ModuleName(name), context);
        }

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(map)
        }
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

#[cfg(test)]
mod test {
    use crate::ast::func::filesystem::{Directory, Path};
    use crate::ast::func::types::ModuleMap;
    use crate::build::Builder;
    use indoc::indoc;

    #[test]
    fn test_simple_fs_import() {
        let snip = indoc! {"
        FROM fs::a IMPORT b
        "};

        let sibling = indoc! {"
        FN a(b) -> c DECL
            ...
        END
        "};

        let mut dir = Directory::new();
        dir.insert("a".to_string(), sibling.to_string().into());

        let ast = Builder::parse(snip, None);
        assert!(ast.is_ok());
        let ast = ast.unwrap();

        let map = ModuleMap::from(ast, dir);
        assert!(map.is_ok());
        let map = map.unwrap();

        println!("{:?}", map)
    }
}
