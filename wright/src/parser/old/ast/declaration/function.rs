//! Structures representing function declarations in wright source code.

use crate::parser::ast::{
    declaration::generics::{GenericConstArg, GenericTypeArg},
    declaration::visibility::Visibility,
    expression::block::Block,
    identifier::Identifier,
    metadata::AstNodeMeta,
    types::TypeInstantiation,
};

use super::where_clause::WhereClause;

/// A function declaration in source code.
#[derive(Debug)]
pub struct FunctionDeclaration<'src> {
    /// The metadata about this AST node.
    pub meta: AstNodeMeta<'src>,
    /// The visibility of this function declaration.
    pub vis: Visibility<'src>,
    /// Is the function marked as `dyn` (not to be used in comile-time expressions).
    pub is_dynamic: bool,
    /// The name of the function.
    pub name: Identifier<'src>,
    /// Any generic type arguments that the function uses.
    pub generic_type_args: Vec<GenericTypeArg<'src>>,
    /// Any generic constant arguments that the function uses.
    pub generic_constant_args: Vec<GenericConstArg<'src>>,
    /// Arguments declared for this function.
    pub args: Vec<FunctionArg<'src>>,
    /// The return type declared for this function.
    pub return_type: Option<TypeInstantiation<'src>>,
    /// Optional clause to define bounds on the generics declared in this function.
    pub where_clause: Option<WhereClause<'src>>,
    /// The function body.
    pub body: Block<'src>,
}

/// A function argument in wright source code.
#[derive(Debug)]
pub struct FunctionArg<'src> {
    /// Metadata about this AST node.
    pub meta: AstNodeMeta<'src>,
    /// The name of this argument.
    pub name: Identifier<'src>,
    /// The type of this argument.
    pub ty: TypeInstantiation<'src>,
}
