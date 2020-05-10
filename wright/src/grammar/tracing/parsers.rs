/// Traced version of nom's
/// [map](https://docs.rs/nom/5.1.1/nom/combinator/fn.map.html)
/// combinator.
pub mod map;

/// Re-export map parser.
pub use map::map;

// todo check docs
mod tag;

/// Re-export tag parser.
pub use tag::tag;
