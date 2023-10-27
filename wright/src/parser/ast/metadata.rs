//! Metadata used to track the source code that produces nodes in the AST.

use crate::filemap::{FileId, FileMap};
use codespan_reporting::files::{Files, Location};

/// The metadata used for determining where in the source code the given node is and what source was parsed to produce
/// it.
#[derive(Debug, Clone, Copy)]
pub struct AstNodeMeta<'src> {
    /// Reference to the file map that this node was parsed on.
    pub file_map: &'src FileMap,
    /// The file id for the file this node is in.
    pub file_id: FileId,
    /// The byte index of the matching string in the source file.
    ///
    /// This may be equal to the index of the next token if the matching source for an AST node is empty, which is
    /// the case in some rare circumstances (i.e. default visibility ommitted).
    pub index: usize,
    /// The matching source code for this node. This carries the byte length in source in it's metadata.
    pub matching_source: &'src str,
}

impl<'src> AstNodeMeta<'src> {
    /// Get the starting location of this AST node.
    pub fn start(&self) -> Location {
        self.file_map
            .location(self.file_id, self.index)
            .expect("AST Node metadata is valid")
    }
}
