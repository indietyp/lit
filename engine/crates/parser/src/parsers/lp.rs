use crate::combinators::comp::comp_ne;
use crate::combinators::is::is_ident;
use crate::combinators::kw::{kw_do_, kw_end, kw_loop};
use crate::combinators::trivia::sep;
use crate::parsers::terms::terms;
use crate::utils::to_ident;
use combine::{Parser, Stream};
use ctrl::Control;
use expr::Expr;
use hir::Hir;
use lexer::Token;

// parse LOOP <ident> DO <terms> END
// return HIR
pub(crate) fn lp<Input>() -> impl Parser<Input, Output = Hir>
where
    Input: Stream<Token = Token>,
    Input::Error: Sized,
{
    (
        kw_loop(),
        is_ident(),
        kw_do_(),
        sep(),
        terms(),
        kw_end(),
        sep(),
    )
        .map(|(start, ident, _, _, terms, end, _)| {
            Hir::Control(Control::Loop {
                lno: start.lno.end_at(&end.lno),
                ident: Box::new(Hir::Expr(Expr::Primitive(to_ident(ident)?))),
                terms: Box::new(terms),
            })
        })
}
