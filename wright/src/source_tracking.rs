//! Types and traits for tracking source code fed to the wright compiler.

use filename::FileName;
use immutable_string::ImmutableString;
use source::SourceId;
use dashmap::DashMap;
use self::source::Source;
use std::sync::Arc;

#[cfg(feature = "reporting")]
use codespan_reporting::files::Error;

pub mod filename;
pub mod fragment;
pub mod immutable_string;
pub mod source;
// pub mod source_ref;

/// A reference to a [Source] in a [SourceMap].
pub type SourceRef = Arc<Source>;

/// Storage for [Source]s used and referenced in compiling a wright project.
#[derive(Debug, Default)]
pub struct SourceMap {
    /// Internally, we use [DashMap] for a concurrent hashmap from [Source::id]s to their [Arc]'d 
    ///
    /// Each source is wrapped in an [Arc] to make them all accessible without holding a reference to this map 
    /// directly. 
    inner: DashMap<SourceId, SourceRef>,
}

impl SourceMap {
    /// Construct a new empty [SourceMap].
    pub fn new() -> Self {
        Default::default()
    }

    /// Add a [Source] to this [SourceMap] and get a [SourceRef] to it after it's added.
    pub fn add(&self, source: Source) -> SourceRef {
        // Put the source in an Arc.
        let source: SourceRef = Arc::new(source);
        // Push the souce to the internal Vec.
        self.inner.insert(source.id, Arc::clone(&source));
        // Return the now-Arc'd source.
        source
    }


    /// Get a reference to a [Source] stored in this [SourceMap] using it's [Source::id].
    /// 
    /// This is currently `O(1)` since [SourceMap] uses a [DashMap] internally. 
    /// 
    /// Returns [None] if the [Source] with the given [Source::id] is not in this [SourceMap].
    pub fn get(&self, id: SourceId) -> Option<SourceRef> {
        self.inner
            .get(&id)
            .map(|source| Arc::clone(&source))
    }
}

#[cfg(feature = "reporting")]
impl<'f> codespan_reporting::files::Files<'f> for SourceMap {
    type FileId = SourceId; 

    type Name = FileName;

    type Source = ImmutableString;

    fn name(&'f self, id: Self::FileId) -> Result<Self::Name, codespan_reporting::files::Error> {
        self.get(id)
            .map(|source| source.name().clone())
            .ok_or(Error::FileMissing)
    }

    fn source(&'f self, id: Self::FileId) -> Result<Self::Source, codespan_reporting::files::Error> {
        self.get(id)
            .map(|source| source.source().clone())
            .ok_or(Error::FileMissing)
    }

    fn line_index(&'f self, id: Self::FileId, byte_index: usize) -> Result<usize, codespan_reporting::files::Error> {
        Ok(self.get(id).ok_or(Error::FileMissing)?.line_index(byte_index))
            
    }

    fn line_range(&'f self, id: Self::FileId, line_index: usize) -> Result<std::ops::Range<usize>, codespan_reporting::files::Error> {
        Ok(self.get(id).ok_or(Error::FileMissing)?.get_line(line_index).range)
    }
}
