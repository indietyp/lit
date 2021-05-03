use crate::ast::control::Control;
use crate::ast::macros::{Macro, MacroAssign};
use crate::ast::node::Node;
use crate::ast::polluted::PollutedNode;
use crate::ast::verbs::{ComparisonVerb, OperatorVerb};
use crate::types::LineNo;
use num_bigint::BigUint;
use num_traits::Zero;
use pest_consume::Error;
use pest_consume::Parser;
use std::task::Poll;

#[derive(Clone)]
struct ParseSettings {
    lno: Option<LineNo>,
}

type ParseResult<T> = std::result::Result<T, Error<Rule>>;
type ParseNode<'i, 'a> = pest_consume::Node<'i, Rule, &'a ParseSettings>;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub(crate) struct LoopParser;

#[pest_consume::parser]
impl LoopParser {
    // Helper Functions
    fn lno(input: &ParseNode) -> LineNo {
        let settings = input.user_data();
        let span = input.as_span();

        settings
            .lno
            .unwrap_or((span.start_pos().0, span.end_pos().1))
    }

    // Terminal Values
    #[allow(non_snake_case, clippy::upper_case_acronyms)]
    fn EOI(_input: ParseNode) -> ParseResult<PollutedNode> {
        Ok(PollutedNode::NoOp)
    }

    // Atoms (smallest unit)
    #[alias(atom)]
    #[allow(non_snake_case, clippy::upper_case_acronyms)]
    fn IDENT(input: ParseNode) -> ParseResult<Node> {
        Ok(Node::Ident(input.as_str().to_string()))
    }

    #[alias(atom)]
    #[allow(non_snake_case, clippy::upper_case_acronyms)]
    fn VALUE(input: ParseNode) -> ParseResult<Node> {
        input
            .as_str()
            .parse::<BigUint>()
            .map(|u| Node::NaturalNumber(u))
            .map_err(|e| input.error(e))
    }

    #[alias(atom)]
    #[allow(non_snake_case, clippy::upper_case_acronyms)]
    fn ZERO(input: ParseNode) -> ParseResult<Node> {
        Ok(Node::NaturalNumber(BigUint::zero()))
    }

    // Comparisons
    fn parse_comp(input: ParseNode) -> ParseResult<Node> {
        let (lhs, verb, rhs) = match_nodes!(input.into_children();
            [atom(lhs), verb, atom(rhs)] => (lhs, verb, rhs)
        );

        Ok(Node::Comparison {
            lhs: Box::new(lhs),
            verb: ComparisonVerb::from(verb),
            rhs: Box::new(rhs),
        })
    }

    #[alias(comp)]
    #[allow(non_snake_case)]
    fn compEqual(input: ParseNode) -> ParseResult<Node> {
        Self::parse_comp(input)
    }

    #[alias(comp)]
    #[allow(non_snake_case)]
    fn compNotEqual(input: ParseNode) -> ParseResult<Node> {
        Self::parse_comp(input)
    }

    #[alias(comp)]
    #[allow(non_snake_case)]
    fn compNotEqual0(input: ParseNode) -> ParseResult<Node> {
        Self::parse_comp(input)
    }

    #[alias(comp)]
    #[allow(non_snake_case)]
    fn compGreaterThan(input: ParseNode) -> ParseResult<Node> {
        Self::parse_comp(input)
    }

    #[alias(comp)]
    #[allow(non_snake_case)]
    fn compGreaterThanIdent(input: ParseNode) -> ParseResult<Node> {
        Self::parse_comp(input)
    }

    #[alias(comp)]
    #[allow(non_snake_case)]
    fn compGreaterEqual(input: ParseNode) -> ParseResult<Node> {
        Self::parse_comp(input)
    }

    #[alias(comp)]
    #[allow(non_snake_case)]
    fn compLessThan(input: ParseNode) -> ParseResult<Node> {
        Self::parse_comp(input)
    }

    #[alias(comp)]
    #[allow(non_snake_case)]
    fn compLessEqual(input: ParseNode) -> ParseResult<Node> {
        Self::parse_comp(input)
    }

    // Operations
    #[alias(op)]
    #[allow(non_snake_case)]
    fn binaryOp(input: ParseNode) -> ParseResult<Node> {
        let (lhs, verb, rhs) = match_nodes!(input.into_children();
            [atom(lhs), verb, atom(rhs)] => (lhs, verb, rhs),
        );

        Ok(Node::BinaryOp {
            lhs: Box::new(lhs),
            verb: OperatorVerb::from(verb),
            rhs: Box::new(rhs),
        })
    }

    // Assignment
    #[alias(expr)]
    fn assign(input: ParseNode) -> ParseResult<Node> {
        let (ident, op) = match_nodes!(input.into_children();
            [IDENT(i), op(o)] => (i, o)
        );

        let lno = Self::lno(&input);

        Ok(Node::Assign {
            lno,
            lhs: Box::new(ident),
            rhs: Box::new(op),
        })
    }

