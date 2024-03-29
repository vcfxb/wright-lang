//! [Fragment] struct and implementation for dealing with fragments of source code.

use std::str::Chars;

/// A fragment of source code.
///
/// An empty fragment at the end of a file can be used in errors to represent a [Token] expected
/// at the end of the file.
///
/// [Token]: crate::parser::lexer::token::Token
#[derive(Clone, Copy, Debug)]
pub struct Fragment<'src> {
    /// Fragments are represented using direct string references into the source file itself.
    pub inner: &'src str,
}

impl<'src> Fragment<'src> {
    /// Get the length (in bytes) of this fragment.
    pub const fn len(&self) -> usize {
        self.inner.len()
    }

    /// Check if the length of this fragment is zero.
    pub const fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Get a pair of pointers, the first one being at the beginning of the fragment, the second one pointing
    /// to the byte after the end of the fragment.
    pub const fn start_and_end(&self) -> (*const u8, *const u8) {
        // Get the pointer to the start of the fragment.
        let start: *const u8 = self.inner.as_ptr();
        // Get a pointer just past the end of the string.
        // SAFETY: the resulting pointer is guarunteed to point at one byte past the end of the string.
        (start, unsafe { start.add(self.len()) })
    }

    /// Return true if both of these [`Fragment`]s point to the exact same slice of source code.
    pub fn ptr_eq(&self, other: &Self) -> bool {
        // Since std::ptr::eq works for fat pointers, we can use it here.
        std::ptr::eq(self.inner, other.inner)
    }

    /// Return true if this [Fragment] is zero bytes long and points to the end of `origin`.
    pub fn is_at_end_of(&self, origin: &Self) -> bool {
        let (start, end) = self.start_and_end();
        let (_, target) = origin.start_and_end();

        start == target && end == target
    }

    /// Return true if this fragment overlaps at all with the other (either one contains the start of the other,
    /// by pointer).
    pub fn overlaps(&self, other: &Self) -> bool {
        // Get start and end pointers for both fragments.
        let (start, end) = self.start_and_end();
        let (other_start, other_end) = other.start_and_end();
        // Check if this fragment contains either end of the other fragment.
        (start <= other_start && other_start < end) || (other_start <= start && start < other_end)
    }

    /// Return true if this fragment entirely contains another fragment using pointers.
    pub fn contains(&self, other: &Self) -> bool {
        // Get start and end pointers for both fragments.
        let (start, end) = self.start_and_end();
        let (other_start, other_end) = other.start_and_end();
        // Check bounds.
        start <= other_start && end >= other_end
    }

    /// Split this fragment into two sub fragments, with the first one being `bytes` long and the second containing the
    /// rest of this fragment.
    ///
    /// # Panics:
    /// - Panics if the byte index is not in the fragment, or if it's on a char boundary.
    pub fn split_at(&self, bytes: usize) -> (Self, Self) {
        // Use str's split_at.
        let (left, right) = self.inner.split_at(bytes);

        (Self { inner: left }, Self { inner: right })
    }

    /// Unsafe version of [`Fragment::split_at`]. Splits this [Fragment] into two subfragments,
    /// where the left one contains the first `bytes` bytes of the fragment, and the right one
    /// contains the rest.
    ///
    /// # Safety
    /// - Undefined Behavior occurs if `bytes` is greater than the length of the [Fragment].
    /// - Undefined Behavior occurs if `bytes` is not on a UTF-8 character boundary.
    /// - See [str::get_unchecked] for more details.
    pub unsafe fn split_at_unchecked(&self, bytes: usize) -> (Self, Self) {
        let left: &str = self.inner.get_unchecked(..bytes);
        let right: &str = self.inner.get_unchecked(bytes..);

        (Fragment { inner: left }, Fragment { inner: right })
    }

    /// Get an iterator over the characters in this fragment.
    pub fn chars(&self) -> Chars<'src> {
        self.inner.chars()
    }

    /// Get the number of bytes between the beginning of `origin` and the beginning of [`self`].
    ///
    /// # Panics:
    /// - Panics if [`self`] is not a fragment within `origin` according to [`Fragment::contains`].
    pub fn offset_from(&self, origin: &Self) -> usize {
        if !origin.contains(self) {
            panic!("This fragment must be contained in the original fragment");
        }

        // Get a pointer to the start of the original fragment.
        let start: *const u8 = origin.inner.as_ptr();
        // Do the same for the subslice.
        let subslice_start: *const u8 = self.inner.as_ptr();

        // SAFETY: Since the subslice is contained (by pointer) by the origin slice, both of them
        // necessarily satisfy the safety requirements of offset_from to be pointers to the same
        // allocation.
        //
        // We can always cast to a usize since this should always be a positive offset, as long
        // as the subslice is contained in the origin fragment.
        unsafe { subslice_start.offset_from(start) as usize }
    }

    /// Get a sub-fragment of this fragment (see [Fragment::contains]) with the whitespace at either end trimmed off.
    /// This is useful when the fragment is used for printing an error message and you only want the visible characters
    /// of it. This will return the fragment unchanged if it is empty.
    ///
    /// See [str::trim] for exact behaviors.
    pub fn trimmed(self) -> Self {
        Fragment {
            inner: self.inner.trim(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::fragment::Fragment;

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
