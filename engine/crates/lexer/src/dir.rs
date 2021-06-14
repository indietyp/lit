#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Directive {
    MacroStart,
    SubStart,
    End,

    If,
    Else,

    Ident(Placeholder),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Placeholder {
    Ident(u32),
    Value(u32),
    Atom(u32),

    Any(u32),
    Expr(u32),
    Terms(u32),

    Comp(u32),
    Op(u32),

    TempIdent(u32),
}
