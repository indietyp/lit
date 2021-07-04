use hir::Hir;
use variants::Errors;

pub(crate) mod assign;
pub(crate) mod lp;
pub(crate) mod terms;
pub(crate) mod whl;

pub(crate) fn parse_file(input: &str) -> Result<Hir, Errors> {
    todo!()
}
