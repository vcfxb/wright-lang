//! Representation for literal expressions in wright source code.

use num::BigUint;

use crate::parser::ast::metadata::AstNodeMeta;

/// An integer in Wright source code.
#[derive(Debug)]
pub struct IntegerLiteral<'src> {
    /// Metadata about this literal in source code.
    pub meta: AstNodeMeta<'src>,
    /// The value represented in source code. 
    pub value: BigUint
}

