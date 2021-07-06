use text_size::{TextRange, TextSize};

#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::ser::SerializeMap;
#[cfg(feature = "serde")]
use serde::{Serialize, Serializer};
use std::collections::HashMap;
use std::fmt;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct ColRange {
    start: TextSize,
    end: TextSize,
}

impl ColRange {
    pub fn new(start: TextSize, end: TextSize) -> Self {
        Self { start, end }
    }

    pub fn start(&self) -> TextSize {
        self.start
    }

    pub fn end(&self) -> TextSize {
        self.end
    }
}

impl fmt::Debug for ColRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}..{:#?}", self.start(), self.end())
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct LineNo {
    pub row: TextRange,
    // this is not a range, as the col can go over multiple lines
    pub col: ColRange,
}

impl LineNo {
    pub fn end_at(&self, lno: &LineNo) -> Self {
        Self {
            row: TextRange::new(self.row.start(), lno.row.end()),
            col: ColRange::new(self.col.start(), lno.col.end()),
        }
    }
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
    pub fn new(row: TextRange, col: ColRange) -> Self {
        Self { row, col }
    }
}
