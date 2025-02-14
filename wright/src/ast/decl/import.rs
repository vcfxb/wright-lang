//! Import declarations.
//! 
//! These are similar to the rust `use ...;` style delcarations however with the caveat that wright currently 
//! only supports a single path in declarations, rather than a tree of items with curly braces. (we also don't support
//! starting with a `::` prefix yet).

use crate::{ast::path::Path, source_tracking::fragment::Fragment};

/// A `use item::from::elsewhere;` declaration in a wright source file.
#[derive(Debug)]
pub struct ImportDecl {
    /// The full matching source of the declaration, whitespace and all.
    pub matching_source: Fragment,

    /// The item being imported.
    pub imported_item: Path,
}
