//! AST models for type signatures in wright source.

use crate::{ast::path::Path, source_tracking::fragment::Fragment};

/// A type signature in source code.
#[derive(Debug)]
#[allow(missing_docs)]
pub enum Type {
    Atomic(AtomicTy),
    Reference(ReferenceTy),
    Constrained(ConstrainedTy),
}

impl Type {
    /// Get the matching source for this type signature in source code.
    pub fn matching_source(&self) -> &Fragment {
        match self {
            Type::Atomic(atomic_ty) => &atomic_ty.matching_source,
            Type::Reference(reference_ty) => &reference_ty.matching_source,
            Type::Constrained(constrained_ty) => &constrained_ty.matching_source,
        }
    }

    /// Attempt to "downcast" this to an atomic type signature if it is one.
    pub fn downcast_primitive(&self) -> Option<&AtomicTy> {
        match self {
            Type::Atomic(atomic) => Some(atomic),
            _ => None,
        }
    }

    /// Attempt to "downcast" this to a reference type signature if it is one.
    pub fn downcast_reference(&self) -> Option<&ReferenceTy> {
        match self {
            Type::Reference(reference) => Some(reference),
            _ => None,
        }
    }

    /// Attempt to "downcast" this to a constrained type signature if it is one.
    pub fn downcast_constrained_ty(&self) -> Option<&ConstrainedTy> {
        match self {
            Type::Constrained(constrained) => Some(constrained),
            _ => None,
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
#[derive(Debug)]
#[allow(missing_docs)]
pub struct AtomicTy {
    pub variant: AtomicTyVariant,
    pub matching_source: Fragment,
}

/// Source code for a reference type signature, such as `@u64`.
#[derive(Debug)]
pub struct ReferenceTy {
    /// The source code of the target type.
    pub target_ty: Box<Type>,
    /// The fragment of the whole reference.
    pub matching_source: Fragment,
}

/// A type with a given set of constraints.
///
/// Constraints in wright are functions that the compiler can verify are strictly [pure]
/// (which is informally defined here, and a point of further work eventually).
///
/// A constrained type declaration lists a base type and then one or more "strictly pure"
/// functions that have a signature exactly matching T -> bool (where T is the constrained type).
///
/// An example of this could be
/// ```text
/// pure func is_even(i: u8) -> bool {
///     i % 2 == 0
/// }
///
/// type EvenU8 = u8 constrain is_even;
/// ```
///
/// The wright compiler can then optimize agressively around these constraints later on (I hope).
///
/// [pure]: https://en.wikipedia.org/w/index.php?title=Pure_function&oldid=1291437073
#[derive(Debug)]
pub struct ConstrainedTy {
    /// The entire type signature from the beginning of the base type
    /// to the end of the last constraining item.
    pub matching_source: Fragment,

    /// The type being constrained.
    pub base_ty: Box<Type>,

    /// The functions constraining it.
    pub constraining_items: Vec<Path>,
}
