use crate::grammar::ast::{BooleanLit, CharLit, Identifier, NumLit, ScopedName, StringLit};
use crate::grammar::model::Fragment;

/// A Pattern used in pattern matching.
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub enum Pattern<'s> {
    NumLit(NumLitPattern<'s>),
    CharLit(CharLit<'s>),
    StringLit(StringLit<'s>),
    BooleanLit(BooleanLit<'s>),
    Identifier(Identifier<'s>),
    ScopedName(ScopedName<'s>),
    Underscore(Underscore<'s>),
}

/// An underscore pattern in source code.
#[derive(Clone, Debug)]
pub struct Underscore<'s> {
    /// Associated fragment in source code.
    pub frag: Fragment<'s>,
}

/// Number literal pattern
#[derive(Clone, Debug)]
pub struct NumLitPattern<'s> {
    /// Associated Fragment in source code.
    pub frag: Fragment<'s>,
    /// Whether the number literal pattern has '-' in front
    pub negative: bool,
    /// Inner number literal value
    pub inner: NumLit<'s>,
}
