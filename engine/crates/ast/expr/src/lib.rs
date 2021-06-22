pub use crate::assign::Assign;
pub use crate::binop::BinOp;
pub use crate::comp::Comp;
pub use crate::primitive::Primitive;
use ctrl::Control;

mod assign;
mod binop;
mod comp;
mod primitive;

pub enum Expr {
    Primitive(Primitive),

    Comp(Comp),
    BinOp(BinOp),
    Assign(Assign),

    Control(Control<Expr>),
}
