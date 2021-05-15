use crate::ast::expr::Expr;
use crate::ast::module::FuncDecl;
use crate::types::LineNo;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionImport {
    pub module: ModuleName,
    pub ident: FunctionName,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionInline {
    pub lno: LineNo,

    pub ident: String,
    // these are already the inline names
    pub params: Vec<String>,
    pub ret: String,

    pub terms: Expr,
}

sum_type! {
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum FunctionContext {
        // This means it is an import
        Import(FunctionImport),

        /// This means it is still a function,
        /// which needs to be inlined
        Func(FuncDecl),

        /// This means it is already inlined
        /// and can be used
        Inline(FunctionInline),
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct ModuleName(pub Vec<String>);
NewtypeDeref! {() pub struct ModuleName(pub Vec<String>); }
NewtypeDerefMut! {() pub struct ModuleName(pub Vec<String>); }

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct FunctionName(pub String);
NewtypeDeref! {() pub struct FunctionName(pub String); }
NewtypeDerefMut! {() pub struct FunctionName(pub String); }

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct FunctionAlias(pub String);
NewtypeDeref! {() pub struct FunctionAlias(pub String); }
NewtypeDerefMut! {() pub struct FunctionAlias(pub String); }

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct FunctionQualName(pub String);
NewtypeDeref! {() pub struct FunctionQualName(pub String); }
NewtypeDerefMut! {() pub struct FunctionQualName(pub String); }

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
impl From<String> for FunctionName {
    fn from(val: String) -> Self {
        Self(val)
    }
}
impl From<&str> for FunctionName {
    fn from(val: &str) -> Self {
        Self(val.into())
    }
}
