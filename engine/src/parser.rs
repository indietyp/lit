use crate::ast::control::Control;
use crate::ast::macros::{Macro, MacroAssign};
use crate::ast::node::{NaturalNumber, Node};
use crate::ast::polluted::PollutedNode;
use crate::ast::verbs::{ComparisonVerb, OperatorVerb};
use crate::types::LineNo;
use either::Either;
use num_bigint::BigUint;
use num_traits::Zero;
use pest_consume::match_nodes;
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
    fn lno(input: ParseNode) -> LineNo {
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

    #[alias(expr)]
    #[allow(non_snake_case, clippy::upper_case_acronyms)]
    fn ELLIPSIS(input_: ParseNode) -> ParseResult<EitherNode> {
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
            .map(|u| EitherNode::Right(Node::NaturalNumber(NaturalNumber(u))))
            .map_err(|e| input.error(e))
    }

    #[alias(atom)]
    #[allow(non_snake_case, clippy::upper_case_acronyms)]
    fn ZERO(_input: ParseNode) -> ParseResult<EitherNode> {
        Ok(EitherNode::Right(Node::NaturalNumber(NaturalNumber(
            BigUint::zero(),
        ))))
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
    fn compIdentNotEqual0(input: ParseNode) -> ParseResult<EitherNode> {
        LoopParserHelpers::parse_comp(input)
    }

    #[alias(comp)]
    #[allow(non_snake_case)]
    fn compGreaterThan(input: ParseNode) -> ParseResult<EitherNode> {
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
        let lno = LoopParserHelpers::lno(input.clone());
        let (ident, op) = match_nodes!(input.into_children();
            [atom(i), op(o)] => (i, o)
        );

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
            terms
                .map(|term| term.either(|left| left, PollutedNode::Pure))
                .collect(),
        ))))
    }

    #[alias(expr)]
    fn loop_(input: ParseNode) -> ParseResult<EitherNode> {
        let lno = LoopParserHelpers::lno(input.clone());
        let (ident, terms) = match_nodes!(input.into_children();
            [atom(a), expr(t)] => (a, t)
        );

        Ok(EitherNode::Left(PollutedNode::Control(Control::Loop {
            lno,
            ident: Box::new(PollutedNode::Pure(ident.right().unwrap())),
            terms: Box::new(terms.left().unwrap()),
        })))
    }

    #[alias(expr)]
    fn while_(input: ParseNode) -> ParseResult<EitherNode> {
        let lno = LoopParserHelpers::lno(input.clone());
        let (comp, terms) = match_nodes!(input.into_children();
            [comp(c), expr(t)] => (c, t)
        );

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
        let lno = LoopParserHelpers::lno(input.clone());
        let (lhs, rhs) = match_nodes!(input.into_children();
            [atom(i), atom(j)] => (i, j)
        );

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
        let lno = LoopParserHelpers::lno(input.clone());
        let lhs = match_nodes!(input.into_children();
            [atom(x)] => x
        );

        Ok(EitherNode::Left(PollutedNode::Macro(Macro::AssignToZero {
            lno,
            lhs: Box::new(lhs.right().unwrap()),
        })))
    }

    #[alias(expr)]
    #[allow(non_snake_case)]
    fn macroAssignToValue(input: ParseNode) -> ParseResult<EitherNode> {
        // x := n
        let lno = LoopParserHelpers::lno(input.clone());
        let (lhs, rhs) = match_nodes!(input.into_children();
            [atom(x), atom(n)] => (x, n)
        );

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
        let lno = LoopParserHelpers::lno(input.clone());
        let (lhs, rhs_lhs, rhs_verb, rhs_rhs) = match_nodes!(input.into_children();
            [atom(x), atom(y), verb, atom(z)] => (x, y, verb, z)
        );

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
        let lno = LoopParserHelpers::lno(input.clone());
        let (lhs, rhs_lhs, rhs_verb, rhs_rhs) = match_nodes!(input.into_children();
            [atom(x), atom(y), verb, atom(n)] => (x, y, verb, n)
        );

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
            [expr(t)] => t
        ))
    }

    #[alias(expr)]
    #[allow(non_snake_case)]
    fn macroConditional(input: ParseNode) -> ParseResult<EitherNode> {
        // IF ... THEN ... ELSE
        let lno = LoopParserHelpers::lno(input.clone());
        let (comp, if_terms, else_terms) = match_nodes!(input.into_children();
            [comp(c), expr(i)] => (c, i, None),
            [comp(c), expr(i), macroElseStmt(e)] => (c, i, Some(e))
        );

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
            [expr(t), EOI(_)] => t
        );

        Ok(terms)
    }

    // Make the parser happy, these always error out.
    #[allow(non_snake_case, clippy::upper_case_acronyms)]
    fn EQ(input: ParseNode) -> ParseResult<EitherNode> {
        Err(input.error("Cannot directly parse EQ"))
    }
    #[allow(non_snake_case, clippy::upper_case_acronyms)]
    fn NE(input: ParseNode) -> ParseResult<EitherNode> {
        Err(input.error("Cannot directly parse NE"))
    }
    #[allow(non_snake_case, clippy::upper_case_acronyms)]
    fn GT(input: ParseNode) -> ParseResult<EitherNode> {
        Err(input.error("Cannot directly parse GT"))
    }
    #[allow(non_snake_case, clippy::upper_case_acronyms)]
    fn GE(input: ParseNode) -> ParseResult<EitherNode> {
        Err(input.error("Cannot directly parse GE"))
    }
    #[allow(non_snake_case, clippy::upper_case_acronyms)]
    fn LT(input: ParseNode) -> ParseResult<EitherNode> {
        Err(input.error("Cannot directly parse LT"))
    }
    #[allow(non_snake_case, clippy::upper_case_acronyms)]
    fn LE(input: ParseNode) -> ParseResult<EitherNode> {
        Err(input.error("Cannot directly parse LE"))
    }
    #[allow(non_snake_case, clippy::upper_case_acronyms)]
    fn OP_PLUS(input: ParseNode) -> ParseResult<EitherNode> {
        Err(input.error("Cannot directly parse OP_PLUS"))
    }
    #[allow(non_snake_case, clippy::upper_case_acronyms)]
    fn OP_MINUS(input: ParseNode) -> ParseResult<EitherNode> {
        Err(input.error("Cannot directly parse OP_MINUS"))
    }
    #[allow(non_snake_case, clippy::upper_case_acronyms)]
    fn OP_MULTIPLY(input: ParseNode) -> ParseResult<EitherNode> {
        Err(input.error("Cannot directly parse OP_MULTIPLY"))
    }
}
