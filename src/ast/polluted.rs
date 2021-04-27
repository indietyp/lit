use crate::ast::control::Control;
use crate::ast::macro_::Macro;
use crate::ast::node::Node;

#[derive(Debug, Clone)]
pub enum PollutedNode {
    Pure(Node),
    Macro(Macro),
    NoOp,

    Control(Control<PollutedNode>),
}

impl PollutedNode {
    pub fn expand(&self) -> Node {
        match self {
            // Control Nodes
            PollutedNode::Control(Control::Terms(t)) => {
                Node::Control(Control::Terms(t.iter().map(|term| term.expand()).collect()))
            }
            PollutedNode::Control(Control::Loop { ident, terms }) => Node::Control(Control::Loop {
                ident: Box::new(ident.expand()),
                terms: Box::new(terms.expand()),
            }),
            PollutedNode::Control(Control::While { comp, terms }) => {
                Node::Control(Control::While {
                    comp: Box::new(comp.expand()),
                    terms: Box::new(terms.expand()),
                })
            }
            PollutedNode::NoOp => Node::Control(Control::Terms(vec![])),
            PollutedNode::Pure(n) => n.clone(),
            PollutedNode::Macro(m) => m.expand(),
        }
    }
}
