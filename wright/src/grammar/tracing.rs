use std::io;
use io::Write;
use std::collections::HashMap;
use termcolor::{ColorChoice, Color, StandardStream, ColorSpec, WriteColor};
use nom::IResult;
use crate::grammar::tracing::input::OptionallyTraceable;

/// Traced versions of nom and wright parsers. These
/// are currently implemented on an as used / as needed basis,
/// so if there are some missing, you would like implemented,
/// or you want the implementation to be more generic,
/// make a pull request.
/// [here](https://github.com/Wright-Language-Developers/Wright-lang/pull/new/master).
pub mod parsers;

/// Module for defining parser input accross all wright and nom parsers.
pub mod input;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct TraceRecord {
    forwards: bool,
    id: usize,
    depth: usize,
    tag: &'static str,
    success: Option<bool>
}

impl TraceRecord {
    /// Construct a new trace record.
    fn new(depth: usize, tag: &'static str, forwards: bool, id: usize) -> Self {
        Self {
            forwards,
            id,
            depth,
            tag,
            success: None,
        }
    }

    /// Label whether a trace record succeeded or not.
    fn success(mut self, l: bool) -> Self {
        self.success = Some(l);
        self
    }
}

/// This object traces the
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TraceInfo {
    /// The current trace depth.
    depth: usize,
    /// Trace including depth and function name.
    inner: Vec<TraceRecord>,
    /// the next id to be assigned.
    next_id: usize,
    /// Active unterminated function calls.
    /// (tag, depth) -> (id, index)
    active_ids: HashMap<(&'static str, usize), (usize, usize)>
}

impl TraceInfo {
    /// Generate a new TraceInfo.
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the length of the trace history.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Get the next available id.
    fn get_next_id(&mut self) -> usize {
        self.next_id += 1;
        self.next_id - 1
    }

    /// Record the start of a function call.
    pub fn start(&mut self, tag: &'static str) {
        self.depth += 1;
        let id = self.get_next_id();
        self.active_ids.insert((tag, self.depth), (id, self.inner.len()));
        self.inner.push(TraceRecord::new(self.depth, tag, true, id));
    }

    /// Record the end of function call.
    ///
    /// ## Panics:
    /// - if depth is 0.
    /// - if a matching function start record was not found.
    pub fn end(&mut self, tag: &'static str, success: bool) {
        assert_ne!(self.depth, 0);

        let (id, index) = self.active_ids.remove(&(tag, self.depth))
            .expect("no matching function start found.");

        self.inner.push(
            TraceRecord::new(self.depth, tag, false, id).success(success));

        self.inner[index].success = Some(success); // set the success of the matching start call.

        self.depth -= 1;
    }

    /// Print this trace to the standard output.
    ///
    /// We use [atty](https://crates.io/crates/atty)
    /// and [termcolor](https://crates.io/crates/termcolor)
    /// to do our best to determine when to color the output.
    ///
    /// This also uses conditional compilation to ensure that
    /// we still reach the standard output in testing environments
    /// (when `cfg!(test)` is true).
    pub fn print(&self) -> io::Result<()> {
        let color_config = if atty::is(atty::Stream::Stdout) {
            ColorChoice::Auto
        } else {ColorChoice::Never};

        // use termsize to try to determine width of terminal so that we avoid
        // ugly text wrapping.
        let term_width = term_size::dimensions()
            .map(|(w, _)| w);

        let mut stdout = StandardStream::stdout(color_config);

        // write header
        #[cfg(not(test))]
        writeln!(&mut stdout, "|{0:>7} |{1:>7} | {2}\n:{3}:{3}:{3}", "->", "<-", "parser", "-".repeat(8))?;
        #[cfg(test)]
        println!("|{0:>7} |{1:>7} | {2}\n:{3}:{3}:{3}", "->", "<-", "parser", "-".repeat(8));

        // color specification.
        let mut success_spec = ColorSpec::new();
        let mut failure_spec = ColorSpec::new();
        success_spec.set_fg(Some(Color::Green));
        failure_spec.set_fg(Some(Color::Red));
        success_spec.set_intense(true);
        failure_spec.set_intense(true);

        for record in self.inner.iter() {
            let labels = (
                if record.forwards {record.id.to_string()} else {"".to_owned()},
                if !record.forwards {record.id.to_string()} else {"".to_owned()}
            );

            #[cfg(not(test))]
            write!(&mut stdout, "|{:>7} |{:>7} |", labels.0, labels.1)?;

            #[cfg(test)]
            print!("|{:>7} |{:>7} |", labels.0, labels.1);

            // get the appropriate color spec.
            let spec = record.success
                .map(|b| if b {&success_spec} else {&failure_spec});

            if spec.is_some() {
                stdout.set_color(spec.unwrap())?;
            }

            // 19 should be the amount of space used to write call level info.
            let text_width = record.tag.len() + 4; // add for for a (✓) or (x)
            // filters terminal width to only be `Some` if it limits the
            // natural output display. Whenever it is some, it will be the required
            // width of the whitespace and arrow.
            let limiting_term_width = term_width
                .filter(|w| *w<19+record.depth-1+"-> ".len()+text_width)
                .filter(|w| *w >= 22+text_width)
                .map(|w| w-19-text_width-1);

            let spaces = limiting_term_width.unwrap_or(record.depth-1);

            let string =
                format!("{}{}", " ".repeat(spaces+1), if record.forwards {"->"} else {"<-"});

            #[cfg(not(test))]
            write!(&mut stdout, "{} ", string)?;

            #[cfg(test)]
            print!("{} ", string);

            stdout.reset()?;

            let success_str = record.success
                .map(|s| if s {"(✓)"} else {"(x)"})
                .unwrap_or("( )");

            #[cfg(not(test))]
            writeln!(&mut stdout, "{} {}", record.tag, success_str)?;

            #[cfg(test)]
            println!("{} {}", record.tag, success_str);
        }

        Ok(())
    }
}

// FIXME link
/// Function to automatically apply tracing information to the
/// remainder and error branches of a nom parser result, or [`IResult`]()
pub fn trace_result<I: OptionallyTraceable, O>(tag: &'static str, res: IResult<I, O>) -> IResult<I, O> {
    res.map(|(r, p)| (r.trace_end_clone(tag, true), p))
        .map_err(|err| err.map_input(|i: I| i.trace_end_clone(tag, false)))
}