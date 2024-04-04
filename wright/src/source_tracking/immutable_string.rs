//! Structure and implementation relating to the representation of source files (as immutable strings) throughout 
//! the Wright compiler and tooling. 

use std::{fs::File, io, str::CharIndices};
use memmap2::Mmap;
use fs4::FileExt;

/// An immutable string that either
/// - References a source string in memory using a `'static` reference,
/// - Owns a source string in memory.
/// - Owns a locked and memory mapped file from the disk.
#[derive(Debug)]
pub struct ImmutableString {
    /// Wrap the internal enum representation. This is to avoid exposing the API for a user to construct an
    /// [ImmutableStringInner] without satisfying certain invariants. 
    inner: ImmutableStringInner
}

impl AsRef<str> for ImmutableString {
    fn as_ref(&self) -> &str {
        self.inner.as_ref()
    }
}

impl ImmutableString {
    /// Wrap the inner representation in this type. 
    #[inline]
    fn from_inner(inner: ImmutableStringInner) -> Self {
        ImmutableString {
            inner
        }
    }

    /// Create a new [ImmutableString] holding the given [File] (assumed to be locked with [fs4]) 
    /// and the [Mmap] mapping that file to memory. 
    /// 
    /// This function requires that the memory mapped by the given 
    /// [Mmap] is valid UTF-8 using [std::str::from_utf8].
    pub(super) fn new_locked_file(file: File, mem_map: Mmap) -> Self {
        Self::from_inner(ImmutableStringInner::LockedFile { 
            locked_file: file, 
            mem_map 
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

    /// Get an iterator over the byte indices into this string of the start of every line.  
    pub fn line_starts(&self) -> Vec<usize> {
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
            let result = if last_was_newline {
                Some(Some(index))
            } else {
                Some(None)
            };

            // Update the boolean based on the consumed character.
            last_was_newline = next == '\n';

            // Return the above result. 
            result
        });

        iter.flatten().collect()
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
impl Drop for ImmutableStringInner {
    fn drop(&mut self) {
        match self {
            // Unlock locked files.
            ImmutableStringInner::LockedFile { locked_file, .. } => {
                locked_file
                    .unlock()
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
        let v: Vec<usize> = ImmutableString::new_static("a\n\nb\nc").line_starts();

        assert_eq!(v.as_slice(), &[0, 2, 3, 5]);
    }
}
