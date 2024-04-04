//! Responsible for keeping track of different files added to the Wright build system.

use crate::parser::fragment::Fragment;
use codespan_reporting::{
    diagnostic::Diagnostic,
    files::{Files, SimpleFile},
    term::Config,
};
use fs4::FileExt;
use memmap2::Mmap;
use std::{fs::File, io, path::PathBuf, pin::Pin, sync::{mpsc, Arc, RwLock}, thread, time::Duration};
use termcolor::{ColorChoice, StandardStream};
use self::{filename::FileName, immutable_string::ImmutableString};

/// Rename import for clarity.
use codespan_reporting::files::Error as CodespanError;

pub mod filename;
pub mod immutable_string;

/// Convenience type alias. Used for types returned from/with [codespan_reporting]. 
pub type CodespanResult<T> = Result<T, CodespanError>;


/// The file map that we use throughout the rest of this crate.
/// 
/// This uses an [Arc]'d [RwLock]'d [Vec] of [Arc]s internally for concurrent access across multiple threads, 
/// so feel free to clone this around since it's really just cloning an [Arc] (should be cheap). 
#[derive(Clone, Debug)]
pub struct FileMap {
    /// This list of files we're keeping track of.
    /// This is similar to the current implementation of [codespan_reporting::files::SimpleFiles],
    /// but we don't use theirs for various reasons. 
    /// 
    /// Having our own implementation like this allows us to:
    /// - Read from files in this map in other threads while new files are being added to this map,
    ///     (thanks to using an [Arc] on each file and [Arc]'d [RwLock] on the whole list). 
    /// - Iterate over files stored in this [FileMap] ([codespan_reporting] does not provide a way to do this). 
    /// - Wrap [`ImmutableString`] in an [`Arc`] to make cloning references to one easy/thread-safe. 
    inner: Arc<RwLock<Vec<Arc<SimpleFile<FileName, Arc<ImmutableString>>>>>>,
}

/// File identifier used to refer to files.
/// 
/// [FileId]s should never be invalid against the [FileMap] that created them, since there is no way to 
/// remove a file from a [FileMap]. 
/// 
/// Internally, these are just indices into the [Vec] inside a [FileMap]. 
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct FileId(usize);

impl FileMap {
    /// Construct a new empty [FileMap].
    pub fn new() -> Self {
        FileMap { inner: Arc::new(RwLock::new(Vec::new())) }
    }

    /// Get an [Arc] reference to a file from the internal [Vec] or return a [`CodespanError::FileMissing`] error.
    fn get(&self, file_id: FileId) -> CodespanResult<Arc<SimpleFile<FileName, ImmutableString>>> {
        // Unwrap the index in the file id.
        let FileId(index) = file_id;
        
        // Get a read-guard to to the internal files vec.
        let read_guard = self.inner
            .read()
            .expect("FileMap lock should not be poisoned");

        // Try to get the file arc, if the index is valid. 
        let file_arc_clone: Option<Arc<_>> = read_guard
            .get(index)
            // Clone the Arc so we can drop the read-guard. 
            .cloned();

        // Drop the read guard -- free this up for writes if needed. 
        drop(read_guard);

        // Return the Arc or a FileMissing error. 
        file_arc_clone.ok_or(CodespanError::FileMissing)
    }

    /// Internal function to add a file to the [Vec]. Public facing functions will need to do some conversion
    /// and then call this.
    /// 
    /// This writes to the internal vector and will block indefinitely if any other function 
    /// is currently reading from this [FileMap].
    fn add(&mut self, name: FileName, source: ImmutableString) -> FileId {
        // Get a write guard to the internal files vec.
        let mut write_guard = self.inner
            .write()
            .expect("FileMap lock should not be poisoned");

        // The file id is just the next index in the vec.
        let file_index: usize = write_guard.len();
        // Push the new file onto the vec. 
        write_guard.push(Arc::new(SimpleFile::new(name, source)));
        // Drop the write guard so that other functions/threads can read from it.
        drop(write_guard);

        // Return the file index as a file id. 
        FileId(file_index)
    }


    /// Add a file from the file system. This file will be opened with read permissions, locked (using [fs4]), 
    /// memory mapped (using [memmap2]), and then added to the file map. 
    /// This file will remain locked and memory-mapped until this [FileMap] is [drop]ped. 
    /// See [ImmutableString::drop] for more details. 
    /// 
    /// The [FileName] in this map will be [FileName::Real] with the [PathBuf] passed to this function.
    pub fn add_file(&mut self, path: PathBuf) -> io::Result<FileId> {
    }

    /// Find the file ID of a given [Fragment] using the fragment's internal pointer.
    pub fn find_fragment(&'src self, fragment: &Fragment<'src>) -> Option<FileId> {
        // Get a read lock on this file map to iterate accross file indices. 
        // This read lock will be dropped when the function returns. 
        let read_lock = self.inner
            .read()
            .expect("FileMap lock should not be poisoned");
        
        // Iterate on file IDs.
        for file_index in 0..read_lock.len() {
            // Use unwrap_unchecked here because all these file indicies should be valid, 
            // and the read_lock prevents more from being added to the FileMap mid loop. 
            let pin_arc_file: &Pin<Arc<_>> = unsafe { read_lock.get(file_index).unwrap_unchecked() };

            // Pin project the file to the source fragment. 
            let source = Pin::

            // Use Fragment::contains to check this. 
            if (Fragment { inner: source.as_ref() }).contains(fragment) {
                return Some(FileId(file_index));
            }
        }

        // If there was no file containing the given fragment, return none.
        None
    }
}


/// The implementation here is basically identical to the one for [codespan_reporting::files::SimpleFiles].
impl<'file_map, 'src: 'file_map> Files<'file_map> for FileMap {
    /// See [FileId] for more info. 
    type FileId = FileId;

    type Name = FileName;

    type Source = ImmutableString;

    fn name(&self, id: Self::FileId) -> CodespanResult<Self::Name> {
        Ok(self.get(id)?.name().clone())
    }

    fn source(&'file_map self, id: Self::FileId) -> CodespanResult<Self::Source> {
        // Clone is cheap/available here now that ImmutableStrings use Arc inside. 
        Ok(self.get(id)?.source().clone())
    }

    fn line_index(&'file_map self, id: Self::FileId, byte_index: usize) -> CodespanResult<usize> {
        self.get(id)?.line_index((), byte_index)
    }

    fn line_range(&'file_map self, id: Self::FileId, line_index: usize) -> CodespanResult<std::ops::Range<usize>> {
        self.get(id)?.line_range((), line_index)
    }
}
