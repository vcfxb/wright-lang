//! Import declaration.

use crate::parser::ast::{identifier::Identifier, metadata::AstNodeMeta, path::Path};

use super::visibility::Visibility;

/// The three different flavors that an import can be.
#[derive(Debug)]
pub enum ImportOptions<'src> {
    /// The import is aliased to a new name in the current file/scope.
    Aliased(Identifier<'src>),
    /// The import ends with `::*` indicating that all items in the imported path are available in scope.
    Glob,
    /// The import is neither aliased or blobbed.
    None,
}

/// An import in source code. These can be aliased using the `as <alias>;` suffix.
/// Glob imports ending with `::*;` are supported without aliasing.
#[derive(Debug)]
pub struct ImportDeclaration<'src> {
    /// The metadata for this node.
    pub meta: AstNodeMeta<'src>,
    /// The visibility of the import, used for re-exporting.
    pub vis: Visibility<'src>,
    /// The path being imported. This could theoretically be a single identifier.
    pub path: Path<'src>,
    /// Any changes to the import (aliasing or glob import).
    pub opts: ImportOptions<'src>,
}
