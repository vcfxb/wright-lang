//! AST Node structures relating to generics.

use crate::parser::ast::{identifier::Identifier, metadata::AstNodeMeta, types::TypeInstantiation};

/// A generic type argument in a type/class/etc declaration.
#[derive(Debug)]
pub struct GenericTypeArg<'src> {
    /// The node metadata for this generic argument.
    pub meta: AstNodeMeta<'src>,
    /// The identifier for the generic type.
    pub name: Identifier<'src>,
}

/// A generic const argument in a type/class/etc declaration.
#[derive(Debug)]
pub struct GenericConstArg<'src> {
    /// The metadata associated with this node.
    pub meta: AstNodeMeta<'src>,
    /// The identifier for this generic constant.
    pub name: Identifier<'src>,
    /// The type expected in the generic instantiation.
    pub ty: TypeInstantiation<'src>,
}
