use core::fmt;
use std::fmt::{Display, Formatter};

use indoc::indoc;

#[cfg(feature = "cli")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::ast::context::CompileContext;
use crate::ast::control::Control;
use crate::ast::variant::UInt;
use crate::ast::verbs::{ComparisonVerb, OperatorVerb};
use crate::errors::{Error, ErrorVariant, StdResult};
use crate::flags::CompileFlags;
use crate::types::LineNo;
use crate::utils::check_errors;

pub static CONST_IDENT: [&str; 1] = ["_zero"];

// Note(bmahmoud): in the future we could also support unary expressions?
#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "cli", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub enum Expr {
    // Smallest Units
    Ident(String),
    NaturalNumber(UInt),

    // Assignment and Expressions
    Comparison {
        lhs: Box<Expr>,
        verb: ComparisonVerb,
        rhs: Box<Expr>,
    },
    BinaryOp {
        lhs: Box<Expr>,
        verb: OperatorVerb,
        rhs: Box<Expr>,
    },
    Assign {
        lno: LineNo,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Control(Control<Expr>),
}

impl Expr {
    pub fn flatten(&self) -> Expr {
        match self {
            Expr::Control(Control::Terms(terms)) => Expr::Control(Control::Terms(
                terms
                    .iter()
                    .flat_map(|node| match node {
                        Expr::Control(Control::Terms(t)) => t
                            .iter()
                            .flat_map(|term| {
                                let flat = term.flatten();

                                match flat {
                                    Expr::Control(Control::Terms(t)) => t,
                                    _ => vec![flat],
                                }
                            })
                            .collect(),
                        Expr::Control(Control::Loop { lno, ident, terms }) => {
                            vec![Expr::Control(Control::Loop {
                                lno: *lno,
                                ident: ident.clone(),
                                terms: Box::new(terms.flatten()),
                            })]
                        }
                        Expr::Control(Control::While { lno, comp, terms }) => {
                            vec![Expr::Control(Control::While {
                                lno: *lno,
                                comp: comp.clone(),
                                terms: Box::new(terms.flatten()),
                            })]
                        }
                        _ => vec![node.clone()],
                    })
                    .collect(),
            )),
            Expr::Control(Control::Loop { lno, ident, terms }) => Expr::Control(Control::Loop {
                lno: *lno,
                ident: ident.clone(),
                terms: Box::new(terms.flatten()),
            }),
            Expr::Control(Control::While { lno, comp, terms }) => Expr::Control(Control::While {
                lno: *lno,
                comp: comp.clone(),
                terms: Box::new(terms.flatten()),
            }),
            _ => self.clone(),
        }
    }

    /* Display human friendly representation */
    pub fn display(&self, indent: u8, level: Option<u8>) -> String {
        let level = level.or(Some(0));
        let spacing = " ".repeat((indent * level.unwrap()) as usize);

        match self {
            Expr::Ident(s) => s.clone(),
            Expr::NaturalNumber(n) => n.to_string(),
            Expr::Comparison { lhs, verb, rhs } => format!(
                "{} {} {}",
                lhs.display(indent, level),
                verb,
                rhs.display(indent, level)
            ),
            Expr::BinaryOp { lhs, verb, rhs } => format!(
                "{} {} {}",
                lhs.display(indent, level),
                verb,
                rhs.display(indent, level)
            ),
            Expr::Assign { lno: _, lhs, rhs } => format!(
                "{s}{lhs} := {rhs}",
                lhs = lhs.display(indent, level),
                rhs = rhs.display(indent, level),
                s = spacing,
            ),
            Expr::Control(Control::Terms(terms)) => terms
                .iter()
                .map(|term| term.display(indent, level))
                .collect::<Vec<String>>()
                .join("\n"),
            Expr::Control(Control::Loop {
                lno: _,
                ident,
                terms,
            }) => format!(
                indoc!(
                    "\n\
                     {s}LOOP {ident} DO
                     {terms}
                     {s}END"
                ),
                ident = ident.display(indent, level),
                terms = terms.display(indent, level.map(|c| c + 1)),
                s = spacing
            ),
            Expr::Control(Control::While {
                lno: _,
                comp,
                terms,
            }) => format!(
                indoc!(
                    "\n\
                     {s}WHILE {comp} DO
                     {terms}
                     {s}END"
                ),
                comp = comp.display(indent, level),
                terms = terms.display(indent, level.map(|c| c + 1)),
                s = spacing
            ),
        }
    }

    pub fn verify(self, context: &mut CompileContext) -> StdResult<Self> {
        match &self {
            Expr::Assign { lhs, rhs: _, lno } => {
                let ident = match *lhs.clone() {
                    Expr::Ident(m) => m,
                    _ => unreachable!(),
                };

                if context.flags.contains(CompileFlags::CNF_CONST)
                    && CONST_IDENT.contains(&ident.as_str())
                {
                    return Err(vec![Error::new(
                        *lno,
                        ErrorVariant::Message(format!(
                            "Tried to assign a value to declared CONST {}, \
                             not allowed with compilation flag CNF_CONST",
                            ident
                        )),
                    )]);
                }

                Ok(self)
            }
            Expr::Control(Control::Terms(t)) => {
                let verify: Vec<_> = t.iter().map(|term| term.clone().verify(context)).collect();
                check_errors(&verify)?;

                Ok(self)
            }
            Expr::Control(Control::While {
                comp,
                terms,
                lno: _,
            }) => {
                let verify = vec![comp.clone().verify(context), terms.clone().verify(context)];
                check_errors(&verify)?;

                Ok(self)
            }
            Expr::Control(Control::Loop {
                ident,
                terms,
                lno: _,
            }) => {
                let verify = vec![ident.clone().verify(context), terms.clone().verify(context)];
                check_errors(&verify)?;

                Ok(self)
            }
            _ => Ok(self),
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display(4, None))
    }
}
