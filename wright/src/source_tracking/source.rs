//! Structure and implementation for storing source code items used by the wright compiler, including
//! source files from disk, source strings used in test cases, and source strings created at
//! run-time by an API consumer.

use super::SourceRef;
use super::{filename::FileName, fragment::Fragment, immutable_string::ImmutableString};
use std::{fs::File, sync::Arc};
use std::io;
use std::path::PathBuf;

#[cfg(feature = "file_memmap")]
use std::{sync::mpsc, thread, time::Duration};

#[cfg(feature = "file_memmap")]
use fs4::FileExt;

#[cfg(feature = "file_memmap")]
use memmap2::Mmap;

#[cfg(feature = "file_memmap")]
use crate::reporting::Diagnostic;

#[cfg(feature = "file_memmap")]
use termcolor::ColorChoice;

/// Amount of time before we should warn the user about locking the file taking too long.
#[cfg(feature = "file_memmap")]
pub const FILE_LOCK_WARNING_TIME: Duration = Duration::from_secs(5);

/// A full source. This is usually a file, but may also be passed in the form of a string for testing.  
#[derive(Debug)]
pub struct Source {
    /// The name of this source file.
    name: FileName,

    /// The content of this source file.
    source: ImmutableString,

    /// A list of byte indicies into the [Source::source] indicating where lines starts.
    line_starts: Vec<usize>,
}

impl Source {
    /// Construct a new [Source].
    fn new(name: FileName, source: ImmutableString) -> Self {
        Source {
            name,
            line_starts: source.line_starts(),
            source,
        }
    }

    /// Create a [Source] using a heap allocated [String].
    pub fn new_from_string(name: FileName, source: String) -> Self {
        Source::new(name, ImmutableString::new_owned(source.into_boxed_str()))
    }

    /// Create a [Source] from a [`&'static str`].
    ///
    /// [`&'static str`]: str
    pub fn new_from_static_str(name: FileName, source: &'static str) -> Self {
        Source::new(name, ImmutableString::new_static(source))
    }

    /// Attempt to memory map a file from the disk into a [Source].
    /// This will likely be faster than reading the file in some cases, and almost always more memory efficient.
    ///
    /// This requires the "file_memmap" feature.
    #[cfg(feature = "file_memmap")]
    pub fn new_mapped_from_disk(path: PathBuf) -> io::Result<Self> {
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
        let (tx, rx) = mpsc::sync_channel::<ChannelMessage>(0);
        let timeout_tx = tx.clone();

        // Thread to lock the file
        thread::spawn(move || match file.lock_exclusive() {
            Ok(_) => tx.send(ChannelMessage::FileLocked(file)),
            Err(err) => tx.send(ChannelMessage::LockingError(err)),
        });

        // Thread to warn user if it takes too long.
        thread::spawn(move || {
            thread::sleep(FILE_LOCK_WARNING_TIME);
            timeout_tx.send(ChannelMessage::FiveSecondWarning)
        });

        // Use an infinite loop to make sure we recieve all the messages from the senders.
        loop {
            match rx.recv() {
                // Emit the diagnostic for the 5-second warning.
                Ok(ChannelMessage::FiveSecondWarning) => {
                    // Make the diagnostic to show to the user.
                    let message = format!(
                        "Getting a file lock on {} has taken more than {} seconds.",
                        path.display(),
                        FILE_LOCK_WARNING_TIME.as_secs()
                    );

                    // Wrap the message in a warning diagnostic and print it.
                    // Add a note to describe what is going on.
                    Diagnostic::warning(message)
                        .with_note("This may be caused by another process holding or failing to release a lock on this file.")
                        .print(ColorChoice::Auto)?;
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

                    // If we get here, the file is valid UTF-8 -- put the memory mapped file in an Immutable string object.
                    return Ok(Source::new(
                        FileName::Real(path),
                        ImmutableString::new_locked_file(file, mem_map),
                    ));
                }

                Err(_) => unreachable!(
                    "The reciever should never reach a state where both senders are closed."
                ),
            }
        }
    }

    /// Read a file from the disk into a source. This reads the file, which may take longer than memory mapping it
    /// as done in [Self::new_mapped_from_disk]. This does not require the same features and dependencies as memory
    /// mapped operations though. This stores the whole file in memory, rather than mapping virtual memory to the disk.
    /// That makes this less memory efficient than [Self::new_mapped_from_disk], which may be important on systems
    /// where ram is constrained.
    ///
    /// Use this if the "file_memmap" is not available for some reason.
    pub fn new_read_from_disk(path: PathBuf) -> io::Result<Self> {
        // Open the file for reading.
        let file: File = File::open(&path)?;
        // Read the file to a string.
        let content: String = io::read_to_string(&file)?;
        // Turn that into a Source.
        Ok(Self::new(
            FileName::Real(path),
            ImmutableString::new_owned(content.into_boxed_str()),
        ))
    }

    /// Get byte indices of where lines start in this [Source].
    pub fn line_starts(&self) -> &[usize] {
        self.line_starts.as_slice()
    }

    /// Get the number of lines in this [Source]. This is identical to [`Self::line_starts`] length. 
    pub fn count_lines(&self) -> usize {
        self.line_starts.len()
    }

    /// Get a line of this [Source] as a [Fragment]. 
    /// The returned [Fragment] will contain the line terminating characters at the end of it. If you don't want those,
    /// use [Fragment::trim_end]. 
    /// 
    /// *Note* that this uses `line_index` which is considered 0-indexed -- when displaying line numbers to the user, 
    /// remember to add 1. 
    /// 
    /// # Panics
    /// - This will panic if you ask for a line index that's higher than or equal to the number returned 
    ///     by [`Self::count_lines`].
    pub fn get_line(self: &Arc<Source>, line_index: usize) -> Fragment {
        if line_index >= self.count_lines() {
            panic!("{} is greater than the number of lines in {}", line_index, self.name);
        }

        // Get the starting byte index of the line. 
        let start_byte_index: usize = self.line_starts[line_index];
        
        // Get the ending byte index of the line / the starting index of the next line/the index of the end of the file.
        let end_byte_index: usize = if line_index + 1 == self.count_lines() {
            self.source.len()
        } else {
            self.line_starts[line_index + 1]
        };

        // Construct the resultant fragment. 
        let frag = Fragment { source: SourceRef(Arc::clone(self)), range: start_byte_index..end_byte_index };
        // Debug assert that the fragment is valid. This should always be true but might be useful for testing. 
        debug_assert!(frag.is_valid());
        // Return constructed fragment. 
        return frag;
    }

    /// Get an iterator over all the lines of this [Source]. This calls [Source::get_line] for each element of
    /// the returned iterator.
    /// 
    /// The returned [Fragment]s will contain the line terminating characters at the end of them. If you don't want 
    /// those, use [Iterator::map] and [Fragment::trim_end].
    pub fn lines(self: Arc<Source>) -> impl Iterator<Item = Fragment> {
        (0..self.count_lines())
            .into_iter()
            .map(move |line_index| self.get_line(line_index))
    }

    /// Get the the source code stored.
    pub const fn source(&self) -> &ImmutableString {
        &self.source
    }

    /// Get the name of this [Source].
    pub const fn name(&self) -> &FileName {
        &self.name
    }
}
