use crate::flags::CompilationFlags;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub struct CompileContext {
    counter: usize,
    pub flags: CompilationFlags,
}

impl CompileContext {
    pub fn new(flags: CompilationFlags) -> Self {
        CompileContext { counter: 0, flags }
    }

    pub fn incr(&mut self) -> usize {
        let cur = self.counter;
        self.counter += 1;

        cur
    }
}
