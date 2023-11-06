//! Statements that bind a value to a symbol, possibly with a type, in the context of a scope.

use crate::parser::ast::metadata::AstNodeMeta;

/// Bind statement
pub struct Bind<'src> {
    pub meta: AstNodeMeta<'src>,
}
