//! Responsible for keeping track of different files added to the Wright build system.

use crate::parser::fragment::Fragment;
use codespan_reporting::{
    diagnostic::Diagnostic,
    files::{Files, SimpleFile},
    term::Config,
};
use fs4::FileExt;
use memmap2::Mmap;
use std::{fs::File, io, path::PathBuf, sync::{mpsc, Arc, RwLock}, thread, time::Duration};
use termcolor::{ColorChoice, StandardStream};
use self::{filename::FileName, immutable_string::ImmutableString};

/// Rename import for clarity.
use codespan_reporting::files::Error as CodespanError;

pub mod filename;
pub mod immutable_string;
pub mod bucket;

/// Convenience type alias. Used for types returned from/with [codespan_reporting]. 
pub type CodespanResult<T> = Result<T, CodespanError>;

/// Amount of time before we should warn the user about locking the file taking too long.
pub const FILE_LOCK_WARNING_TIME: Duration = Duration::from_secs(5);



/// The file map that we use throughout the rest of this crate.
/// 
/// This uses an [Arc]'d [RwLock]'d [Vec] of [Arc]s internally for concurrent access across multiple threads, 
/// so feel free to clone this around since it's really just cloning an [Arc] (should be cheap). 
#[derive(Clone, Debug)]
pub struct FileMap<'src> {
    /// This list of files we're keeping track of.
    /// This is similar to the current implementation of [codespan_reporting::files::SimpleFiles],
    /// but we don't use theirs because we need to iterate over each [SimpleFile] manually for various
    /// parts of the implementation (and we need to wrap each file in an [Arc] for sharing across threads). 
    inner: Arc<RwLock<Vec<Arc<SimpleFile<FileName, ImmutableString<'src>>>>>>,
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

impl<'src> FileMap<'src> {
    /// Construct a new empty [FileMap].
    pub fn new() -> Self {
        FileMap { inner: Arc::new(RwLock::new(Vec::new())) }
    }

    /// Get an [Arc] reference to a file from the internal [Vec] or return a [`CodespanError::FileMissing`] error.
    fn get(&self, file_id: FileId) -> CodespanResult<Arc<SimpleFile<FileName, ImmutableString<'src>>>> {
        // Unwrap the index in the file id.
        let FileId(index) = file_id;
        
        // Get a read-guard to to the internal files vec.
        let read_guard = self.inner
            .read()
            .expect("FileMap lock should not be poisoned");

