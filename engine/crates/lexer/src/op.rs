// We do not yet say what exactly they are, we just assume that they are
// an operator, now what they do. Could be used for unary or something else.

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Op {
    Plus,
    Minus,
    Star,
    Slash,
}
