//! Module for representing pieces of the Abstract Syntax Tree.

use super::Spanned;
use std::str::FromStr;
use std::ops::*;

/// Binary operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryOp {
    /// Addition. (`+`)
    Add,
    /// Subtraction. (`-`)
    Subtract,
    /// Multiplication. (`*`)
    Multiply,
    /// Division. (`/`)
    Divide,
    /// Modulo. (`%`)
    Mod,
    /// Logical OR. (`||`)
    LOR,
    /// Bitwise OR. (`|`)
    OR,
    /// Logical AND. (`&&`)
    LAND,
    /// Bitwise AND. (`&`)
    AND,
    /// XOR. (`^`)
    XOR,
    /// Equal. (`==`)
    EQ,
    /// Not Equal. (`!=`)
    NE,
    /// Less than. (`<`)
    LT,
    /// Greater than. (`>`)
    GT,
    /// Greater than or equal. (`>=`)
    GTE,
    /// Less than or equal. (`<=`)
    LTE,
    /// Left bit shift. (`<<`)
    LShift,
    /// Signed (Arithmetic) right shift. (`>>`)
    RShift,
    /// Unsigned (Logical) right shift. (`>>>`)
    URShift,
    /// Member reference. (`.`)
    Dot,
    /// Member reference. (`::`)
    DoubleColon,
    /// Subscript. (`a[b]`)
    Subscript,
    /// Assign value to a variable. (`=`)
    Assign,
    /// Assign to result of self and other in specified operation.
    OpAssign(Box<BinaryOp>)
}

/// Unary operations.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum UnaryOp {
    /// Negate. (`-`)
    Neg,
    /// Logical NOT. (`!`)
    LNOT,
    /// Bitwise NOT. (`~`)
    NOT,
    /// Parentheses (`(expression)`)
    Parentheses,
}

/// Possible visibility modifiers.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum VisibilityModifier {
    /// Hidden: only accessible in same struct/impl block.
    ///
    /// There is one exception here: any component, hidden or otherwise, can always access itself
    /// and any of its required components and traits.
    Hidden,
    /// Private: only accessible in the same module.
    Private,
    /// Public: accessible anywhere.
    Public,
}

/// Modifiers for variables and arguments.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum VarType {
    /// Mutable Variable. (`var`)
    Var,
    /// Immutable Value. (`val`)
    Val,
}

/// Wright's primitive types.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum PrimitiveType {
    /// Boolean true or false.
    Boolean,
    /// Unsigned 8 bit integer. (byte)
    U8,
    /// Signed 8 bit integer.
    I8,
    /// Unsigned 16 bit integer.
    U16,
    /// Signed 16 bit integer.
    I16,
    /// UTF-8 encoded character.
    Character,
    /// Unsigned 32 bit integer.
    U32,
    /// Signed 32 bit integer.
    I32,
    /// Unsigned 64 bit integer.
    U64,
    /// Signed 64 bit integer.
    I64,
    /// String
    Str,
    /// 32 bit floating point.
    F32,
    /// 64 bit floating point.
    F64,
    /// Unit / Void type.
    Void,
}

#[derive(Clone, Debug)]
pub enum ModuleRef<'i> {
    Inner(Box<Spanned<ModuleRef<'i>>>),
    Term(Spanned<&'i str>),
}

/// Type in Wright.
#[derive(Debug, Clone)]
pub enum Type<'input> {
    /// Array of a Type, with an optional size.
    Array(Box<Spanned<Type<'input>>>, Option<Box<Spanned<Expression<'input>>>>),
    /// Primitive Type.
    Primitive(PrimitiveType),
    /// User defined type.
    User(Spanned<ModuleRef<'input>>, Vec<Spanned<Type<'input>>>),
    /// Self type.
    SelfType,
    /// Function type.
    Function(Vec<Spanned<Type<'input>>>, Option<Box<Spanned<Type<'input>>>>),
}

pub type LiteralParse<T> = Result<T, <T as FromStr>::Err>;

