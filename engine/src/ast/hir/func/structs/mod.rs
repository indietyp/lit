use funcname::FuncName;
use modname::ModuleName;

use crate::ast::expr::Expr;
use crate::ast::hir::func::decl::FuncDecl;
use crate::types::LineNo;

pub mod funcname;
pub mod modname;
pub mod qualname;

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
pub struct FuncAlias(String);
NewtypeDeref! {() pub struct FuncAlias(String); }
NewtypeDerefMut! {() pub struct FuncAlias(String); }
