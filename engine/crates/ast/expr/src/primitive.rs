use lexer::Kind;
use variants::UInt;

pub struct Value<Type> {
    value: Type,

    kind: Vec<Kind>
}

pub enum Primitive {
    Ident(Value<String>),
    Number(Value<UInt>),
}
