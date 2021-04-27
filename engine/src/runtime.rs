use crate::eval::exec::Exec;
use crate::eval::types::{ExecutionResult, Variables};
use std::collections::HashMap;

pub struct Runtime {
    exec: Exec,
    locals: Variables,
    running: bool,
}

impl Runtime {
    pub fn new(exec: Exec) -> Self {
        Runtime {
            exec,
            locals: HashMap::new(),
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
        self.locals.clear();
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
