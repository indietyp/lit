use hir::Hir;
use variants::Errors;

// TODO: macro declaration
// TODO: undefined lazy eval

pub(crate) mod assign;
pub(crate) mod lp;
pub(crate) mod noop;
pub(crate) mod terms;
pub(crate) mod whl;

pub(crate) fn parse_file(input: &str) -> Result<Hir, Errors> {
    todo!()
}
