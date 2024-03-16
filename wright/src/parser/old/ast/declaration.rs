//! AST Node structure for declarations in a Wright source file.

pub mod class;
pub mod r#enum;
pub mod function;
pub mod generics;
pub mod import;
pub mod module;
pub mod r#type;
pub mod union;
pub mod visibility;
pub mod where_clause;

use self::visibility::Visibility;
use super::{identifier::Identifier, metadata::AstNodeMeta};

/// A top-level declaration in source code.
#[derive(Debug)]
pub enum Declaration<'src> {
    Module(module::ModuldeDeclaration<'src>),
    Import(import::ImportDeclaration<'src>),
    Class(class::ClassDeclaration<'src>),
    Union(union::UnionDeclaration<'src>),
    Type(r#type::TypeDeclaration<'src>),
    Enum(r#enum::EnumDeclaration<'src>),

    Function,
    Trait,
    Implementation,
    Struct,
    Record,
    Constraint,
}

/// A struct declaration in source code.
#[derive(Debug)]
pub struct StructDeclaration<'src> {
    /// The metadata for this node.
    pub meta: AstNodeMeta<'src>,
    /// The struct's visibility.
    pub vis: Visibility<'src>,
    /// The struct's name.
    pub name: Identifier<'src>,
}