#[derive(Debug, Clone)]
pub enum Expression<'e> {
    SelfIdentifier,
    Identifier(&'e str),
    LitInt(LiteralParse<u64>),
    LitFloat(LiteralParse<f64>),
    LitBool(LiteralParse<bool>),
    LitStr(&'e str),
    LitChar(LiteralParse<char>),
    BinaryExpr(BinaryOp, Box<Spanned<Expression<'e>>>, Box<Spanned<Expression<'e>>>),
    UnaryExpr(UnaryOp, Box<Spanned<Expression<'e>>>),
    Block {
        tag: Option<Spanned<&'e str>>,
        statements: Vec<Spanned<Statement<'e>>>,
        expression: Box<Spanned<Expression<'e>>>
    },
    Conditional {
        if_conditions: Box<Spanned<Expression<'e>>>,
        if_content: Box<Spanned<Expression<'e>>>,
        elif: Vec<(Spanned<Expression<'e>>, Spanned<Expression<'e>>)>,
        else_conditions: Option<Box<Spanned<Expression<'e>>>>,
        else_content: Option<Box<Spanned<Expression<'e>>>>,
    },
    FunctionCall(Box<Spanned<Expression<'e>>>, Vec<Spanned<Expression<'e>>>),
    Cast(Box<Spanned<Expression<'e>>>, Spanned<Type<'e>>),
    /// 0: var type
    /// 1: name / identifier
    /// 2: iterator
    /// 3: block
    ForEachLoop(Spanned<VarType>, Box<Spanned<Expression<'e>>>, Box<Spanned<Expression<'e>>>, Box<Spanned<Expression<'e>>>),
    WhileLoop(Box<Spanned<Expression<'e>>>, Box<Spanned<Expression<'e>>>),
    Lambda(Vec<Spanned<FnArg<'e>>>, Box<Spanned<Expression<'e>>>),
    Match(Box<Spanned<Expression<'e>>>, Vec<Spanned<MatchExpression<'e>>>),
    VariableDeclaration(Spanned<VarType>, Box<Spanned<Expression<'e>>>, Option<Spanned<Type<'e>>>),
}

impl<'i> Spanned<Expression<'i>> {
    /// Construct a spanned binary expression from two spanned expressions.
    pub fn combine(fst: Self, snd: Self, op: BinaryOp) -> Self {
        let s = fst.span.start().0 as usize;
        let e = snd.span.end().0 as usize;
        Spanned::new(Expression::BinaryExpr(op, Box::new(fst), Box::new(snd)), s, e)
    }
    /// Similar to [`combine`](#method.combine) except
    /// it uses [`BinaryOp::OpAssign(op)`](./ast/enum.BinaryOp.html#variant.OpAssign).
    pub fn assign_combine(fst: Self, snd: Self, op: BinaryOp) -> Self {
        Self::combine(fst, snd, BinaryOp::OpAssign(Box::new(op)))
    }
}

#[derive(Debug, Clone)]
pub struct MatchExpression<'e>(pub Spanned<Expression<'e>>, pub Spanned<Expression<'e>>);

#[derive(Debug, Clone)]
pub enum Statement<'s> {
    Expression(Expression<'s>),
    Return(Expression<'s>),
    Continue(Spanned<&'s str>),
    Break(Option<Spanned<&'s str>>, Option<Spanned<Expression<'s>>>),
}

/// Modifiers for structural declarations.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum StructuralModifier {
    /// Inline:
    /// Functions calls are inlined,
    /// Classes and Unions are stored raw, rather than behind a pointer.
    Inline,
    External,
    Internal,
}


#[derive(Debug, Clone)]
pub enum Structural<'i> {
    Import(Option<Spanned<VisibilityModifier>>, Spanned<ModuleRef<'i>>),
    Function(FunctionDeclaration<'i>),
    Constant(ConstantDeclaration<'i>),
    UnionDeclaration {
        visibility: Option<Spanned<VisibilityModifier>>,
        modifiers: Vec<Spanned<StructuralModifier>>,
        name: Spanned<&'i str>,
        generics: Option<Spanned<GenericsDeclaration<'i>>>,
        variants: Vec<Spanned<UnionVariant<'i>>>,
    },
    ClassDeclaration {
        modifiers: Vec<Spanned<StructuralModifier>>,
        visibility: Option<Spanned<VisibilityModifier>>,
        name: Spanned<&'i str>,
        generics: Option<Spanned<GenericsDeclaration<'i>>>,
        fields: Vec<Spanned<FieldDeclaration<'i>>>,
    },
    ComponentDeclaration {
        modifiers: Vec<Spanned<StructuralModifier>>,
        visibility: Option<Spanned<VisibilityModifier>>,
        name: Spanned<&'i str>,
        generics: Option<Spanned<GenericsDeclaration<'i>>>,
        supers: Vec<Spanned<Type<'i>>>,
        alias: Spanned<Type<'i>>,
        items: Vec<ComponentItem<'i>>,
    },
    TraitDeclaration {
        modifiers: Vec<Spanned<StructuralModifier>>,
        visibility: Option<Spanned<VisibilityModifier>>,
        name: Spanned<&'i str>,
        generics: Option<Spanned<GenericsDeclaration<'i>>>,
        supers: Vec<Spanned<Type<'i>>>,
        items: Vec<ComponentItem<'i>>,
    },
    Documentation(Spanned<&'i str>),
    Impl {
        generics: Option<Spanned<GenericsDeclaration<'i>>>,
        /// The type being implemented, or implemented on; A concrete class or union.
        base: Spanned<Type<'i>>,
        /// Optionally, the component or trait being implemented.
        ty: Option<Spanned<Type<'i>>>,
        where_clause: Option<WhereClause<'i>>,
        items: Vec<ComponentItem<'i>>,
    },
    Type(TypeAlias<'i>),
}

#[derive(Clone, Debug)]
pub struct Module<'i>(pub Vec<Structural<'i>>);
#[derive(Clone, Debug)]
pub struct UnionVariant<'i>(pub Spanned<&'i str>, pub Spanned<Type<'i>>);
#[derive(Clone, Debug)]
pub struct WhereClause<'i>(pub Vec<Spanned<(Spanned<Type<'i>>, Spanned<Vec<Spanned<Type<'i>>>>)>>);
#[derive(Clone, Debug)]
pub struct FieldDeclaration<'i>(pub Spanned<VarType>, pub Option<Spanned<()>>, pub Spanned<&'i str>, pub Spanned<Type<'i>>);
#[derive(Debug, Clone)]
pub struct GenericsDeclaration<'i>(pub Vec<Spanned<&'i str>>);
#[derive(Debug, Clone)]
pub struct TypeAlias<'i>(pub Option<Spanned<VisibilityModifier>>, pub Spanned<Type<'i>>, pub Option<Spanned<Type<'i>>>);
#[derive(Debug, Clone)]
pub struct FnArg<'input>(pub Spanned<VarType>, pub Spanned<&'input str>, pub Option<Spanned<Type<'input>>>);

#[derive(Debug, Clone)]
pub enum ComponentItem<'i> {
    Fn(FunctionDeclaration<'i>),
    Const(ConstantDeclaration<'i>),
    Type(TypeAlias<'i>),
}
#[derive(Debug, Clone)]
pub struct FunctionDeclaration<'i> {
    pub modifiers: Vec<Spanned<StructuralModifier>>,
    pub visibility: Option<Spanned<VisibilityModifier>>,
    pub name: Spanned<&'i str>,
    pub generics: Option<Spanned<GenericsDeclaration<'i>>>,
    pub return_type: Spanned<Type<'i>>,
    pub args: Vec<Spanned<FnArg<'i>>>,
    pub code: Option<Spanned<Expression<'i>>>,
}
#[derive(Clone, Debug)]
pub struct ConstantDeclaration<'i> {
    pub modifiers: Vec<Spanned<StructuralModifier>>,
    pub visibility: Option<Spanned<VisibilityModifier>>,
    pub name: Spanned<&'i str>,
    pub ty: Spanned<Type<'i>>,
    pub val: Spanned<Expression<'i>>,
}

