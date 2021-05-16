use std::collections::HashMap;

use itertools::Itertools;

use crate::ast::context::CompileContext;
use crate::ast::hir::func::decl::FuncDecl;
use crate::ast::hir::func::structs::modname::ModuleName;
use crate::ast::hir::func::structs::qualname::FuncQualName;
use crate::ast::hir::func::structs::{FuncContext, FuncImport, FuncInline};
use crate::ast::hir::func::utils::{could_not_find_function, could_not_find_module};
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
        let qual: FuncQualName = (self.module.clone(), self.ident.clone()).into();

        let module_ctx = context
            .modules
            .get(&self.module)
            .map_or(Err(could_not_find_module(None, &self.module)), |f| Ok(f))?;

        let func_ctx = module_ctx.get(&self.ident).map_or(
            Err(could_not_find_function(None, &self.module, &self.ident)),
            |f| Ok(f),
        )?;

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
        if let Err(err) = func_name {
            errors.extend(err)
        }
        if let Err(err) = params {
            errors.extend(err)
        }
        if let Err(err) = ret {
            errors.extend(err)
        }
        if !errors.is_empty() {
            return Err(errors);
        }
        let func_name = func_name.unwrap();
        let params = params.unwrap();
        let ret = ret.unwrap();

        let qual = (module.clone(), func_name.into()).into();
        context.dive(qual, |context, stack, locals| {
            let counts = stack.into_iter().counts();
            let counts: HashMap<_, _> = counts.into_iter().filter(|(k, v)| *v > 1).collect();

            // recursion detection, if something is more than twice on the callstack just error out.
            if !counts.is_empty() {
                return Err(counts
                    .into_iter()
                    .map(|(k, v)| {
                        Error::new_from_code(
                            Some(self.lno),
                            ErrorCode::FunctionRecursionDetected {
                                module: module.join("::"),
                                func: func_name,
                                count: Some(v),
                            },
                        )
                    })
                    .collect());
            }

            let terms = self.terms.lower(context)?;
            // TODO: do we really need to prefix everything?
            //  this means that calls() get potentially double prefixed
            terms.prefix(context);

            let inline = FuncInline {
                lno: self.lno,
                ident: func_name,
                params,
                ret,
                terms,
            };

            Ok(inline)
        })
    }
}
