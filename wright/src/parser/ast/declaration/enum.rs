//! Enum declarations in wright souce code.

use crate::parser::ast::{
    declaration::visibility::Visibility, expression::Expression, identifier::Identifier,
    metadata::AstNodeMeta, types::TypeInstantiation,
};

/// An enumeration in source code.
#[derive(Debug)]
pub struct EnumDeclaration<'src> {
    /// The metadata for this AST node.
    pub meta: AstNodeMeta<'src>,
    /// The visibility of the enum.
    pub vis: Visibility<'src>,
    /// The name of the enum
    pub name: Identifier<'src>,
    /// The parent type or enumeration that this enumeration is a strict subset of.
    pub parent: TypeInstantiation<'src>,
}

/// A variant of an enum in an enum declaration.
#[derive(Debug)]
pub struct EnumVariant<'src> {
    /// The metadata for this AST node.
    pub meta: AstNodeMeta<'src>,
    /// The name of this variant of the enum.
    pub name: Identifier<'src>,
    /// The value that this variant represents.
    pub value: Expression<'src>,
}
