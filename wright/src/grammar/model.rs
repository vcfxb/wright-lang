
use codespan::{Files, FileId, Span, ByteOffset, ByteIndex};

use nom::{AsBytes, Compare, CompareResult, ExtendInto, FindSubstring, FindToken, InputIter, InputLength, InputTake};

/// A piece of source code. Generally used to replace strings in the nom parser,
/// this structure stores extra information about the location of a fragment of
/// source code.
#[derive(Debug, Clone)]
pub struct Fragment<'source> {
    /// A reference to the parent Files object, which stores all source code
    /// being processed.
    pub files: &'source Files,
    handle: FileId,
    span: Span,
    /// The fragment of source code represented by this object.
    pub source: &'source str,
}

impl<'s> Fragment<'s> {
    /// Construct a new parser input from a handle into a
    /// [Files](https://docs.rs/codespan/0.5.0/codespan/struct.Files.html)
    /// object.
    pub fn new(files: &'s Files, handle: FileId) -> Self {
        let source = files.source(handle);
        let span = files.source_span(handle);
        Self {
            files,
            handle,
            span,
            source
        }
    }

    /// Get the span associated with this fragment of source code.
    #[inline]
    pub fn get_span(&self) -> Span {self.span}

    /// Get the ending index of this fragment.
    /// Identical to `self.get_span().end()`.
    #[inline]
    pub fn end(&self) -> ByteIndex {self.span.end()}

    /// Get the starting index of this fragment.
    /// Identical to `self.get_span().start()`.
    #[inline]
    pub fn start(&self) -> ByteIndex {self.span.start()}

    /// Get the length of this fragment.
    /// Identical to `self.source.len()`.
    #[inline]
    pub fn len(&self) -> usize {self.source.len()}

    /// Get the handle of this fragment's file in the parent
    /// [Files](https://docs.rs/codespan/0.5.0/codespan/struct.Files.html)
    /// object.
    pub fn get_handle(&self) -> FileId {self.handle}
}

impl<'s> AsBytes for Fragment<'s> {

    #[inline]
    fn as_bytes(&self) -> &[u8] {
        self.source.as_bytes()
    }
}

impl<'s, 'o, T> Compare<T> for Fragment<'s> where &'o str: Compare<T>, 's:'o {
    #[inline]
    fn compare(&self, t: T) -> CompareResult {
        self.source.compare(t)
    }

    #[inline]
    fn compare_no_case(&self, t: T) -> CompareResult {
        self.source.compare_no_case(t)
    }
}


impl<'s> ExtendInto for Fragment<'s> {
    type Item = char;
    type Extender = String;

    #[inline]
    fn new_builder(&self) -> String {
        String::new()
    }

    #[inline]
    fn extend_into(&self, acc: &mut Self::Extender) {
        self.source.extend_into(acc)
    }
}

impl<'s> FindSubstring<&str> for Fragment<'s> {

    #[inline]
    fn find_substring(&self, substr: &str) -> Option<usize> {
        self.source.find_substring(substr)
    }
}

impl<'s, 'o, T> FindToken<T> for Fragment<'s>
    where &'o str: FindToken<T>, 's:'o
{
    fn find_token(&self, t: T) -> bool {
        self.source.find_token(t)
    }
}

impl<'s> InputIter for Fragment<'s> {
    type Item = <&'s str as InputIter>::Item;
    type Iter = <&'s str as InputIter>::Iter;
    type IterElem = <&'s str as InputIter>::IterElem;

    #[inline]
    fn iter_indices(&self) -> Self::Iter {
        self.source.iter_indices()
    }

    #[inline]
    fn iter_elements(&self) -> Self::IterElem {
        self.source.iter_elements()
    }

    #[inline]
    fn position<P>(&self, predicate: P) -> Option<usize>
        where P: Fn(Self::Item) -> bool {
        self.source.position(predicate)
    }

    #[inline]
    fn slice_index(&self, count: usize) -> Option<usize> {
        self.source.slice_index(count)
    }
}

impl<'s> InputLength for Fragment<'s> {
    #[inline]
    fn input_len(&self) -> usize {self.len()}
}

impl<'s> InputTake for Fragment<'s> {
    fn take(&self, count: usize) -> Self {
        let mut frag2 = self.clone();
        frag2.source = &self.source[..count];
        frag2.span = Span::new(self.start(), self.start() + ByteOffset(count as i64));
        frag2
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        let mut frag2 = self.clone();
        let mut frag3 = self.clone();
        frag2.source = &self.source[..count];
        frag3.source = &self.source[count..];
        frag2.span = Span::new(self.start(), self.start() + ByteOffset(count as i64));
        frag3.span = Span::new(self.start() + ByteOffset(count as i64), self.end());
        (frag2, frag3)
    }
}

