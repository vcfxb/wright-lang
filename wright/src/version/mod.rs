/// `VERSION` const is a version constant in the form of a `&'static str`
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// `get_version()` returns Wright's version as a string.
/// This should be identical to its version in cargo.
pub fn get_version() -> String {
    VERSION.to_string()
}