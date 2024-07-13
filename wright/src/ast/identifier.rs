//! [Identifier]s are used throughout wright as variable names, type names, function names, etc.
//! Their modeling is pretty simple, and is defined here.
//! 
//! [Identifier]: https://en.wikipedia.org/wiki/Identifier

use crate::source_tracking::fragment::Fragment;

/// Identifiers are used as names for variables, functions, modules, etc. 
/// These are defined using [Fragment]s of source code, which will contain the identifier itself.
#[derive(Debug, Clone)]
pub struct Identifier {
    /// The fragment of source code containing the identifier.
    pub fragment: Fragment,
}
