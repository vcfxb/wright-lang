//! Types and traits for tracking source code fed to the wright compiler.

use std::sync::{Arc, RwLock};
use self::source::Source;

pub mod immutable_string;
pub mod fragment;
pub mod filename;
pub mod source;

/// Storage for a list of [Source]s used and reference in compiling a wright project. 
#[derive(Debug)]
pub struct SourceMap {
    /// Internally, we use an [Arc] [RwLock] [Vec] to create a concurrent mutable list.
    /// 
    /// Each source is wrapped in an [Arc] to make them all accessible without needing to use [RwLock::read].
    inner: Arc<RwLock<Vec<Arc<Source>>>>
}

/// A reference to a [Source] in a [SourceMap].
pub type SourceRef = Arc<Source>;

impl SourceMap {
    /// Construct a new empty [SourceMap]. 
    pub fn new() -> Self {
        SourceMap { inner: Arc::new(RwLock::new(Vec::new())) }
    }

    /// Add a [Source] to this [SourceMap] and get a [SourceRef] to it after it's added. 
    pub fn add(&self, source: Source) -> SourceRef {
        // Put the source in an Arc.
        let source: Arc<Source> = Arc::new(source);

        // Get a write guard to the internal list. 
        let mut write_guard = self.inner
            .write()
            .expect("Should be able to acquire write guard");

        // Push the souce to the internal Vec.
        write_guard.push(Arc::clone(&source));
        // Drop the write guard -- make sure other functions can access this source map.
        drop(write_guard);
        // Return the now-Arc'd source. 
        source
    }
}

impl Default for SourceMap {
    fn default() -> Self {
        SourceMap::new()
    }
}
