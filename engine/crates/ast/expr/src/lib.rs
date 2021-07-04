pub use crate::assign::Assign;
pub use crate::binop::BinOp;
pub use crate::comp::Comp;
pub use crate::primitive::Primitive;
use ctrl::Control;

pub mod assign;
pub mod binop;
pub mod comp;
pub mod primitive;

#[derive(Debug, Clone)]
pub enum Expr {
    Primitive(Primitive),

    Comp(Comp),
    BinOp(BinOp),
    Assign(Assign),

    Control(Control<Expr>),
}
