bitflags! {
    #[derive(Debug, Clone)]
    pub struct CompilationFlags: u8 {
        // Language Features
        const LOOP       = 0b0000_0001;
        const WHILE      = 0b0000_0010;

        // Runtime Features
        const RETAIN_LNO = 0b1000_0001;

        // Compound Enum
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
