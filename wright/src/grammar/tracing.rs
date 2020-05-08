use std::io;
use io::Write;
use termcolor::{ColorChoice, Color, StandardStream, ColorSpec, WriteColor};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct TraceRecord {
    depth: usize,
    tag: &'static str,
    success: Option<bool>
}

impl TraceRecord {
    /// Construct a new trace record.
    fn new(depth: usize, tag: &'static str) -> Self {
        Self {
            depth, tag, success: None
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
    inner: Vec<TraceRecord>
}

impl TraceInfo {
    /// Generate a new TraceInfo.
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the depth of this tracing object.
    pub fn get_depth(&self) -> usize {
        self.depth
    }

    /// Record the start of a function call.
    pub fn start(&mut self, tag: &'static str) {
        self.depth += 1;
        self.inner.push(TraceRecord::new(self.depth, tag));
    }

    /// try to reverse find from a specific index.
    /// will not include the specified index.
    /// searches for another record of the same depth and tag.
    fn rfind(&self, index: usize) -> Option<usize> {
        if index >= 1 && index < self.inner.len() {
            let depth = self.inner[index].depth;
            let tag = self.inner[index].tag;

            let mut i = index-1;
            // iterate backwards through record history
            // and try to find one with matching depth and tag.
            while i > 0 {
                let r = &self.inner[i];
                // if found, return.
                if r.depth == depth && r.tag == tag {
                    return Some(i);
                }
                i -= 1;
            }
            None
        } else {None}
    }

    /// Record the end of function call.
    ///
    /// If a previous function call can be found of the
    /// same name and depth as this one, it will be labeled
    /// with the same success marker.
    ///
    /// ## Panics:
    /// -- if depth is 0.
    pub fn end(&mut self, tag: &'static str, success: bool) {
        assert_ne!(self.depth, 0);
        self.inner.push(TraceRecord::new(self.depth, tag).success(success));

        // attempt to label the matching record.
        if let Some(i) = self.rfind(self.inner.len()-1) {
            self.inner[i].success = Some(success);
        }

        self.depth -= 1;
    }

    /// Print this trace to the standard output.
    ///
    /// We use [atty](https://crates.io/crates/atty)
    /// and [termcolor](https://crates.io/crates/termcolor)
    /// to do our best to determine when to color the output.
    pub fn print(&self) -> io::Result<()> {
        let color_config = if atty::is(atty::Stream::Stdout) {
            ColorChoice::Auto
        } else {ColorChoice::Never};

        // use termsize to try to determine width of terminal so that we avoid
        // ugly text wrapping.
        let term_width = term_size::dimensions()
            .map(|(w,h)| w);

        let mut stdout = StandardStream::stdout(color_config);

        // write header
        writeln!(&mut stdout, "|{:>7} |{:>7} | {}", "->", "<-", "parser")?;

        // color specification.
        let mut success_spec = ColorSpec::new();
        let mut failure_spec = ColorSpec::new();
        success_spec.set_fg(Some(Color::Green));
        failure_spec.set_fg(Some(Color::Red));
        success_spec.set_intense(true);
        failure_spec.set_intense(true);

        // record tagging.
        let mut tagged_records = Vec::new();

        let mut prev_depth = 0;
        let mut fw: usize = 0;
        let mut index = 0;

        for record in self.inner.iter() {
            // is this entry into a parser a function call
            let forwards = record.depth >= prev_depth;
            if forwards {
                fw += 1;
                tagged_records.push((fw, record));
            } else {
                let t = self
                    .rfind(index)
                    .map(|i| tagged_records[i].0)
                    .unwrap_or(0);
                tagged_records.push((t, record));
            }
            index += 1;
            prev_depth = record.depth;
        }

        prev_depth = 0;

        for (level, record)  in tagged_records {
            let forwards = record.depth >= prev_depth;

            // write call level info.
            write!(&mut stdout, "|{:>7} |{:>7} |",
                if forwards {level.to_string()} else {"".to_owned()},

                if !forwards && level == 0 {"---".to_owned()}
                else if !forwards {level.to_string()}
                else {"".to_owned()}
            )?;

            // get the appropriate color spec.
            let spec = record.success
                .map(|b| if b {&success_spec} else {&failure_spec});

            if spec.is_some() {
                stdout.set_color(spec.unwrap())?;
            }

            // 19 should be the amount of space used to write call level info.
            let text_width = record.tag.len();
            // filters terminal width to only be `Some` if it limits the
            // natural output display. Whenever it is some, it will be the required
            // width of the whitespace and arrow.
            let limiting_term_width = term_width
                .filter(|w| *w<19+record.depth-1+"-> ".len()+text_width)
                .filter(|w| *w >= 22+text_width)
                .map(|w| w-19-text_width-1);
            write!(&mut stdout, "{1:>0$} ",
                limiting_term_width.unwrap_or(record.depth-1+2),
                if forwards {"-> "} else {"<- "}
            )?;
            stdout.reset()?;
            writeln!(&mut stdout, "{}", record.tag)?;
        }

        Ok(())
    }
}