use crate::ast::node::Node;
use crate::ast::polluted::PollutedNode;
#[cfg(feature = "cli")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "cli", derive(JsonSchema))]
pub struct FuncImport {
    ident: Box<Node>,
    alias: Option<Box<Node>>,
}

// TODO: Func recursion detection on expand

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "cli", derive(JsonSchema))]
pub enum Func {
    // Import corresponds to
    // from path1::path2::path3 import (ident1 as alias1, ident2)
    Import {
        path: Vec<Node>,
        funcs: Vec<FuncImport>,
    },
    // Decl corresponds to
    // fn ident(arg1, arg2, arg3, ...) -> ret decl
    //  terms
    // end
    Decl {
        ident: Box<Node>,
        args: Vec<Node>,
        terms: Vec<PollutedNode>,
        ret: Box<Node>,
    },
    // Call corresponds to:
    // lhs := func(arg1, arg2, arg3, ...)
    Call {
        lhs: Box<Node>,
        func: Box<Node>,
        args: Vec<Node>,
    },
}
