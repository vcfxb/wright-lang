//! Module declaration.

use crate::parser::ast::{
    declaration::visibility::Visibility, identifier::Identifier, metadata::AstNodeMeta,
};

/// A module declaration in Wright source.
///
/// These are always in the form `[vis] mod <name>;`.
#[derive(Debug)]
pub struct ModuldeDeclaration<'src> {
    /// The metadata about this node.
    pub meta: AstNodeMeta<'src>,
    /// The visibility of this module.
    pub vis: Visibility<'src>,
    /// The name of this module (which will be searched for in the file system at the time of module resolution).
    pub name: Identifier<'src>,
}
