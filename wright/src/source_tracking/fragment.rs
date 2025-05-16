//! [Fragment] struct and implementation for dealing with fragments of source code.

use super::SourceRef;
use derive_more::Display;
use std::{ops::Range, str::Chars, sync::Arc};

#[cfg(doc)]
use crate::source_tracking::source::Source;

/// A fragment of source code.
///
/// This can be part of (or all of) a [Source].
#[derive(Clone, Debug, Display)]
#[display("{}", "self.as_str()")]
pub struct Fragment {
    /// The [Source] that this fragment is in.
    pub source: SourceRef,

    /// Fragments are represented using byte ranges in the [Source] referenced by [Fragment::source].
    ///
    /// This [Fragment] is considered invalid if this range is out of order or either end of it is not
    /// on a char boundary in source according to [str::is_char_boundary].
    pub range: Range<usize>,
}

impl Fragment {
    /// Check that this [Fragment] is valid, and references a real existing (though possibly empty) part of
    /// the [Fragment::source].
    pub fn is_valid(&self) -> bool {
        // Get a string reference to the whole source.
        let source_as_str: &str = self.source.source().as_ref();

        // Check validity.
        self.range.end >= self.range.start
            && source_as_str.is_char_boundary(self.range.start)
            && source_as_str.is_char_boundary(self.range.end)
    }

    /// Get the [str] represented by this [Fragment].
    ///
    /// # Panics
    /// - This will [panic] in the unlikely event that [Fragment::range] is out of bounds or lands between char
    ///     boundaries for [Fragment::source].
    pub fn as_str(&self) -> &str {
        &self.source.source().as_str()[self.range.clone()]
    }

    /// Get the length (in bytes) of this [Fragment].
    /// Does not check this [Fragment] for validity.
    pub const fn len(&self) -> usize {
        self.range.end.saturating_sub(self.range.start)
    }

    /// Check if this fragment has a [`Fragment::len`] `== 0`.
    /// Does not check this [Fragment] for validity.
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Check if this fragment is empty at the end of it's source.
    ///
    /// Uses [debug_assert] to check for validity.
    pub fn is_empty_at_end_of_source(&self) -> bool {
        debug_assert!(self.is_valid());

        self.source.source().as_str().len() <= self.range.start
    }

    /// Return true if this [Fragment] entirely contains another [Fragment] and they're from the same [Source] by
    /// [Source::id].
    ///
    /// If `other` is empty, it can still be considered to be contained in this [Fragment] if its
    /// [Fragment::range] is entirely within `self`'s [Fragment::range] (basically whether the location of the empty
    /// fragment is in this one).
    pub fn contains(&self, other: &Self) -> bool {
        self.source.id == other.source.id
            && self.range.start <= other.range.start
            && self.range.end >= other.range.end
    }

    /// Get the number of bytes between the beginning of `origin` and the beginning of `self`.
    ///
    /// # Panics:
    /// - Panics if `self` is not a [Fragment] within `origin` according to [`Fragment::contains`].
    pub fn offset_from(&self, origin: &Self) -> usize {
        if !origin.contains(self) {
            panic!("This fragment must be contained in the original fragment");
        }

        self.range.start - origin.range.start
    }

