use crate::types::LineNo;

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
