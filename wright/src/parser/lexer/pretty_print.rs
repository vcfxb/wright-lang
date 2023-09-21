//! Lexer pretty printer.

use crate::parser::lexer::{IndexedLexer, IndexedToken};

use super::Lexer;
use codespan_reporting::files::{Files, SimpleFile};
use std::cmp;
use std::io::Write;
use std::{fmt::Display, ops::Range};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(Default)]
struct PrettyPrinter {
    print_lines: [String; 2],
}

impl PrettyPrinter {
    fn flush(&mut self) -> anyhow::Result<()> {
        // Use termcolor to print in different colors.
        let mut out = StandardStream::stdout(ColorChoice::Always);
        // Print source code default.
        writeln!(&mut out, "{}", self.print_lines[0])?;
        // Print token info in cyan
        out.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
        writeln!(&mut out, "{}", self.print_lines[1])?;
        // Reset after printing is over.
        out.set_color(ColorSpec::new().set_reset(true))?;
        // Reset the print_lines.
        self.print_lines = [Default::default(), Default::default()];
        Ok(())
    }
}

impl<'a> Lexer<'a> {
    /// Print in pretty format the source code and the tokens it matched to under it.
    pub fn debug_pretty_print<Name: Display + Clone, Source: AsRef<str>>(
        source: &SimpleFile<Name, Source>,
    ) -> anyhow::Result<()> {
        // Create a pretty-printer to use for colored text
        let mut pp: PrettyPrinter = Default::default();
        // Print a header line to indicate columns.
        println!("line ({:#10})", "byte");
        // Get the source code as a str ref.
        let source_str: &str = source.source().as_ref();
        // Get the token iterator for the source code.
        let mut token_iter = IndexedLexer::new(source_str)
            // Go from byte start indices to byte ranges in the source string
            .map(|IndexedToken { index, token }| (index..index + token.length, token))
            // Make it peekable so that we can consume the iterator conditionally
            .peekable();

        // Make an iterator over the line byte-index ranges.
        let mut line_range_iter = source_str
            .lines()
            // Use enumerate to get line indices.
            .enumerate()
            // Get line byte-index ranges for each line. Use `.unwrap()` beacause
            // all the indices out of enumerate should be valid.
            .map(|(line_index, _)| (line_index, source.line_range((), line_index).unwrap()))
            // Use `.peekable()` to make it conditionally consubable.
            .peekable();

        // Make a utility function to get the matching source for a byte-index range.
        let matching_source = |range: Range<usize>| -> String {
            source_str[range]
                // Use `.replace()` to make sure tabs are printed in the
                // same width in a predictable way.
                .replace('\t', "    ")
                // Also use replace to avoid double-printing newline characters if they exist.
                // Do replace them with a space though, to avoid underflow on subtraction in formatting.
                .replace(['\r', '\n'], " ")
        };

        // Iterate on the lines of the source file.
        while let Some((line_index, line_range)) = line_range_iter.next() {
            // Set the print headers if empty.
            if pp.print_lines[0].is_empty() {
                pp.print_lines[0] = format!("{:04} ({:#010x}): ", line_index, line_range.start);
                pp.print_lines[1] = format!("{:04} ({:#010x}): ", line_index, line_range.start);
            }

            // Consume all tokens that end (and therefore start also) on this line.
            while let Some((token_range, token)) =
                token_iter.next_if(|(token_range, _)| token_range.end <= line_range.end)
            {
                // Get the matching source code for the token.
                let matched = matching_source(token_range);

                // Make a string representation of the token to print in the debug.
                let token_string: String = token.to_string();

                // Get the width of the display as the max of the two string character (not byte) lengths. Add two to the
                // token length to represent the square brackets added later.
                let width: usize =
                    cmp::max(token_string.chars().count() + 2, matched.chars().count());

                // Add source to first line and token info to second line as appopriate. Add two to the source with for the
                // square brackets.
                pp.print_lines[0].push_str(format!("{matched:<width$}").as_str());
                pp.print_lines[1].push_str(format!("[{token_string:<0$}]", width - 2).as_str());
            }

            // Check if the next available token still starts on this line (it definitely ends later than this line).
            if let Some((token_range, token)) =
                token_iter.next_if(|(token_range, _)| token_range.start < line_range.end)
            {
                // The token ends on the next line (or later) so limit the matching source range to
                // the end of this line.
                let matching_source_range = token_range.start..line_range.end;
                // Get the matching source code for the part of the token that's on this line.
                let matched: String = matching_source(matching_source_range);
                // Make a string representation of the token to print in the debug.
                let token_string: String = token.to_string();

                // Get the width of the display as the max of the two string character (not byte) lengths. Add one to the
                // token length to represent the square bracket added later.
                let width: usize =
                    cmp::max(token_string.chars().count() + 1, matched.chars().count());

                // Add source to first line and token info to second line as appopriate.
                pp.print_lines[0].push_str(format!("{matched:<width$}").as_str());
                pp.print_lines[1].push_str(format!("[{token_string:<0$}", width - 1).as_str());

                // Flush the print_lines.
                pp.flush()?;

                // Keep flushing lines until we reach the end of the multi-line token.
                while let Some((add_line_index, add_line_range)) = line_range_iter
                    .next_if(|(_, add_line_range)| add_line_range.end < token_range.end)
                {
                    let matched = matching_source(add_line_range.clone());
                    // Print the line of source
                    pp.print_lines[0] = format!(
                        "{:04} ({:#010x}): {matched}",
                        add_line_index, add_line_range.start
                    );
                    pp.print_lines[1] =
                        format!("{:04} ({:#010x}):", add_line_index, add_line_range.start);
                    pp.flush()?;
                }

                // Finally if the token is partially in another line, begin that one too.
                // Not that this does not consume the next line off the line iterator.
                // The next iteration of the loop will do that.
                let continues_on_next_line = line_range_iter
                    .peek()
                    .filter(|(_, add_line_range)| token_range.end > add_line_range.start);

                // If it does continue on the next line, start the line.
                if let Some((add_line_index, add_line_range)) = continues_on_next_line {
                    // Get the matching source.
                    let matching_source_range: Range<usize> = add_line_range.start..token_range.end;
                    let matched: String = matching_source(matching_source_range);

                    // Calculate the number of spaces to put before the closing bracket.
                    let space = matched.chars().count() - 1;

                    // Add the match and the closing bracket.
                    pp.print_lines[0] = format!(
                        "{:04} ({:#010x}): {matched}",
                        add_line_index, add_line_range.start
                    );
                    pp.print_lines[1] = format!(
                        "{:04} ({:#010x}): {:space$}]",
                        add_line_index, add_line_range.start, ""
                    );
                }
            } else {
                // The next token is on the next line, just flush the print_lines and move on.
                pp.flush()?;
            }
        }

        Ok(())
    }
}
