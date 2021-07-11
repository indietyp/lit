use itertools::Itertools;
use lazy_static::lazy_static;
use std::fmt;
use std::fmt::Formatter;

bitflags! {
    pub struct MacroModifier: u16 {
        const CASE_INSENSITIVE = 0b0001;
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Directive {
    Macro(MacroModifier),
    // starts the substitution
    Sub,
    End,

    If,
    Else,

    Sep,

    Placeholder(Placeholder),
}

impl fmt::Display for Directive {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Macro(modifier) => format!("‘@macro/[{:?}]‘", modifier),
                Self::Sub => "‘@sub‘".into(),
                Self::End => "‘@end‘".into(),
                Self::If => "‘@if‘".into(),
                Self::Else => "‘@else‘".into(),
                Self::Sep => "‘@sep‘".into(),
                Self::Placeholder(placeholder) => format!("{}", placeholder),
            }
        )
    }
}

bitflags! {
    pub struct PlaceholderVariant: u16 {
        // none means that it will be never matched
        // effectively being a match blocker
        const NONE      = 0b0000;

        const IDENT     = 0b0001;
        const NUMBER    = 0b0010;
        const PRIMITIVE = 0b0011;

        const COMP = 0b0001 << 4;
        const OP   = 0b0010 << 4;

        const EXPR  = 0b0001 << 8;
        const BLOCK = 0b0010 << 8;

        const ANY = 0b1111_1111_1111_1111;
    }
}

impl fmt::Display for PlaceholderVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        lazy_static! {
            static ref MAPPING: [(PlaceholderVariant, &'static str); 6] = [
                (PlaceholderVariant::IDENT, "i"),
                (PlaceholderVariant::NUMBER, "n"),
                (PlaceholderVariant::COMP, "c"),
                (PlaceholderVariant::OP, "o"),
                (PlaceholderVariant::EXPR, "e"),
                (PlaceholderVariant::BLOCK, "b"),
            ];
        }

        let mut variants: Vec<String> = vec![];
        for idx in 0..MAPPING.len() {
            let (variant, value) = MAPPING[idx];
            if self.contains(variant) {
                variants.push(value.into());
            }
        }

        f.write_str(variants.into_iter().sorted().join("").as_str())
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Placeholder {
    // Match placeholders are going to be matched, and are defined in the match
    // block and then used in the substitution block
    Match {
        variant: PlaceholderVariant,
        index: u32,
    },
    // Sub placeholders are invalid in the match block and can be used to generate
    // values (currently only temporary variables), those are replaced at runtime
    Sub {
        variant: PlaceholderVariant,
        index: u32,
    },
}

impl fmt::Display for Placeholder {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(
            match self {
                Placeholder::Match { variant, index } => format!("%{}/{}", index, variant),
                Placeholder::Sub { variant, index } => format!("${}/{}", index, variant),
            }
            .as_str(),
        )
    }
}
