pub enum Primitive {
    Ident { value: String, kind: Vec<Kind> },
    Number { value: UInt, kind: Vec<Kind> },
}
