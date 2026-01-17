use crate::ast::identifier::Identifier;
use crate::ast::literal::{BooleanLiteral, IntegerLiteral};
use crate::source_tracking::fragment::Fragment;

/// Atoms of an expression -- these are individual tokens from the lexer that are valid as an
/// expression all on their own.
#[derive(Debug)]
#[allow(missing_docs)]
pub enum Atom {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    BooleanLiteral(BooleanLiteral),
}

impl Atom {
    /// Get the matching fragment of source code.
    pub fn fragment(&self) -> &Fragment {
        match self {
            Atom::Identifier(i) => &i.fragment,
            Atom::IntegerLiteral(lit) => &lit.fragment,
            Atom::BooleanLiteral(lit) => &lit.fragment,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[allow(missing_docs)]
pub enum UnaryOperation {
    Reference,
    Dereference,
    Negate,
    BooleanNot,
    BitwiseNot,
}
