//! AST node representations to do with type instantiations in wright source code.

use super::{expression::Expression, metadata::AstNodeMeta, path::Path};

/// A use of a type in source code.
#[derive(Debug)]
pub struct TypeInstantiation<'src> {
    /// The metadata for this node.
    pub meta: AstNodeMeta<'src>,
    /// The type's name, possibly at the end of a path to resolve it.
    /// This path will usually probably be one identifier long.
    pub typename: Path<'src>,
    /// Any types used as generic arguments to make this a concrete type.
    pub generic_type_arguments: Vec<TypeInstantiation<'src>>,
    /// Any generic constants used to construct this type, in order.
    pub generic_const_arguments: Vec<Expression<'src>>,
    /// Optional constraint that modifies this type.
    pub constrain_clause: Option<Expression<'src>>,
}
