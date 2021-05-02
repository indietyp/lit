use std::collections::HashMap;

use js_sys::Map;
use num_bigint::BigUint;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsValue, UnwrapThrowExt};

use crate::ast::polluted::PollutedNode;
use crate::build::Builder;
use crate::eval::exec::Exec;
use crate::flags::CompilationFlags;
use crate::runtime::Runtime;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[wasm_bindgen(js_name = Runtime)]
#[derive(Serialize, Deserialize)]
pub struct JavaScriptRuntime {
    runtime: Runtime,
}

#[wasm_bindgen(js_class = Runtime)]
impl JavaScriptRuntime {
    #[wasm_bindgen(constructor)]
    pub fn new(exec: &JsValue, locals: Map) -> Result<JavaScriptRuntime, JsValue> {
        let exec: Exec = exec.into_serde().unwrap_throw();
        let mut locs: HashMap<String, BigUint> = HashMap::new();

        let mut errors = vec![];
        locals.for_each(&mut |value, key| {
            if key.as_string().is_some() {
                if value.as_string().is_none() {
                    errors
                        .push("Value is not a string! (This is currently a limitation with WASM)");
                } else {
                    locs.insert(
                        key.as_string().unwrap(),
                        BigUint::from_str(&value.as_string().unwrap()).unwrap(),
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
            runtime: Runtime::new(exec, if locs.is_empty() { None } else { Some(locs) }),
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

    pub fn context(&self) -> JsValue {
        JsValue::from_serde(&self.runtime.context()).unwrap()
    }
}

#[wasm_bindgen(js_name = Builder)]
#[derive(Serialize, Deserialize)]
pub struct JavaScriptBuilder {
    builder: Builder,
}

#[wasm_bindgen(js_class = Builder)]
impl JavaScriptBuilder {
    pub fn parse(source: &str) -> Result<JsValue, JsValue> {
        let result = Builder::parse(source, None);

        return if result.is_ok() {
            Ok(JsValue::from_serde(&result.ok().unwrap()).unwrap())
        } else {
            Err(JsValue::from_str(&format!("{}", result.err().unwrap())))
        };
    }

    pub fn compile(ast: &JsValue, flags: Option<CompilationFlags>) -> Result<JsValue, JsValue> {
        let mut ast: Vec<PollutedNode> = ast.into_serde().unwrap();
        let result = Builder::compile(&mut ast, flags);

        Ok(JsValue::from_serde(&result).unwrap())
    }

    pub fn eval(ast: &JsValue, locals: Map) -> Result<JavaScriptRuntime, JsValue> {
        JavaScriptRuntime::new(ast, locals)
    }
}
