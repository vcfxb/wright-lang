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

    /// Return true if this fragment overlaps at all with the other (either one contains the start of the other, 
    /// by pointer).
    pub fn overlaps(&self, other: &Self) -> bool {
        // Get the pointer to the start of the string. 
        let (start, len) = (self.inner.as_ptr(), self.len());
        // Get a pointer just past the end of the string. 
        // SAFETY: the resulting pointer is guarunteed to point at one byte past the end of the string. 
        let end = unsafe { start.add(len) };

        // Do the same thing for the other fragment. 
        let (other_start, len) = (other.inner.as_ptr(), other.len());
        let other_end = unsafe { other_start.add(len) };

        // Check bounds. 
        (start <= other_start && other_start < end) || (other_start <= start && start < other_end)
    }

    /// Split this fragment into two sub fragments, with the first one being `bytes` long and the second containing the
    /// rest of this fragment. 
    pub fn split(&self, bytes: usize) -> (Self, Self) {
        (Self { inner: &self.inner[..bytes] }, Self { inner: &self.inner[bytes..]})
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
}
