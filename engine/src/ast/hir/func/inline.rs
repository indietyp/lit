use std::collections::HashMap;

use itertools::Itertools;

use crate::ast::context::CompileContext;
use crate::ast::hir::func::decl::FuncDecl;
use crate::ast::hir::func::structs::modname::ModuleName;
use crate::ast::hir::func::structs::qualname::FuncQualName;
use crate::ast::hir::func::structs::{FuncContext, FuncImport, FuncInline};
use crate::ast::hir::func::utils::{could_not_find_function, could_not_find_module, prefix_ident};
use crate::errors::{Error, ErrorCode, StdResult};

pub trait Inline {
    fn inline(&self, context: &mut CompileContext, module: &ModuleName) -> StdResult<FuncInline>;
}

impl Inline for FuncContext {
    fn inline(&self, context: &mut CompileContext, module: &ModuleName) -> StdResult<FuncInline> {
        match self {
            FuncContext::Import(imp) => imp.inline(context, module),
            FuncContext::Func(func) => func.inline(context, module),
            FuncContext::Inline(inline) => inline.inline(context, module),
        }
    }
}

impl Inline for FuncImport {
    fn inline(&self, context: &mut CompileContext, module: &ModuleName) -> StdResult<FuncInline> {
        let func_ctx = {
            let module_ctx = context
                .modules
                .get_mut(&self.module)
                .map_or(Err(could_not_find_module(None, &self.module)), Ok)?;

            module_ctx
                .get(&self.ident)
                .map_or(
                    Err(could_not_find_function(None, &self.module, &self.ident)),
                    Ok,
                )?
                .clone()
        };

        match func_ctx {
            FuncContext::Import(imp) => imp.inline(context, &self.module),
            FuncContext::Func(func) => func.inline(context, module),
            FuncContext::Inline(inline) => inline.inline(context, module),
        }
    }
}

impl Inline for FuncInline {
    fn inline(&self, _: &mut CompileContext, _: &ModuleName) -> StdResult<FuncInline> {
        Ok(self.clone())
    }
}

impl Inline for FuncDecl {
    fn inline(&self, context: &mut CompileContext, module: &ModuleName) -> StdResult<FuncInline> {
        let mut errors = vec![];

        // parse and unwrap the different needed values, doing this at the start
        // and without ? to accumulate errors
        let func_name = self.get_ident();
        let params = self.get_params();
        let ret = self.get_ret();
        if let Err(err) = func_name.clone() {
            errors.extend(err)
        }
        if let Err(err) = params.clone() {
            errors.extend(err)
        }
        if let Err(err) = ret.clone() {
            errors.extend(err)
        }
        if !errors.is_empty() {
            return Err(errors);
        }
        let func_name = func_name.unwrap();
        let params = params.unwrap();
        let ret = ret.unwrap();

        let qual: FuncQualName = (module.clone(), func_name.clone().into()).into();
        context.dive(qual.clone(), module.clone(), |context| {
            let counts = context
                .get_stack()
                .clone()
                .into_iter()
                .map(|s| s.caller)
                .filter(|s| s.is_some())
                .map(|s| s.unwrap())
                .counts();

            let counts: HashMap<_, _> = counts.into_iter().filter(|(_, v)| *v > 1).collect();

            // recursion detection, if something is more than twice on the callstack just error out.
            if !counts.is_empty() {
                return Err(counts
                    .into_iter()
                    .map(|(_, v)| {
                        Error::new_from_code(
                            Some(self.lno),
                            ErrorCode::FunctionRecursionDetected {
                                stack: context
                                    .get_stack()
                                    .iter()
                                    .map(|frame| {
                                        frame
                                            .clone()
                                            .caller
                                            .map(|m| m.to_string())
                                            .unwrap_or_else(|| frame.clone().module.to_string())
                                    })
                                    .collect(),
                                module: module.join("::"),
                                func: func_name.clone(),
                                count: Some(v),
                            },
                        )
                    })
                    .collect());
            }

            let count = context.incr_inline(qual.clone());

            // Note(bmahmoud) this means that inner calls will be double prefixed!
            let terms = self.terms.lower(context)?;
            let terms = terms.prefix(context, &qual, &count);

            let inline = FuncInline {
                lno: self.lno,
                ident: func_name.clone(),
                params: params
                    .clone()
                    .into_iter()
                    .map(|param| prefix_ident(&qual, &count, &param))
                    .collect(),
                ret: prefix_ident(&qual, &count, &ret),
                terms,
            };

            Ok(inline)
        })
    }
}
