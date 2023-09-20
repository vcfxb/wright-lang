//! Various [AST] (abstract syntax tree) constructs used in Wright.
//! 
//! [AST]: https://en.wikipedia.org/wiki/Abstract_syntax_tree

pub mod expressions;

/// The possible visibilities of a declaration in Wright. 
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Visibility {
    /// Externally public.
    Public, 
    /// Package private. 
    Package,
    /// Module/file private. 
    Private
}

/// An identifier in the source code being parsed. 
pub type Identifier<'src> = &'src str;

/// A double-colon seperated path to a module, type, or function in Wright source code.
/// 
/// Note that this can be only a single identifier in length, signalling a path that's in current scope. 
#[derive(Debug)]
pub struct Path<'src> {
    /// The first identifier in the path, read left-to-right. 
    pub head: Identifier<'src>,
    /// The rest of the path.
    pub tail: Option<Box<Path<'src>>>
}

/// A generic type argument in a type/class/etc declaration. 
#[derive(Debug)]
pub struct GenericTypeArg<'src> {
    /// The identifier for the generic type. 
    pub name: Identifier<'src>
}

/// A generic const argument in a type/class/etc declaration. 
#[derive(Debug)]
pub struct GenericConstArg<'src> {
    /// The identifier for this generic constant.
    pub name: Identifier<'src>,
    /// The type expected in the generic instantiation. 
    pub ty: TypeInstantiation<'src>
}

/// A reference to or use of a type in source code. 
#[derive(Debug)]
pub struct TypeInstantiation<'src> {
    /// The type's name, possibly at the end of a path to resolve it. 
    pub typename: Path<'src>,
    /// Any types used as generic arguments to make this a concrete type. 
    pub generic_type_arguments: Vec<TypeInstantiation<'src>>,
    /// 
    pub generic_const_arguments: Vec<()>,
}

/// A top-level declaration in source code.
#[derive(Debug)]
pub enum Declaration<'src> {
    Class(ClassDeclaration<'src>),
    Struct(StructDeclaration<'src>),
    Union(UnionDeclaration<'src>),
    Type(TypeDeclaration<'src>),
    Enum(EnumDeclaration<'src>)
}

/// A class declaration in source code. 
#[derive(Debug)]
pub struct ClassDeclaration<'src> {
    /// The class's visibility. 
    pub vis: Visibility,
    /// The class's name. 
    pub name: Identifier<'src>
}

/// A struct declaration in source code. 
#[derive(Debug)]
pub struct StructDeclaration<'src> {
    /// The struct's visibility. 
    pub vis: Visibility,
    /// The struct's name. 
    pub name: Identifier<'src>
}

/// A union declaration in source code. 
#[derive(Debug)]
pub struct UnionDeclaration<'src> {
    /// The visibility of the union. 
    pub vis: Visibility,
    /// The name of the union. 
    pub name: Identifier<'src>
}

/// A type alias in source code.
#[derive(Debug)]
pub struct TypeDeclaration<'src> {
    /// The type alias's visibility. 
    pub vis: Visibility,
    /// The name of the type. 
    pub name: Identifier<'src>,
    /// The generic type arguments that need to be passed to this type.
    pub generic_type_arguments: Vec<GenericTypeArg<'src>>,
    /// The generic constant arguments that need to be passed to this type.
    pub generic_const_arguments: Vec<GenericConstArg<'src>>,
    /// The type being aliased to. 
    pub dest: TypeInstantiation<'src>
}

/// An enumeration in source code. 
#[derive(Debug)]
pub struct EnumDeclaration<'src> {
    /// The visibility of the enum.
    pub vis: Visibility,
    /// The name of the enum
    pub name: Identifier<'src>,
    /// The parent type or enumeration that this enumeration is a strict subset of. 
    pub parent: TypeInstantiation<'src>
}

#[derive(Debug)]
pub struct Expression {
    
}
