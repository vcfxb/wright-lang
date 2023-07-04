//! Expressions in wright source code.
//!
//! Expression structures are decently uniform and
//! created using macros.

use derive_more::From;

macro_rules! unary_expr {
    (
        $(#[$meta:meta])*
        $name:ident
    ) => {
        $(#[$meta])*
        #[derive(Clone, Debug)]
        pub struct $name {
            pub expr: Box<Expression>,
        }

        impl $name {
            paste::paste! {
                #[doc = "Construct a new [`" $name "`] around a given [`Expression`]."]
                #[allow(unused)]
                #[inline]
                pub fn new(expr: Expression) -> Self {
                    Self {
                        expr: Box::new(expr)
                    }
                }
            }
        }
    };
}

macro_rules! unary_exprs {
    (
        $(
            $(#[$meta:meta])*
            $name:ident,
        )*
    ) => {
        $(
            unary_expr! {
                $(#[$meta])*
                $name
            }
        )*
    };
}

/// An expression in wright source code.
#[derive(Clone, Debug, From)]
pub enum Expression {
    Block(Block),
    Parens(Parens),
    Bang(Bang),
    Tilde(Tilde),
}

unary_exprs! {
    /// Bang (`!expr`) is the logical not operator.
    Bang,

    /// Tilde (`~expr`) is used as the bitwise not operator.
    Tilde,

    /// Parens (`(expr)`) is used for expression grouping.
    Parens,

    /// Block (`{expr}`) is used for scope.
    Block,
}
