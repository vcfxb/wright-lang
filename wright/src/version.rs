//! The version module holds one constant, which defines the version of wright being used.

/// `VERSION` const is a version constant in the form of a `&'static str`
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");