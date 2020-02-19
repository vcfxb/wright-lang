/// Module for parsing literal numbers in wright source code.
pub(crate) mod num_lit;

/// Module for parsing character literals in Wright source code.
pub(crate) mod char_lit;

/// Module for parsing string literals in Wright source code.
pub(crate) mod string_lit;

/// Module for parsing boolean literals in wright source code.
pub(crate) mod boolean_lit;

#[cfg(test)]
mod num_lit_tests;

#[cfg(test)]
mod char_lit_tests;

#[cfg(test)]
mod string_lit_tests;

#[cfg(test)]
mod boolean_lit_tests;
