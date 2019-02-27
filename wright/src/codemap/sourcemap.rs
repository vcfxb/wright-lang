use crate::codemap;
use codemap::charspan::*;

use std::path::PathBuf;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use std::borrow::Cow;

/// The type of the source, or the source's origin.
#[derive(Clone, Debug)]
pub enum SourceType {
    /// A file on the disk.
    Real(PathBuf),
    /// A virtual file, either a tasting file or an internally generated file
    Virtual(String),
}

/// A piece of source code.
#[derive(Clone, Debug)]
pub struct Source<'source> {
    /// The name and origin of the source.
    pub name: SourceType,
    /// The content of the source, as a vector of characters
    /// (because I don't want to deal with variable-width characters of utf-8.)
    pub content: Vec<char>,
    /// Span within the `CodeMap` that corresponds to this source.
    /// This will be assigned to the Source by the code map it is a part of.
    pub(super) span: CharSpan,
    /// A list of spans corresponding to each line, useful
    /// for efficient error reporting and stuff.
    pub(super) line_index: Cow<'source, Vec<CharSpan>>,
}

impl Display for SourceType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            SourceType::Virtual(s) => write!(f, "{}", s),
            SourceType::Real(pb) => write!(f, "{}", pb.to_string_lossy())
        }
    }
}

impl Source {
    /// Only for this crate.
    ///
    /// This function should be called every time one of these is constructed to
    /// generate newline indexes.
    pub(super) fn generate_indexes(mut self) -> Self {
        if self.content.is_empty() {return self}
        let mut index1 = self.span.start;
        let mut index2 = self.span.start+1;
        for c in &self.content {
            if *c == '\n' {
                self.line_index.push(CharSpan::new(index1, index2));
                index1 = index2;
                index2 = index1 + 1;
            } else {
                index2 += 1;
            }
        }
        if *self.content.last().unwrap() != '\n' {
            self.line_index.push(CharSpan::new(index1, index2));
        }
        self
    }

    /// Get `(line, col)` location in a file from a `CharIndex`.
    ///
    /// Both `line` and `col` are 1 indexed.
    ///
    /// Returns `None` if `loc` is not in this source.
    /// Lines and columns are both 1 indexed.
    ///
    /// ### Panics:
    /// - If internal state is broken.
    pub fn location(&self, loc: CharIndex) -> Option<(u32, u32)> {
        if !self.span.contains(loc) {None}
        let mut line: u32 = 1;
        for line_span in &self.line_index {
            if line_span.contains(loc) {
                return Some((line, loc - line_span.start + 1))
            } else { line+=1; }
        }
        panic!("Bad state; location not found in line_indexes: \n\
            loc: {}, \n\
            span:{:?},\n\
            line_index: {:?}", loc, self.span, self.line_index);
    }

    /// Get line's `CharSpan` at a given line number.
    /// Returns `None` if the line number is greater than the number of lines in
    /// the source.
    ///
    /// `line_num` is 1 indexed.
    pub fn line_span(&self, line_num: u32) -> Option<CharSpan> {
        if self.line_index.len() < line_num || line_num == 0 {None}
        else {Some(self.line_index[line_num-1])}
    }

    /// The number of lines in this source.
    /// O(1) due to internal use of Vec::len().
    pub fn num_lines(&self) -> u32 {self.line_index.len() as u32}

    /// Returns a clone on write vector of spans, in order, corresponding to the
    /// lines in the source.
    pub fn line_spans(&self) -> Cow<Vec<CharSpan>> {Cow::Borrowed(&self.line_index)}
}