    /// Get a [Chars] [Iterator] over the [char]acters in this [Fragment].
    pub fn chars(&self) -> Chars<'_> {
        self.as_str().chars()
    }

    /// Get a sub-fragment of this fragment (see [Fragment::contains]) with the whitespace at either end trimmed off.
    /// This will return the fragment unchanged if it is empty.
    ///
    /// This calls [Fragment::trim_start] and then [Fragment::trim_end] internally and should match the behavior of
    /// [str::trim].
    ///
    /// If this returns an empty [Fragment] it will be at the end of the parent [Fragment].
    pub fn trimmed(self) -> Self {
        self.trim_start().trim_end()
    }

    /// Get a sub-fragment of this fragment (see [Fragment::contains]) with the whitespace trimmed off the end.
    /// This will return it unchanged if empty.
    ///
    /// See [str::trim_end] for exact behaviors.
    pub fn trim_end(mut self) -> Self {
        // Get the string representation of this fragment.
        let original_str: &str = self.as_str();
        // Trim it.
        let trimmed_str: &str = original_str.trim_end();
        // Calculate the new end of the range.
        let new_end: usize = self.range.start + trimmed_str.len();
        // Update self.
        self.range = self.range.start..new_end;
        // Return the updated self.
        self
    }

    /// Get a sub-fragment of this fragment (see [Fragment::contains]) with the whitespace trimmed off the start.
    /// This will return it unchanged if empty.
    ///
    /// See [str::trim_start] for exact behaviors.
    pub fn trim_start(mut self) -> Self {
        // Get the string representation of this fragment.
        let original_str: &str = self.as_str();
        // Trim it.
        let trimmed_str: &str = original_str.trim_start();
        // Calculate the new start of the range.
        let new_start: usize = self.range.end - trimmed_str.len();
        // Update self.
        self.range = new_start..self.range.end;
        // Return the updated self.
        self
    }

    /// Split this [Fragment] into two sub-[Fragment]s, the left containing the first `bytes_from_start`
    /// bytes, and the right containing the rest.
    ///
    /// # Panics
    /// - This will panic if the provided `bytes_from_start` does not land on a unicode character boundary or is larger
    ///     than the length of this fragment according to [str::is_char_boundary].
    pub fn split_at(&self, bytes_from_start: usize) -> (Self, Self) {
        // Check boundaries.
        if !self.as_str().is_char_boundary(bytes_from_start) {
            panic!("Cannot split in the middle of a unicode character");
        }

        self.split_at_unchecked(bytes_from_start)
    }

    /// This is the same as [Fragment::split_at] except it does not check that the created fragments are valid or
    /// that either can call [Fragment::as_str] without panicking.
    /// Use with caution.
    ///
    /// Note that this is not technically `unsafe`, since all bugs that may emerge from abuse/misuse here are logic
    /// bugs (not memory or concurrency bugs).
    pub fn split_at_unchecked(&self, bytes_from_start: usize) -> (Self, Self) {
        // Calculate ranges.
        let left_range: Range<usize> = self.range.start..(self.range.start + bytes_from_start);
        let right_range: Range<usize> = (self.range.start + bytes_from_start)..self.range.end;

        // Construct fragments.
        (
            Fragment {
                source: self.source.clone(),
                range: left_range,
            },
            Fragment {
                source: self.source.clone(),
                range: right_range,
            },
        )
    }

    /// Move the start of this [Fragment] forward by a given number of bytes.
    ///
    /// # Panics
    /// - Panics if the advancing by `bytes` would create an invalid [Fragment].
    pub fn advance_by(&mut self, bytes: usize) {
        // Bounds check.
        if !self.as_str().is_char_boundary(bytes) {
            panic!("Advancing by {bytes} bytes would create an invalid fragment.");
        }

        self.advance_by_unchecked(bytes);
    }

    /// This is the same as [Fragment::advance_by] except without the bounds checking. Use carefully or the updated
    /// [Fragment] will be invalid.
    #[inline]
    pub fn advance_by_unchecked(&mut self, bytes: usize) {
        self.range.start += bytes;
    }

    /// Retain up to `bytes` bytes of this [Fragment].
    ///
    /// # Panics
    /// - Panics if the updated [Fragment] would be invalid.
    pub fn retain(&mut self, bytes: usize) {
        // Bounds check.
        if !self.as_str().is_char_boundary(bytes) {
            panic!("Retaining to {bytes} bytes would create an invalid fragment.");
        }

        self.retain_unchecked(bytes);
    }

    /// This is the same as [Fragment::retain] except without the bounds checking. Use carefully or the updated
    /// [Fragment] will be invalid.
    #[inline]
    pub fn retain_unchecked(&mut self, bytes: usize) {
        self.range.end = self.range.start + bytes;
    }

    /// Get a [Range] of line indices (0-indexed, see [Source::get_line]) that this fragment overlaps.
    pub fn line_indices(&self) -> Range<usize> {
        let start_line_index: usize = self.source.line_index(self.range.start);

        // Subtract one when doing the end because if this fragment ends at the end of a line, we don't want to include
        // the next line (obo -- range is exclusive).
        let ending_line_index: usize = self.source.line_index(self.range.end - 1);

        // Return the range.
        start_line_index..ending_line_index
    }

    /// Get the line number (not index) that this line starts on.
    ///
    /// This re-calculates [Fragment::line_indices], which may be expensive on very large files, so use with care.
    pub fn starts_on_line(&self) -> usize {
        self.line_indices().start + 1
    }

    /// Get the number of bytes between the start of the line that this [Fragment] starts on and the start of this
    /// [Fragment]
    pub fn starting_col_index(&self) -> usize {
        let line_start_index = Arc::clone(&self.source)
            .get_line(self.line_indices().start)
            .range
            .start;

        self.range.start - line_start_index
    }

    /// Compute the "cover" over two [Fragment]s -- that is, the [Fragment] containing both of them and all the source
    /// code in between.
    ///
    /// # Panics
    /// - If the [Fragment]s are from different sources.
    pub fn cover(lhs: &Fragment, rhs: &Fragment) -> Fragment {
        use std::cmp;
        assert_eq!(lhs.source.id, rhs.source.id, "fragments must be from same source");

        Fragment {
            source: Arc::clone(&lhs.source),
            range: cmp::min(lhs.range.start, rhs.range.start)
                ..cmp::max(lhs.range.end, rhs.range.end),
        }
    }
}

impl PartialEq for Fragment {
    /// Fragment equality is based on referencing the same [Source] using [Arc::ptr_eq] and having the same
    /// [Fragment::range].
    fn eq(&self, other: &Self) -> bool {
        self.source.id == other.source.id && self.range == other.range
    }
}

impl Eq for Fragment {}

#[cfg(test)]
mod tests {
    use super::Fragment;
    use crate::source_tracking::{filename::FileName, source::Source};
    use std::sync::Arc;

    /// Utility function to create a one-off fragment over a static string.
    fn from_static(s: &'static str) -> Fragment {
        let source = Source::new_from_static_str(FileName::None, s);
        let arc = Arc::new(source);

        Fragment {
            range: 0..arc.source().as_ref().len(),
            source: arc,
        }
    }

    #[test]
    fn test_split_single() {
        let a = from_static("+");
        let (left, right) = a.split_at(1);
        assert_eq!(left.as_str(), "+");
        assert_eq!(right.as_str(), "");
    }

    #[test]
    fn test_offset_from() {
        let a = from_static("abcde");
        let (b, c) = a.split_at(2);
        assert_eq!(b.offset_from(&a), 0);
        assert_eq!(c.offset_from(&a), 2);
    }

    #[test]
    #[should_panic]
    fn test_offset_panics() {
        let a = from_static("abc");
        let b = from_static("def");
        a.offset_from(&b);
    }

    #[test]
    fn test_trimmed_is_contained() {
        let a = from_static("  aa aa  ");
        let b = a.clone().trimmed();
        assert!(a.contains(&b));
        assert_eq!(b.len(), 5);
    }

    #[test]
    fn trimmed_empty() {
        let empty = from_static("");
        assert_eq!(empty.clone().trimmed(), empty);
    }

    #[test]
    fn trimmed_whitespace() {
        let w = from_static("  ");
        assert!(w.clone().trimmed().is_empty());
    }
}
