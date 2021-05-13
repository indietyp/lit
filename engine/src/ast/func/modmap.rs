use std::collections::HashMap;
use std::fs::read_to_string;

use either::Either;
use itertools::Itertools;

use crate::ast::func::filesystem::Directory;
use crate::ast::func::modctx::{ModuleContext, ModuleContextHashMap};
use crate::ast::func::types::{FunctionContext, FunctionImport, FunctionName, ModuleName};
use crate::ast::module::{Imp, ImpFunc, Module};
use crate::ast::node::Node;
use crate::ast::polluted::PollutedNode;
use crate::build::Builder;
use crate::errors::{Error, ErrorCode, ErrorVariant};
use crate::utils::check_errors;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleMap(pub ModuleHashMap);
pub type ModuleHashMap = HashMap<ModuleName, ModuleContext>;

type ImpFuncKeyFrom = (Vec<String>, Module);
type ImpFuncKey = (ImpFuncKeyFrom, ImpFunc);
type CacheResult = Result<ModuleContextHashMap, Vec<Error>>;

#[derive(Debug)]
struct Cache {
    wildcard: HashMap<(Vec<String>, Module), CacheResult>,
    impfunc: HashMap<ImpFuncKey, CacheResult>,
}

impl ModuleMap {
    pub fn new() -> Self {
        ModuleMap(HashMap::new())
    }

    pub fn insert(&mut self, key: ModuleName, value: ModuleContext) -> Option<ModuleContext> {
        self.0.insert(key, value)
    }
}

impl ModuleMap {
    /// Parses a directory of modules into their AST representation,
    /// strips their module body as a normalization step. They are only used as function/import referer.
    fn parse(directory: Directory) -> Result<HashMap<Vec<String>, Module>, Vec<Error>> {
        let preliminary: HashMap<_, _> = directory.walk().collect();
        let files = preliminary
            .iter()
            .map(|(k, v)| (k, Builder::parse(v.as_str(), None)));

        let results: Vec<_> = files
            .clone()
            .map(|(_, v)| v.map_err(|err| vec![Error::new_from_parse(err)]))
            .collect();
        check_errors(&results)?;

        // we know these are all safe thanks to check_errors
        Ok(files
            .map(|(k, v)| (k.clone(), v.unwrap()))
            .map(|(k, v)| {
                let mut module = v;
                module.code = PollutedNode::NoOp;

                (k, module)
            })
            .collect())
    }

    /// Utility function to find a specific module by [`Imp`]. THis uses modules to check,
    /// if the target module does **not** start fs and the module is not yet present, search for it
    /// in the local lib folder and add it to the `modules` HashMap.
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
        let mut module = modules.get(&module_name).cloned();
        if module.is_none() && module_name.first().cloned() != Some("fs".to_string()) {
            let mut buffer = PathBuf::new();
            buffer.push("./lib");
            buffer.extend(module_name.clone());

            let mut inserted = 0usize;
            // there are multiple extension that we support, search all of them
            for extension in &["lp", "loop", "while", "wh"] {
                let mut buf = buffer.clone();
                buf.set_extension(extension);

                if buf.exists() && buf.is_file() {
                    let contents =
                        read_to_string(buf).map_err(|err| vec![Error::new_from_io(err)])?;
                    let mut contents =
                        Builder::parse(contents.as_str(), None).map_err(Error::new_from_parse)?;
                    // erase all code
                    contents.code = PollutedNode::NoOp;

                    modules.insert(module_name.clone(), contents.clone());
                    module = Some(contents);
                    inserted += 1;
                }
            }

            if inserted > 1 {
                return Err(vec![Error::new_from_code(
                    Some(imp.lno),
                    ErrorCode::MultipleModuleCandidates {
                        module: module_name.join("::"),
                        count: inserted,
                    },
                )]);
            }
        }

        match module {
            None => Err(vec![Error::new_from_code(
                Some(imp.lno),
                ErrorCode::CouldNotFindModule {
                    module: module_name.join("::"),
                },
            )]),
            Some(module) => Ok((module_name, module)),
        }
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
            .clone()
            .into_iter()
            .enumerate()
            .find(|(_, h)| *h == *to.0);

