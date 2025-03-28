//! AST models for type signatures in wright source.

use crate::source_tracking::fragment::Fragment;

/// The atomic types of wright -- primitive numeric types, boolean, char, etc.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum AtomicTyVariant {
    Bool,
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    F32,
    F64,
    Char
}

/// An atomic type signature in wright source code.
#[derive(Clone, Debug)]
#[allow(missing_docs)]
pub struct AtomicTy {
    pub variant: AtomicTyVariant,
    pub matching_source: Fragment
}


