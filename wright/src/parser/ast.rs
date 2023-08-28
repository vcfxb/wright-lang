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


pub enum Declaration {
    
}
