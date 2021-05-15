use crate::ast::context::CompileContext;
use crate::ast::hir::func::decl::FuncDecl;
use crate::ast::hir::func::types::{
    FunctionContext, FunctionImport, FunctionInline, FunctionQualName, ModuleName,
};
use crate::ast::hir::func::utils::{could_not_find_function, could_not_find_module};
use crate::errors::StdResult;
use std::collections::HashSet;

pub trait Inline {
    fn inline(
        &self,
        context: &mut CompileContext,
        module: &ModuleName,
        history: &mut HashSet<FunctionQualName>,
    ) -> StdResult<FunctionInline>;
}

impl Inline for FunctionContext {
    fn inline(
        &self,
        context: &mut CompileContext,
        module: &ModuleName,
        history: &mut HashSet<FunctionQualName>,
    ) -> StdResult<FunctionInline> {
        match self {
            FunctionContext::Import(imp) => imp.inline(context, module, history),
            FunctionContext::Func(func) => func.inline(context, module, history),
            FunctionContext::Inline(inline) => inline.inline(context, module, history),
        }
    }
}

impl Inline for FunctionImport {
    fn inline(
        &self,
        context: &mut CompileContext,
        module: &ModuleName,
        history: &mut HashSet<FunctionQualName>,
    ) -> StdResult<FunctionInline> {
        let qual: FunctionQualName = (self.module.clone(), self.ident.clone()).into();

        let module_ctx = context
            .modules
            .get(&self.module)
            .map_or(Err(could_not_find_module(None, &self.module)), |f| Ok(f))?;

        let func_ctx = module_ctx.get(&self.ident).map_or(
            Err(could_not_find_function(None, &self.module, &self.ident)),
            |f| Ok(f),
        )?;

        match func_ctx {
            FunctionContext::Import(imp) => imp.inline(context, &self.module, history),
            FunctionContext::Func(func) => func.inline(context, module, history),
            FunctionContext::Inline(inline) => inline.inline(context, module, history),
        }
    }
}

impl Inline for FunctionInline {
    fn inline(
        &self,
        _: &mut CompileContext,
        _: &ModuleName,
        _: &mut HashSet<FunctionQualName>,
    ) -> StdResult<FunctionInline> {
        Ok(self.clone())
    }
}

impl Inline for FuncDecl {
    fn inline(
        &self,
        context: &mut CompileContext,
        module: &ModuleName,
        history: &mut HashSet<FunctionQualName>,
    ) -> StdResult<FunctionInline> {
        let terms = self.terms.lower(context)?;
        terms.prefix(context);

        todo!()
    }
}
