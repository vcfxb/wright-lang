//! Class declarations in wright source code.

use crate::parser::ast::{
    declaration::generics::{GenericConstArg, GenericTypeArg},
    declaration::visibility::Visibility,
    identifier::Identifier,
    metadata::AstNodeMeta,
    types::TypeInstantiation,
};

/// A class declaration in source code.
#[derive(Debug)]
pub struct ClassDeclaration<'src> {
    /// The metadata for this node.
    pub meta: AstNodeMeta<'src>,
    /// The class's visibility.
    pub vis: Visibility<'src>,
    /// The class's name.
    pub name: Identifier<'src>,
    /// Generic types that this class declares.
    pub generic_type_arguments: Vec<GenericTypeArg<'src>>,
    /// Generic constants that this class declares.
    pub generic_const_arguments: Vec<GenericConstArg<'src>>,
    /// The fields of the class.
    pub fields: Vec<ClassField<'src>>,
}

/// A class field declaration in a class declaration.
#[derive(Debug)]
pub struct ClassField<'src> {
    /// AST Node metadata.
    pub meta: AstNodeMeta<'src>,
    /// Visibility of this class field.
    pub vis: Visibility<'src>,
    /// Is this class field mutable by default or is it set-once.
    /// This is based on whether `const` is specified before the field name.
    pub mutable: bool,
    /// The type of the field.
    pub ty: TypeInstantiation<'src>,
}
