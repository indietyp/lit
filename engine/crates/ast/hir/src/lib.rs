// The HIR = Higher Level Representation has to go through 2 steps
// 1) Undefined pattern matching into Macros
// 2) lower macros
// 3) lower functions

use ctrl::Control;
use expr::{Comp, Expr, Primitive};
use fnc::Func;
use mcr::Unknown;

#[derive(Debug, Clone)]
pub enum Hir {
    Expr(Expr),
    Func(Func),

    // These need to be lowered, to account for not yet lowered things
    Control(Control<Hir, Primitive, Comp>),

    // To be determined via Macro-Matching
    Unknown(Unknown),

    // No Operation
    NoOp,
}
