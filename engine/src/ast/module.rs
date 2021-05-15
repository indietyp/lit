#[cfg(feature = "cli")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::ast::hir::func::decl::FuncDecl;
use crate::ast::hir::func::imp::Imp;
use crate::ast::hir::Hir;

// Used as a container for all Module related codes, this contains:
// - imports (imp)
// - declarations (decl)
// - code (code)
// Code is replaced with NoOp if it is loaded via an import
#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "cli", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Module {
    // Import corresponds to
    // from path1::path2::path3 import (ident1 as alias1, ident2)
    // on top of the file.
    pub imp: Vec<Imp>,
    // Decl corresponds to
    // fn ident(arg1, arg2, arg3, ...) -> ret decl
    //  terms
    // end
    // always after imports
    pub decl: Vec<FuncDecl>,

    // actual code that is run on execution
    pub code: Hir,
}
