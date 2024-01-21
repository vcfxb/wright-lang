//! Responsible for keeping track of different files added to the Wright build system.

use codespan_reporting::{files::{Files, SimpleFile}, term::Config, diagnostic::Diagnostic};
use derive_more::Display;
use fs4::FileExt;
use memmap2::Mmap;
use termcolor::{ColorChoice, StandardStream};
use std::{path::PathBuf, io, fs::File, sync::mpsc, thread, time::Duration};
use crate::parser::fragment::Fragment;

/// Rename import for clarity. 
use codespan_reporting::files::Error as CodespanError;

/// Convenience type alias. 
type CodespanResult<T> = Result<T, CodespanError>;

/// Amount of time before we should warn the user about locking the file taking too long. 
const FILE_LOCK_WARNING_TIME: Duration = Duration::from_secs(5);

/// Used to represent different file names used throughout this crate.
#[derive(Debug, Display, Clone)]
pub enum FileName {
    /// A real file on the user's computer.
    #[display(fmt = "{}", "_0.display()")]
    Real(PathBuf),
    /// A named test-case in this crate's source code.
    Test(&'static str),
    /// The interactive Wright repl.
    #[display(fmt = "REPL:{}", line_number)]
    Repl { line_number: usize },
    /// An un-named test case in this crate's source code.
    #[display(fmt = "<NO_NAME>")]
    None,
}

/// An immutable string that either references a source file in memory using an `&` reference or using a [Box]. 
#[derive(Debug)]
enum ImmutableString<'src> {
    /// An immutable reference to an existing string. 
    Reference(&'src str),

    /// An owned immutable string. 
    Owned(Box<str>),

    /// A locked, memory mapped file from the OS. 
    LockedFile {
        /// The locked file that needs to be unlocked when this object is dropped. 
        locked_file: File,
        /// The memory locked file -- this is expected to be locked before
        /// one creates it in the file 
        mem_map: Mmap,
    }
}

/// The file map that we use throughout the rest of this crate.
pub struct FileMap<'src> {
    /// This is just a list of files we're keeping track of. 
    /// This is identical to the current implementation of [codespan_reporting::files::SimpleFiles],
    /// but we don't use theirs because we need to iterate over the [SimpleFile]s manually for various 
    /// parts of the implementation.
    inner: Vec<SimpleFile<FileName, ImmutableString<'src>>>
}


impl<'src> FileMap<'src> {
    /// Construct a new empty [FileMap]. 
    pub const fn new() -> Self {
        FileMap { inner: Vec::new() }
    }

