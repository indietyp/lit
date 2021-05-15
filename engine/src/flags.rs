use wasm_bindgen::prelude::*;

bitflags! {
    #[wasm_bindgen]
    pub struct CompileFlags: u16 {
        //-- Language Features --//
        const LOOP       = 0b0000_0001;
        const WHILE      = 0b0000_0010;

        //-- Optimization Features --//
        // enable dedicated zero variable (needs const variable)
        const OPT_ZERO = 0b0001_0100_0000;

        //-- Configuration --//
        // instead of rewriting the LNO on compilation, let them stay pre expansion
        const CNF_RETAIN_LNO = 0b0001_0000;
        // Strict mode will no longer rewrite LOOP into WHILE, and macros are disabled.
        const CNF_STRICT_MODE = 0b0010_0000;
        // enable const variables
        // (assignment to CONST declared variables is forbidden - internal only)
        const CNF_CONST = 0b0100_0000;

        //-- Compound Enum --//
        const LOOP_AND_WHILE = Self::LOOP.bits | Self::WHILE.bits;
    }
}

impl CompileFlags {
    pub fn clear(&mut self) {
        self.bits = 0;
    }
}

impl Default for CompileFlags {
    fn default() -> Self {
        CompileFlags::LOOP | CompileFlags::WHILE
    }
}
