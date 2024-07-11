//! Types and functions for handling references to [Source]s -- this is done frequently, since [Source]s themselves
//! can be expensive to clone and pass around.

use super::{fragment::Fragment, source::Source};
use std::{ops::Deref, sync::Arc};

#[cfg(doc)]
use super::SourceMap;

/// A reference to a [Source] in a [SourceMap].
///
/// This is cheap to [Clone] since it uses an [Arc] internally.
///
/// Equality on this struct is checked using [Arc::ptr_eq] -- this cannot be used for checking if
/// two [Source]s contain identical content.
#[derive(Debug)]
pub struct SourceRef(pub Arc<Source>);

impl SourceRef {
    /// See [Source::get_line].
    ///
    /// This is a convenience function to unwrap/pass through the reciever type where [Deref] might not automatically.
    pub fn get_line(&self, line_index: usize) -> Fragment {
        Source::get_line(self.0.clone(), line_index)
    }
}
