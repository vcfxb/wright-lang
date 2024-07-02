//! Implementation of drawing [Diagnostic]s, which is non-trivial.
//!
//! [Diagnostic]: super::Diagnostic

use crate::source_tracking::filename::FileName;
use super::{owned_string::OwnedString, style::Style, Diagnostic, Highlight};
use std::{collections::HashMap, io, ops::Range};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use terminal_link::Link;
use terminal_size::Width;

/// The color used to write notes at the end of diagnostics. 
pub const NOTE_COLOR: Color = Color::Cyan;

/// A struct to hold information used in the rendering of diagnostics. 
#[allow(missing_debug_implementations)]
pub struct Renderer<W: WriteColor> {
    /// The writer being written to. 
    pub writer: W,
    /// The style being used. 
    pub style: Style,
    /// Whether the write target supports terminal emulator hyperlink escapes. 
    pub supports_hyperlinks: bool,
    /// The width of the terminal/terminal emulator being written to, if known. 
    pub width: Option<u16>
}

/// Create a [Renderer] for the standard output. 
pub fn for_stdout(color_choice: ColorChoice, style: Style) -> Renderer<StandardStream> {
    let stream = StandardStream::stdout(color_choice);
    let supports_hyperlinks = stream.supports_hyperlinks();

    Renderer {
        writer: stream,
        style,
        supports_hyperlinks,
        width: terminal_size::terminal_size().map(|(Width(w), _)| w),
    }
}

/// Create a [Renderer] for the standard error. 
pub fn for_stderr(color_choice: ColorChoice, style: Style) -> Renderer<StandardStream> {
    let stream = StandardStream::stderr(color_choice);
    let supports_hyperlinks = stream.supports_hyperlinks();

    Renderer {
        writer: stream,
        style,
        supports_hyperlinks,
        width: terminal_size::terminal_size().map(|(Width(w), _)| w),
    }
}

impl<W: WriteColor> Renderer<W> {
    /// Draw a [Diagnostic].
    pub fn draw_diagnostic(&mut self, diagnostic: &Diagnostic) -> io::Result<()> {
        // Draw the header line at the top of the diagnostic. 
        self.draw_diagnostic_header(diagnostic)?;

        // Draw the section with all the highlights. 
        if diagnostic.primary_highlight.is_some() {
            self.draw_code_section(diagnostic)?;
        }

        // Draw the note. Add an extra blank line between the fragment/title and the note.
        if let Some(note) = diagnostic.note.as_ref() {
            // Draw a single blank (just vertical char) separating the highlight section from the note section. 
            writeln!(self.writer, "{}", self.style.vertical_char())?;
            self.draw_note(note)?;
        }

        // Write an extra newline at the end to provide space between diagnostics. 
        writeln!(self.writer)?;

        // If there was no error writing the diagnostic, return Ok.
        Ok(())
    }


    /// Draw the header a the top of a [Diagnostic]. 
    /// 
    /// i.e.
    /// ```text
    /// | error[code]: Message goes here.\n
    /// ```
    fn draw_diagnostic_header(&mut self, diagnostic: &Diagnostic) -> io::Result<()> {
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
        writeln!(self.writer, ": {}", diagnostic.message)
    }

