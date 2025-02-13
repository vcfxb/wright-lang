//! AST node models representing literal values in source code.

use num::BigUint;

use crate::source_tracking::fragment::Fragment;

/// An integer literal from source. This only contains unsigned integers as writing negative numbers is considered
/// to be a combination of an integer literal with a unary negation.
#[derive(Debug)]
pub struct IntegerLiteral {
    /// The [Fragment] of source code containing this integer literal.
    pub fragment: Fragment,

    /// The value of the integer parsed from the matching source.
    pub value: BigUint,
}

/// A boolean literal from source.
#[derive(Debug)]
pub struct BooleanLiteral {
    /// The [Fragment] of source code containing this boolean literal.
    pub fragment: Fragment,

    /// The value of the boolean literal.
    pub value: bool,
}
