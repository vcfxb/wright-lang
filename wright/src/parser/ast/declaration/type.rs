//! Type alias declarations in wright source code.

use crate::parser::ast::{
    declaration::generics::{GenericConstArg, GenericTypeArg},
    declaration::visibility::Visibility,
    identifier::Identifier,
    metadata::AstNodeMeta,
    types::TypeInstantiation,
};

use super::where_clause::WhereClause;

/// A type alias in source code.
#[derive(Debug)]
pub struct TypeDeclaration<'src> {
    /// The metadata for this node.
    pub meta: AstNodeMeta<'src>,
    /// The type alias's visibility.
    pub vis: Visibility<'src>,
    /// The name of the type.
    pub name: Identifier<'src>,
    /// The generic type arguments that need to be passed to this type.
    pub generic_type_arguments: Vec<GenericTypeArg<'src>>,
    /// The generic constant arguments that need to be passed to this type.
    pub generic_const_arguments: Vec<GenericConstArg<'src>>,
    /// Optional clause to define bounds on the generics declared in this type alias.
    pub where_clause: Option<WhereClause<'src>>,
    /// The type being aliased to. This is optional, as traits may declare associated types.
    pub dest: Option<TypeInstantiation<'src>>,
}
