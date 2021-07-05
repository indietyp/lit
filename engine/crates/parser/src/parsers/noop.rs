use crate::combinators::is::is_ellipsis;
use combine::{Parser, Stream};
use hir::Hir;
use lexer::Token;

// parse ...
// return HIR
pub(crate) fn noop<Input>() -> impl Parser<Input, Output = Hir>
where
    Input: Stream<Token = Token>,
    Input::Error: Sized,
{
    is_ellipsis().map(|_| Hir::NoOp)
}
