use crate::grammar::model::Fragment;

/// A wright primitive type.
/// Wright has almost the same set of primitive types as rust.
/// The ones here all correspond almost equivalently to their rust
/// counterparts. The exception is String, which acts more like a
/// Java String or rust's `&str`.
#[allow(missing_docs)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum PrimitiveTypeVariant {
    Char,
    U8, I8, U16, I16, U32, I32, U64, I64, U128, I128,
    Bool,
    String,
}

/// A primitive type in source code.
#[derive(Copy, Clone, Debug)]
pub struct PrimitiveType<'s> {
    /// Associated fragment in source code.
    pub frag: Fragment<'s>,
    /// Represented variant.
    pub variant: PrimitiveTypeVariant
}

/// A user defined type.
pub struct UserType<'s> {
    /// Associated fragment in source code.
    pub frag: Fragment<'s>
}

/// A type in source code.
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub enum Type<> {
    Primitive(PrimitiveTypeVariant)
}
