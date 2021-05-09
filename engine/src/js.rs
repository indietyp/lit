use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};

use crate::ast::node::Node;
use crate::ast::polluted::PollutedNode;
use crate::build::Builder;
use crate::eval::exec::Exec;
use crate::eval::types::Variables;
use crate::flags::CompilationFlags;
use crate::runtime::Runtime;
use crate::utils::set_panic_hook;
use js_sys::Map;
use js_sys::Math::ceil;
use num_bigint::BigUint;
use num_traits::AsPrimitive;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
import {Node, PollutedNode, Exec} from "./schema";
"#;

#[wasm_bindgen(start)]
pub fn main() {
    set_panic_hook()
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Node")]
    pub type INode;

    #[wasm_bindgen(typescript_type = "PollutedNode")]
    pub type IPollutedNode;

    #[wasm_bindgen(typescript_type = "Exec")]
    pub type IExec;

    #[wasm_bindgen(typescript_type = "Map<string, number>")]
    pub type IVariables;
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
                let val = value.as_f64();
                if val.is_none() || val.unwrap() < 0. {
                    errors.push("Value is not a number or is smaller then 0.");
                } else {
                    variables.insert(
                        key.as_string().unwrap(),
                        BigUint::from(ceil(val.unwrap()) as u64),
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
    pub fn parse(source: &str, flags: Option<CompilationFlags>) -> Result<IPollutedNode, JsValue> {
        Builder::parse(source, None, flags)
            .map(|val| JsValue::from_serde(&val).unwrap().unchecked_into())
            .map_err(|err| JsValue::from_str(format!("{}", err).as_str()))
    }

    pub fn compile(ast: &IPollutedNode, flags: Option<CompilationFlags>) -> Result<INode, JsValue> {
        let mut ast: Vec<PollutedNode> = ast.into_serde().unwrap();
        let result =
            Builder::compile(&mut ast, flags).map_err(|err| JsValue::from_serde(&err).unwrap())?;

        Ok(JsValue::from_serde(&result).unwrap().unchecked_into())
    }

    pub fn exec(exec: IExec, locals: IVariables) -> Result<JavaScriptRuntime, JsValue> {
        JavaScriptRuntime::new(exec, locals)
    }

    pub fn eval(ast: &INode) -> Result<IExec, JsValue> {
        let ast: Node = ast
            .into_serde()
            .map_err(|err| JsValue::from_str(format!("{}", err).as_str()))?;

        let exec = Exec::new(ast);

        Ok(JsValue::from_serde(&exec).unwrap().unchecked_into())
    }

    pub fn display(ast: &INode, indent: u8) -> Result<String, JsValue> {
        let ast: Node = ast
            .into_serde()
            .map_err(|err| JsValue::from_str(format!("{}", err).as_str()))?;

        Ok(ast.display(indent, None))
    }
}
