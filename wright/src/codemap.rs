//! Module to track a map of the source code. 

use std::{collections::HashMap, path::PathBuf};

/// Map of full source code, including perhaps multiple files
pub struct CodeMap {
    /// The files inclduded in this code map.
    files: HashMap<PathBuf, FileMap>,
}

impl CodeMap {
    /// Constrcut a new empty code map.
    pub fn new() -> Self {
        Self { files: HashMap::new() }
    }
}

/// Map of source code contained in a single file. 
pub struct FileMap {
    /// An owned instance of the file's full source code. 
    full_source: String,
    /// An ordered list of byte indices into `full_source` that track the first byte index (not char index)
    /// of the start of each line. These should always be preceded by the start of the file or a '\n' character. 
    new_line_indices: Vec<usize>,
}

impl FileMap {
    /// Create a new file map.
    pub fn new(source: String) -> Self {
        // Check if source string is empty -- return empty file map if so. 
        if source.is_empty() {
            return Self {
                full_source: source,
                new_line_indices: Vec::new()
            }
        }

        // Avoid reallocating the new line index vactor by getting a decent count of the number of newlines in the 
        // string to begin with. 
        let line_iterator = source.as_str().lines();
        let line_count_size_hint = line_iterator.size_hint();
        let line_count_estimate = line_count_size_hint.1.unwrap_or(line_count_size_hint.0);
        let mut new_line_indices: Vec<usize> = Vec::with_capacity(line_count_estimate);

        // Push the begining of the file, since the first line of the file starts at the begining of the file. 
        new_line_indices.push(0);
        
        // Iterate through the source code as bytes and push a new index just past each newline character (0x0A).
        let bytes = source.as_bytes();
        for index in 0..bytes.len() {
            // Check if this byte is a newline character and the next index is valid. 
            if bytes[index] == 0x0A && index+1 < bytes.len() {
                new_line_indices.push(index+1);
            }
        }
        
        return Self { full_source: source, new_line_indices };
    }
}