        if let Some((idx, circular)) = circular {
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
        cache: &mut Cache,
    ) -> Result<ModuleContextHashMap, Vec<Error>> {
        // check if we already cached the result
        let cache_key = (to.0.clone(), to.1.clone());
        if let Some(cached) = cache.wildcard.get(&cache_key) {
            println!("Hit the wildcard cache");
            return cached.clone();
        }

        Self::catch_circular(from, to, history.clone())?;

        let mut history = history.unwrap_or_default();
        history.push(to.0.clone());

        let mut imports: ModuleContextHashMap = HashMap::new();
        let mut errors: Vec<Error> = vec![];

        // add our functions declarations (if we have any)
        for func in &to.1.decl {
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
        for imp in &to.1.imp {
            let res = Self::find_module(modules, imp.clone());
            if let Err(err) = res {
                errors.extend(err);
                continue;
            }
            let (module_name, module) = res.unwrap();

            let module_imports = match &imp.funcs {
                Either::Left(remote) => {
                    let mut individual = vec![];
                    for import in remote {
                        let import = Self::resolve_impfunc(
                            from,
                            (&module_name, &module),
                            &import,
                            modules,
                            Some(history.clone()),
                            cache,
                        );
                        if let Err(err) = import {
                            errors.extend(err);
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
                        cache,
                    );

                    if let Err(err) = module_imports {
                        errors.extend(err);
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

        let res = if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(imports)
        };

        cache.wildcard.insert(cache_key, res.clone());
        res
    }

    // Search for a single import,
    // this means the semantics of
    // FROM x::y::z IMPORT a as b
    // or
    // FROM x::y::z IMPORT (a as b, c, d as e)
    fn resolve_impfunc(
        from: (&Vec<String>, &Module),
        to: (&Vec<String>, &Module),
        target: &ImpFunc,
        modules: &mut HashMap<Vec<String>, Module>,
        history: Option<Vec<Vec<String>>>,
        cache: &mut Cache,
    ) -> Result<ModuleContextHashMap, Vec<Error>> {
        let cache_key = ((to.0.clone(), to.1.clone()), target.clone());
        if let Some(cached) = cache.impfunc.get(&cache_key) {
            println!("Hit the impfunc cache");
            return cached.clone();
        }

        Self::catch_circular(from, to, history.clone())?;

        // This means this is a single import.
        let mut imports: ModuleContextHashMap = HashMap::new();
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
                match *target.clone().alias.unwrap_or(target.clone().ident) {
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

            let res = Ok(imports);
            cache.impfunc.insert(cache_key, res.clone());
            return res;
        }

        // This flattens all imports. Every import is (module, vec<import>),
        // this flattens the result into (module, import), with that approach
        // we can also guarantee unique results.
        // We also eliminate the same import using .unique()
        let local_imports =
            to.1.imp.clone()
                .into_iter()
                .filter(|imp| imp.funcs.is_left())
                .flat_map(|imp| -> Vec<(Imp, ImpFunc)> {
                    imp.funcs.clone().unwrap_left().into_iter().map(|func| (imp.clone(), func)).collect()
                })
                .filter(|(_, func)| match *func.clone().alias.unwrap_or(func.clone().ident) {
                    Node::Ident(m) => m,
                    _ => unreachable!()
                } == match *target.ident.clone() {
                    Node::Ident(m) => m,
                    _ => unreachable!()
                })
                .unique()
                .next();

        // if there is an import matching our target alias/ident, then use that to find the correct target.
        if let Some((imp, func)) = local_imports {
            let (module_name, module) = Self::find_module(modules, imp)?;

            let res = Self::resolve_impfunc(
                from,
                (&module_name, &module),
                &func,
                modules,
                Some(history),
                cache,
            );

            cache.impfunc.insert(cache_key, res.clone());
            return res;
        }

        // Look in all wildcards if we find our target function return it, if not return all errors
        // collected and a [`ErrorKind::CouldNotFindFunction`].
        // Due to how we search, this means that we potentially search multiple times every wildcard,
        // which is not optional and definitely something that could use a [REWORK].
        // You could imagine creating a cache of some sorts, due to code size and speed our current
        // implementation is good enough for educational purposes.
        let wildcards: Vec<_> = to.1.imp.iter().filter(|i| i.funcs.is_right()).collect();
        for wildcard in wildcards {
            let res = Self::find_module(modules, wildcard.clone());
            if let Err(err) = res {
                errors.extend(err);
                continue;
            }
            let (module_name, module) = res.unwrap();

            let res = Self::resolve_wildcard(
                from,
                (&module_name, &module),
                modules,
                Some(history.clone()),
                cache,
            );
            if let Err(err) = res {
                errors.extend(err);
                continue;
            }
            let wildcard_context = res.unwrap();
            let func_name: FunctionName = match *target.ident.clone() {
                Node::Ident(m) => m,
                _ => unreachable!(),
            }
            .into();
            let func = wildcard_context.get(&func_name);

            if let Some(func) = func {
                imports.insert(
                    match *target.clone().alias.unwrap_or(target.clone().ident) {
                        Node::Ident(m) => m,
                        _ => unreachable!(),
                    }
                    .into(),
                    func.clone(),
                );

                // early return, we do not need to look at the others
                let res = Ok(imports);
                cache.impfunc.insert(cache_key, res.clone());
                return res;
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

        let res = Err(errors);
        cache.impfunc.insert(cache_key, res.clone());
        res
    }

    /// Creates an import map, this means it will follow and resolve all imports to their destination.
    /// A imports b from C, which imports b from D -> A will point to b
    /// things that are not under the fs namespace are searched under lib and added lazily to the ModuleMap
    fn resolve(
        from: (&Vec<String>, &Module),
        modules: &mut HashMap<Vec<String>, Module>,
        cache: &mut Cache,
    ) -> Result<ModuleContextHashMap, Vec<Error>> {
        // wildcard means to add everything we have in the module
        // how to avoid searching twice?
        let mut context: ModuleContextHashMap = HashMap::new();
        let mut errors = vec![];

        for imp in &from.1.imp {
            let res = Self::find_module(modules, imp.clone());
            if let Err(err) = res {
                errors.extend(err);
                continue;
            }
            let (module_name, module) = res.unwrap();

            let results = match &imp.funcs {
                Either::Left(funcs) => {
                    let mut results = vec![];
                    for func in funcs {
                        let res = Self::resolve_impfunc(
                            from,
                            (&module_name, &module),
                            &func,
                            modules,
                            None,
                            cache,
                        );
                        if let Err(err) = res {
                            errors.extend(err);
                            continue;
                        }
                        let res = res.unwrap();
                        results.push(res);
                    }
                    results
                }
                Either::Right(_) => {
                    let res =
                        Self::resolve_wildcard(from, (&module_name, &module), modules, None, cache);
                    if let Err(err) = res {
                        errors.extend(err);
                        vec![]
                    } else {
                        vec![res.unwrap()]
                    }
                }
            };

            for res in results {
                let overlapping: Vec<_> = res.keys().filter(|k| context.contains_key(k)).collect();

                for collision in overlapping.clone() {
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
    /// names (without wildcard import). The thorough check of wildcard violations happens at a
    /// later date, to be exact they happen in the resolution stage.
    fn basic_collision_check(modules: &HashMap<Vec<String>, Module>) -> Result<(), Vec<Error>> {
        let mut errors = vec![];

        for (name, module) in modules {
            let decl = module.clone().decl;
            let imp = module.clone().imp;

            // chain the declarations and import names together
            let flat: HashMap<_, _> = decl
                .into_iter()
                .map(|d| match *d.ident {
                    Node::Ident(m) => m,
                    _ => unreachable!(),
                })
                .chain(
                    imp.iter()
                        // ignore wildcard imports, as we do not know yet if they can collide
                        .filter(|i| (*i).funcs.is_left())
                        .flat_map(|i| i.clone().funcs.unwrap_left())
                        // first try to use the alias, if there is none use the real name
                        .map(|i| match *i.clone().alias.unwrap_or(i.ident) {
                            Node::Ident(m) => m,
                            _ => unreachable!(),
                        }),
                )
                .counts();

            // use counts() from itertools to determine if there are more than 1 present
            let duplicates: HashMap<_, _> = flat.into_iter().filter(|(_, v)| *v > 1).collect();

            // if there are any duplicates add them to the error list.
            for (func, count) in duplicates {
                errors.push(Error::new(
                    (0, 0),
                    ErrorVariant::ErrorCode(ErrorCode::FunctionNameCollision {
                        module: name.clone().join("::"),
                        func: func.clone(),
                        count: Some(count),
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

    /// The [`ModuleMap.resolve`] function only resolves the imports,
    /// but does not add the actual functions, this is done by this specific function.
    /// It fetches all declared functions for each loaded module and attaches them to the
    /// [`ModuleMap`]. This function also check potential collisions.
    fn insert_funcs(
        modules: &HashMap<Vec<String>, Module>,
        context: &mut ModuleMap,
    ) -> Result<(), Vec<Error>> {
        let mut errors = vec![];

        for (name, module) in modules {
            let module_name = ModuleName(name.clone());
            // create or get a new context, there could be instances where a function is loaded
            // but has no actual other import
            let mut ctx = context
                .0
                .get_mut(&module_name)
                .cloned()
                .unwrap_or_else(ModuleContext::new);

            for func in &module.decl {
                let function_name: FunctionName = match *func.clone().ident {
                    Node::Ident(m) => m,
                    _ => unreachable!(),
                }
                .into();

                // something with that name already exists in the NS
                // we know we fail, but just continue and get some more errors
                if ctx.0.contains_key(&function_name) {
                    errors.push(Error::new_from_code(
                        Some(func.lno),
                        ErrorCode::FunctionNameCollision {
                            module: name.join("::"),
                            func: function_name.0,
                            count: None,
                        },
                    ));
                    continue;
                }

                ctx.0
                    .insert(function_name, FunctionContext::Func(func.clone()));
            }

            context.0.insert(module_name, ctx);
        }

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(())
        }
    }

    /// Creates a new ModuleMap from [`Module`] and a local filesystem (done through [`Directory`])
    /// The from method does a distinctive 3 step process:
    /// 1) parse the fs modules and flatten them
    /// 2) check if there are any collisions in the modules
    /// 3) recursively resolve all imports to their destination,
    ///     this means A -> B -> C is resolved to A -> C
    /// 4) insert all functions to all modules (includes another collision check)
    /// 5) return the result or return all errors where we could recover from
    pub fn from(main: Module, directory: Directory) -> Result<ModuleMap, Vec<Error>> {
        // The directory is always prefixed with fs::,
        // while all others are looking into the /lib/ folder
        let mut modules: HashMap<Vec<String>, Module> = Self::parse(directory)?;

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
        let mut cache = Cache {
            wildcard: HashMap::new(),
            impfunc: HashMap::new(),
        };

        // iterate over all modules, not only Main so that we can be sure everything is included
        for (name, module) in modules.clone() {
            let res = Self::resolve((&name, &module), &mut modules, &mut cache);
            if let Err(err) = res {
                errors.extend(err);
                continue;
            }
            let context = ModuleContext(res.unwrap());
            map.insert(ModuleName(name), context);
        }

        let res = Self::insert_funcs(&modules, &mut map);
        if let Err(err) = res {
            errors.extend(err)
        }

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(map)
        }
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::ast::control::Control;
    use crate::ast::func::filesystem::Directory;
    use crate::ast::func::modctx::ModuleContext;
    use crate::ast::func::modmap::ModuleMap;
    use crate::ast::func::types::FunctionContext::{Func, Import};
    use crate::ast::func::types::{FunctionImport, ModuleName};
    use crate::ast::module::FuncDecl;
    use crate::ast::node::Node;
    use crate::ast::polluted::PollutedNode;
    use crate::build::Builder;
    use crate::errors::Error;

    #[test]
    fn test_fs_import() -> Result<(), Vec<Error>> {
        let snip = indoc! {"
        FROM fs::a IMPORT b
        "};

        let sibling = indoc! {"
        FN b(b) -> c DECL
            ...
        END
        "};

        let mut dir = Directory::new();
        dir.insert("a".to_string(), sibling.to_string().into());

        let ast = Builder::parse(snip, None).map_err(|err| vec![Error::new_from_parse(err)])?;
        let map = ModuleMap::from(ast, dir)?;

        let mut expected = ModuleMap::new();
        expected.insert(vec!["fs", "main"].into(), {
            let mut ctx = ModuleContext::new();
            ctx.0.insert(
                "b".into(),
                Import(FunctionImport {
                    module: vec!["fs", "a"].into(),
                    ident: "b".into(),
                }),
            );

            ctx
        });
        expected.insert(vec!["fs", "a"].into(), {
            let mut ctx = ModuleContext::new();
            ctx.0.insert(
                "b".into(),
                Func(FuncDecl {
                    lno: (1, 3),

                    ident: Box::new(Node::Ident("b".into())),
                    params: vec![Node::Ident("b".into())],
                    ret: Box::new(Node::Ident("c".into())),

                    terms: Box::new(PollutedNode::Control(Control::Terms(vec![
                        PollutedNode::NoOp,
                    ]))),
                }),
            );
            ctx
        });

        assert_eq!(map, expected);

        Ok(())
    }

    #[test]
    fn test_fs_nested_import_with_alias() -> Result<(), Vec<Error>> {
        let snip = indoc! {"
        FROM fs::a IMPORT b
        "};

        let module_a = indoc! {"
        FROM fs::b IMPORT c as b
        "};
        let module_b = indoc! {"
        FN c(d) -> e DECL
            ...
        END
        "};

        let mut dir = Directory::new();
        dir.insert("a".to_string(), module_a.to_string().into());
        dir.insert("b".to_string(), module_b.to_string().into());

        let ast = Builder::parse(snip, None).map_err(|err| vec![Error::new_from_parse(err)])?;
        let map = ModuleMap::from(ast, dir)?;

        let mut expected = ModuleMap::new();
        expected.insert(vec!["fs", "main"].into(), {
            let mut ctx = ModuleContext::new();
            ctx.0.insert(
                "b".into(),
                Import(FunctionImport {
                    module: vec!["fs", "b"].into(),
                    ident: "c".into(),
                }),
            );

            ctx
        });
        expected.insert(vec!["fs", "a"].into(), {
            let mut ctx = ModuleContext::new();
            ctx.0.insert(
                "b".into(),
                Import(FunctionImport {
                    module: vec!["fs", "b"].into(),
                    ident: "c".into(),
                }),
            );

            ctx
        });
        expected.insert(vec!["fs", "b"].into(), {
            let mut ctx = ModuleContext::new();
            ctx.0.insert(
                "c".into(),
                Func(FuncDecl {
                    lno: (1, 3),

                    ident: Box::new(Node::Ident("c".into())),
                    params: vec![Node::Ident("d".into())],
                    ret: Box::new(Node::Ident("e".into())),

                    terms: Box::new(PollutedNode::Control(Control::Terms(vec![
                        PollutedNode::NoOp,
                    ]))),
                }),
            );
            ctx
        });

        assert_eq!(map, expected);

        Ok(())
    }

    #[test]
    fn test_std_import() -> Result<(), Vec<Error>> {
        let snip = indoc! {"
        FROM std::math IMPORT max
        "};

        let dir = Directory::new();
        let ast = Builder::parse(snip, None).map_err(|err| vec![Error::new_from_parse(err)])?;
        let map = ModuleMap::from(ast, dir)?;

        let module_name = vec!["fs", "main"].into();
        let main = map.0.get(&module_name);
        assert!(main.is_some());
        let main = main.unwrap().clone();

        let mut expected = ModuleContext::new();
        expected.0.insert(
            "max".into(),
            Import(FunctionImport {
                module: vec!["std", "math"].into(),
                ident: "max".into(),
            }),
        );

        assert_eq!(main, expected);

        let math_name = vec!["std", "math"].into();
        assert!(map.0.contains_key(&math_name));

        Ok(())
    }

    #[test]
    fn test_std_wildcard() -> Result<(), Vec<Error>> {
        todo!()
    }

    #[test]
    fn test_fs_wildcard() -> Result<(), Vec<Error>> {
        let snip = indoc! {"
        FROM fs::a IMPORT d as c
        "};

        let module_a = indoc! {"
        FROM fs::b IMPORT *
        "};
        let module_b = indoc! {"
        FN c(d) -> e DECL
            ...
        END
        FN d(d) -> e DECL
            ...
        END
        "};

        let mut dir = Directory::new();
        dir.insert("a".to_string(), module_a.to_string().into());
        dir.insert("b".to_string(), module_b.to_string().into());

        let ast = Builder::parse(snip, None).map_err(|err| vec![Error::new_from_parse(err)])?;
        let map = ModuleMap::from(ast, dir)?;

        let module_name = vec!["fs", "main"].into();
        let main = map.0.get(&module_name);
        assert!(main.is_some());
        let main = main.unwrap().clone();

        let mut expected = ModuleContext::new();
        expected.0.insert(
            "c".into(),
            Import(FunctionImport {
                module: vec!["fs", "b"].into(),
                ident: "d".into(),
            }),
        );

        assert_eq!(main, expected);

        Ok(())
    }

    #[test]
    fn test_circular_import() {
        todo!()
    }

    #[test]
    fn test_import_name_clash() {
        todo!()
    }

    #[test]
    fn test_fn_name_clash() {
        todo!()
    }

    #[test]
    fn test_wildcard_name_clash() {
        todo!()
    }
}
