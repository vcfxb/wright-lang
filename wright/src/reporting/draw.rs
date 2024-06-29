//! Implementation of drawing [Diagnostic]s, which is non-trivial.
//!
//! [Diagnostic]: super::Diagnostic

use crate::source_tracking::{filename::FileName, fragment::Fragment, SourceRef};
use super::{box_drawing, owned_string::OwnedString, style::Style, Diagnostic, Highlight};
use std::{io, sync::Arc};
use termcolor::{Color, ColorSpec, WriteColor};
use terminal_link::Link;
use terminal_size::Width;

/// The color used to write notes at the end of diagnostics. 
pub const NOTE_COLOR: Color = Color::Cyan;

/// A struct to hold information used in drawing of diagnostics. 
pub struct Draw<'w, W: WriteColor> {
    /// The writer being written to. 
    writer: &'w mut W,
    /// The style being used. 
    style: Style,
    /// Whether the write target supports terminal emulator hyperlink escapes. 
    supports_hyperlinks: bool,
    /// The width of the terminal/terminal emulator being written to, if known. 
    width: Option<u16>
}

impl<'w, W: WriteColor> Draw<'w, W> {
    /// Draw a [Diagnostic].
    pub fn draw_diagnostic(&mut self, diagnostic: &Diagnostic) -> io::Result<()> {
        // Create a color spec to use as we write to the writer.
        let mut spec: ColorSpec = ColorSpec::new();
        
        // Set the color spec for the severity and code.
        spec.set_fg(Some(diagnostic.severity.color()));
        self.writer.set_color(&spec)?;
        
        // Write the opening bar before turning bold. 
        write!(self.writer, "{} ", self.style.vertical_char())?;

        spec.set_bold(true);
        self.writer.set_color(&spec)?;
        write!(self.writer, "{}", diagnostic.severity)?;

        // If the diagnostic has a code, write it. 
        if let Some(code) = diagnostic.code.as_ref() {
            write!(self.writer, " [{code}]")?;
        }

        // Reset the color to write the message.
        spec.set_fg(None);
        self.writer.set_color(&spec)?;

        // Write the message and a new line.
        writeln!(self.writer, ": {}", diagnostic.message)?;

        // Draw the primary highlight if there is one. 
        if let Some(highlight) = diagnostic.primary_highlight.as_ref() {
            self.draw_highlight(highlight, diagnostic.severity.color())?;
        }

        // Draw all secondary highlights. Use green as the color for secondary highlights.
        for highlight in diagnostic.secondary_highlights.iter() {
            self.draw_highlight(highlight, Color::Green)?;
        }

        // Draw the note. Add an extra blank line between the fragment/title and the note.
        if let Some(note) = diagnostic.note.as_ref() {
            writeln!(self.writer, "{}", self.style.vertical_char())?;
            self.draw_note(note)?;
        }

        // Write an extra newline at the end to provide space between diagnostics. 
        spec.clear();
        self.writer.set_color(&spec)?;
        writeln!(self.writer)?;

        // If there was no error writing the diagnostic, return Ok.
        Ok(())
    }

    
    /// Draw a code [Highlight]. 
    fn draw_highlight(&self, highlight: &Highlight, highlight_color: Color) -> io::Result<()> {
        // Create a color spec to use while drawing this Highlight. 
        let mut spec = ColorSpec::new();
        // Get a reference to the fragment that we're printing.
        let fragment: &Fragment = &highlight.fragment;
        // Get a reference to the source that the fragment is from.
        let source: &SourceRef = &fragment.source;
        // Calculate the line indices of the fragment in it's parent source.
        let line_indices = fragment.line_indices();
        // Get the column on that line that the fragment starts on. Add 1 to make this 1-indexed. 
        let col_num = fragment.range.start - source.get_line(line_indices.start).range.start + 1;
        // Get the display width of the highest line number. 
        let line_nums_width = f64::log10(line_indices.end as f64).ceil() as usize;

        // Set the color . 
        spec.set_fg(Some(highlight_color));
        w.set_color(&spec)?;
        
        // Write a horizontal bar above the highlight. 
        let divider_width = terminal_size::terminal_size()
            // Subrtact 1 here because we also print a branch character first. 
            .map(|(Width(w), _)| w - 1)
            .unwrap_or(40);

        let divider = style.horizontal_char()
            .unwrap_or('=')
            .to_string()
            .repeat(divider_width as usize);

        writeln!(w, "{}{}", style.vertical_right_char().or(style.horizontal_char()).unwrap_or('='), divider)?;

        // and print the file name we're pulling from with a vertical bar preceding it.
        write!(w, "{} ", style.vertical_char())?;
        // Clear out the spec for the file name itself. If possible, write it as a hyperlink. 
        spec.clear();
        w.set_color(&spec)?;

        // Create a string that represents the location. 
        let location = match (source.name(), supports_hyperlinks) {
            // In cases of real files printing to terminals that support hyperlinks, create a hyperlink. 
            (FileName::Real(path), true) => {
                let link_text = format!("{}:{}:{col_num}", source.name(), line_indices.start + 1);
                let link_url = format!("file://localhost{}", path.canonicalize()?.display());
                Link::new(&link_text, &link_url).to_string()
            }

            _ => format!("{}:{}:{col_num}", source.name(), line_indices.start + 1),
        };

        // Write the location and the message. 
        writeln!(w, "[{location}]: {}", highlight.message)?;
        // Write a small horizontal bar above the line numbers.
        writeln!(w, "{}", style.vertical_right_char().unwrap_or(style.vertical_char()))?;

        // Print each line. Use 
        for line_index in line_indices {
            // Get the source fragment with any extra whitespace at the end trimmed off. 
            let line = source.get_line(line_index).trim_end();
            // Write the line number. 
            write!(w, "{0} {1:>2$} {0} ", style.vertical_char(), line_index + 1, line_nums_width)?;

        }

        // Write an extra line at the end so that whatever comes next starts at the begining of a fresh line. 
        writeln!(w)?;
        
        Ok(())
    }

