use text_size::TextRange;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct LineNo {
    pub row: TextRange,
    pub col: TextRange,
}

impl LineNo {
    pub fn new(row: TextRange, col: TextRange) -> Self {
        Self { row, col }
    }
}
