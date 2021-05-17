use crate::ast::expr::Expr;
use crate::eval::op::BinaryOpExec;
use crate::eval::types::{ChangeLog, ExecutionResult, Variables};
use crate::types::LineNo;
#[cfg(feature = "cli")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "cli", derive(JsonSchema))]
pub struct AssignExec {
    lhs: String,
    rhs: BinaryOpExec,

    lno: LineNo,
    exhausted: bool,
}

impl AssignExec {
    pub fn step(&mut self, locals: &mut Variables) -> Option<ExecutionResult> {
        if self.exhausted {
            return None;
        }

        let value = self.rhs.exec(locals);
        locals.insert(self.lhs.clone(), value);
        self.exhausted = true;

        Some(ExecutionResult(
            self.lno.0,
            vec![ChangeLog::Ident(self.lhs.clone())],
        ))
    }

    pub fn new(node: Expr) -> Self {
        match node {
            Expr::Assign { lhs, rhs, lno } => AssignExec {
                lhs: match *lhs {
                    Expr::Ident(m) => m,
                    _ => unreachable!(),
                },
                rhs: BinaryOpExec::new(*rhs),
                lno,
                exhausted: false,
            },
            _ => unreachable!(),
        }
    }

    pub fn renew(&self) -> Self {
        AssignExec {
            lhs: self.lhs.clone(),
            rhs: self.rhs.renew(),
            lno: self.lno,
            exhausted: false,
        }
    }
}
