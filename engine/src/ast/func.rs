use crate::ast::node::Node;
use crate::ast::polluted::PollutedNode;
use crate::types::LineNo;
#[cfg(feature = "cli")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
pub struct FuncImportStmt {
    pub ident: Box<Node>,
    pub alias: Option<Box<Node>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "cli", derive(JsonSchema))]
pub struct FuncImport {
    pub lno: LineNo,

    pub path: Vec<Node>,
    pub funcs: Vec<FuncImportStmt>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "cli", derive(JsonSchema))]
pub struct FuncCall {
    pub ident: Box<Node>,
    pub args: Vec<Node>,
}

// TODO: Func recursion detection on expand
// TODO: on inline check if argument count is correct
// TODO: outline, import and inline and name collision detection.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "cli", derive(JsonSchema))]
pub enum Func {
    // Import corresponds to
    // The import always comes in pairs, meaning that it is always a vec
    // from path1::path2::path3 import (ident1 as alias1, ident2)
    Import(Vec<FuncImport>),
    // Decl corresponds to
    // Declaration always comes in pairs in as a section, meaning that it is always a vec.
    // fn ident(arg1, arg2, arg3, ...) -> ret decl
    //  terms
    // end
    Decl(Vec<FuncDecl>),
    // Call corresponds to:
    // lhs := func(arg1, arg2, arg3, ...)
    Call {
        lno: LineNo,

        lhs: Box<Node>,
        rhs: FuncCall,
    },
}
