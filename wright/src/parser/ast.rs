//! Abstract syntax tree representation for Wright source code.

use codespan_reporting::files::Files;
use crate::filemap::{FileId, FileMap};
use super::{fragment::Fragment, lexer::{token::{Token, TokenTy}, Lexer}};

pub mod identifier;
pub mod expression;

/// The context needed to parse AST nodes and create errors when it doesn't work out. 
pub struct AstGeneratorContext<'src> {
    /// The ID of the file in [AstGeneratorContext::file_map] being parsed. 
    /// Useful for emitting parser errors. 
    file_id: FileId,

    /// Immutable reference to the [FileMap] containing the source file. 
    /// Useful for emitting parser errors. 
    file_map: &'src FileMap<'src>,

    /// The full source code of the file being parsed. 
    full_source: Fragment<'src>,

    /// The lexer that's being operated on. Just about every parser should work by 
    /// pulling/peeking tokens from this lexer. 
    pub lexer: Lexer<'src>
}

/// Trait implemented by all AST node types. 
pub trait AstNode<'src> {
    /// The type of the error that should be returned if a node of this type cannot be parsed. 
    type Error: 'src;

    /// Get the associated fragment of source code. All [AstNode]s should have one of these. 
    fn fragment(&self) -> Fragment<'src>;

    /// Parse a node of this type from an [AstGeneratorContext], pulling tokens from it as necessary. 
    /// 
    /// If parsing a node of this type is not possible, return an error with any necessary info. 
    fn try_parse(ctx: &mut AstGeneratorContext<'src>) -> Result<Self, Self::Error>
    where Self: Sized;
}

impl<'src> AstGeneratorContext<'src> {
    /// Construct a new [AstGeneratorContext] for parsing a given file and generating its AST. 
    /// 
    /// # Panics
    /// - This function will panic if the given `file_id` is not in the given `file_map`.
    pub fn new(file_id: FileId, file_map: &'src FileMap<'src>) -> Self {
        // Get the full source of the given file. 
        let full_source = Fragment { 
            inner: file_map.source(file_id).expect("Found file in file_map") 
        };

        AstGeneratorContext {
            file_id,
            file_map,
            full_source,
            lexer: Lexer::new(full_source.inner)
        }
    }

    /// Fork this [AstGeneratorContext] producing a new (identical) one that can be used for
    /// parsing that may fail without modifying this one. This is equivalent to clone under the hood,
    /// but is named differently to reflect the different use case and match up with [Lexer::fork].
    /// 
    /// If you parse sucessfully on the forked [AstGeneratorContext], you can use [AstGeneratorContext::update]
    /// to push that progress back to this [AstGeneratorContext]. 
    pub const fn fork(&self) -> Self {
        Self { 
            file_id: self.file_id,
            file_map: self.file_map,
            full_source: self.full_source,
            lexer: self.lexer.fork() 
        }
    }

    /// Update this [AstGeneratorContext] to match the position and state of the other one. This is designed to work
    /// exclusively with [AstGeneratorContext]s previously created by calling [AstGeneratorContext::fork] 
    /// on this one. To that end, the only field that actually gets copied over/updated from `to` is 
    /// [AstGeneratorContext::lexer]. 
    pub fn update(&mut self, to: &Self) {
        self.lexer = to.lexer;
    }

    /// Peek the next [Token] from the internal [Lexer] without consuming it or making progress. 
    #[inline]
    pub fn peek_token(&self) -> Option<Token<'src>> {
        self.lexer.fork().next_token()
    }

    /// Peek a [Fragment] -- this is mostly for error reporting, when you need to get the location you 
    /// expected something to be. This returns the [Fragment] for the next [Token] (without consuming it) if
    /// there is one, and if not, returns a zero-length fragment at the end of the file being parsed. 
    pub fn peek_fragment(&self) -> Fragment<'src> {
        self.peek_token()
            .map(|token| token.fragment)
            .unwrap_or_else(|| {
                // Use this to get a fragment of "" at the end of the file. 
                Fragment { inner: &self.full_source.inner[self.full_source.len()..] }
            })
    }

    /// Consume the next [Token] from the internal [Lexer]. 
    #[inline]
    pub fn next_token(&mut self) -> Option<Token<'src>> {
        self.lexer.next_token()
    }

    /// Peek the next [Token] from the internal [Lexer] and return true if it exists and 
    /// the [Token::variant] is equal to `kind`. 
    pub fn next_is(&self, kind: TokenTy) -> bool {
        self.peek_token()
            .map(|token| token.variant == kind)
            .unwrap_or(false)
    }

    /// Consume and return the next [Token] from the internal [Lexer] if it exists and 
    /// the [Token::variant] is equal to `kind`.
    /// 
    /// See also: [AstGeneratorContext::next_is]
    pub fn next_if_is(&mut self, kind: TokenTy) -> Option<Token<'src>> {
        if self.next_is(kind) { 
            self.next_token()
        } else {
            None
        }
    }

    /// Peek the next [Token] from the internal [Lexer] and return true if it exists and 
    /// the [Token::variant] matches any of the ones produced by `kinds`.
    pub fn next_is_any(&self, kinds: impl IntoIterator<Item = TokenTy>) -> bool {
        self.peek_token()
            .map(|token| kinds.into_iter().any(|kind| kind == token.variant))
            .unwrap_or(false)
    }

    /// Consume and return the next [Token] from the internal [Lexer] if it exists and 
    /// the [Token::variant] matches any of the ones produced by `kinds`. 
    pub fn next_if_is_any(&mut self, kinds: impl IntoIterator<Item = TokenTy>) -> Option<Token<'src>> {
        if self.next_is_any(kinds) {
            self.next_token()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{filemap::{FileMap, FileName}, parser::fragment::Fragment};
    use super::AstGeneratorContext;

    #[test]
    fn peek_fragment_empty_file() {
        let mut file_map = FileMap::new();
        let source = "";
        let id = file_map.add_str_ref(FileName::Test("peek_empty_frag"), source);
        let ast_gen_ctx = AstGeneratorContext::new(id, &file_map);

        assert!(ast_gen_ctx.peek_fragment().ptr_eq(&Fragment { inner: source }));
    }

    #[test]
    fn peek_fragment_end_of_file() {
        let mut file_map = FileMap::new();
        let source = "Test";
        let id = file_map.add_str_ref(FileName::Test("peek_end_of_file_frag"), source);
        let mut ast_gen_ctx = AstGeneratorContext::new(id, &file_map);

        assert!(ast_gen_ctx.next_token().unwrap().fragment.ptr_eq(&Fragment { inner: source }));
        
        let end_frag = ast_gen_ctx.peek_fragment();

        assert!(Fragment { inner: source }.contains(&end_frag));
        assert!(end_frag.is_empty());
        assert_eq!(end_frag.inner, "");
    }

    #[test]
    fn construct_ast_gen_ctx() {
        let mut file_map = FileMap::new();

        let str_ref = "Test String A";

        let id_a = file_map.add_str_ref(FileName::Test("string ref"), str_ref);
        let id_b = file_map.add_string(FileName::Test("owned string"), "Test String B".to_owned());

        AstGeneratorContext::new(id_a, &file_map);
        AstGeneratorContext::new(id_b, &file_map);
    }
}