    /// Get a reference to a file from the internal [Vec] or return a [`CodespanError::FileMissing`] error. 
    fn get(&self, file_id: <Self as Files<'src>>::FileId) -> CodespanResult<&SimpleFile<FileName, ImmutableString<'src>>> {
        self.inner.get(file_id).ok_or(CodespanError::FileMissing)
    }

    /// Internal function to add a file to the vec. Public facing functions will need to do some conversion
    /// and then call this. 
    fn add(&mut self, name: FileName, source: ImmutableString<'src>) -> <Self as Files<'src>>::FileId {
        // The file id is just the next index in the vec.
        let file_id: usize = self.inner.len();
        self.inner.push(SimpleFile::new(name, source));
        file_id
    }

    /// Add a file (in the form of an owned string) to the file map. 
    pub fn add_string(&mut self, name: FileName, source: String) -> <Self as Files<'src>>::FileId {
        self.add(name, ImmutableString::Owned(source.into_boxed_str()))
    }

    /// Add a file (in the form of a string reference) to the file map. 
    pub fn add_str_ref(&mut self, name: FileName, source: &'src str) -> <Self as Files<'src>>::FileId {
        self.add(name, ImmutableString::Reference(source))
    }

    /// Add a file from the file system. This file will be 
    /// opened with read permissions, locked, memory mapped, 
    /// and then added to the file map. The file name in the memory map will be the [PathBuf] passed to this function. 
    pub fn add_file(&mut self, path: PathBuf) -> io::Result<<Self as Files<'src>>::FileId> {
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
        thread::spawn(move || { 
            match file.lock_exclusive() {
                Ok(_) => tx.send(ChannelMessage::FileLocked(file)),
                Err(err) => tx.send(ChannelMessage::LockingError(err))
            }
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
                    let message = format!("Getting a file lock on {} has taken more than {} seconds.", path.display(), FILE_LOCK_WARNING_TIME.as_secs());
                    let diagnostic: Diagnostic<<FileMap<'src> as Files<'src>>::FileId> = Diagnostic::note().with_message(message);
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
                            // Make sure we unlock the file if there's an issue memory mapping it. 
                            .map_err(|err| {
                                file.unlock().map_err(|err| eprintln!("Error unlocking file: {:?}", err)).ok();
                                err
                            })
                    }?;

                    // Double check that the file is valid utf-8. If not, return an IO error. 
                    let raw_data: &[u8] = mem_map.as_ref();
                    let as_str: Result<&str, std::str::Utf8Error> = std::str::from_utf8(raw_data);
                    if as_str.is_err() {
                        // The file is not valid for us so we should unlock it and return an error. 
                        file.unlock().map_err(|err| eprintln!("Error unlocking file: {:?}", err)).ok();
                        return Err(io::Error::new(io::ErrorKind::InvalidData, as_str.unwrap_err()));
                    }

                    // The file's contents are valid utf-8, add them to the file map. 
                    return Ok(self.add(FileName::Real(path), ImmutableString::LockedFile { locked_file: file, mem_map }));
                }

                Err(_) => unreachable!("The reciever should never reach a state where both senders are closed."),
            }    
        }
    }

    /// Find the file ID of a given [Fragment] using the fragment's internal pointer. 
    pub fn find_fragment(&self, fragment: &Fragment<'src>) -> Option<<Self as Files<'src>>::FileId> {
        // Iterate on file IDs. 
        for file_id in 0..self.inner.len() {
            // Use expect because all of these file IDs should be fine. 
            let source: &str = self.source(file_id).expect("All file IDs here are valid");
            if (Fragment { inner: source }).contains(fragment) {
                return Some(file_id);
            }
        }

        // If there was no file containing the given fragment, return none. 
        None
    }
}

/// Implement drop here to make sure that the files get unlocked as they go out of scope/use.
impl<'src> Drop for ImmutableString<'src> {
    fn drop(&mut self) {
        match self {
            // Unlock locked files.
            ImmutableString::LockedFile { locked_file, .. } => {
                locked_file.unlock()
                    // Log the error if there is one, 
                    .map_err(|io_err: io::Error| eprintln!("{}", io_err))
                    // Discard value of result
                    .ok();
            }

            // All other types drop trivially.  
            ImmutableString::Owned(_) | ImmutableString::Reference(_) => {}
        }
    }
}

/// The implementation here is basically identical to the one for [codespan_reporting::files::SimpleFiles]. 
impl<'src> Files<'src> for FileMap<'src> {
    /// File IDs here are just indices into [FileMap]'s internal [Vec]. 
    type FileId = usize;

    type Name = FileName;

    type Source = &'src str;

    fn name(&self, id: Self::FileId) -> Result<Self::Name, codespan_reporting::files::Error> {
        Ok(self.get(id)?.name().clone())
    }

    fn source(&'src self, id: Self::FileId) -> Result<Self::Source, codespan_reporting::files::Error> {
        Ok(self.get(id)?.source().as_ref())
    }

    fn line_index(&self, id: Self::FileId, byte_index: usize) -> Result<usize, codespan_reporting::files::Error> {
        self.get(id)?.line_index((), byte_index)
    }

    fn line_range(&self, id: Self::FileId, line_index: usize) -> Result<std::ops::Range<usize>, codespan_reporting::files::Error> {
        self.get(id)?.line_range((), line_index)
    }
}

impl<'src> AsRef<str> for ImmutableString<'src> {
    fn as_ref(&self) -> &str {
        match self {
            ImmutableString::Reference(str) => str,
            ImmutableString::Owned(str) => &str,
            ImmutableString::LockedFile { mem_map, .. } => {
                // Get a direct reference to the data that is in the memory map. 
                let raw_data: &[u8] = mem_map.as_ref();
                // SAFETY: UTF-8 validity is checked when the file is added to the file map. 
                unsafe { std::str::from_utf8_unchecked(raw_data) }
            }
        }
    }
}
