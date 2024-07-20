//! [Path]s are used in import statements, and can take the place of an [Identifier] in many people.

use super::identifier::Identifier;
use crate::source_tracking::fragment::Fragment;

/// A double-colon separated path/reference to a module/function. This can be used in an `import` declaration and
/// some other places. [Path]s with length of 1 are just [Identifier]s -- [Identifier]s can be considered paths in some
/// instances.
#[derive(Debug, Clone)]
pub struct Path {
    /// The [Fragment] of source code containing the full source of this path (including the double-colon separators).
    pub full_path: Fragment,

    /// The first (left-most) identifier in this [Path]. This can also be considered the "root" of the path --
    /// the module that the following item/identifier can be found in.
    pub head: Identifier,

    /// The rest of the [Path], following the first separator.
    pub tail: Vec<Identifier>,
}
