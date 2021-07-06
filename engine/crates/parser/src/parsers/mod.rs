use hir::Hir;
use variants::Errors;

// TODO: macro declaration
// TODO: undefined lazy eval

pub(crate) mod assign;
pub(crate) mod fnc;
pub(crate) mod lp;
pub(crate) mod mcr;
pub(crate) mod noop;
pub(crate) mod terms;
pub(crate) mod unknown;
pub(crate) mod whl;

// This should parse in the following order:
// 1) import
// 2) macro
// 3) func decl
// 4) code
pub(crate) fn parse_file(input: &str) -> Result<Hir, Errors> {
    todo!()
}
