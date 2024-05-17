//! Owned strings are used throughout the error reporting code, since in many cases, our error messages and codes
//! will be static (or at least partially so).
//!
//! The code here will ultimately be very similar to a [`Cow<'static, str>`]
//!
//! [`Cow<'static, str>`]: std::borrow::Cow

use derive_more::{Display, From};

/// An owned string used for error reporting. Since many error messages and codes are static, we use this struct
/// to provide [`&'static str`]s as an option when a heap allocated string is not necessary.
///
/// [`&'static str`]: str
#[derive(Debug, From, Display)]
pub enum OwnedString {
    /// A static string reference.
    Static(&'static str),

    /// An owned [String].
    Owned(String),
}

impl AsRef<str> for OwnedString {
    fn as_ref(&self) -> &str {
        match self {
            OwnedString::Static(str_ref) => str_ref,
            OwnedString::Owned(string) => string.as_ref(),
        }
    }
}
