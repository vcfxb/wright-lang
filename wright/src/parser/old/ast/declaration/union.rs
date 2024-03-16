//! Union declarations in wright source code.

use crate::parser::ast::{
    declaration::generics::{GenericConstArg, GenericTypeArg},
    declaration::visibility::Visibility,
    identifier::Identifier,
    metadata::AstNodeMeta,
    types::TypeInstantiation,
};

use super::where_clause::WhereClause;

/// A union declaration in source code.
#[derive(Debug)]
pub struct UnionDeclaration<'src> {
    /// The metadata for this node.
    pub meta: AstNodeMeta<'src>,
    /// The visibility of the union.
    pub vis: Visibility<'src>,
    /// The name of the union.
    pub name: Identifier<'src>,
    /// Generic types that this class declares.
    pub generic_type_arguments: Vec<GenericTypeArg<'src>>,
    /// Generic constants that this class declares.
    pub generic_const_arguments: Vec<GenericConstArg<'src>>,
    /// Optional clause to define bounds on the generic types declared in this union.
    pub where_clause: Option<WhereClause<'src>>,
    /// The variants available to this union.
    pub variants: Vec<UnionVariant<'src>>,
}

/// A variant of a union declaration in source code.
#[derive(Debug)]
pub struct UnionVariant<'src> {
    /// The metadata for this node.
    pub meta: AstNodeMeta<'src>,
    /// The name of this variant of the union.
    pub name: Identifier<'src>,
    /// The type of this variant of the union.
    pub ty: TypeInstantiation<'src>,
}
