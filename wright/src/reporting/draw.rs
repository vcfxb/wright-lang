//! Implementation of drawing [Diagnostic]s, which is non-trivial.
//!
//! [Diagnostic]: super::Diagnostic

use std::io;
use termcolor::{ColorSpec, WriteColor};
use super::Diagnostic;

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
}

