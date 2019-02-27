//! Utilities for tracking locations and spans of locations in source code
//! as well as printing user information based on said locations.

use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;
use std::sync::Weak;
use std::process::exit;

pub mod codemap_report;
pub mod charspan;
pub mod sourcemap;

use self::charspan::*;
use self::sourcemap::*;

/// A `CodeMap` is a structure used for tracking all source code loaded into memory.
///
/// This module was significantly influenced by the
/// [codespan crate](https://github.com/brendanzab/codespan).
///
#[derive(Clone)]
pub struct CodeMap {
    /// `sources` contains all of the source code loaded into the code map.
    ///
    /// These sources will always be sorted in the order which they were loaded.
    /// This means that their `CharSpan`s will always be in consecutive order.
    ///
    /// The sources are each stored behind `Arc`s so that they can be accessed
    /// in their own threads for parsing and other manipulations.
    pub sources: Vec<Arc<Source>>,
    /// Number of characters loaded.
    chars: u64,
    verbose: bool,
}

impl Default for CodeMap {
    fn default() -> Self {Self::new(false)}
}

impl CodeMap {
    /// Construct a new `CodeMap`.
    pub fn new(verbose: bool) -> Self {CodeMap {
        sources: Vec::new(),
        chars: 0,
        verbose
    }}
    
    fn get_next_char_index(&self) -> CharIndex {
        if let Some(n) = self.sources.last() {n.span.end}
        else {1}
    }

    /// Add a file to the code map from the disk.
    /// This function will open and read the file, printing any errors to the
    /// standard error, and exiting with status code 1 if the file cannot
    /// be opened or read properly. (The file must exist and be valid UTF-8, etc.)
    pub fn add_real(&mut self, name: PathBuf) {
        match File::open(name.clone()) {
            Ok(mut f) => {
                let mut s = String::new();
                match f.read_to_string(&mut s) {
                    Ok(_) => {
                        let chars: Vec<char> = s.chars().collect();
                        let len = chars.len() as u32;
                        self.chars += chars.len() as u64;
                        let arc = Arc::new(Source {
                            name: SourceType::Real(name.clone()),
                            content: chars,
                            span: CharSpan::new(
                                self.get_next_char_index(),
                                self.get_next_char_index()+len),
                            line_index: vec![]
                        }.generate_indexes());
                        self.sources.push(arc);
                        if self.verbose {
                            eprintln!("Loaded {}. {}/{} characters used.",
                                      name.display(),
                                      self.chars,
                                      std::u32::MAX);
                        }
                    },
                    Err(err) => {
                        eprintln!("Could not read {}: {}", name.display(), err);
                        exit(1)
                    },
                }
            },
            Err(err) => {
                eprintln!("Could not open {}: {}", name.display(), err);
                exit(1)
            },
        }
    }
    /// Add a virtual source to the code map.
    pub fn add_virtual(&mut self, name: impl Into<String>, content: impl Into<String>) {
        let name: String = name.into();
        let content: String = content.into();
        let chars: Vec<char> = content.chars().collect();
        let len = chars.len() as u32;
        self.chars += chars.len() as u64;
        let arc = Arc::new(Source {
            name: SourceType::Virtual(name.clone()),
            content: chars,
            span: CharSpan::new(self.get_next_char_index(), self.get_next_char_index()+len),
            line_index: vec![]
        }.generate_indexes());
        self.sources.push(arc);
        if self.verbose {eprintln!("Loaded virtual source {}. {}/{} characters used.", name, self.chars, std::u32::MAX);}
    }

    /// Find the source associated with a given `CharIndex` via binary search.
    /// (this operation is O(log n), where n is the number of sources loaded)
    /// If index is out of bounds, or there are no sources, None is returned.
    pub fn get_source(&self, index: CharIndex) -> Option<Weak<Source>> {
        let index: CharIndex = index.into();
        if index >= self.get_next_char_index() {None}
        else if index == 0 {None}
        else {
            // this code looks gross, but there's not much i can do about it
            Some(Arc::downgrade(&self.sources[self.sources.as_slice()
                .binary_search_by(|file: &Arc<Source>| {
                    //dbg!(file.span.partial_cmp(&index));
                    file.span.partial_cmp(&index).unwrap()
                }).expect("Binary search returned error")]))
        }
    }

}
