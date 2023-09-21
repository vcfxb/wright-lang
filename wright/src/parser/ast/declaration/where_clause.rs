//! Structural representation of `where` clauses in wright.

use crate::parser::ast::{metadata::AstNodeMeta, types::TypeInstantiation};

/// A where clause in wright source code.
#[derive(Debug)]
pub struct WhereClause<'src> {
    /// The metadata for this AST node.
    pub meta: AstNodeMeta<'src>,
    /// The type bounds defined in this where clause.
    pub bounds: Vec<TypeBound<'src>>,
}

/// A bound on a type defined in a where clause.
#[derive(Debug)]
pub struct TypeBound<'src> {
    /// The metadata for this AST node.
    pub meta: AstNodeMeta<'src>,
    /// The type being bound.
    pub lhs_ty: TypeInstantiation<'src>,
    /// The trait being required of it.
    pub requirement: TypeInstantiation<'src>,
}
