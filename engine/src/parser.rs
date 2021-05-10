use either::Either;
use num_bigint::BigUint;
use num_traits::Zero;
use pest_consume::match_nodes;
use pest_consume::Error;
use pest_consume::Parser;

use crate::ast::control::Control;
use crate::ast::func::{Func, FuncCall};
use crate::ast::macros::{Macro, MacroAssign};
use crate::ast::module::{FuncDecl, Imp, ImpFunc, Module};
use crate::ast::node::Node;
use crate::ast::polluted::PollutedNode;
use crate::ast::variant::UInt;
use crate::ast::verbs::{ComparisonVerb, OperatorVerb};
use crate::types::LineNo;

#[derive(new, Clone)]
pub struct ParseSettings {
    lno: Option<LineNo>,
}

type ParseResult<T> = std::result::Result<T, Error<Rule>>;
type ParseNode<'i, 'a> = pest_consume::Node<'i, Rule, &'a ParseSettings>;

#[derive(Parser)]
#[grammar = "grammar.pest"]
#[allow(clippy::upper_case_acronyms)]
pub struct LoopParser;

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

    fn parse_comp(input: ParseNode) -> ParseResult<Node> {
        let (lhs, verb, rhs) = match_nodes!(<LoopParser>; input.into_children();
            [atom(lhs), verb, atom(rhs)] => (lhs, verb, rhs)
        );

        Ok(Node::Comparison {
            lhs: Box::new(lhs),
            verb: ComparisonVerb::from(verb.as_str()),
            rhs: Box::new(rhs),
        })
    }
}

#[pest_consume::parser]
#[allow(clippy::upper_case_acronyms)]
impl LoopParser {
    // Terminal Values
    #[allow(non_snake_case, clippy::upper_case_acronyms)]
    fn EOI(_input: ParseNode) -> ParseResult<PollutedNode> {
        Ok(PollutedNode::NoOp)
    }

    #[alias(expr)]
    #[allow(non_snake_case, clippy::upper_case_acronyms)]
    fn ELLIPSIS(_input: ParseNode) -> ParseResult<PollutedNode> {
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
            .map(|u| Node::NaturalNumber(UInt(u)))
            .map_err(|e| input.error(e))
    }

    #[alias(atom)]
    #[allow(non_snake_case, clippy::upper_case_acronyms)]
    fn ZERO(_input: ParseNode) -> ParseResult<Node> {
        Ok(Node::NaturalNumber(UInt(BigUint::zero())))
    }

    // Comparisons
    #[alias(comp)]
    #[allow(non_snake_case)]
    fn compEqual(input: ParseNode) -> ParseResult<Node> {
        LoopParserHelpers::parse_comp(input)
    }

    #[alias(comp)]
    #[allow(non_snake_case)]
    fn compNotEqual(input: ParseNode) -> ParseResult<Node> {
        LoopParserHelpers::parse_comp(input)
    }

    #[alias(comp)]
    #[allow(non_snake_case)]
    fn compNotEqual0(input: ParseNode) -> ParseResult<Node> {
        LoopParserHelpers::parse_comp(input)
    }

    #[alias(comp)]
    #[allow(non_snake_case)]
    fn compIdentNotEqual0(input: ParseNode) -> ParseResult<Node> {
        LoopParserHelpers::parse_comp(input)
    }

    #[alias(comp)]
    #[allow(non_snake_case)]
    fn compGreaterThan(input: ParseNode) -> ParseResult<Node> {
        LoopParserHelpers::parse_comp(input)
    }

    #[alias(comp)]
    #[allow(non_snake_case)]
    fn compGreaterEqual(input: ParseNode) -> ParseResult<Node> {
        LoopParserHelpers::parse_comp(input)
    }

    #[alias(comp)]
    #[allow(non_snake_case)]
    fn compLessThan(input: ParseNode) -> ParseResult<Node> {
        LoopParserHelpers::parse_comp(input)
    }

