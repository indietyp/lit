mod decl;
mod imp;

use crate::imp::Imp;
pub use decl::Decl;

pub struct Module {
    pub imp: Vec<Imp>,

    pub decl: Decl,

    pub code: Hir,
}
