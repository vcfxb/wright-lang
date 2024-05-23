//! Implementation of drawing [Diagnostic]s, which is non-trivial.
//!
//! [Diagnostic]: super::Diagnostic

use std::io;
use termcolor::{ColorSpec, WriteColor};
use super::{Diagnostic, Highlight};

/// Prefix added to the beginning of every line in a section (such as a [Highlight] or [Diagnostic::note])
/// when unicode is not available. 
const ASCII_SECTION_PREFIX: char = '|';

/// Prefix added to the beginning of every line in a section (such as a [Highlight] or [Diagnostic::note])
/// when unicode is available.
const UNICODE_SECTION_PREFIX: char = '\u{2503}';

/// Character to print on the first/header line of a section that connects to the prefix characters on following lines. 
const UNICODE_SECTION_OPENER: char = '\u{250F}';

/// Character to print on the first/header line of a section that connects to the prefix characters on following lines. 
const ASCII_SECTION_OPENER: char = '=';

/// Character to print after the last line of a section that connects to the prefix characters on previous lines. 
const UNICODE_SECTION_CLOSER: char = '\u{2579}';

/// Character to print after the last line of a section that connects to the prefix characters on previous lines. 
/// 
/// This is just a space for ASCII as there is not a great way to close sections otherwise. 
const ASCII_SECTION_CLOSER: char = ' ';


/// Draw a [Diagnostic] to a [WriteColor] reciever, optionally using unicode. 
pub fn draw<W: WriteColor>(diagnostic: &Diagnostic, w: &mut W, write_unicode: bool) -> io::Result<()> {
    // Create a color spec to use as we write to the writer.
    let mut spec: ColorSpec = ColorSpec::new();
    // Set the color spec for the severity and code. 
    spec.set_intense(true).set_fg(Some(diagnostic.severity.color()));
    w.set_color(&spec)?;

    // Write the severity and code.
    write!(w, "{}", diagnostic.severity)?;

    if let Some(code) = diagnostic.code.as_ref() {
        write!(w, " [{code}]")?;
    }

    // Reset the color to write the message. 
    spec.set_fg(None);
    w.set_color(&spec)?;

    // Write the message and a new line.
    writeln!(w, ": {}", diagnostic.message)?;


    // If there was no error writing the diagnostic, return Ok.
    Ok(())
}

#[cfg(test)] 
mod tests {
    use termcolor::NoColor;
    use crate::reporting::Diagnostic;

    /// Create a buffer with no colors to write the diagnostic to, write the diagnostic, and then return the string 
    /// in the buffer after writing. 
    fn test_diagnostic(d: &Diagnostic, write_unicode: bool) -> String {
        // Use a byte vec as a test buffer to write to without color. 
        let mut buffer: NoColor<Vec<u8>> = NoColor::new(Vec::new());
        
        // Write to buffer.
        super::draw(d, &mut buffer, write_unicode)
            .expect("Wrote diagnostic to buffer");

        return String::from_utf8(buffer.into_inner())
            .expect("Buffer contained valid UTF-8");
    }

    #[test]
    fn test_basic_error() {
        // Create a test diagnostic.
        let d: Diagnostic = Diagnostic::error("test error");
        
        // It should look the same with and without unicode.
        let without_unicode = test_diagnostic(&d, false);
        let with_unicode = test_diagnostic(&d, true);

        assert_eq!(without_unicode, "error: test error\n");
        assert_eq!(without_unicode, with_unicode);
    }

    #[test]
    fn test_basic_error_with_code() {
        // Create a test diagnostic.
        let d: Diagnostic = Diagnostic::error("test error")
            .with_code("TEST_001");
        
        // It should look the same with and without unicode.
        let without_unicode = test_diagnostic(&d, false);
        let with_unicode = test_diagnostic(&d, true);

        assert_eq!(without_unicode, "error [TEST_001]: test error\n");
        assert_eq!(without_unicode, with_unicode);
    }

    // Run this with `cargo test test_unicode_vs_ascii -- --include-ignored --nocapture` to print the constants \
    // used in this module.
    #[test]
    #[ignore = "print-debugging tests ignored by default"]
    fn test_unicode_vs_ascii() {
        use super::{ASCII_SECTION_CLOSER, ASCII_SECTION_OPENER, ASCII_SECTION_PREFIX, UNICODE_SECTION_CLOSER, UNICODE_SECTION_OPENER, UNICODE_SECTION_PREFIX};

        // Print unicode section: 
        println!("{UNICODE_SECTION_OPENER} Note: \
                \n{UNICODE_SECTION_PREFIX} Test note \
                \n{UNICODE_SECTION_PREFIX} \
                \n{UNICODE_SECTION_CLOSER}");

        
        // Print ASCII section: 
        println!("{ASCII_SECTION_OPENER} Note: \
                \n{ASCII_SECTION_PREFIX} Test note \
                \n{ASCII_SECTION_PREFIX} \
                \n{ASCII_SECTION_CLOSER}");
    }
}