    /// Draw a [Diagnostic::note].
    fn draw_note(&mut self, note: &OwnedString) -> io::Result<()> {
        // Create a color spec to use while drawing. 
        let mut color_spec = ColorSpec::new();
    
        // Set the color spec to announce the note. 
        color_spec.set_fg(Some(NOTE_COLOR));
        self.writer.set_color(&color_spec)?;
        write!(self.writer, "{} ", self.style.vertical_char())?;
    
        // Make the rest of this line italic and underlined. 
        color_spec
            .set_italic(true)
            .set_underline(true);
    
        self.writer.set_color(&color_spec)?;
    
        // Announce the note. 
        writeln!(self.writer, "Note:")?;
    
        // Clear/reset the color. 
        color_spec.clear();
        self.writer.set_color(&color_spec)?;
    
        // Write the note with the preceding vertical bars.
        for line in note.as_ref().lines() {
            writeln!(self.writer, "{} {line}", self.style.vertical_char())?;
        }
    
        // If written successfully, return Ok. 
        Ok(())
    }
}



#[cfg(test)]
mod tests {
    use std::{io, sync::Arc};

    use crate::{reporting::{style::Style, Diagnostic, Highlight}, source_tracking::{filename::FileName, fragment::Fragment, source::Source}};
    use indoc::indoc;
    use termcolor::{ColorChoice, NoColor};

    /// Create a buffer with no colors to write the diagnostic to, write the diagnostic, and then return the string
    /// in the buffer after writing.
    fn test_diagnostic(d: &Diagnostic, style: Style) -> String {
        // Use a byte vec as a test buffer to write to without color.
        let mut buffer: NoColor<Vec<u8>> = NoColor::new(Vec::new());

        // Write to buffer.
        super::draw(d, &mut buffer, style).expect("Wrote diagnostic to buffer");

        return String::from_utf8(buffer.into_inner()).expect("Buffer contained valid UTF-8");
    }

    #[test]
    fn test_basic_error() {
        // Create a test diagnostic.
        let d: Diagnostic = Diagnostic::error("test error");

        // It should look the same with and without unicode.
        let without_unicode = test_diagnostic(&d, Style::Ascii);
        let with_unicode = test_diagnostic(&d, Style::UnicodeLight);

        assert_eq!(without_unicode, "error: test error\n");
        assert_eq!(without_unicode, with_unicode);
    }

    #[test]
    fn test_basic_error_with_code() {
        // Create a test diagnostic.
        let d: Diagnostic = Diagnostic::error("test error").with_code("TEST_001");

        // It should look the same with and without unicode.
        let without_unicode = test_diagnostic(&d, Style::Ascii);
        let with_unicode = test_diagnostic(&d, Style::UnicodeLight);

        assert_eq!(without_unicode, "error [TEST_001]: test error\n");
        assert_eq!(without_unicode, with_unicode);
    }

    #[test]
    fn print_note() -> io::Result<()> {
        let d = Diagnostic::info("test").with_note("This is a sample note.");

        d.print(ColorChoice::Auto)
    }

    
    #[test]
    fn print_with_highlight_and_note() -> io::Result<()> {

        let source = Source::new_from_static_str(FileName::Test("test.wr"), indoc! {"
            func main() {
                wright::println(\"Hello World!\");
            }
        "});

        let frag = Fragment { source: Arc::new(source), range: 0..12 };

        let d = Diagnostic::error("test")
            .with_code("TEST001")
            .with_primary_highlight(Highlight::new(frag, "main() defined here"))
            .with_note("This is a sample note.");

        d.print(ColorChoice::Auto)
    }
}
