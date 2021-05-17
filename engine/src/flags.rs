use wasm_bindgen::prelude::*;

bitflags! {
    #[wasm_bindgen]
    pub struct CompileFlags: u16 {
        //-- Language Features --//
        const LOOP          = 0b0001;
        const WHILE         = 0b0010;

        //-- Configuration --//
        // instead of rewriting the LNO on compilation, let them stay pre expansion
        const CNF_RETAIN_LNO = 0b0001 << 4;
        // enable const variables (Assignment to CONST var is forbidden)
        const CNF_CONST      = 0b0010 << 4;

        //-- Optimization Features --//
        // enable dedicated zero variable (needs const conf enabled)
        const OPT_ZERO       = 0b0001 << 8 | Self::CNF_CONST.bits;

        //-- Strict Mode --//
        // Disable Macro Expansion
        const STRCT_NO_MACRO = 0b0001 << 12;
        // Disable Functions
        const STRCT_NO_FUNC  = 0b0010 << 12;
        // Disable Loop lowering to WHILE
        const STRCT_NO_LPLWR = 0b0100 << 12;
        // Enable complete strict mode
        const STRCT          = 0b1111 << 12;

        //-- Compound Enum --//
        const LOOP_AND_WHILE = Self::LOOP.bits | Self::WHILE.bits;
    }
}

#[wasm_bindgen]
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
