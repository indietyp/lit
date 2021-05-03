use crate::ast::control::Control;
use crate::ast::macros::{Macro, MacroAssign};
use crate::ast::node::Node;
use crate::ast::polluted::PollutedNode;
use crate::ast::verbs::{ComparisonVerb, OperatorVerb};
use crate::types::LineNo;
use either::Either;
use num_bigint::BigUint;
use num_traits::Zero;
use pest_consume::Error;
use pest_consume::Parser;

#[derive(new, Clone)]
pub struct ParseSettings {
    lno: Option<LineNo>,
}

type ParseResult<T> = std::result::Result<T, Error<Rule>>;
type ParseNode<'i, 'a> = pest_consume::Node<'i, Rule, &'a ParseSettings>;

type EitherNode = Either<PollutedNode, Node>;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub(crate) struct LoopParser;
struct LoopParserHelpers {}

impl LoopParserHelpers {
    // Helper Functions
    fn lno(input: &ParseNode) -> LineNo {
        let settings = input.user_data();
        let span = input.as_span();

        settings
            .lno
            .unwrap_or((span.start_pos().line_col().0, span.end_pos().line_col().0))
    }

    fn parse_comp(input: ParseNode) -> ParseResult<EitherNode> {
        let (lhs, verb, rhs) = match_nodes!(<LoopParser>; input.into_children();
            [atom(lhs), verb, atom(rhs)] => (lhs, verb, rhs)
        );

        Ok(Either::Right(Node::Comparison {
            lhs: Box::new(lhs.right().unwrap()),
            verb: ComparisonVerb::from(verb.as_str()),
            rhs: Box::new(rhs.right().unwrap()),
        }))
    }
}

#[pest_consume::parser]
impl LoopParser {
    // Terminal Values
    #[allow(non_snake_case, clippy::upper_case_acronyms)]
    fn EOI(_input: ParseNode) -> ParseResult<EitherNode> {
        Ok(Either::Left(PollutedNode::NoOp))
    }

    // Atoms (smallest unit)
    #[alias(atom)]
    #[allow(non_snake_case, clippy::upper_case_acronyms)]
    fn IDENT(input: ParseNode) -> ParseResult<EitherNode> {
        Ok(Either::Right(Node::Ident(input.as_str().to_string())))
    }

    #[alias(atom)]
    #[allow(non_snake_case, clippy::upper_case_acronyms)]
    fn VALUE(input: ParseNode) -> ParseResult<EitherNode> {
        input
            .as_str()
            .parse::<BigUint>()
            .map(|u| EitherNode::Right(Node::NaturalNumber(u)))
            .map_err(|e| input.error(e))
    }

    #[alias(atom)]
    #[allow(non_snake_case, clippy::upper_case_acronyms)]
    fn ZERO(_input: ParseNode) -> ParseResult<EitherNode> {
        Ok(EitherNode::Right(Node::NaturalNumber(BigUint::zero())))
    }

    // Comparisons
    #[alias(comp)]
    #[allow(non_snake_case)]
    fn compEqual(input: ParseNode) -> ParseResult<EitherNode> {
        LoopParserHelpers::parse_comp(input)
    }

    #[alias(comp)]
    #[allow(non_snake_case)]
    fn compNotEqual(input: ParseNode) -> ParseResult<EitherNode> {
        LoopParserHelpers::parse_comp(input)
    }

    #[alias(comp)]
    #[allow(non_snake_case)]
    fn compNotEqual0(input: ParseNode) -> ParseResult<EitherNode> {
        LoopParserHelpers::parse_comp(input)
    }

    #[alias(comp)]
    #[allow(non_snake_case)]
    fn compGreaterThan(input: ParseNode) -> ParseResult<EitherNode> {
        LoopParserHelpers::parse_comp(input)
    }

    #[alias(comp)]
    #[allow(non_snake_case)]
    fn compGreaterThanIdent(input: ParseNode) -> ParseResult<EitherNode> {
        LoopParserHelpers::parse_comp(input)
    }