    /// Draw the code section of a [Diagnostic] -- the section that displays highlighted fragments. 
    /// 
    /// This is pretty complex, and contains most of the core logic for rendering code fragments. 
    /// 
    /// Assumes that [Diagnostic::primary_highlight] is [Some]. 
    fn draw_code_section(&mut self, diagnostic: &Diagnostic) -> io::Result<()> {
        // Get a reference to the primary highlight. 
        let primary_highlight: &Highlight = diagnostic.primary_highlight
            .as_ref()
            .expect("diagnostic has primary highlight");
        
        // Get the vertical char for the style.
        let vertical = self.style.vertical_char();
        // Get the line number and column number to print. 
        let primary_line_range = primary_highlight.fragment.line_indices();
        let primary_line_num  = primary_line_range.start + 1;
        // Get the column on that line that the fragment starts on. Add 1 to make this 1-indexed. 
        let primary_col_num = primary_highlight.fragment.starting_col_index() + 1;

        // Write the file name where this diagnostic originated -- this is considered to be the file
        // that the primary highlight is from.

        // Create a string that represents the location. 
        let primary_location = match (primary_highlight.fragment.source.name(), self.supports_hyperlinks) {
            // In cases of real files printing to terminals that support hyperlinks, create a hyperlink. 
            (name @ FileName::Real(path), true) => {
                let link_text = format!("{name}:{primary_line_num}:{primary_col_num}");
                let link_url = format!("file://localhost{}", path.canonicalize()?.display());
                Link::new(&link_text, &link_url).to_string()
            }

            (name , _) => format!("{name}:{primary_line_num}:{primary_col_num}"),
        };

        // Use '.' for a horizontal dashed char when using ascii. 
        let horizontal_dashed = self.style.horizontal_dashed_char().unwrap_or('.');
        // Get a vertical-right branch character, or just a vertical character on ascii. 
        let vertical_right_branch = self.style.vertical_right_char().unwrap_or(self.style.vertical_char());
        // Get a horizontal char with a down branch for above the start of the code colunm (after the line numbers column).
        // Use '.' on ascii once again.
        let horizontal_down_branch = self.style.down_horizontal_char().unwrap_or('.');

        // We need to know the maximum line index and minimum line index. 
        // By default these will be the ones for the primary highlight. 
        let mut max_line_index: usize = primary_highlight.fragment.line_indices().end;
        let mut min_line_index: usize = primary_highlight.fragment.line_indices().start;
        
        // Iterate through all the secondary highlights to determine if there are any lower or higher than 
        // the indices for the primary highlight. 
        for secondary_highlight in diagnostic.secondary_highlights.iter() {
            let Range { start, end } = secondary_highlight.fragment.line_indices();

            max_line_index = std::cmp::max(max_line_index, end);
            min_line_index = std::cmp::min(min_line_index, start);
        }

        // Get the width of the highest line number we'll need to write.
        let line_num_width = f64::log10((max_line_index + 1) as f64).ceil() as usize;
        // Use this to write spaces in the line number column for lines that get skipped.
        let skip_line_nums = " ".repeat(line_num_width + 2);

        // Reset the colorspec.
        self.writer.set_color(&ColorSpec::new())?;

        // Write our dashed divider. 
        writeln!(&mut self.writer, "{vertical_right_branch}{a}{horizontal_down_branch}{b}",
            // Add two for spaces.
            a = horizontal_dashed.to_string().repeat(line_num_width + 2),
            // Default to 60 char term width if unknown.
            // Do subtraction to make sure we get the right length. 
            b = horizontal_dashed.to_string().repeat(self.width.unwrap_or(60) as usize - (line_num_width + 4))
        )?;

        // Write our location at the top of the code colum under the horizontal divider. 
        write!(&mut self.writer, "{vertical}{skip_line_nums}{vertical} ")?;
        // Write the location in bold.  
        self.writer.set_color(&ColorSpec::new().set_bold(true))?;
        writeln!(self.writer, "[{primary_location}]:")?;

        // Categorize highlights by source file -- use gids to identify sources. 
        let mut highlights_by_source: HashMap<u64, Vec<&Highlight>> = HashMap::with_capacity(diagnostic.secondary_highlights.len() + 1);

        // Start with the primary highlight. 
        highlights_by_source
            .entry(primary_highlight.fragment.source.gid())
            .or_default()
            .push(primary_highlight);

        // Go through all of the secondary highlights.
        for highlight in diagnostic.secondary_highlights.iter() {
            highlights_by_source
                .entry(highlight.fragment.source.gid())
                .or_default()
                .push(highlight);
        }

        // Now that we've categorized all the highlights by source (using source gids) we can handle the sources one
        // at a time starting with the one for the primary highlight. 
        // TODO: 

        for line_indice in min_line_index..max_line_index {
            writeln!(&mut self.writer, "{vertical} {line_num:>line_num_width$} {vertical}", line_num = line_indice + 1)?;
        }


        Ok(())
    }

    
    /// Draw a code [Highlight]. 
    fn draw_highlight(&self, highlight: &Highlight, highlight_color: Color) -> io::Result<()> {
        unimplemented!()
        // // Create a color spec to use while drawing this Highlight. 
        // let mut spec = ColorSpec::new();
        // // Get a reference to the fragment that we're printing.
        // let fragment: &Fragment = &highlight.fragment;
        // // Get a reference to the source that the fragment is from.
        // let source: &SourceRef = &fragment.source;
        // // Calculate the line indices of the fragment in it's parent source.
        // let line_indices = fragment.line_indices();
        // // Get the column on that line that the fragment starts on. Add 1 to make this 1-indexed. 
        // let col_num = fragment.range.start - source.get_line(line_indices.start).range.start + 1;
        // // Get the display width of the highest line number. 
        // let line_nums_width = f64::log10(line_indices.end as f64).ceil() as usize;

        // // Set the color . 
        // spec.set_fg(Some(highlight_color));
        // w.set_color(&spec)?;
        
        // // Write a horizontal bar above the highlight. 
        // let divider_width = terminal_size::terminal_size()
        //     // Subrtact 1 here because we also print a branch character first. 
        //     .map(|(Width(w), _)| w - 1)
        //     .unwrap_or(40);

        // let divider = style.horizontal_char()
        //     .unwrap_or('=')
        //     .to_string()
        //     .repeat(divider_width as usize);

        // writeln!(w, "{}{}", style.vertical_right_char().or(style.horizontal_char()).unwrap_or('='), divider)?;

        // // and print the file name we're pulling from with a vertical bar preceding it.
        // write!(w, "{} ", style.vertical_char())?;
        // // Clear out the spec for the file name itself. If possible, write it as a hyperlink. 
        // spec.clear();
        // w.set_color(&spec)?;

        // // Create a string that represents the location. 
        // let location = match (source.name(), supports_hyperlinks) {
        //     // In cases of real files printing to terminals that support hyperlinks, create a hyperlink. 
        //     (FileName::Real(path), true) => {
        //         let link_text = format!("{}:{}:{col_num}", source.name(), line_indices.start + 1);
        //         let link_url = format!("file://localhost{}", path.canonicalize()?.display());
        //         Link::new(&link_text, &link_url).to_string()
        //     }

        //     _ => format!("{}:{}:{col_num}", source.name(), line_indices.start + 1),
        // };

        // // Write the location and the message. 
        // writeln!(w, "[{location}]: {}", highlight.message)?;
        // // Write a small horizontal bar above the line numbers.
        // writeln!(w, "{}", style.vertical_right_char().unwrap_or(style.vertical_char()))?;

        // // Print each line. Use 
        // for line_index in line_indices {
        //     // Get the source fragment with any extra whitespace at the end trimmed off. 
        //     let line = source.get_line(line_index).trim_end();
        //     // Write the line number. 
        //     write!(w, "{0} {1:>2$} {0} ", style.vertical_char(), line_index + 1, line_nums_width)?;

        // }

        // // Write an extra line at the end so that whatever comes next starts at the begining of a fresh line. 
        // writeln!(w)?;
        
        // Ok(())
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

    use crate::{reporting::{style::Style, Diagnostic, Highlight}, source_tracking::{filename::FileName, fragment::Fragment, source::Source, source_ref::SourceRef}};
    use indoc::indoc;
    use termcolor::{ColorChoice, NoColor};

    use super::Renderer;

    /// Create a buffer with no colors to write the diagnostic to, write the diagnostic, and then return the string
    /// in the buffer after writing.
    fn test_diagnostic(d: &Diagnostic, style: Style) -> String {
        // Use a byte vec as a test buffer to write to without color.
        let mut buffer: NoColor<Vec<u8>> = NoColor::new(Vec::new());

        // Write to buffer.
        let mut renderer = Renderer { writer: &mut buffer, style, supports_hyperlinks: false, width: None };
        renderer.draw_diagnostic(d).expect("Wrote diagnostic to buffer");

        return String::from_utf8(buffer.into_inner()).expect("Buffer contained valid UTF-8");
    }

    // #[test]
    // fn test_basic_error() {
    //     // Create a test diagnostic.
    //     let d: Diagnostic = Diagnostic::error("test error");

    //     // It should look the same with and without unicode.
    //     let without_unicode = test_diagnostic(&d, Style::Ascii);
    //     let with_unicode = test_diagnostic(&d, Style::UnicodeLight);

    //     assert_eq!(without_unicode, "error: test error\n");
    //     assert_eq!(without_unicode, with_unicode);
    // }

    // #[test]
    // fn test_basic_error_with_code() {
    //     // Create a test diagnostic.
    //     let d: Diagnostic = Diagnostic::error("test error").with_code("TEST_001");

    //     // It should look the same with and without unicode.
    //     let without_unicode = test_diagnostic(&d, Style::Ascii);
    //     let with_unicode = test_diagnostic(&d, Style::UnicodeLight);

    //     assert_eq!(without_unicode, "error [TEST_001]: test error\n");
    //     assert_eq!(without_unicode, with_unicode);
    // }

    // #[test]
    // fn print_note() -> io::Result<()> {
    //     let d = Diagnostic::info("test").with_note("This is a sample note.");

    //     d.print(ColorChoice::Auto)
    // }

    
    #[test]
    fn print_with_highlight_and_note() -> io::Result<()> {

        let source = Source::new_from_static_str(FileName::Test("test.wr"), indoc! {"
            func main() {
                wright::println(\"Hello World!\");
            }
        "});

        let frag = Fragment { source: SourceRef(Arc::new(source)), range: 0..12 };

        let d = Diagnostic::error("test")
            .with_code("TEST001")
            .with_primary_highlight(Highlight::new(frag, "main() defined here"))
            .with_note("This is a sample note.");

        d.print(ColorChoice::Auto)
    }
}
