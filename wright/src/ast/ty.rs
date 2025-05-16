//! AST models for type signatures in wright source.

use crate::source_tracking::fragment::Fragment;

/// A type signature in source code.
#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub enum Type {
    Atomic(AtomicTy),
    Reference(ReferenceTy),
}

impl Type {
    /// Get the matching source for this type signature in source code.
    pub fn matching_source(&self) -> &Fragment {
        match self {
            Type::Atomic(atomic_ty) => &atomic_ty.matching_source,
            Type::Reference(reference_ty) => &reference_ty.matching_source,
        }
    }

    /// Attempt to "downcast" this to an atomic type signature if it is one.
    pub fn downcast_primitive(&self) -> Option<&AtomicTy> {
        match self {
            Type::Atomic(atomic) => Some(atomic),
            _ => None
        }
    }

    /// Attempt to "downcast" this to a reference type signature if it is one.
    pub fn downcast_reference(&self) -> Option<&ReferenceTy> {
        match self {
            Type::Reference(reference) => Some(reference),
            _ => None
        }
    }
}

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
    Char,
}

/// An atomic type signature in wright source code.
#[derive(Clone, Debug)]
#[allow(missing_docs)]
pub struct AtomicTy {
    pub variant: AtomicTyVariant,
    pub matching_source: Fragment,
}

/// Source code for a reference type signature, such as `@u64`.
#[derive(Debug, Clone)]
pub struct ReferenceTy {
    /// The source code of the target type.
    pub target_ty: Box<Type>,
    /// The fragment of the whole reference.
    pub matching_source: Fragment,
}
