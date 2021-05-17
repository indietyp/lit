use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};

use crate::ast::expr::Expr;
use crate::build::Builder;
use crate::eval::exec::Exec;
use crate::flags::CompileFlags;
use crate::runtime::Runtime;
use crate::utils::set_panic_hook;

use crate::ast::hir::func::fs::Directory;
use crate::ast::module::Module;
use crate::errors::Error;
use js_sys::Map;
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
import {Expr, Hir, Exec, Module, Path} from "./schema";

// sadly we need to do this one manually
type Directory = { [key: string]: Path };
"#;

#[wasm_bindgen(start)]
pub fn main() {
    set_panic_hook()
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Expr")]
    pub type IExpr;

    #[wasm_bindgen(typescript_type = "Hir")]
    pub type IHir;

    #[wasm_bindgen(typescript_type = "Module")]
    pub type IModule;

    #[wasm_bindgen(typescript_type = "Exec")]
    pub type IExec;

    #[wasm_bindgen(typescript_type = "Map<string, number>")]
    pub type IVariables;

    #[wasm_bindgen(typescript_type = "Error[]")]
    pub type IErrors;

    #[wasm_bindgen(typescript_type = "Directory")]
    pub type IDirectory;
}

#[wasm_bindgen(js_name = Runtime)]
#[derive(Serialize, Deserialize)]
pub struct JavaScriptRuntime {
    runtime: Runtime,
}

#[wasm_bindgen(js_class = Runtime)]
impl JavaScriptRuntime {
    #[wasm_bindgen(constructor)]
    pub fn new(exec: IExec, locals: IVariables) -> Result<JavaScriptRuntime, JsValue> {
        let exec: Exec = exec.into_serde().unwrap_throw();
        let locals: Map = locals.unchecked_into::<Map>();

        let mut variables: HashMap<String, BigUint> = HashMap::new();
        let mut errors = vec![];

        locals.for_each(&mut |value, key| {
            if key.as_string().is_some() {
                let val: Option<f64> = value.as_f64();
                if val.is_none() || val.unwrap() < 0. {
                    errors.push("Value is not a number or is smaller then 0.");
                } else {
                    variables.insert(
                        key.as_string().unwrap(),
                        BigUint::from(val.unwrap().ceil() as u64),
                    );
                }
            } else {
                errors.push("Key is not a valid type detected");
            }
        });

        if !errors.is_empty() {
            return Result::Err(JsValue::from_serde(&errors).unwrap());
        }

        Result::Ok(JavaScriptRuntime {
            runtime: Runtime::new(
                exec,
                if variables.is_empty() {
                    None
                } else {
                    Some(variables)
                },
            ),
        })
    }

    pub fn step(&mut self) -> JsValue {
        let value = self.runtime.step();

        value
            .map(|v| JsValue::from_serde(&v).unwrap())
            .unwrap_or(JsValue::UNDEFINED)
    }

    pub fn reset(&mut self) {
        self.runtime.reset()
    }

    pub fn is_running(&self) -> bool {
        self.runtime.is_running()
    }

    pub fn context(&self) -> IVariables {
        JsValue::from_serde(&self.runtime.context())
            .unwrap()
            .unchecked_into()
    }
}

#[wasm_bindgen(js_name = Builder)]
#[derive(Serialize, Deserialize)]
pub struct JavaScriptBuilder {
    builder: Builder,
}

#[wasm_bindgen(js_class = Builder)]
impl JavaScriptBuilder {
    pub fn parse(source: &str) -> Result<IModule, JsValue> {
        Builder::parse(source, None)
            .map(|val| JsValue::from_serde(&val).unwrap().unchecked_into())
            .map_err(Error::new_from_parse)
            .map_err(|err| vec![err])
            .map_err(|err| JsValue::from_serde(&err).unwrap())
    }

    pub fn compile(
        module: &IModule,
        flags: JsValue,
        fs: Option<IDirectory>,
    ) -> Result<IExpr, JsValue> {
        let mut module: Module = module.into_serde().unwrap();
        let fs: Option<Directory> = fs.map(|fs| fs.into_serde().unwrap());
        let flags = if flags.is_undefined() {
            None
        } else {
            flags
                .as_f64()
                .map(|flags| CompileFlags::from_bits(flags as u16))
        }
        .flatten();

        let result = Builder::compile(&mut module, flags, fs)
            .map_err(|err| JsValue::from_serde(&err).unwrap())?;

        Ok(JsValue::from_serde(&result).unwrap().unchecked_into())
    }

    pub fn exec(exec: IExec, locals: IVariables) -> Result<JavaScriptRuntime, JsValue> {
        JavaScriptRuntime::new(exec, locals)
    }

    pub fn eval(expr: &IExpr) -> Result<IExec, JsValue> {
        let expr: Expr = expr
            .into_serde()
            .map_err(|err| JsValue::from_str(format!("{}", err).as_str()))?;

        let exec = Exec::new(expr);

        Ok(JsValue::from_serde(&exec).unwrap().unchecked_into())
    }

    pub fn display(expr: &IExpr, indent: u8) -> Result<String, JsValue> {
        let expr: Expr = expr
            .into_serde()
            .map_err(|err| JsValue::from_str(format!("{}", err).as_str()))?;

        Ok(expr.display(indent, None))
    }
}
