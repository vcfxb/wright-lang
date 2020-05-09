use codespan::{ByteIndex, ByteOffset, FileId, Files, Span};

use nom::error::{ErrorKind, ParseError};
use nom::lib::std::ops::{Range, RangeFrom, RangeFull, RangeTo};
use nom::lib::std::str::FromStr;
use nom::Err;
use nom::{
    AsBytes, Compare, CompareResult, ExtendInto, FindSubstring, FindToken, IResult, InputIter,
    InputLength, InputTake, InputTakeAtPosition, Needed, Offset, ParseTo, Slice,
};
use crate::grammar::tracing::TraceInfo;
use crate::grammar::tracing::input::{OptionallyTraceable};
use std::fmt::Debug;

/// A piece of source code. Generally used to replace strings in the nom parser,
/// this structure stores extra information about the location of a fragment of
/// source code.
#[derive(Debug, Clone)]
pub struct Fragment<'source> {
    /// A reference to the parent Files object, which stores all source code
    /// being processed.
    files: &'source Files<String>,
    handle: FileId,
    span: Span,
    /// The fragment of source code represented by this object.
    source: &'source str,
    /// An optional additional field that traces the parsing of
    tracer: Option<TraceInfo>
}

/// An error when attempting to merge two fragments.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum FragmentError {
    /// Fragments are from different files, and cannot be merged.
    HandleMismatch,
    /// Fragments are not in the same `Files<String>` object, and cannot be merged.
    FilesRefMismatch,
}

impl<'s> Fragment<'s> {
    /// Construct a new parser input from a handle into a
    /// [Files](https://docs.rs/codespan/0.5.0/codespan/struct.Files.html)
    /// object.
    pub fn new(files: &'s Files<String>, handle: FileId) -> Self {
        let source = files.source(handle);
        let span = files.source_span(handle);
        Self {
            files,
            handle,
            span,
            source,
            tracer: None
        }
    }

    /// Enable parse tracing for this fragment.
    pub fn enable_trace(&mut self) {
        self.tracer = Some(TraceInfo::new());
    }

    /// Get the span associated with this fragment of source code.
    #[inline]
    pub fn get_span(&self) -> Span {
        self.span
    }

    /// Get the ending index of this fragment.
    /// Identical to `self.get_span().end()`.
    #[inline]
    pub fn end(&self) -> ByteIndex {
        self.span.end()
    }

    /// Get the starting index of this fragment.
    /// Identical to `self.get_span().start()`.
    #[inline]
    pub fn start(&self) -> ByteIndex {
        self.span.start()
    }

    /// Get the length of this fragment.
    /// Identical to `self.source.len()`.
    #[inline]
    pub fn len(&self) -> usize {
        self.source.len()
    }

    /// Get the source code of this fragment.
    #[inline]
    pub fn source(&self) -> &'s str {
        self.source
    }

    /// Get reference to files object.
    #[inline]
    pub fn files(&self) -> &'s Files<String> {
        self.files
    }

    /// Get the handle of this fragment's file in the parent
    /// [Files](https://docs.rs/codespan/0.5.0/codespan/struct.Files.html)
    /// object.
    #[inline]
    pub fn get_handle(&self) -> FileId {
        self.handle
    }

    /// Check whether two `Fragment`s overlap. returns true if and only if
    /// both Fragments are from the same `Files<String>` object, and both have
    /// the same handle, and have an area of overlap.
    #[inline]
    pub fn overlap(fst: &Self, snd: &Self) -> bool {
        fst.get_handle() == snd.get_handle()
            && std::ptr::eq(fst.files(), snd.files())
            && !fst.get_span().disjoint(snd.get_span())
    }
}

impl<'s> OptionallyTraceable for Fragment<'s> {
    fn trace_start(&mut self, tag: &'static str) {
        if self.tracer.is_some() {
            let mut t = self.tracer.clone().unwrap();
            t.start(tag);
            self.tracer = Some(t);
        }
    }

    fn trace_end(&mut self, tag: &'static str, success: bool) {
        if self.tracer.is_some() {
            let mut t = self.tracer.clone().unwrap();
            t.end(tag, success);
            self.tracer = Some(t);
        }
    }

    fn get_trace(&self) -> Option<TraceInfo> {
        self.tracer.clone()
    }
}


impl<'s> AsBytes for Fragment<'s> {
    #[inline]
    fn as_bytes(&self) -> &[u8] {
        self.source().as_bytes()
    }
}

impl<'s, 'o, T> Compare<T> for Fragment<'s>
where
    &'o str: Compare<T>,
    's: 'o,
{
    #[inline]
    fn compare(&self, t: T) -> CompareResult {
        self.source().compare(t)
    }

    #[inline]
    fn compare_no_case(&self, t: T) -> CompareResult {
        self.source().compare_no_case(t)
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
        self.source().extend_into(acc)
    }
}

impl<'s> FindSubstring<&str> for Fragment<'s> {
    #[inline]
    fn find_substring(&self, substr: &str) -> Option<usize> {
        self.source().find_substring(substr)
    }
}

