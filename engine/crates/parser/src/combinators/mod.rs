#[macro_use]
pub(crate) mod macros;

pub(crate) mod comp;
pub(crate) mod directive;
pub(crate) mod is;
pub(crate) mod kw;
pub(crate) mod op;
pub(crate) mod trivia;

#[cfg(test)]
fn check_single_kind<
    T: ::combine::Parser<crate::stream::LexerStream, Output = ::lexer::Token, PartialState = ()>,
>(
    input: &str,
    combinator: fn() -> T,
) {
    let stream = crate::stream::LexerStream::new(input);

    let result = combinator().parse(stream);

    if let Err(error) = result {
        println!("{:?}", error);
        panic!("Encountered Error")
    }
}
