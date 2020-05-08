use crate::grammar::model::Fragment;
use std::fmt::Debug;

/// An identifier in Wright source code.
/// There is only one field here, the source code of the identifier.
/// This is because the identifier itself will be the same as the
/// source.
#[derive(Clone, Debug)]
pub struct Identifier<SourceCodeReference: Copy + Debug> {
    /// Reference to associated source code.
    pub source: SourceCodeReference,
}

/// A scoped, or qualified, name.
#[derive(Clone, Debug)]
pub struct ScopedName<SourceCodeReference: Clone + Debug> {
    /// The source code fragment.
    pub frag: SourceCodeReference,
    /// The sequence of simple identifiers.
    /// Example: foo::bar::baz -> [ foo, bar ]
    pub path: Vec<SourceCodeReference>,
    /// The final simple identifier
    /// Example: foo::bar::baz -> baz
    pub name: Identifier<SourceCodeReference>,
}

/// Numerical literal in wright source code.
/// i.e. `10`, `0xCa1a0`, `0b0101_0101`, `100_000`
#[derive(Clone, Debug)]
pub struct NumLit<'s> {
    /// Associated fragment of source code.
    pub frag: Fragment<'s>,
    /// Represented value.
    pub inner: u128,
}

/// Character literal in wright source code.
/// i.e `'a', '\n', '\u{01f441}', '\x00', '♦'`
/// see [this page](https://doc.rust-lang.org/reference/tokens.html#ascii-escapes) for escape
/// information.
#[derive(Clone, Debug)]
pub struct CharLit<'s> {
    /// Associated fragment of source code.
    pub frag: Fragment<'s>,
    /// Represented Value.
    pub inner: char,
}

/// String literal in wright source code.
/// i.e. `"hello world", "with \n newline \n escapes"`
/// The parser for string literals also includes all the escape characters
/// [here](https://doc.rust-lang.org/reference/tokens.html#ascii-escapes).
/// Raw-strings and Byte-strings (like those in rust) are not currently
/// supported but may be added in the future.
#[derive(Clone, Debug)]
pub struct StringLit<'s> {
    /// Associated fragment of source code.
    pub frag: Fragment<'s>,
    /// Represented string value. (not a reference into source code because
    /// source code may contain escaped characters.)
    pub inner: String,
}

/// Boolean literal in wright source code.
/// i.e. `true`, `false`.
#[derive(Clone, Debug)]
pub struct BooleanLit<'s> {
    /// Associated fragment in source code.
    pub frag: Fragment<'s>,
    /// Represented value.
    pub inner: bool,
}

/// `self` literal in wright source code.
#[derive(Clone, Debug)]
pub struct SelfLit<'s> {
    /// Associated fragment in source code.
    pub frag: Fragment<'s>,
}
