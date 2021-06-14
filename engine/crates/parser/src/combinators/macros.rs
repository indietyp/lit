#[macro_export]
macro_rules! simple_combinator {
    ($prefix:tt, $name:tt, $( $pattern:pat )|+) => {
        ::paste::paste! {
            pub fn [<$prefix _ $name>]<Input>() -> impl ::combine::Parser<Input, Output = ::lexer::Token, PartialState = ()>
            where
                Input: ::combine::Stream<Token = ::lexer::Token>,
                Input::Error: ::combine::ParseError<Input::Token, Input::Range, Input::Position>,
            {
                let f: fn(::lexer::Token) -> bool = |token| ::std::matches!(token.kind, $($pattern)|*);
                ::combine::satisfy(f).expected(::std::stringify!($name))
            }
        }
    };
}
