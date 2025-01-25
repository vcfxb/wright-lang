//! Utility module that builds on top of the functionality of the global [mod@supports_unicode] crate by adding a fallback
//! global static, and a function that always indicates lack of unicode support if the crate/feature is not enabled.

#[cfg(feature = "supports-unicode")]
use ::supports_unicode as supports_unicode_crate;

#[cfg(feature = "supports-unicode")]
use core::sync::atomic::AtomicBool;

/// Should all output force the use of ASCII characters only?
#[cfg(feature = "supports-unicode")]
pub static FORCE_ASCII: AtomicBool = AtomicBool::new(false);

/// Set the global [FORCE_ASCII] static.
#[cfg(feature = "supports-unicode")]
pub fn set_force_ascii(force_ascii: bool) {
    use core::sync::atomic::Ordering;

    FORCE_ASCII.store(force_ascii, Ordering::Release);
}

/// Should we be writing unicode out to the user's terminal?
pub fn supports_unicode() -> bool {
    #[cfg(feature = "supports-unicode")]
    {
        use core::sync::atomic::Ordering;

        !FORCE_ASCII.load(Ordering::Acquire) && supports_unicode_crate::supports_unicode()
    }

    #[cfg(not(feature = "supports-unicode"))]
    {
        false
    }
}