impl<'s, 'o, T> FindToken<T> for Fragment<'s>
where
    &'o str: FindToken<T>,
    's: 'o,
{
    #[inline]
    fn find_token(&self, t: T) -> bool {
        self.source().find_token(t)
    }
}

impl<'s> InputIter for Fragment<'s> {
    type Item = <&'s str as InputIter>::Item;
    type Iter = <&'s str as InputIter>::Iter;
    type IterElem = <&'s str as InputIter>::IterElem;

    #[inline]
    fn iter_indices(&self) -> Self::Iter {
        self.source().iter_indices()
    }

    #[inline]
    fn iter_elements(&self) -> Self::IterElem {
        self.source().iter_elements()
    }

    #[inline]
    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.source().position(predicate)
    }

    #[inline]
    fn slice_index(&self, count: usize) -> Option<usize> {
        self.source().slice_index(count)
    }
}

impl<'s> InputLength for Fragment<'s> {
    #[inline]
    fn input_len(&self) -> usize {
        self.len()
    }
}

impl<'s> InputTake for Fragment<'s> {
    fn take(&self, count: usize) -> Self {
        let mut frag2 = self.clone();
        frag2.source = &self.source()[..count];
        frag2.span = Span::new(self.start(), self.start() + ByteOffset(count as i64));
        frag2
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        let mut frag2 = self.clone();
        let mut frag3 = self.clone();
        frag2.source = &self.source()[..count];
        frag3.source = &self.source()[count..];
        frag2.span = Span::new(self.start(), self.start() + ByteOffset(count as i64));
        frag3.span = Span::new(self.start() + ByteOffset(count as i64), self.end());
        (frag3, frag2) // (part after, part before)
    }
}

impl<'s> InputTakeAtPosition for Fragment<'s> {
    type Item = char;

    fn split_at_position<P, E: ParseError<Self>>(&self, predicate: P) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.source().find(predicate) {
            Some(i) => Ok(self.take_split(i)),
            None => Err(Err::Incomplete(Needed::Size(1))),
        }
    }

    fn split_at_position1<P, E: ParseError<Self>>(
        &self,
        predicate: P,
        e: ErrorKind,
    ) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.source().find(predicate) {
            Some(0) => Err(Err::Error(E::from_error_kind(self.clone(), e))),
            Some(i) => Ok(self.take_split(i)),
            None => Err(Err::Incomplete(Needed::Size(1))),
        }
    }

    fn split_at_position_complete<P, E: ParseError<Self>>(
        &self,
        predicate: P,
    ) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.source().find(predicate) {
            Some(i) => Ok(self.take_split(i)),
            None => Ok(self.take_split(self.input_len())),
        }
    }

    fn split_at_position1_complete<P, E: ParseError<Self>>(
        &self,
        predicate: P,
        e: ErrorKind,
    ) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.source().find(predicate) {
            Some(0) => Err(Err::Error(E::from_error_kind(self.clone(), e))),
            Some(i) => Ok(self.take_split(i)),
            None => {
                if self.len() == 0 {
                    Err(Err::Error(E::from_error_kind(self.clone(), e)))
                } else {
                    Ok(self.take_split(self.input_len()))
                }
            }
        }
    }
}

impl<'s> Offset for Fragment<'s> {
    fn offset(&self, second: &Self) -> usize {
        (second.span.start() - self.span.start()).0 as usize
    }
}

impl<'s, R: FromStr> ParseTo<R> for Fragment<'s> {
    #[inline]
    fn parse_to(&self) -> Option<R> {
        self.source().parse_to()
    }
}

impl<'s> Slice<Range<usize>> for Fragment<'s> {
    fn slice(&self, range: Range<usize>) -> Self {
        let mut result = self.clone();
        result.source = &self.source()[range.clone()];
        result.span = Span::new(
            self.span.start() + ByteOffset(range.start as i64),
            self.span.start() + ByteOffset(range.end as i64),
        );
        result
    }
}

impl<'s> Slice<RangeFrom<usize>> for Fragment<'s> {
    fn slice(&self, range: RangeFrom<usize>) -> Self {
        self.slice(range.start..self.len())
    }
}

impl<'s> Slice<RangeTo<usize>> for Fragment<'s> {
    fn slice(&self, range: RangeTo<usize>) -> Self {
        self.slice(0..range.end)
    }
}

impl<'s> Slice<RangeFull> for Fragment<'s> {
    fn slice(&self, _: RangeFull) -> Self {
        self.clone()
    }
}

/// Trait for all types that have associated fragments in source code.
pub trait HasSourceReference<SourceCodeReference: Clone + Debug> {
    /// Get reference to the associated fragment of source code.
    fn get_source_ref(&self) -> &SourceCodeReference;

    /// Get a clone of the associated fragment of source code.
    fn get_source_clone(&self) -> SourceCodeReference {
        self.get_source_ref().clone()
    }
}

impl<'s> Into<String> for Fragment<'s> {
    fn into(self) -> String {self.source().to_owned()}
}

