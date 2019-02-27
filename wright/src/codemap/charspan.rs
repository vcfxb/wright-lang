use std::cmp::{PartialOrd, Ordering, PartialEq};

/// The type used to store char indexes.
pub type CharIndex = u32;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct CharSpan {
    /// Start (inclusive)
    pub start: CharIndex,
    /// End (exclusive)
    pub end: CharIndex,
}


impl CharSpan {
    /// Construct new CharSpan.
    /// The constructed CharSpan will always be valid.
    /// If the provided end is before the provided start, they will be swapped.
    pub fn new(start: CharIndex, end: CharIndex) -> CharSpan {
        let c_s = CharSpan{start, end};
        if c_s.is_valid() {c_s}
        else {CharSpan{start: end, end: start}}
    }

    /// Check if one CharSpan contains another
    pub fn contains_span(&self, other: &Self) -> bool {
        self.start <= other.start &&
            self.start <= other.end &&
            self.end >= other.start &&
            self.end >= other.end
    }

    /// Check if a CharSpan ends after it starts
    pub const fn is_valid(&self) -> bool { self.start <= self.end }

    /// Check if a CharIndex is in the CharSpan
    pub fn contains(&self, index: CharIndex) -> bool {
        index >= self.start && index < self.end
    }
}

impl PartialEq<CharSpan> for CharIndex {
    fn eq(&self, other: &CharSpan) -> bool { other.contains(*self) }
}

impl PartialOrd<CharSpan> for CharIndex {
    fn partial_cmp(&self, other: &CharSpan) -> Option<Ordering> {
        if other.contains(*self) {Some(Ordering::Equal)}
        else if self < &other.start {Some(Ordering::Less)}
        else if self >= &other.end {Some(Ordering::Greater)}
        else {panic!("Impossible comparison state {} in {:?}", self, other);}
    }
}

impl PartialEq<CharIndex> for CharSpan {
    fn eq(&self, other: &CharIndex) -> bool {
        self.contains(*other)
    }
}

impl PartialOrd<CharIndex> for CharSpan {
    fn partial_cmp(&self, other: &CharIndex) -> Option<Ordering> {
        if self.contains(*other) {Some(Ordering::Equal)}
        else if other > self {Some(Ordering::Less)}
        else if other < self {Some(Ordering::Greater)}
        else {panic!("Impossible comparison state {:?} cmp {}", self, other);}
    }
}