        // Try to get the file arc, if the 
        let file_arc_clone: Option<Arc<_>> = read_guard
            .get(index)
            // Clone the arc so we can drop the read-guard. 
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
    fn add(&mut self, name: FileName, source: ImmutableString<'src>) -> FileId {
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

    /// Add a file (in the form of an owned string) to the file map.
    pub fn add_string(&mut self, name: FileName, source: String) -> FileId {
        self.add(name, ImmutableString::new_owned(source.into_boxed_str()))
    }

    /// Add a file (in the form of a string reference) to the file map.
    pub fn add_str_ref(&mut self, name: FileName, source: &'src str) -> FileId {
        self.add(name, ImmutableString::new_reference(source))
    }

    /// Add a file from the file system. This file will be opened with read permissions, locked (using [fs4]), 
    /// memory mapped (using [memmap2]), and then added to the file map. 
    /// This file will remain locked and memory-mapped until this [FileMap] is [drop]ped. 
    /// See [ImmutableString::drop] for more details. 
    /// 
    /// The [FileName] in this map will be [FileName::Real] with the [PathBuf] passed to this function.
    pub fn add_file(&mut self, path: PathBuf) -> io::Result<FileId> {
        // Make a one-off enum here to use for channel messages.
        enum ChannelMessage {
            /// The file was successfully locked.
            FileLocked(File),
            /// There was an error locking the file.
            LockingError(io::Error),
            /// File is taking a long time to lock.
            FiveSecondWarning,
        }

        // Open the file for reading.
        let file: File = File::open(&path)?;

        // Create two threads and a mpsc channel for warning the user if
        // locking the file takes longer than 5 seconds.
        let (tx, rx) = mpsc::sync_channel::<ChannelMessage>(1);
        let timout_tx = tx.clone();

        // Thread to lock the file
        thread::spawn(move || match file.lock_exclusive() {
            Ok(_) => tx.send(ChannelMessage::FileLocked(file)),
            Err(err) => tx.send(ChannelMessage::LockingError(err)),
        });

        // Thread to warn user if it takes too long.
        thread::spawn(move || {
            thread::sleep(FILE_LOCK_WARNING_TIME);
            timout_tx.send(ChannelMessage::FiveSecondWarning)
        });

        // Use an infinite loop to make sure we recieve all the messages from the senders.
        loop {
            match rx.recv() {
                // Emit the diagnostic for the 5-second warning.
                Ok(ChannelMessage::FiveSecondWarning) => {
                    // Get a lock on the standard out so that we don't get interrupted here.
                    let stdout = StandardStream::stdout(ColorChoice::Auto);
                    let mut stdout = stdout.lock();

                    // Make the diagnostic to show to the user.
                    let message = format!(
                        "Getting a file lock on {} has taken more than {} seconds.",
                        path.display(),
                        FILE_LOCK_WARNING_TIME.as_secs()
                    );

                    let diagnostic: Diagnostic<FileId> = Diagnostic::note().with_message(message);

                    // Emit the diagnostic to the user.
                    codespan_reporting::term::emit(&mut stdout, &Config::default(), self, &diagnostic)
                        // Convert from the potential codespan error to a normal IO err. 
                        .map_err(|cs_err: CodespanError| match cs_err {
                            CodespanError::Io(io_err) => io_err,
                            _ => unreachable!("We should not see any other codespan errors here, since we do not reference files in this diagnostic."),
                        })?
                }

                // Handle any io errors locking the file by returning them.
                Ok(ChannelMessage::LockingError(io_err)) => return Err(io_err),

                // Handle success by finishing adding the file to the FileMap.
                Ok(ChannelMessage::FileLocked(file)) => {
                    // The file is now locked, we can memmory map it and add it ro the vec.
                    // SAFETY: The file should be locked at this point so undefined behaviour from concurrent
                    // modification is avoided.
                    let mem_map: Mmap = unsafe {
                        Mmap::map(&file)
                            // Make sure we (at least try to) unlock the file if there's an issue memory mapping it.
                            .map_err(|err| {
                                file.unlock()
                                    .map_err(|err| eprintln!("Error unlocking file: {:?}", err))
                                    .ok();
                                err
                            })
                    }?;

                    // Double check that the file is valid utf-8. If not, return an IO error.
                    let raw_data: &[u8] = mem_map.as_ref();
                    
                    if let Err(utf8_error) = std::str::from_utf8(raw_data) {
                        // The file is not valid for us so we should unlock it and return an error.
                        file.unlock()
                            .map_err(|err| eprintln!("Error unlocking file: {:?}", err))
                            .ok();

                        return Err(io::Error::new(io::ErrorKind::InvalidData, utf8_error));
                    }

                    // If we get here, the file is valid UTF-8 -- add it to the file map. 
                    return Ok(self.add(
                        FileName::Real(path),
                        ImmutableString::new_locked_file(file, mem_map)
                    ));
                }

                Err(_) => unreachable!(
                    "The reciever should never reach a state where both senders are closed."
                ),
            }
        }
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
            let source: ImmutableString = unsafe { self.source(FileId(file_index)).unwrap_unchecked() };

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
impl<'file_map, 'src: 'file_map> Files<'file_map> for FileMap<'src> {
    /// See [FileId] for more info. 
    type FileId = FileId;

    type Name = FileName;

    type Source = ImmutableString<'src>;

    fn name(&'file_map self, id: Self::FileId) -> CodespanResult<Self::Name> {
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
