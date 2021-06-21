pub use crate::primitive::Primitive;

mod binop;
mod comp;
mod primitive;

pub enum Expr {
    Primitive(Primitive),

    Comp(Comp),
    BinOp(BinOp),
    Assign(Assign),

    Control(Control<Expr>)
}
