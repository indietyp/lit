use crate::combinators::kw::{kw_do, kw_end};
use crate::combinators::trivia::sep;
use crate::parsers::lp::lp;
use crate::parsers::noop::noop;

use crate::parsers::whl::whl;
use combine::parser::combinator::no_partial;

use crate::parsers::unknown::unknown;
use combine::{attempt, between, many, optional, satisfy, sep_by, sep_end_by, Parser, Stream};
use ctrl::Control;
use hir::Hir;
use lexer::Token;
use mcr::Unknown;
use variants::LineNo;

fn term<Input>(unkn: bool) -> impl Parser<Input, Output = Hir, PartialState = ()>
where
    Input: Stream<Token = Token>,
    Input::Error: Sized,
{
    let combinator = attempt(whl()) //
        .or(attempt(lp()) //
            .or(if unkn {
                attempt(noop())
                    .or(attempt(block().map(|block| block.terms)) //
                        .or(attempt(unknown())))
                    .left()
            } else {
                attempt(noop()).right()
            }));

    no_partial(combinator)
}

fn terms_<Input>(unknown: bool) -> impl Parser<Input, Output = Hir, PartialState = ()>
where
    Input: Stream<Token = Token>,
    Input::Error: Sized,
{
    let combinator = sep_end_by::<Vec<_>, _, _, _>(attempt(term(unknown)), sep()).map(|terms| {
        if terms.is_empty() {
            Hir::Control(Control::Block {
                terms: vec![Hir::NoOp],
            })
        } else {
            let mut block: Vec<Hir> = vec![];
            let mut backlog: Vec<Token> = vec![];

            fn push_backlog(block: &mut Vec<Hir>, backlog: &mut Vec<Token>) {
                if backlog.is_empty() {
                    return;
                }

                block.push(Hir::Unknown(Unknown::Tokens(backlog.clone())));
                backlog.clear();
            }

            for term in terms {
                match term {
                    Hir::Unknown(Unknown::Token(token)) => backlog.push(token.clone()),
                    _ => {
                        push_backlog(&mut block, &mut backlog);
                        block.push(term)
                    }
                }
            }

            push_backlog(&mut block, &mut backlog);

            Hir::Control(Control::Block { terms: block })
        }
    });

    no_partial(combinator)
}

parser! {
    pub(crate) fn terms[Input](unknown: bool)(Input) -> Hir
    where [Input: Stream<Token = Token>]
    {
        terms_(*unknown)
    }
}

pub(crate) struct Block {
    pub(crate) lno: LineNo,
    pub(crate) terms: Hir,
}

fn block_<Input>() -> impl Parser<Input, Output = Block, PartialState = ()>
where
    Input: Stream<Token = Token>,
    Input::Error: Sized,
{
    no_partial(
        ((kw_do(), sep()), terms(true), kw_end()).map(|((start, _), terms, end)| Block {
            lno: start.lno.end_at(&end.lno),
            terms,
        }),
    )
}

parser! {
    pub(crate) fn block[Input]()(Input) -> Block
    where [Input: Stream<Token = Token>]
    {
        block_()
    }
}
