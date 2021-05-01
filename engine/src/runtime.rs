use crate::eval::exec::Exec;
use crate::eval::types::{ExecutionResult, Variables};
use bitflags::_core::str::FromStr;
use js_sys::Map;
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Runtime {
    exec: Exec,
    initial: Option<Variables>,
    locals: Variables,
    running: bool,
}

impl Runtime {
    pub fn new(exec: Exec, locals: Option<Variables>) -> Self {
        Runtime {
            exec,
            initial: locals.clone(),
            locals: locals.unwrap_or(HashMap::new()),
            running: true,
        }
    }

    pub fn step(&mut self) -> Option<ExecutionResult> {
        let result = self.exec.step(&mut self.locals);

        if result.is_none() {
            self.running = false
        }

        result
    }

    pub fn reset(&mut self) {
        self.locals = self.initial.clone().unwrap_or(HashMap::new());

        self.exec = self.exec.renew();
        self.running = true;
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn context(&self) -> Variables {
        self.locals.clone()
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct JavaScriptRuntime {
    runtime: Runtime,
}

#[wasm_bindgen]
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
        self.runtime.running
    }

    pub fn context(&self) -> JsValue {
        JsValue::from_serde(&self.runtime.context()).unwrap()
    }
}
