use std::fmt;
use std::fmt::Formatter;

bitflags! {
    pub struct MacroModifier: u16 {
        const CaseInsensitive = 0b0001;
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Directive {
    Macro(MacroModifier),
    // starts the substition
    Sub,
    End,

    If,
    Else,

    Placeholder(Placeholder),
}

impl fmt::Display for Directive {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Macro(modifier) => format!("‘@macro/[{:?}]‘", modifier),
                Self::Sub => "‘@sub‘".into(),
                Self::End => "‘@end‘".into(),
                Self::If => "‘@if‘".into(),
                Self::Else => "‘@else‘".into(),
                Self::Placeholder(placeholder) => format!("{}", placeholder),
            }
        )
    }
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

impl fmt::Display for Placeholder {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(
            match self {
                Self::Ident(n) => format!("%i.{}", n),
                Self::Value(n) => format!("%v.{}", n),
                Self::Atom(n) => format!("%a.{}", n),
                Self::Any(n) => format!("%_.{}", n),
                Self::Expr(n) => format!("%e.{}", n),
                Self::Terms(n) => format!("%t.{}", n),
                Self::Comp(n) => format!("%c.{}", n),
                Self::Op(n) => format!("%o.{}", n),
                Self::TempIdent(n) => format!("$t.{}", n),
            }
            .as_str(),
        )
    }
}
