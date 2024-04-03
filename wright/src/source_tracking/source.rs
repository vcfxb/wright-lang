//! Structure and implementation for storing source code items used by the wright compiler, including 
//! source files from disk, source strings used in test cases, and source strings created at 
//! run-time by an API consumer.

use crate::reporting::Diagnostic;

use super::{filename::FileName, immutable_string::ImmutableString};
use std::time::Duration;
use std::fs::File;
use std::io;
use std::thread;
use std::sync::mpsc;
use std::path::PathBuf;
use fs4::FileExt;
use memmap2::Mmap;

/// Amount of time before we should warn the user about locking the file taking too long.
pub const FILE_LOCK_WARNING_TIME: Duration = Duration::from_secs(5);

/// A full source. This is usually a file, but may also be passed in the form of a string for testing.  
#[derive(Debug)]
pub struct Source {
    /// The name of this source file. 
    name: FileName,

    /// The content of this source file. 
    source: ImmutableString,

    /// A list of byte indicies into the [Source::source] indicating where lines starts. 
    line_starts: Vec<usize>
}

impl Source {
    /// Attempt to load a file from the disk into a source.  
    pub fn new_from_disk(path: PathBuf) -> io::Result<Self> {
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
                    Diagnostic::warning(message).print()?;
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
                    let immut_string: ImmutableString = ImmutableString::new_locked_file(file, mem_map);

                    return Ok(Source { 
                        name: FileName::Real(path), 
                        line_starts: immut_string.line_starts(),
                        source: immut_string
                    });
                }

                Err(_) => unreachable!(
                    "The reciever should never reach a state where both senders are closed."
                ),
            }
        }

    }
}
