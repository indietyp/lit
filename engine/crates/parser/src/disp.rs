use ctrl::Control;
use expr::{Assign, BinOp, Comp, Expr, Primitive};
use fnc::Func;
use hir::Hir;

use indoc::indoc;
use mcr::Unknown;
use textwrap::Options;
use variants::LineNo;

static INDENTATION_LEVEL: usize = 2;

pub(crate) trait CompactRepresentation {
    fn compact(&self, indent: Option<usize>) -> String;
}

impl CompactRepresentation for Hir {
    fn compact(&self, indent: Option<usize>) -> String {
        let indent = indent.unwrap_or(0);

        match self {
            Hir::Expr(e) => e.compact(Some(indent)),
            Hir::Func(f) => f.compact(Some(indent)),
            Hir::Control(c) => c.compact(Some(indent)),
            Hir::Unknown(u) => u.compact(Some(indent)),
            Hir::NoOp => "NoOp".to_string(),
        }
    }
}

impl CompactRepresentation for Unknown {
    fn compact(&self, indent: Option<usize>) -> String {
        let text: String = match self {
            Unknown::Token(token) => vec![token.clone()],
            Unknown::Tokens(tokens) => tokens.clone(),
        }
        .into_iter()
        .map(|value| value.content)
        .collect::<Vec<_>>()
        .join(" ");
        let wrapped: String =
            textwrap::wrap(text.as_str(), Options::new(25).break_words(false)).join("\n");
        let wrapped = textwrap::indent(&wrapped, " ".repeat(INDENTATION_LEVEL).as_str());

        format!(
            indoc!(
                "\
                Unknown:
                {}"
            ),
            wrapped
        )
    }
}

impl CompactRepresentation for Primitive {
    fn compact(&self, _: Option<usize>) -> String {
        match self {
            Primitive::Ident { value, .. } => value.clone(),
            Primitive::Number { value, .. } => value.to_string(),
        }
    }
}

impl CompactRepresentation for LineNo {
    fn compact(&self, _: Option<usize>) -> String {
        format!(
            "[{:#?}:{:#?}->{:#?}:{:#?}]",
            self.row.start(),
            self.col.start(),
            self.row.end(),
            self.col.end()
        )
    }
}

impl CompactRepresentation for Comp {
    fn compact(&self, indent: Option<usize>) -> String {
        format!(
            "{} {} {}",
            self.lhs.compact(indent),
            self.verb.to_string(),
            self.rhs.compact(indent)
        )
    }
}

impl CompactRepresentation for Expr {
    fn compact(&self, indent: Option<usize>) -> String {
        match self {
            Expr::Primitive(p) => p.compact(indent),
            Expr::Comp(c) => c.compact(indent),
            Expr::BinOp(BinOp { lhs, verb, rhs, .. }) => format!(
                "{} {} {}",
                lhs.compact(indent),
                verb.to_string(),
                rhs.compact(indent)
            ),
            Expr::Assign(Assign { lno, lhs, rhs }) => format!(
                "Assign@{}: {} := {}",
                lno.compact(indent),
                lhs.compact(indent),
                rhs.compact(indent)
            ),
            Expr::Control(c) => c.compact(indent),
        }
    }
}

impl CompactRepresentation for Func {
    fn compact(&self, indent: Option<usize>) -> String {
        todo!()
    }
}

impl<Type, Primitive, Comp> CompactRepresentation for Control<Type, Primitive, Comp>
where
    Type: CompactRepresentation + Clone,
    Primitive: CompactRepresentation,
    Comp: CompactRepresentation,
{
    fn compact(&self, indent: Option<usize>) -> String {
        let value = match self {
            Control::Block { terms } => terms
                .iter()
                .map(|term| term.clone().compact(indent))
                .collect::<Vec<_>>()
                .join("\n"),
            Control::Loop { lno, ident, terms } => format!(
                indoc!(
                    "
                    Loop@{}:
                      Ident: {}
                      Terms:
                    {}
                    "
                ),
                lno.compact(indent),
                ident.compact(indent),
                terms.compact(indent.or(Some(0)).map(|value| value + 1))
            ),
            Control::While { lno, comp, terms } => format!(
                indoc!(
                    "\
                    While@{}:
                      Comp: {}
                      Terms:
                    {}
                    "
                ),
                lno.compact(indent),
                comp.compact(indent),
                terms.compact(indent.or(Some(0)).map(|value| value + 1))
            ),
        };

        textwrap::indent(
            value.as_str(),
            " ".repeat(indent.unwrap_or(0) * INDENTATION_LEVEL).as_str(),
        )
    }
}
