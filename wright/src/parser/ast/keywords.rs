//! Keywords in wight source code. This module is just used for converting
//! any identifiers in wright source code to a keyword if necessary.

/// A keyword in wright source code.
#[derive(Clone, Copy, Debug)]
pub enum Keyword {
    /// `class`
    Class,
    /// `struct`
    Struct,
    /// `trait`
    Trait,
    /// `enum`
    Enum,
    /// `union`
    Union,
}
