//! Internal representations for values in Wright.
use parser::ast::Visibility;

#[derive(Debug, Clone, Eq, PartialEq)]
/// Enum for wright primitives.
pub enum Primitive {
    Boolean(Boolean),
    Integer(Integer),
    Fraction(Fraction),
    Char(Char),
    String(Str),
    List(List<Primitive>),
}

/// Trait applied to all numerical primitives of wright.
/// All of these functions can be accessed by a compiler annotation in Wright
/// ("? ...").
/// i.e. for integer multiplication in wright,
/// "let c: integer = a*b;"
/// will expand to "let c: integer = integer.add(a,b);"
/// and add is defined somewhere in wright's standard library as
/// " ... pub func add(left: integer, right: integer) -> integer { return ?add(left, right); } ... "
///
/// The functions that take &self as an argument are internal primitive casts, which extend through
/// wright's standard library.
pub trait WrightNumber {
    /// Converts value to an internal integer.
    fn to_int(&self) -> Integer;
    /// Converts value to an internal fraction.
    fn to_fraction(&self) -> Fraction;
    // todo: more
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
/// Internal representation for integers.
pub struct Integer {
    negative: bool,
    value: usize,
}

impl Integer {
    /// Constructor.
    pub fn new(val: usize, neg: bool) -> Integer {
        Integer { value: val, negative: neg }
    }
}

impl WrightNumber for Integer {
    fn to_int(&self) -> Self { return *self; }
    fn to_fraction(&self) -> Fraction {
        Fraction {
            negative: self.negative,
            u: self.value,
            n: 0,
            d: 1,
        }
    }
}

#[derive(Debug,Copy, Clone, Eq, PartialEq)]
/// Internal representation for fractions, which wright uses over floating point numbers because of
/// floating point imprecision.
pub struct Fraction {
    negative: bool,
    u: usize,
    n: usize,
    d: usize,
}

impl Fraction {
    /// Constructor.
    pub fn new(integer: usize, numerator: usize, denominator: usize, neg: bool) -> Fraction {
        Fraction { u: integer, n: numerator, d: denominator, negative: neg }
    }
}


impl WrightNumber for Fraction {
    fn to_int(&self) -> Integer {
        Integer {
            negative: self.negative,
            value: self.u,
        }
    }
    fn to_fraction(&self) -> Self { return *self; }
}

#[derive(Debug,Copy, Clone, Eq, PartialEq)]
/// Internal representation for booleans.
pub struct Boolean {
    pub value: bool,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
/// Internal representation for chars in wright.
pub struct Char {
    pub value: char,
}

#[derive(Debug, Clone, Eq, PartialEq)]
/// Internal representation for strings.
pub struct Str {
    pub value: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
/// Internal representation for lists in Wright.
pub struct List<T> {
    pub value: Vec<T>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
/// Enum for types.
pub enum Type {
    Primitive(Primitive),
    UserDefined(UserDefinedClass),
    // todo: Generic Internal (?)
    Generic,
}

// todo
#[derive(Debug, Clone, Eq, PartialEq)]
/// Struct for user defined classes.
pub struct UserDefinedClass {
    primitive_fields: Vec<(Visibility, Primitive)>,
    object_fields: Vec<(Visibility, UserDefinedClass)>,
}