    // Control Structures
    #[alias(expr)]
    fn terms(input: ParseNode) -> ParseResult<PollutedNode> {
        let terms = match_nodes!(input.into_children();
            [expr(expr)..] => expr
        );

        Ok(PollutedNode::Control(Control::Terms(terms.collect())))
    }

    #[alias(expr)]
    fn loop_(input: ParseNode) -> ParseResult<PollutedNode> {
        let (ident, terms) = match_nodes!(input.into_children();
            [atom(a), terms(t)] => (a, t)
        );

        let lno = Self::lno(&input);

        Ok(PollutedNode::Control(Control::Loop {
            lno,
            ident: Box::new(ident),
            terms: Box::new(terms),
        }))
    }

    #[alias(expr)]
    fn while_(input: ParseNode) -> ParseResult<PollutedNode> {
        let (comp, terms) = match_nodes!(input.into_children();
            [comp(c), terms(t)] => (c, t)
        );

        let lno = Self::lno(&input);

        Ok(PollutedNode::Control(Control::While {
            lno,
            comp: Box::new(comp),
            terms: Box::new(terms),
        }))
    }

    // Macro collection (aliased as expr)

    #[alias(expr)]
    #[allow(non_snake_case)]
    fn macroAssignToIdent(input: ParseNode) -> ParseResult<PollutedNode> {
        // x := y
        let (lhs, rhs) = match_nodes!(input.into_children();
            [IDENT(i), IDENT(j)] => (i, j)
        );
        let lno = Self::lno(&input);

        Ok(PollutedNode::Macro(Macro::AssignToIdent {
            lno,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }))
    }

    #[alias(expr)]
    #[allow(non_snake_case)]
    fn macroAssignToZero(input: ParseNode) -> ParseResult<PollutedNode> {
        // x := 0
        let lhs = match_nodes!(input.into_children();
            [IDENT(x)] => x
        );
        let lno = Self::lno(&input);

        Ok(PollutedNode::Macro(Macro::AssignToZero {
            lno,
            lhs: Box::new(lhs),
        }))
    }

    #[alias(expr)]
    #[allow(non_snake_case)]
    fn macroAssignToValue(input: ParseNode) -> ParseResult<PollutedNode> {
        // x := n
        let (lhs, rhs) = match_nodes!(input.into_children();
            [IDENT(x), VALUE(n)] => (x, n)
        );
        let lno = Self::lno(&input);

        Ok(PollutedNode::Macro(Macro::AssignToValue {
            lno,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }))
    }

    #[alias(expr)]
    #[allow(non_snake_case)]
    fn macroAssignToIdentOpIdent(input: ParseNode) -> ParseResult<PollutedNode> {
        // x := y * z
        let (lhs, rhs_lhs, rhs_verb, rhs_rhs) = match_nodes!(input.into_children();
            [IDENT(x), IDENT(y), verb, IDENT(z)] => (x, y, verb, z)
        );
        let lno = Self::lno(&input);

        Ok(PollutedNode::Macro(Macro::AssignToOpIdent {
            lno,
            lhs: Box::new(lhs),
            rhs: MacroAssign {
                lhs: Box::new(rhs_lhs),
                verb: OperatorVerb::from(rhs_verb),
                rhs: Box::new(rhs_rhs),
            },
        }))
    }

    #[alias(expr)]
    #[allow(non_snake_case)]
    fn macroAssignToIdentExtOpValue(input: ParseNode) -> ParseResult<PollutedNode> {
        // x := y * n
        let (lhs, rhs_lhs, rhs_verb, rhs_rhs) = match_nodes!(input.into_children();
            [IDENT(x), IDENT(y), verb, VALUE(n)] => (x, y, verb, n)
        );
        let lno = Self::lno(&input);

        Ok(PollutedNode::Macro(Macro::AssignToOpExtValue {
            lno,
            lhs: Box::new(lhs),
            rhs: MacroAssign {
                lhs: Box::new(rhs_lhs),
                verb: OperatorVerb::from(rhs_verb),
                rhs: Box::new(rhs_rhs),
            },
        }))
    }

    // Conditionals
    #[allow(non_snake_case)]
    fn macroElseStmt(input: ParseNode) -> ParseResult<PollutedNode> {
        Ok(match_nodes!(input.into_children();
            [terms(t)] => t
        ))
    }

    #[alias(expr)]
    #[allow(non_snake_case)]
    fn macroConditional(input: ParseNode) -> ParseResult<PollutedNode> {
        // IF ... THEN ... ELSE
        let (comp, if_terms, else_terms) = match_nodes!(input.into_children();
            [comp(c), terms(i)] => (c, i, None),
            [comp(c), terms(i), macroElseStmt(e)] => (c, i, Some(e))
        );
    }
}
