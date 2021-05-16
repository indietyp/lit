use crate::ast::context::CompileContext;
use crate::ast::control::Control;
use crate::ast::expr::Expr;
use crate::ast::hir::func::inline::Inline;
use crate::ast::hir::func::module::ctx::ModuleContext;
use crate::ast::hir::func::structs::funcname::FuncName;
use crate::ast::hir::func::structs::modname::ModuleName;
use crate::ast::hir::func::structs::FuncContext;
use crate::ast::hir::func::{utils, FuncCall};
use crate::build::Builder;
use crate::errors::{Error, ErrorCode, StdResult};
use crate::types::LineNo;
use itertools::Itertools;

pub fn lower_call(
    context: &mut CompileContext,
    lno: LineNo,
    lhs: Expr,
    rhs: FuncCall,
) -> StdResult<Expr> {
    let module = context.get_current_frame().clone().module;

    let (func_name, func_ctx) = {
        let module_ctx = context
            .modules
            .get(&module)
            .map_or(
                Err(utils::could_not_find_module(Some(lno), &module)),
                |ctx| Ok(ctx),
            )?
            .clone();

        let func_name: FuncName = rhs.get_ident()?.into();

        let func_ctx = module_ctx
            .get(&func_name)
            .map_or(
                Err(utils::could_not_find_function(
                    Some(lno),
                    &module,
                    &func_name,
                )),
                |f| Ok(f),
            )?
            .clone();

        (func_name, func_ctx)
    };

    let inline = func_ctx.inline(context, &module)?;

    {
        let module_ctx = context.modules.get_mut(&module).map_or(
            Err(utils::could_not_find_module(Some(lno), &module)),
            |ctx| Ok(ctx),
        )?;

        // cache the inline result for further use
        module_ctx.insert(func_name.clone(), FuncContext::Inline(inline.clone()));
    }

    // check param length
    if rhs.args.len() != inline.params.len() {
        return Err(vec![Error::new_from_code(
            Some(lno),
            ErrorCode::FunctionUnexpectedNumberOfArguments {
                module: module.to_string(),
                func: func_name.to_string(),
                expected: inline.params.len(),
                got: rhs.args.len(),
            },
        )]);
    }

    let param_to_arg: Vec<_> = inline.params.into_iter().zip_eq(rhs.args).collect();
    let mut expr = vec![];
    let mut errors = vec![];

    // map the parameters
    for (param, arg) in param_to_arg {
        let stmt = format!(
            "{} := {}",
            param,
            match arg {
                Expr::Ident(m) => m,
                Expr::NaturalNumber(n) => n.0.to_string(),
                _ => unreachable!(),
            }
        );

        let compiled = Builder::ext_parse_and_compile(stmt.as_str(), context, Some(lno));
        if let Err(err) = compiled {
            errors.extend(err);
            continue;
        }
        expr.push(compiled.unwrap());
    }

    // push the actual function definition
    expr.push(inline.terms.clone());

    // process the assignment to a value
    let stmt = format!(
        "{} := {}",
        match lhs {
            Expr::Ident(m) => m,
            _ => unreachable!(),
        },
        inline.ret
    );
    let compiled = Builder::ext_parse_and_compile(stmt.as_str(), context, Some(lno));
    if let Err(err) = compiled {
        errors.extend(err);
    } else {
        expr.push(compiled.unwrap());
    }

    // now either error out or wrap into terms
    if !errors.is_empty() {
        Err(errors)
    } else {
        Ok(Expr::Control(Control::Terms(expr)))
    }
}
