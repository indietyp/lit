// Control Structures have in their body potentially
// polluted information, these need to changed/unpolluted via
// macro expansion
#[derive(Debug, Clone)]
pub enum Control<TNode> {
    Terms(Vec<TNode>),
    Loop {
        ident: Box<TNode>,
        terms: Box<TNode>,
    },
    While {
        comp: Box<TNode>,
        terms: Box<TNode>,
    },
}
