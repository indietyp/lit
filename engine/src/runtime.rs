use serde::{Deserialize, Serialize};

use crate::eval::exec::Exec;
use crate::eval::types::{ExecutionResult, Variables};

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
            locals: locals.unwrap_or_default(),
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
        self.locals = self.initial.clone().unwrap_or_default();

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
