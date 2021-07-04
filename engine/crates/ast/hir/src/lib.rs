// The HIR = Higher Level Representation has to go through 2 steps
// 1) Undefined pattern matching into Macros
// 2) lower macros
// 3) lower functions

use ctrl::Control;
use expr::Expr;

#[derive(Debug, Clone)]
pub enum Hir {
    Expr(Expr),
    Func(Func),
    Control(Control<Hir>),

    // To be determined via Macro-Matching
    Macro(Macro),
    Undefined(Undefined),

    // No Operation
    NoOp,
}
