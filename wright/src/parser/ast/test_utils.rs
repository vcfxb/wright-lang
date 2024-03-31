//! Test utilities for checking AST generation code.
//! The functions in this module are only available when running cargo test. 

use crate::filemap::{FileId, FileMap, FileName};
use super::{AstGeneratorContext, AstNode};

/// Run a parser against a given test string of source code.
pub fn test_parser<'file_map, 'src: 'file_map, T: AstNode<'file_map, 'src>>(test_src: &'src str) -> Result<T, T::Error>
where T: Sized + 'src
{
    // Construct a new file map.
    let mut file_map: FileMap = FileMap::new();

    // Add the test string to create a file ID. 
    let file_id: FileId = file_map.add_str_ref(FileName::None, test_src);

    // Create the ast generator context. 
    let mut ctx: AstGeneratorContext = AstGeneratorContext::new(file_id, &file_map);

    T::try_parse(&mut ctx)
}
