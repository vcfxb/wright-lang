//! A fully qualified sympbol path in wright source code.
//!
//! Path items are separated using `::` similar to rust.

use std::borrow::Cow;

use super::{identifier::Identifier, metadata::AstNodeMeta};

/// A double-colon seperated path to a module, type, or function in Wright source code.
///
/// Note that this can be only a single identifier in length, signaling a path/identifier that's in current scope.
#[derive(Debug)]
pub struct Path<'src> {
    /// Node metadata.
    pub meta: AstNodeMeta<'src>,
    /// The first part of the path, read left to right.
    pub head: Identifier<'src>,
    /// The rest of the path.
    pub tail: Option<Box<Path<'src>>>,
}
