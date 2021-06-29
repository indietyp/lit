use text_size::TextRange;

#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::ser::SerializeMap;
#[cfg(feature = "serde")]
use serde::{Serialize, Serializer};
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct LineNo {
    pub row: TextRange,
    pub col: TextRange,
}

#[cfg(feature = "serde")]
impl Serialize for LineNo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;

        let row = {
            let row_start: u32 = self.row.start().into();
            let row_end: u32 = self.row.end().into();

            let mut map = HashMap::new();
            map.insert("start", row_start);
            map.insert("end", row_end);

            map
        };

        let col = {
            let col_start: u32 = self.col.start().into();
            let col_end: u32 = self.col.end().into();

            let mut map = HashMap::new();
            map.insert("start", col_start);
            map.insert("end", col_end);

            map
        };

        map.serialize_entry("row", &row)?;
        map.serialize_entry("col", &col)?;

        map.end()
    }
}

#[cfg(feature = "schema")]
impl JsonSchema for LineNo {}

impl LineNo {
    pub fn new(row: TextRange, col: TextRange) -> Self {
        Self { row, col }
    }
}
