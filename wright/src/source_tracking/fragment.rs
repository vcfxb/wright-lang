//! [Fragment] struct and implementation for dealing with fragments of source code.

use std::{ops::Range, str::Chars, sync::Arc};
use super::SourceRef;

#[cfg(doc)]
use super::Source;

/// A fragment of source code. 
/// 
/// This can be part of (or all of) a [Source].
#[derive(Clone, Debug)]
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
        let source_as_str: &str = self.source.as_ref().source().as_ref();

        // Check validity.
        self.range.end >= self.range.start &&
        source_as_str.is_char_boundary(self.range.start) &&
        source_as_str.is_char_boundary(self.range.end)
    }

    /// Get the [str] represented by this [Fragment].
    /// 
    /// # Panics
    /// - This will [panic] in the unlikely event that [Fragment::range] is out of bounds or lands between char
    ///     boundaries for [Fragment::source].
    pub fn as_str(&self) -> &str {
        &self.source.as_ref().source().as_ref()[self.range.clone()]
    }
 
    /// Get the length (in bytes) of this [Fragment].
    pub const fn len(&self) -> usize {
        self.range.end.saturating_sub(self.range.start)
    }

    /// Check if this fragment has a [Fragment::len] == 0.
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Return true if this [Fragment] overlaps at all with the other (either one contains the start of the other).
    /// 
    /// This will return false if the [Fragment]s reference different [Source]s. 
    pub fn overlaps(&self, other: &Self) -> bool {
        // Check source equality. 
        Arc::ptr_eq(&self.source, &other.source) && (
            self.range.start <= other.range.start && other.range.start < self.range.end ||
            other.range.start <= self.range.start && self.range.start < other.range.end
        )
    }

    /// Return true if this [Fragment] entirely contains another [Fragment] and they're from the same [Source].
    pub fn contains(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.source, &other.source) &&
        self.range.start <= other.range.start && 
        self.range.end >= other.range.end
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
    /// See [str::trim] for exact behaviors.
    pub fn trimmed(mut self) -> Self {
        // Get the string representation of this fragment.
        let original_str: &str = self.as_str();
        // Trim it. 
        let trimmed_str: &str = original_str.trim();
        // Get the offset as the byte difference between the start of the two pointers.
        // SAFETY: The requirements for offset_from are trivially satisfied when using substrings.
        let offset: isize = unsafe { trimmed_str.as_ptr().offset_from(original_str.as_ptr()) };
        // Calculate the new start of the range. 
        let new_start: usize = self.range.start + offset as usize;
        // Calculate the new end of the range. 
        let new_end: usize = new_start + trimmed_str.len();
        // Update self.
        self.range = new_start..new_end;
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
    pub fn split_at_unchecked(&self, bytes_from_start: usize) -> (Self, Self) {
        // Calculate ranges.
        let left_range: Range<usize> = self.range.start..(self.range.start + bytes_from_start);
        let right_range: Range<usize> = (self.range.start + bytes_from_start)..self.range.end;

        // Construct fragments.
        (
            Fragment { source: Arc::clone(&self.source), range: left_range }, 
            Fragment { source: Arc::clone(&self.source), range: right_range }
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

    /// This is the same as [Fragment::advance_by] except without the bounds checking. Use carefully or the created 
    /// [Fragment]s will be invalid. 
    #[inline]
    pub fn advance_by_unchecked(&mut self, bytes: usize) {
        self.range.start += bytes;
    }
}

impl PartialEq for Fragment {
    /// Fragment equality is based on referencing the same [Source] using [Arc::ptr_eq] and having the same 
    /// [Fragment::range].
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.source, &other.source) && self.range == other.range
    }
}

impl Eq for Fragment {}


#[cfg(test)]
mod tests {
    use super::Fragment;

    #[test]
    fn test_overlap() {
        let a = Fragment {
            inner: "Test string",
        };

        let b = Fragment {
            inner: &a.inner[3..],
        };

        let c = Fragment {
            inner: &a.inner[..a.len() - 3],
        };

        let d = Fragment {
            inner: "other string",
        };

        assert!(a.overlaps(&b));
        assert!(b.overlaps(&c));
        assert!(c.overlaps(&a));
        assert!(!a.overlaps(&d));
    }

    #[test]
    fn test_split_single() {
        let a = Fragment { inner: "+" };
        let (left, right) = a.split_at(1);
        assert_eq!(left.inner, "+");
        assert_eq!(right.inner, "");
    }

    #[test]
    fn test_offset_from() {
        let a = Fragment { inner: "abcde" };
        let (b, c) = a.split_at(2);
        assert_eq!(b.offset_from(&a), 0);
        assert_eq!(c.offset_from(&a), 2);
    }

    #[test]
    #[should_panic]
    fn test_offset_panics() {
        let a = Fragment { inner: "abc" };
        let b = Fragment { inner: "def" };
        a.offset_from(&b);
    }

    #[test]
    fn test_is_at_end_of() {
        let a = Fragment { inner: "abc" };
        let b = a.split_at(a.len()).1;
        let c = Fragment {
            inner: &a.inner[a.len()..],
        };

        assert!(b.is_at_end_of(&a));
        assert!(c.is_at_end_of(&a));
    }

    #[test]
    fn test_trimmed_is_contained() {
        let a = Fragment { inner: "  aa aa  " };
        let b = a.trimmed();
        assert!(a.contains(&b));
    }

    #[test]
    fn trimmed_empty() {
        let empty = Fragment { inner: "" };
        assert!(empty.trimmed().ptr_eq(&empty));
    }

    #[test]
    fn trimmed_whitespace() {
        let w = Fragment { inner: "  " };
        assert!(w.contains(&w.trimmed()));
        assert!(w.trimmed().is_empty());
        assert!(w.overlaps(&w.trimmed()));
    }
}
