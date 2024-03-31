//! Structure and implementation relating to the representation of source files (as immutable strings) throughout 
//! the Wright compiler and tooling. 

use std::{fs::File, io, sync::Arc};
use memmap2::Mmap;
use fs4::FileExt;

/// An immutable string that either
/// - References a source string in memory using an `&'src` reference,
/// - Owns a source string in memory.
/// - Owns a locked and memory mapped file from the disk.
#[derive(Debug, Clone)]
pub struct ImmutableString<'src> {
    /// Wrap the internal representation in an [Arc] to make this easy to clone and pass around. 
    inner: Arc<ImmutableStringInner<'src>>
}

impl<'src> AsRef<str> for ImmutableString<'src> {
    fn as_ref(&self) -> &str {
        self.inner.as_ref().as_ref()
    }
}

impl<'src> ImmutableString<'src> {
    /// Create a new [ImmutableString] holding the given [File] (assumed to be locked with [fs4]) 
    /// and the [Mmap] mapping that file to memory. 
    /// 
    /// This function requires that the memory mapped by the given 
    /// [Mmap] is valid UTF-8 using [std::str::from_utf8].
    pub(super) fn new_locked_file(file: File, mem_map: Mmap) -> Self {
        ImmutableString { 
            inner: Arc::new(ImmutableStringInner::LockedFile { 
                locked_file: file, 
                mem_map 
            }) 
        }
    }

    /// Create a new [ImmutableString] that owns a string allocated on the heap. 
    pub(super) fn new_owned(boxed_str: Box<str>) -> Self {
        ImmutableString {
            inner: Arc::new(ImmutableStringInner::Owned(boxed_str))
        }
    }

    /// Create a new [ImmutableString] referencing a string directly. 
    pub(super) fn new_reference(str_ref: &'src str) -> Self {
        ImmutableString {
            inner: Arc::new(ImmutableStringInner::Reference(str_ref))
        }
    }
}

/// The internal enum representation of the immutable string. 
#[derive(Debug)]
enum ImmutableStringInner<'src> {
    /// An immutable reference to an existing string.
    Reference(&'src str),

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
impl<'src> Drop for ImmutableStringInner<'src> {
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
            ImmutableStringInner::Owned(_) | ImmutableStringInner::Reference(_) => {}
        }
    }
}

impl<'src> AsRef<str> for ImmutableStringInner<'src> {
    fn as_ref(&self) -> &str {
        match self {
            ImmutableStringInner::Reference(str) => str,
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

