
/// A chunk of bytecode.
#[derive(Debug, Clone)]
pub struct Chunk {
    name: String,
    inner: Vec<u8>,
}

/// Offset or address into a chunk, in bytes.
pub type Offset = usize;

impl Chunk {
    /// Constructor. Create a new chunk.
    pub fn new(name: String) -> Self {
        Self {
            name,
            inner: Vec::new(),
        }
    }

    /// Get this chunk's name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the size of this chunk (in bytes)
    pub fn size(&self) -> usize {
        self.inner.len()
    }

    /// Get the capacity of this chunk (in bytes)
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    /// Reserve additional space in one allocation.
    pub fn reserve(&mut self, count: usize) {
        self.inner.reserve(count)
    }

}
