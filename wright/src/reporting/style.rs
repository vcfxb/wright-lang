//! The different styles that can be used to draw a [Diagnostic].

use supports_unicode::Stream;

use super::box_drawing;
#[cfg(doc)]
use super::Diagnostic;

/// The styles that can be used while drawing a [Diagnostic]. 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum Style {
    UnicodeHeavy,
    UnicodeLight,
    Ascii
}

impl Style {
    /// Get the recommended style for a given stream based on whether that stream supports unicode. 
    pub fn for_stream(stream: Stream) -> Self {
        if supports_unicode::on(stream) {
            Self::UnicodeHeavy
        } else {
            Self::Ascii
        }
    }

    /// The vertical character that should be used while printing with this style. 
    pub const fn vertical_char(self) -> char {
        match self {
            Self::Ascii => '|',
            Self::UnicodeHeavy => box_drawing::heavy::VERTICAL,
            Self::UnicodeLight => box_drawing::light::VERTICAL,
        }
    }

    /// If there is one, get the character for this style representing a vertical bar with a branch right. 
    pub const fn vertical_right_char(self) -> Option<char> {
        match self {
            Self::Ascii => None,
            Self::UnicodeHeavy => Some(box_drawing::heavy::VERTICAL_RIGHT),
            Self::UnicodeLight => Some(box_drawing::light::VERTICAL_RIGHT),
        }
    }

    /// If there is one, get the character for this style representing a horizontal half bar from the center to the left. 
    pub const fn left_char(self) -> Option<char> {
        match self {
            Self::Ascii => None,
            Self::UnicodeHeavy => Some(box_drawing::heavy::LEFT),
            Self::UnicodeLight => Some(box_drawing::light::LEFT),
        }
    }

    /// Get a character to use while drawing horizontal dividing lines. 
    pub const fn horizontal_char(self) -> Option<char> {
        match self {
            Self::Ascii => None,
            Self::UnicodeHeavy => Some(box_drawing::heavy::HORIZONTAL),
            Self::UnicodeLight => Some(box_drawing::light::HORIZONTAL),
        }
    }

    /// Get a character to use while drawing dashed horizontal lines. 
    pub const fn horizontal_dashed_char(self) -> Option<char> {
        match self {
            Style::UnicodeHeavy => Some(box_drawing::heavy::HORIZONTAL_DASHED),
            Style::UnicodeLight => Some(box_drawing::light::HORIZONTAL_DASHED),
            Style::Ascii => None,
        }
    }

    /// Get a horizontal character with a downward branch if available. 
    pub const fn down_horizontal_char(self) -> Option<char> {
        match self {
            Style::UnicodeHeavy => Some(box_drawing::heavy::DOWN_HORIZONTAL),
            Style::UnicodeLight => Some(box_drawing::light::DOWN_HORIZONTAL),
            Style::Ascii => None,
        } 
    }

    /// Check if this style is a unicode style. This includes [Style::UnicodeHeavy] and [Style::UnicodeLight].
    pub const fn is_unicode(self) -> bool {
        // Use usize cast to make this possible in const contexts. 
        (self as usize) != (Self::Ascii as usize)
    }
}
