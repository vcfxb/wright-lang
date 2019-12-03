use codespan::{
    Span,
    ByteIndex,
    ByteOffset,
    FileId,
    Files
};

use codespan_reporting::diagnostic::*;

use crate::grammar::lexer::Token;

/// All token parsers must implement this trait.
pub trait Parser {
    /// This parser's rule name.
    const RULE: &'static str;

    /// Attempt to parse a token from a lexer's source code.
    fn try_parse(lexer: &mut Lexer) -> Option<Token>;

    /// Parse a token from a lexer's source code or return a diagnostic.
    fn do_parse(lexer: &mut Lexer) -> Result<Token, Diagnostic>;
}

/// A tool to scan and get tokens from Wright source code.
pub struct Lexer<'s> {
    span: Span,
    index: ByteIndex,
    tokens: Vec<Token>,
    source: &'s str,
    char_stack: Vec<char>,

    /// Reference to original Files database.
    pub files_ref: &'s Files,
    /// Handle of the file being parsed by this lexer.
    pub handle: FileId,
}

impl<'s> Lexer<'s> {
    /// Construct a new Lexer for an item in a Files database.
    pub fn new(files: &'s Files, handle: FileId) -> Self {
        let source = files.source(handle);
        let mut chars: Vec<char> = source.chars().collect();
        chars.reverse();
        let mut s = Lexer {
            span: files.source_span(handle),
            index: ByteIndex::default(),
            tokens: Vec::new(),
            files_ref: files,
            handle,
            source,
            char_stack: chars,
        };
        s.index = s.span.start();
        s
    }

    /// Try to parse a token from this lexer's source code.
    pub fn try_parse<T:Parser>(&mut self, _: T) -> Option<Token> {
        T::try_parse(self)
    }

    /// Parse a token from this lexer's source code.
    pub fn do_parse<T:Parser>(&mut self, _: T) -> Result<Token, Diagnostic> {
        T::do_parse(self)
    }

    /// Check if this lexer is at the end of input.
    pub fn is_at_end(&self) -> bool {
        self.char_stack.is_empty()
    }

    /// Peek at the next character. Returns null character if there is not a next
    /// character (this lexer is at end of input).
    pub fn peek(&self) -> char {
        self.lookahead(0)
    }

    /// Lookahead at a char several chars in front of the lexer's read head.
    /// lookahead of 0 is next character, and is identical to calling `peek`.
    /// If index is past the end of the file a null character ('\0') is returned.
    pub fn lookahead(&self, lookahead: usize) -> char {
        self.char_stack
            .get(self.char_stack.len()-1-lookahead)
            .map(|c|*c)
            .unwrap_or('\0')
    }

    /// Conditionally advances if (and only if) the next character is equal to
    /// argument ch. Returns true if this lexer advanced (if the character was a match)
    /// and false otherwise.
    pub fn matches(&mut self, ch: char) -> bool {
        if self.peek() == ch {
            self.next();
            true
        } else {
            false
        }
    }

    /// Advance the lexer, returning the next character. Returns null character
    /// (`'\0'`) if at the end of the source.
    pub fn next(&mut self) -> char {
        let c = self.char_stack.pop();
        if c.is_some() {
            let ch = c.unwrap();
            self.index += ByteOffset(ch.len_utf8() as i64);
            ch
        } else {
            self.index = self.span.end();
            '\0'
        }
    }

    /// Advance the lexer a certain number of times. Return the string consumed.
    /// Returns None if the lexer passes end of source.
    pub fn advance(&mut self, count: usize) -> Option<String> {
        if self.lookahead(count-1) == '\0' {None}
        else {
            let mut s = String::new();
            for i in 0..count {
                s.push(self.next());
            }
            Some(s)
        }
    }

    /// Retrieve the index of the parser head of this lexer.
    pub fn get_index(&self) -> ByteIndex {self.index}
}