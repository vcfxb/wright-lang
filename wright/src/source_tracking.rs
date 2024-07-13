//! Types and traits for tracking source code fed to the wright compiler.

use self::source::Source;
use dashmap::DashMap;
use source::SourceId;
use std::sync::Arc;

pub mod filename;
pub mod fragment;
pub mod immutable_string;
pub mod source;

/// A reference to a [Source] in a [SourceMap].
pub type SourceRef = Arc<Source>;

/// Storage for [Source]s used and referenced in compiling a wright project.
///
/// [Clone]ing is cheap, since this uses an [Arc] internally.
#[derive(Debug, Default, Clone)]
pub struct SourceMap {
    /// Internally, we use [DashMap] for a concurrent hashmap from [Source::id]s to their [Arc]'d
    ///
    /// Each source is wrapped in an [Arc] to make them all accessible without holding a reference to this map
    /// directly.
    inner: Arc<DashMap<SourceId, SourceRef>>,
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
        self.inner.get(&id).map(|source| Arc::clone(&source))
    }
}

