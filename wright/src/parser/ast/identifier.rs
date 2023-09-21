//! Identifiers in wright source code.

use crate::filemap::FileId;

/// An identifier in the source code being parsed.
///
/// This does not use the traditional metadata struct, since there's no additional node data beyond that from the
/// source code.
#[derive(Debug, Clone, Copy)]
pub struct Identifier<'src> {
    /// The file handle for the file this token is in.
    pub file_id: FileId,
    /// The byte index of the matching string in the source file.
    pub index: usize,
    /// The identifier string itself. The byte length in source code
    /// is carried in the metadata.
    pub inner: &'src str,
}