    #[alias(comp)]
    #[allow(non_snake_case)]
    fn compGreaterEqual(input: ParseNode) -> ParseResult<EitherNode> {
        LoopParserHelpers::parse_comp(input)
    }

    #[alias(comp)]
    #[allow(non_snake_case)]
    fn compLessThan(input: ParseNode) -> ParseResult<EitherNode> {
        LoopParserHelpers::parse_comp(input)
    }

    #[alias(comp)]
    #[allow(non_snake_case)]
    fn compLessEqual(input: ParseNode) -> ParseResult<EitherNode> {
        LoopParserHelpers::parse_comp(input)
    }

    // Operations
    #[alias(op)]
    #[allow(non_snake_case)]
    fn binaryOp(input: ParseNode) -> ParseResult<EitherNode> {
        let (lhs, verb, rhs) = match_nodes!(input.into_children();
            [atom(lhs), verb, atom(rhs)] => (lhs, verb, rhs),
        );

        Ok(EitherNode::Right(Node::BinaryOp {
            lhs: Box::new(lhs.right().unwrap()),
            verb: OperatorVerb::from(verb.as_str()),
            rhs: Box::new(rhs.right().unwrap()),
        }))
    }

    // Assignment
    #[alias(expr)]
    fn assign(input: ParseNode) -> ParseResult<EitherNode> {
        let (ident, op) = match_nodes!(input.into_children();
            [IDENT(i), op(o)] => (i, o)
        );

        let lno = LoopParserHelpers::lno(&input);

        Ok(EitherNode::Right(Node::Assign {
            lno,
            lhs: Box::new(ident.right().unwrap()),
            rhs: Box::new(op.right().unwrap()),
        }))
    }

    // Control Structures
    #[alias(expr)]
    fn terms(input: ParseNode) -> ParseResult<EitherNode> {
        let terms = match_nodes!(input.into_children();
            [expr(expr)..] => expr
        );

        Ok(EitherNode::Left(PollutedNode::Control(Control::Terms(
            terms.map(|term| term.left().unwrap()).collect(),
        ))))
    }

    #[alias(expr)]
    fn loop_(input: ParseNode) -> ParseResult<EitherNode> {
        let (ident, terms) = match_nodes!(input.into_children();
            [atom(a), terms(t)] => (a, t)
        );

        let lno = LoopParserHelpers::lno(&input);

        Ok(EitherNode::Left(PollutedNode::Control(Control::Loop {
            lno,
            ident: Box::new(PollutedNode::Pure(ident.right().unwrap())),
            terms: Box::new(terms.left().unwrap()),
        })))
    }

    #[alias(expr)]
    fn while_(input: ParseNode) -> ParseResult<EitherNode> {
        let (comp, terms) = match_nodes!(input.into_children();
            [comp(c), terms(t)] => (c, t)
        );

        let lno = LoopParserHelpers::lno(&input);

        Ok(EitherNode::Left(PollutedNode::Control(Control::While {
            lno,
            comp: Box::new(PollutedNode::Pure(comp.right().unwrap())),
            terms: Box::new(terms.left().unwrap()),
        })))
    }

    // Macro collection (aliased as expr)

    #[alias(expr)]
    #[allow(non_snake_case)]
    fn macroAssignToIdent(input: ParseNode) -> ParseResult<EitherNode> {
        // x := y
        let (lhs, rhs) = match_nodes!(input.into_children();
            [IDENT(i), IDENT(j)] => (i, j)
        );
        let lno = LoopParserHelpers::lno(&input);

        Ok(EitherNode::Left(PollutedNode::Macro(
            Macro::AssignToIdent {
                lno,
                lhs: Box::new(lhs.right().unwrap()),
                rhs: Box::new(rhs.right().unwrap()),
            },
        )))
    }

