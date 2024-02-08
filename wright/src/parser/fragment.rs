//! [Fragment] struct and implementation for dealing with fragments of source code.

use std::str::Chars;

/// A fragment of source code. 
#[derive(Clone, Copy, Debug)]
pub struct Fragment<'src> {
    /// Fragments are represented using direct string references into the source file itself. 
    pub inner: &'src str
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
    const fn start_and_end(&self) -> (*const u8, *const u8) {
        // Get the pointer to the start of the fragment. 
        let start: *const u8 = self.inner.as_ptr();
        // Get a pointer just past the end of the string. 
        // SAFETY: the resulting pointer is guarunteed to point at one byte past the end of the string. 
        (start, unsafe { start.add(self.len()) })
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
    /// Panics if the byte index is not in the fragment, or if it's on a char boundary. 
    pub fn split(&self, bytes: usize) -> (Self, Self) {
        // Use str's split_at. 
        let (left, right) = self.inner.split_at(bytes);

        (Self { inner: left }, Self { inner: right })
    }

    /// Get an iterator over the characters in this fragment. 
    pub fn chars(&self) -> Chars<'src> {
        self.inner.chars()
    }
}


#[cfg(test)]
mod tests {
    use crate::parser::fragment::Fragment;

    #[test]
    fn test_overlap() {
        let a = Fragment { inner: "Test string" };
        let b = Fragment { inner: &a.inner[3..] };
        let c = Fragment { inner: &a.inner[..a.len()-3] };
        let d = Fragment { inner: "other string" };

        assert!(a.overlaps(&b));
        assert!(b.overlaps(&c));
        assert!(c.overlaps(&a));
        assert!(!a.overlaps(&d));
    }

    #[test]
    fn test_split_single() {
        let a = Fragment { inner: "+" };
        let (left, right) = a.split(1);
        assert_eq!(left.inner, "+");
        assert_eq!(right.inner, "");
    }
}
