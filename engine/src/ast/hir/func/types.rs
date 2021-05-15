use crate::ast::expr::Expr;
use crate::ast::hir::func::decl::FuncDecl;

use crate::types::LineNo;
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FuncImport {
    pub module: ModuleName,
    pub ident: FuncName,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FuncInline {
    pub lno: LineNo,

    pub ident: String,
    // these are already the inline names
    pub params: Vec<String>,
    pub ret: String,

    pub terms: Expr,
}

sum_type! {
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum FuncContext {
        // This means it is an import
        Import(FuncImport),

        /// This means it is still a function,
        /// which needs to be inlined
        Func(FuncDecl),

        /// This means it is already inlined
        /// and can be used
        Inline(FuncInline),
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct ModuleName(pub Vec<String>);
NewtypeDeref! {() pub struct ModuleName(pub Vec<String>); }
NewtypeDerefMut! {() pub struct ModuleName(pub Vec<String>); }

impl ModuleName {
    pub fn main() -> Self {
        ModuleName(vec!["fs".to_string(), "main".to_string()])
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct FuncName(pub String);
NewtypeDeref! {() pub struct FuncName(pub String); }
NewtypeDerefMut! {() pub struct FuncName(pub String); }

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct FuncAlias(pub String);
NewtypeDeref! {() pub struct FuncAlias(pub String); }
NewtypeDerefMut! {() pub struct FuncAlias(pub String); }

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct FuncQualName(pub String);
NewtypeDeref! {() pub struct FuncQualName(pub String); }
NewtypeDerefMut! {() pub struct FuncQualName(pub String); }

impl From<Vec<String>> for ModuleName {
    fn from(val: Vec<String>) -> Self {
        Self(val)
    }
}
impl From<Vec<&str>> for ModuleName {
    fn from(val: Vec<&str>) -> Self {
        Self(val.iter().map(|v| v.to_string()).collect())
    }
}
impl From<String> for FuncName {
    fn from(val: String) -> Self {
        Self(val)
    }
}
impl From<&str> for FuncName {
    fn from(val: &str) -> Self {
        Self(val.into())
    }
}
impl From<String> for FuncQualName {
    fn from(val: String) -> Self {
        Self(val)
    }
}
impl From<&str> for FuncQualName {
    fn from(val: &str) -> Self {
        Self(val.into())
    }
}
impl From<(ModuleName, FuncName)> for FuncQualName {
    fn from((module, func): (ModuleName, FuncName)) -> Self {
        Self(
            module
                .0
                .into_iter()
                .chain(std::iter::once(func.0))
                .collect_vec()
                .join("::"),
        )
    }
}
