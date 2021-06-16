#[macro_use]
extern crate newtype_derive;

pub mod uint;
pub mod lno;

pub use uint::UInt;
pub use lno::LineNo;
