use wasm_bindgen::prelude::*;

bitflags! {
    #[wasm_bindgen]
    pub struct CompilationFlags: u16 {
        //-- Language Features --//
        const LOOP       = 0b0000_0001;
        const WHILE      = 0b0000_0010;

        //-- Optimization Features --//
        // enable dedicated zero variable
        const OPT_ZERO = 0b0001_0000_0000;

        //-- Configuration --//
        // instead of rewriting the LNO on compilation, let them stay pre expansion
        const CNF_RETAIN_LNO = 0b0001_0000;
        // Strict mode will no longer rewrite LOOP into WHILE, and macros are disabled.
        const CNF_STRICT_MODE = 0b0010_0000;

        //-- Compound Enum --//
        const LOOP_AND_WHILE = Self::LOOP.bits | Self::WHILE.bits;
    }
}

impl CompilationFlags {
    pub fn clear(&mut self) {
        self.bits = 0;
    }
}

impl Default for CompilationFlags {
    fn default() -> Self {
        CompilationFlags::LOOP | CompilationFlags::WHILE
    }
}
