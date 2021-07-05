use crate::call::BoundCall;
pub use crate::call::Call;

mod call;

#[derive(Debug, Clone)]
pub enum Func {
    BoundCall(BoundCall),
}
