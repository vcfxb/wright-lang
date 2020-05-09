use crate::grammar::ast::{BooleanLit, CharLit, Identifier, NumLit, ScopedName, StringLit};
use crate::grammar::model::Fragment;
use std::fmt::Debug;

/// A Pattern used in pattern matching.
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub enum Pattern<SourceCodeReference: Clone + Debug> {
    NumLit(NumLitPattern<SourceCodeReference>),
    CharLit(CharLit<SourceCodeReference>),
    StringLit(StringLit<SourceCodeReference>),
    BooleanLit(BooleanLit<SourceCodeReference>),
    Identifier(Identifier<SourceCodeReference>),
    ScopedName(ScopedName<SourceCodeReference>),
    Underscore(Underscore<SourceCodeReference>),
}

/// An underscore pattern in source code.
#[derive(Clone, Debug)]
pub struct Underscore<SourceCodeReference: Clone + Debug> {
    /// Associated source code.
    pub source: SourceCodeReference,
}

/// Number literal pattern
#[derive(Clone, Debug)]
pub struct NumLitPattern<SourceCodeReference: Clone + Debug> {
    /// Associated source code reference.
    pub source: SourceCodeReference,
    /// Whether the number literal pattern has '-' in front
    pub negative: bool,
    /// Inner number literal value
    pub inner: NumLit<SourceCodeReference>,
}