    #[alias(comp)]
    #[allow(non_snake_case)]
    fn compLessEqual(input: ParseNode) -> ParseResult<Node> {
        LoopParserHelpers::parse_comp(input)
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
            verb: OperatorVerb::from(verb.as_str()),
            rhs: Box::new(rhs),
        })
    }

    // Assignment
    #[alias(expr)]
    fn assign(input: ParseNode) -> ParseResult<PollutedNode> {
        let lno = LoopParserHelpers::lno(input.clone());

        let (ident, op) = match_nodes!(input.clone().into_children();
            [atom(i), op(o)] => (i, o)
        );

        Ok(PollutedNode::Pure(Node::Assign {
            lno,
            lhs: Box::new(ident),
            rhs: Box::new(op),
        }))
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
        let lno = LoopParserHelpers::lno(input.clone());
        let (ident, terms) = match_nodes!(input.into_children();
            [atom(a), expr(t)] => (a, t)
        );

        Ok(PollutedNode::Control(Control::Loop {
            lno,
            ident: Box::new(PollutedNode::Pure(ident)),
            terms: Box::new(terms),
        }))
    }

    #[alias(expr)]
    fn while_(input: ParseNode) -> ParseResult<PollutedNode> {
        let lno = LoopParserHelpers::lno(input.clone());
        let (comp, terms) = match_nodes!(input.into_children();
            [comp(c), expr(t)] => (c, t)
        );

        Ok(PollutedNode::Control(Control::While {
            lno,
            comp: Box::new(PollutedNode::Pure(comp)),
            terms: Box::new(terms),
        }))
    }

    // Macro collection (aliased as expr)
    #[alias(expr)]
    #[allow(non_snake_case)]
    fn macroAssignToIdent(input: ParseNode) -> ParseResult<PollutedNode> {
        // x := y
        let lno = LoopParserHelpers::lno(input.clone());
        let (lhs, rhs) = match_nodes!(input.into_children();
            [atom(i), atom(j)] => (i, j)
        );

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
        let lno = LoopParserHelpers::lno(input.clone());
        let lhs = match_nodes!(input.into_children();
            [atom(x)] => x
        );

        Ok(PollutedNode::Macro(Macro::AssignToZero {
            lno,
            lhs: Box::new(lhs),
        }))
    }

    #[alias(expr)]
    #[allow(non_snake_case)]
    fn macroAssignToValue(input: ParseNode) -> ParseResult<PollutedNode> {
        // x := n
        let lno = LoopParserHelpers::lno(input.clone());
        let (lhs, rhs) = match_nodes!(input.into_children();
            [atom(x), atom(n)] => (x, n)
        );

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
        let lno = LoopParserHelpers::lno(input.clone());
        let (lhs, rhs_lhs, rhs_verb, rhs_rhs) = match_nodes!(input.into_children();
            [atom(x), atom(y), verb, atom(z)] => (x, y, verb, z)
        );

        Ok(PollutedNode::Macro(Macro::AssignToIdentBinOpIdent {
            lno,
            lhs: Box::new(lhs),
            rhs: MacroAssign {
                lhs: Box::new(rhs_lhs),
                verb: OperatorVerb::from(rhs_verb.as_str()),
                rhs: Box::new(rhs_rhs),
            },
        }))
    }

    #[alias(expr)]
    #[allow(non_snake_case)]
    fn macroAssignToIdentExtOpValue(input: ParseNode) -> ParseResult<PollutedNode> {
        // x := y * n
        let lno = LoopParserHelpers::lno(input.clone());
        let (lhs, rhs_lhs, rhs_verb, rhs_rhs) = match_nodes!(input.into_children();
            [atom(x), atom(y), verb, atom(n)] => (x, y, verb, n)
        );

        Ok(PollutedNode::Macro(Macro::AssignToIdentExtBinOpValue {
            lno,
            lhs: Box::new(lhs),
            rhs: MacroAssign {
                lhs: Box::new(rhs_lhs),
                verb: OperatorVerb::from(rhs_verb.as_str()),
                rhs: Box::new(rhs_rhs),
            },
        }))
    }

    // Conditionals
    #[allow(non_snake_case)]
    fn macroElseStmt(input: ParseNode) -> ParseResult<PollutedNode> {
        Ok(match_nodes!(input.into_children();
            [expr(t)] => t
        ))
    }

    #[alias(expr)]
    #[allow(non_snake_case)]
    fn macroConditional(input: ParseNode) -> ParseResult<PollutedNode> {
        // IF ... THEN ... ELSE
        let lno = LoopParserHelpers::lno(input.clone());
        let (comp, if_terms, else_terms) = match_nodes!(input.into_children();
            [comp(c), expr(i)] => (c, i, None),
            [comp(c), expr(i), macroElseStmt(e)] => (c, i, Some(e))
        );

        Ok(PollutedNode::Macro(Macro::Conditional {
            lno,
            comp: Box::new(comp),
            if_terms: Box::new(if_terms),
            else_terms: Box::new(else_terms),
        }))
    }

    #[alias(expr)]
    #[allow(non_snake_case)]
    fn macroFnCall(input: ParseNode) -> ParseResult<PollutedNode> {
        // func(arg1, arg2, arg3)
        let lno = LoopParserHelpers::lno(input.clone());

        let (lhs, func, args): (Node, Node, Vec<Node>) = match_nodes!(input.into_children();
            [atom(lhs), atom(func), atom(args)..] => (lhs, func, args.collect())
        );

        let node = PollutedNode::Function(Func::Call {
            lno,
            lhs: Box::new(lhs),
            rhs: FuncCall {
                ident: Box::new(func),
                args,
            },
        });

        Ok(node)
    }

    // Function Definition
    #[allow(non_snake_case)]
    fn funcRet(input: ParseNode) -> ParseResult<Node> {
        let ident: Node = match_nodes!(input.into_children();
            [atom(item)] => item
        );
        Ok(ident)
    }

    #[allow(non_snake_case)]
    fn funcDef(input: ParseNode) -> ParseResult<FuncDecl> {
        let lno = LoopParserHelpers::lno(input.clone());

        let (ident, params, ret, terms): (Node, Vec<Node>, Node, PollutedNode) = match_nodes!(input.into_children();
            [atom(ident), atom(params).., funcRet(ret), expr(terms)] => (ident, params.collect(), ret, terms)
        );

        let decl = FuncDecl {
            lno,

            ident: Box::new(ident),
            params,
            ret: Box::new(ret),

            terms: Box::new(terms),
        };

        Ok(decl)
    }

    fn functions(input: ParseNode) -> ParseResult<Vec<FuncDecl>> {
        let funcs: Vec<FuncDecl> = match_nodes!(input.into_children();
            [funcDef(funcs)..] => funcs.collect()
        );

        Ok(funcs)
    }

    #[allow(non_snake_case)]
    fn importFunc(input: ParseNode) -> ParseResult<ImpFunc> {
        let (ident, alias): (Node, Option<Node>) = match_nodes!(input.into_children();
            [atom(ident), atom(alias)] => (ident, Some(alias)),
            [atom(ident)] => (ident, None),
        );

        let node = ImpFunc {
            ident: Box::new(ident),
            alias: alias.map(|a| Box::new(a)),
        };

        Ok(node)
    }

    #[allow(non_snake_case)]
    fn importStmt(input: ParseNode) -> ParseResult<Vec<ImpFunc>> {
        let stmt = match_nodes!(input.into_children();
            [importFunc(stmts)..] => stmts.collect()
        );

        Ok(stmt)
    }

    fn import(input: ParseNode) -> ParseResult<Imp> {
        let lno = LoopParserHelpers::lno(input.clone());

        let (path, stmt): (Vec<Node>, Vec<ImpFunc>) = match_nodes!(input.into_children();
            [atom(path).., importStmt(stmt)] => (path.collect(), stmt)
        );

        let import = Imp {
            lno,

            path,
            funcs: stmt,
        };

        Ok(import)
    }

    fn imports(input: ParseNode) -> ParseResult<Vec<Imp>> {
        let imports: Vec<Imp> = match_nodes!(input.into_children();
            [import(imports)..] => imports.collect()
        );

        Ok(imports)
    }

    // Initialization Rule
    pub(crate) fn grammar(input: ParseNode) -> ParseResult<Module> {
        let (imp, decl, code): (Option<Vec<Imp>>, Option<Vec<FuncDecl>>, PollutedNode) = match_nodes!(input.into_children();
            [imports(i), functions(f), expr(t), EOI(_)] => (Some(i), Some(f), t),
            [imports(i), functions(f), EOI(_)] => (Some(i), Some(f), PollutedNode::NoOp),
            [functions(f), expr(t), EOI(_)] => (None, Some(f), t),
            [imports(i), expr(t), EOI(_)] => (Some(i), None, t),
            [expr(t), EOI(_)] => (None, None, t)
        );

        Ok(Module {
            imp: imp.unwrap_or_default(),
            decl: decl.unwrap_or_default(),
            code,
        })
    }

    // Make the parser happy, these always error out.
    #[allow(non_snake_case, clippy::upper_case_acronyms)]
    fn EQ(input: ParseNode) -> ParseResult<Node> {
        Err(input.error("Cannot directly parse EQ"))
    }
    #[allow(non_snake_case, clippy::upper_case_acronyms)]
    fn NE(input: ParseNode) -> ParseResult<Node> {
        Err(input.error("Cannot directly parse NE"))
    }
    #[allow(non_snake_case, clippy::upper_case_acronyms)]
    fn GT(input: ParseNode) -> ParseResult<Node> {
        Err(input.error("Cannot directly parse GT"))
    }
    #[allow(non_snake_case, clippy::upper_case_acronyms)]
    fn GE(input: ParseNode) -> ParseResult<Node> {
        Err(input.error("Cannot directly parse GE"))
    }
    #[allow(non_snake_case, clippy::upper_case_acronyms)]
    fn LT(input: ParseNode) -> ParseResult<Node> {
        Err(input.error("Cannot directly parse LT"))
    }
    #[allow(non_snake_case, clippy::upper_case_acronyms)]
    fn LE(input: ParseNode) -> ParseResult<Node> {
        Err(input.error("Cannot directly parse LE"))
    }
    #[allow(non_snake_case, clippy::upper_case_acronyms)]
    fn OP_PLUS(input: ParseNode) -> ParseResult<Node> {
        Err(input.error("Cannot directly parse OP_PLUS"))
    }
    #[allow(non_snake_case, clippy::upper_case_acronyms)]
    fn OP_MINUS(input: ParseNode) -> ParseResult<Node> {
        Err(input.error("Cannot directly parse OP_MINUS"))
    }
    #[allow(non_snake_case, clippy::upper_case_acronyms)]
    fn OP_MULTIPLY(input: ParseNode) -> ParseResult<Node> {
        Err(input.error("Cannot directly parse OP_MULTIPLY"))
    }
}
