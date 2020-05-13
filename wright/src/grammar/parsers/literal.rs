/// Module for parsing literal numbers in wright source code.
pub(crate) mod num_lit;
#[cfg(test)]
mod num_lit_tests;

/// Module for parsing character literals in Wright source code.
pub(crate) mod char_lit;
#[cfg(test)]
mod char_lit_tests;

/// Module for parsing string literals in Wright source code.
pub(crate) mod string_lit;
#[cfg(test)]
mod string_lit_tests;

/// Module for parsing boolean literals in wright source code.
pub(crate) mod boolean_lit;
#[cfg(test)]
mod boolean_lit_tests;

/// `self` literal in wright source code.
pub(crate) mod self_lit;
#[cfg(test)]
mod self_lit_tests;

/// Wright identifier parser.
pub(self) mod identifier;
#[cfg(test)]
mod identifier_tests;

/// Wright scoped name parser.
pub(crate) mod scoped_name;
#[cfg(test)]
mod scoped_name_tests;
