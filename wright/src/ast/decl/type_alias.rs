//! Type alias declarations in wright source code.
//!

use crate::{
    ast::{identifier::Identifier, ty::Type},
    source_tracking::fragment::Fragment,
};

/// A type alias in wright source code.
#[derive(Debug)]
pub struct TypeAlias {
    /// Full matching source including whitespace.
    pub matching_source: Fragment,

    /// The name of the new/aliased type.
    pub new_type_name: Identifier,

    /// The type aliased to if any (we support abstract `pub type Void;`) style declarations.
    pub target_type: Option<Type>,
}