    #[alias(expr)]
    #[allow(non_snake_case)]
    fn macroAssignToZero(input: ParseNode) -> ParseResult<EitherNode> {
        // x := 0
        let lhs = match_nodes!(input.into_children();
            [IDENT(x)] => x
        );
        let lno = LoopParserHelpers::lno(&input);

        Ok(EitherNode::Left(PollutedNode::Macro(Macro::AssignToZero {
            lno,
            lhs: Box::new(lhs.right().unwrap()),
        })))
    }

    #[alias(expr)]
    #[allow(non_snake_case)]
    fn macroAssignToValue(input: ParseNode) -> ParseResult<EitherNode> {
        // x := n
        let (lhs, rhs) = match_nodes!(input.into_children();
            [IDENT(x), VALUE(n)] => (x, n)
        );
        let lno = LoopParserHelpers::lno(&input);

        Ok(EitherNode::Left(PollutedNode::Macro(
            Macro::AssignToValue {
                lno,
                lhs: Box::new(lhs.right().unwrap()),
                rhs: Box::new(rhs.right().unwrap()),
            },
        )))
    }

    #[alias(expr)]
    #[allow(non_snake_case)]
    fn macroAssignToIdentOpIdent(input: ParseNode) -> ParseResult<EitherNode> {
        // x := y * z
        let (lhs, rhs_lhs, rhs_verb, rhs_rhs) = match_nodes!(input.into_children();
            [IDENT(x), IDENT(y), verb, IDENT(z)] => (x, y, verb, z)
        );
        let lno = LoopParserHelpers::lno(&input);

        Ok(EitherNode::Left(PollutedNode::Macro(
            Macro::AssignToIdentBinOpIdent {
                lno,
                lhs: Box::new(lhs.right().unwrap()),
                rhs: MacroAssign {
                    lhs: Box::new(rhs_lhs.right().unwrap()),
                    verb: OperatorVerb::from(rhs_verb.as_str()),
                    rhs: Box::new(rhs_rhs.right().unwrap()),
                },
            },
        )))
    }

    #[alias(expr)]
    #[allow(non_snake_case)]
    fn macroAssignToIdentExtOpValue(input: ParseNode) -> ParseResult<EitherNode> {
        // x := y * n
        let (lhs, rhs_lhs, rhs_verb, rhs_rhs) = match_nodes!(input.into_children();
            [IDENT(x), IDENT(y), verb, VALUE(n)] => (x, y, verb, n)
        );
        let lno = LoopParserHelpers::lno(&input);

        Ok(EitherNode::Left(PollutedNode::Macro(
            Macro::AssignToIdentExtBinOpValue {
                lno,
                lhs: Box::new(lhs.right().unwrap()),
                rhs: MacroAssign {
                    lhs: Box::new(rhs_lhs.right().unwrap()),
                    verb: OperatorVerb::from(rhs_verb.as_str()),
                    rhs: Box::new(rhs_rhs.right().unwrap()),
                },
            },
        )))
    }

    // Conditionals
    #[allow(non_snake_case)]
    fn macroElseStmt(input: ParseNode) -> ParseResult<EitherNode> {
        Ok(match_nodes!(input.into_children();
            [terms(t)] => t
        ))
    }

    #[alias(expr)]
    #[allow(non_snake_case)]
    fn macroConditional(input: ParseNode) -> ParseResult<EitherNode> {
        // IF ... THEN ... ELSE
        let (comp, if_terms, else_terms) = match_nodes!(input.into_children();
            [comp(c), terms(i)] => (c, i, None),
            [comp(c), terms(i), macroElseStmt(e)] => (c, i, Some(e))
        );
        let lno = LoopParserHelpers::lno(&input);

        Ok(EitherNode::Left(PollutedNode::Macro(Macro::Conditional {
            lno,
            comp: Box::new(comp.right().unwrap()),
            if_terms: Box::new(if_terms.left().unwrap()),
            else_terms: Box::new(else_terms.map(|e| e.left().unwrap())),
        })))
    }

    // initialization rule
    pub(crate) fn grammar(input: ParseNode) -> ParseResult<EitherNode> {
        let terms = match_nodes!(input.into_children();
            [terms(t), EOI(_)] => t
        );

        Ok(terms)
    }
}
