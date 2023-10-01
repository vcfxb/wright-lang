//! Various [AST] (abstract syntax tree) constructs used in Wright.
//!
//! [AST]: https://en.wikipedia.org/wiki/Abstract_syntax_tree

use std::{borrow::Cow, io};
use ptree::TreeItem;

pub mod declaration;
pub mod expression;
pub mod identifier;
pub mod metadata;
pub mod path;
pub mod types;

/// Trait used to make it easier to pretty print AST nodes. 
trait AstNode {
    /// Write this AST node's info out to something that implements [`io::Write`]. 
    fn write_self(&self, w: &mut dyn io::Write, style: &ptree::Style) -> io::Result<()>;

    /// Get a list of this node's childeren if any. 
    fn children(&self) -> Cow<[&dyn AstNode]>;
}

impl<'src> TreeItem for &'src dyn AstNode {
    type Child = Self;

    fn write_self<W: io::Write>(&self, f: &mut W, style: &ptree::Style) -> io::Result<()> {
        // Pass call daynamically onto AST node. 
        AstNode::write_self(*self, f, style)
    }

    fn children(&self) -> Cow<[Self::Child]> {
        // Pass call dynamically onto AST node. 
        AstNode::children(*self)
    }
}
