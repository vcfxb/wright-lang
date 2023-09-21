//! A node in the AST representing the visibility of a declaration.

use crate::parser::ast::metadata::AstNodeMeta;

/// The possible visibilities of a declaration in Wright.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum VisibilityTy {
    /// Externally public.
    Public,
    /// Package private.
    Package,
    /// Module/file private. This is default.
    Private,
}

/// A visibility modifier in wright source.
#[derive(Clone, Debug)]
pub struct Visibility<'src> {
    /// Node metadata
    pub meta: AstNodeMeta<'src>,
    /// Which visibility is represented.
    pub variant: VisibilityTy,
}
