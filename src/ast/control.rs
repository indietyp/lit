use num_bigint::BigUint;

use crate::ast::macro_::Macro;
use crate::ast::node::Node;
use crate::ast::polluted::PollutedNode;
use crate::ast::verbs::{ComparisonVerb, OperatorVerb};
use crate::flags::CompilationFlags;
use crate::types::LineNo;
use crate::utils::private_random_identifier;

// Control Structures have in their body potentially
// polluted information, these need to changed/unpolluted via
// macro expansion
#[derive(Debug, Clone)]
pub enum Control<TNode> {
    Terms(Vec<TNode>),
    Loop {
        lno: LineNo,
        ident: Box<TNode>,
        terms: Box<TNode>,
    },
    While {
        lno: LineNo,
        comp: Box<TNode>,
        terms: Box<TNode>,
    },
}
