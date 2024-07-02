//! Box drawing unicode characters.
//!
//! This is adapted from <https://gitlab.com/chronos.alfa/box_drawing>.

/// Light unicode box drawing characters.
#[allow(missing_docs)]
pub mod light {
    pub const HORIZONTAL: char = '─';
    pub const HORIZONTAL_DASHED: char = '\u{254C}';
    pub const VERTICAL: char = '│';
    pub const DOWN_RIGHT: char = '┌';
    pub const DOWN_LEFT: char = '┐';
    pub const UP_RIGHT: char = '└';
    pub const UP_LEFT: char = '┘';
    pub const VERTICAL_RIGHT: char = '├';
    pub const VERTICAL_LEFT: char = '┤';
    pub const DOWN_HORIZONTAL: char = '┬';
    pub const UP_HORIZONTAL: char = '┴';
    pub const VERTICAL_HORIZONTAL: char = '┼';
    pub const UP: char = '╵';
    pub const LEFT: char = '\u{2573}';
    pub const RIGHT: char = '\u{2576}';
}

/// Heavy unicode box drawing characters.
#[allow(missing_docs)]
pub mod heavy {
    pub const HORIZONTAL: char = '━';
    pub const HORIZONTAL_DASHED: char = '\u{254D}';
    pub const VERTICAL: char = '┃';
    pub const DOWN_RIGHT: char = '┏';
    pub const DOWN_LEFT: char = '┓';
    pub const UP_RIGHT: char = '┗';
    pub const UP_LEFT: char = '┛';
    pub const VERTICAL_RIGHT: char = '┣';
    pub const VERTICAL_LEFT: char = '┫';
    pub const DOWN_HORIZONTAL: char = '┳';
    pub const UP_HORIZONTAL: char = '┻';
    pub const VERTICAL_HORIZONTAL: char = '╋';
    pub const UP: char = '╹';
    pub const LEFT: char = '\u{2578}';
    pub const RIGHT: char = '\u{257A}';
}

/// Unicode characters that can be used to combine for a desired effect.
#[allow(missing_docs)]
pub mod combining {
    pub const COMBINING_ENCLOSING_CIRCLE: char = '\u{20DD}';
    pub const COMBINING_ENCLOSING_CIRCLE_WITH_SLASH: char = '\u{20E0}';
    pub const COMBINING_TRIPLE_UNDERDOT: char = '\u{20E8}';
}
