//! Modeling and implementation pertaining to the severity of a diagnostic from the compiler/wright system. 

use termcolor::Color;
use derive_more::Display;

/// The severity of a [Diagnostic].
/// 
/// [Diagnostic]: super::Diagnostic
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
pub enum Severity {
    /// A compiler bug. Something internally happened that wasn't supposed to. 
    #[display(fmt = "bug")]
    Bug,

    /// An irrecoverable error due to user input -- i.e. syntax errors, etc.
    #[display(fmt = "error")]
    Error,

    /// A warning about something non-fatal but not ideal.
    #[display(fmt = "warning")]
    Warning, 

    /// An info message. Likely rarely used. 
    #[display(fmt = "info")]
    Info,
}


impl Severity {
    /// Get the default color to display a diagnostic of this type with if the terminal supports it. 
    pub const fn color(self) -> Color {
        match self {
            Self::Bug => Color::Magenta,
            Self::Error => Color::Red,
            Self::Warning => Color::Yellow,
            Self::Info => Color::Cyan
        }
    }
}
