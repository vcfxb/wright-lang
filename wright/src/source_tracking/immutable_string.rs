//! Structure and implementation relating to the representation of source files (as immutable strings) throughout
//! the Wright compiler and tooling.

use std::{str::CharIndices, sync::Arc};

#[cfg(feature = "file_memmap")]
use fs4::fs_std::FileExt;

#[cfg(feature = "file_memmap")]
use memmap2::Mmap;

#[cfg(feature = "file_memmap")]
use std::{fs::File, io};

/// An immutable string that either
/// - References a source string in memory using a `'static` reference,
/// - Owns a source string in memory.
/// - Owns a locked and memory mapped file from the disk.
///
/// This uses an [Arc] internally to make cloning cheap.
#[derive(Debug, Clone)]
pub struct ImmutableString {
    /// Wrap the internal enum representation. This is to avoid exposing the API for a user to construct an
    /// [ImmutableStringInner] without satisfying certain invariants.
    inner: Arc<ImmutableStringInner>,
}

impl ImmutableString {
    /// Wrap the inner representation in this type.
    #[inline]
    fn from_inner(inner: ImmutableStringInner) -> Self {
        ImmutableString {
            inner: Arc::new(inner),
        }
    }

    /// Create a new [ImmutableString] holding the given [File] (assumed to be locked with [fs4])
    /// and the [Mmap] mapping that file to memory.
    ///
    /// This function requires that the memory mapped by the given
    /// [Mmap] is valid UTF-8 using [std::str::from_utf8].
    #[cfg(feature = "file_memmap")]
    pub(super) fn new_locked_file(file: File, mem_map: Mmap) -> Self {
        Self::from_inner(ImmutableStringInner::LockedFile {
            locked_file: file,
            mem_map,
        })
    }

    /// Create a new [ImmutableString] that owns a string allocated on the heap.
    pub(super) fn new_owned(boxed_str: Box<str>) -> Self {
        Self::from_inner(ImmutableStringInner::Owned(boxed_str))
    }

    /// Create a new [ImmutableString] referencing a string directly.
    pub(super) fn new_static(str_ref: &'static str) -> Self {
        Self::from_inner(ImmutableStringInner::Static(str_ref))
    }

    /// Get a list of byte indices into this [ImmutableString] of the start of every line.
    pub fn line_starts(&self) -> impl Iterator<Item = usize> + use<'_> {
        // Make a iterator over this string's characters and their byte indices.
        let mut char_indices: CharIndices = self.as_ref().char_indices();
        // Track whether the previous character was a newline using a bool -- this starts as true, so that the first
        // character of a source is considered to be starting a newline.
        let mut last_was_newline: bool = true;

        // Create a custom iterator that flattens to give us indices immediately following \n characters.
        let iter = std::iter::from_fn(move || {
            // If the next char indice is none, return none. There are no lines on empty strings.
            let (index, next) = char_indices.next()?;

            // Determine whether to list this character's index as starting a new line.
            let result = Some(last_was_newline.then_some(index));

            // Update the boolean based on the consumed character.
            last_was_newline = next == '\n';

            // Return the above result.
            result
        });

        iter.flatten()
    }

    /// Get this [ImmutableString] as a [str] reference.
    /// This just calls [AsRef::as_ref].
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }

    /// Get the length of this [ImmutableString] in bytes.
    /// See [str::len].
    pub fn len(&self) -> usize {
        self.as_str().len()
    }

    /// Check if this [ImmutableString] is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl AsRef<str> for ImmutableString {
    fn as_ref(&self) -> &str {
        (*self.inner).as_ref()
    }
}

/// The internal enum representation of the immutable string.
#[derive(Debug)]
enum ImmutableStringInner {
    /// An immutable reference to an existing static string.
    Static(&'static str),

    /// An owned immutable string.
    Owned(Box<str>),

    /// A locked, memory mapped file from the disk.
    #[cfg(feature = "file_memmap")]
    LockedFile {
        /// The locked file that gets unlocked when this struct is dropped.
        locked_file: File,

        /// The memory mapped file.
        ///
        /// # Safety
        /// - Undefined  behavior occurs if the file on disk is modified while memory mapped. Always lock the
        ///     file (in this crate's case, using [fs4]) before creating this [Mmap] for it.
        ///     See [Mmap] for more details.
        /// - This struct assumes that any memory-mapped files have their UTF-8 validity checked by the caller.
        ///     Specificically the [ImmutableString::as_ref] method relies on [std::str::from_utf8_unchecked],
        ///     so if you do not ensure the [Mmap] is valid UTF-8, you will run into undefined behavior.
        mem_map: Mmap,
    },
}

/// Implement [Drop] to make sure that the files from disk get unlocked as they go out of scope/use.
#[cfg(feature = "file_memmap")]
impl Drop for ImmutableStringInner {
    fn drop(&mut self) {
        match self {
            // Unlock locked files.
            ImmutableStringInner::LockedFile { locked_file, .. } => {
                FileExt::unlock(locked_file)
                    // Log the error if there is one,
                    .map_err(|io_err: io::Error| eprintln!("{}", io_err))
                    // Discard value of result
                    .ok();
            }

            // All other types drop trivially.
            ImmutableStringInner::Owned(_) | ImmutableStringInner::Static(_) => {}
        }
    }
}

impl AsRef<str> for ImmutableStringInner {
    fn as_ref(&self) -> &str {
        match self {
            ImmutableStringInner::Static(str) => str,
            ImmutableStringInner::Owned(str) => str,

            #[cfg(feature = "file_memmap")]
            ImmutableStringInner::LockedFile { mem_map, .. } => {
                // Get a direct reference to the data that is in the memory map.
                let raw_data: &[u8] = mem_map.as_ref();
                // SAFETY: UTF-8 validity is checked when the file is added to the file map, or by the API consumer.
                unsafe { std::str::from_utf8_unchecked(raw_data) }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ImmutableString;

    #[test]
    fn test_line_starts() {
        let v: Vec<usize> = ImmutableString::new_static("a\n\nb\nc").line_starts().collect();

        assert_eq!(v.as_slice(), &[0, 2, 3, 5]);
    }
}
