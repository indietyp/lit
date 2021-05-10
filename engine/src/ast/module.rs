use crate::ast::node::Node;
use crate::ast::polluted::PollutedNode;
use crate::types::LineNo;
#[cfg(feature = "cli")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub(crate) mod filesystem {
    use either::Either;
    use std::collections::HashMap;

    type FileContents = String;
    type Directory = HashMap<String, Box<Path>>;
    type Path = Either<FileContents, Box<Directory>>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "cli", derive(JsonSchema))]
pub struct FuncDecl {
    pub lno: LineNo,

    pub ident: Box<Node>,
    pub params: Vec<Node>,
    pub ret: Box<Node>,

    pub terms: Box<PollutedNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "cli", derive(JsonSchema))]
pub struct ImpFunc {
    pub ident: Box<Node>,
    pub alias: Option<Box<Node>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "cli", derive(JsonSchema))]
pub struct Imp {
    pub lno: LineNo,

    pub path: Vec<Node>,
    pub funcs: Vec<ImpFunc>,
}

// Used as a container for all Module related codes, this contains:
// - imports (imp)
// - declarations (decl)
// - code (code)
// Code is replaced with NoOp if it is loaded via an import
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub code: PollutedNode,
}
