pub use crate::call::BoundCall;
pub use crate::call::Call;

pub mod call;

#[derive(Debug, Clone)]
pub enum Func {
    BoundCall(BoundCall),
}
