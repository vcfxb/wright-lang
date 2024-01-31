//! First pass lexer that gets run on the source code and returns a series of tokens with their associated [Fragment]s.
//! 
//! Note that this will strip out comments and whitespace, returning only fragments that match one of the paterns 
//! defined for tokens. 

use super::fragment::Fragment;

/// Constant table of single character tokens and the characters that match them. 
pub const SINGLE_CHAR_TOKENS: &[(char, TokenTy)] = &[
    ('(', TokenTy::LeftParen),
    (')', TokenTy::RightParen),
    ('[', TokenTy::LeftBracket),
    (']', TokenTy::RightBracket),
    ('{', TokenTy::LeftCurly),
    ('}', TokenTy::RightCurly),
    ('@', TokenTy::At),
    (';', TokenTy::Semi),
    ('?', TokenTy::Question),
    (',', TokenTy::Comma),
    ('#', TokenTy::Hash),
    ('$', TokenTy::Dollar),
];

/// Tokens that can be either a single character or upgraded with an
/// equals sign. 
pub const POSSIBLE_EQ_UPGRADE_TOKENS: &[(char, TokenTy, TokenTy)] = &[
    ('!', TokenTy::Bang, TokenTy::BangEq),
    ('%', TokenTy::Mod, TokenTy::ModEq),
    ('^', TokenTy::Xor, TokenTy::XorEq),
    ('*', TokenTy::Star, TokenTy::StarEq),
    ('+', TokenTy::Plus, TokenTy::PlusEq),
    ('/', TokenTy::Div, TokenTy::DivEq),
];

/// Characters that can produce different tokens when followed by an equals sign or themselves. 
pub const POSSIBLE_EQ_OR_DOUBLED_UPGRADE_TOKENS: &[(char, TokenTy, TokenTy, TokenTy)] = &[
    ('&', TokenTy::And, TokenTy::AndEq, TokenTy::AndAnd),
    ('|', TokenTy::Or, TokenTy::OrEq, TokenTy::OrOr),
    ('<', TokenTy::Lt, TokenTy::LtEq, TokenTy::LtLt),
    ('>', TokenTy::Gt, TokenTy::GtEq, TokenTy::GtGt),
    (':', TokenTy::Colon, TokenTy::ColonEq, TokenTy::ColonColon),
];

/// Characters that can produce different tokens when followed by an equals sign or 
/// a `>` for arrows.
pub const POSSIBLE_EQ_OR_ARROW_UPGRADE_TOKENS: &[(char, TokenTy, TokenTy, TokenTy)] = &[
    ('-', TokenTy::Minus, TokenTy::MinusEq, TokenTy::SingleArrow),
    ('=', TokenTy::Eq, TokenTy::EqEq, TokenTy::DoubleArrow),
];

/// The lexical analyser for wright. This produces a series of tokens that make up the larger elements of the language. 
#[derive(Debug)]
pub struct Lexer<'src> {
    /// The remaining source code that has not been processed and returned as a token from the iterator yet. 
    pub remaining: Fragment<'src>,
}

/// A token in wright source code. 
#[derive(Debug)]
pub struct Token<'src> {
    /// What type of token this is. 
    pub variant: TokenTy,
    /// The matching fragment of source code -- this contains the location and length data for the token. 
    pub fragment: Fragment<'src>
}

/// The different types of tokens in wright source.
#[rustfmt::skip] // Turn off auto reformat. 
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenTy {
    LeftCurly, RightCurly,
    LeftBracket, RightBracket,
    LeftParen, RightParen,

    Plus, PlusEq,
    Star, StarEq,
    Div, DivEq,
    Xor, XorEq,
    Mod, ModEq,
    Bang, BangEq,

    Minus, MinusEq, SingleArrow,
    Eq, EqEq, DoubleArrow,

    Lt, LtEq, LtLt,
    Gt, GtEq, GtGt,
    And, AndEq, AndAnd,
    Or, OrEq, OrOr,
    Colon, ColonEq, ColonColon,

    At,
    Tilde,
    Underscore,
    Semi,
    Dot,
    Comma,
    Hash,
    Question,
    Dollar,

    Identifier,

    OuterDocComment, OuterBlockDocComment,
    InnerDocComment, InnerBlockDocComment,

    KwRecord,
    KwType,
    KwEnum,
    KwUnion,
    KwFunc,
    KwRepr,
    KwImpl,
    KwConstraint,
    KwTrait,
    KwUse,
    KwAs,
    KwConst,
    KwMod,
    KwIf,
    KwElse,
    KwFor,
    KwIn,
    KwWhile,
    KwTrue,
    KwFalse,
    KwLoop,
    KwWhere,

    /// Unknown character in lexer fragment. 
    Unknown
}

impl<'src> Lexer<'src> {
    /// Get the number of bytes remaining that we need to transform into tokens. 
    pub const fn bytes_remaining(&self) -> usize {
        self.remaining.len()
    }

    /// Construct a new lexer over a given reference to a source string. 
    pub const fn new(source: &'src str) -> Self {
        Lexer { remaining: Fragment { inner: source } }
    }

    /// Try to match a fragment recognized to be an identifier or keyword to
    /// a keyword or return [TokenTy::Identifier]. 
    fn identifier_or_keyword(fragment: Fragment<'src>) -> TokenTy {
        use TokenTy::*;

        match fragment.inner {
            "record" => KwRecord,
            "type" => KwType,
            "enum" => KwEnum,
            "union" => KwUnion,
            "func" => KwFunc,
            "repr" => KwRepr,
            "impl" => KwImpl,
            "constraint" => KwConstraint,
            "trait" => KwTrait,
            "const" => KwConst,
            "where" => KwWhere,
            
            "use" => KwUse,
            "as" => KwAs,
            "mod" => KwMod,
            
            "if" => KwIf,
            "else" => KwElse,

            "for" => KwFor,
            "in" => KwIn,
            "while" => KwWhile,
            "loop" => KwLoop,

            "true" => KwTrue,
            "false" => KwFalse,

            _ => Identifier
        }
    }

    /// Make a token by splitting a given number of bytes off of the `self.remaining` fragment
    /// and labeling them with the given kind. 
    fn split_token(&mut self, bytes: usize, kind: TokenTy) -> Token<'src> {
        let (token_fragment, new_remaining_fragment) = self.remaining.split(bytes);
        self.remaining = new_remaining_fragment;
        Token { variant: kind, fragment: token_fragment }
    }

    /// Get the next token from the lexer.
    pub fn next_token(&mut self) -> Option<Token<'src>> {
        // If the remaining input is empty, there is no token. 
        if self.remaining.is_empty() {
            return None;
        }

        // Otherwise create a char iterator on the fragment. 
        // This one will be mainly used to check for shorter tokens -- a new one may be created later
        // to check for identifiers and strings. 
        let mut char_indices = self.remaining.inner.chars();

        // Get the next character from the iterator. 
        let next_char = char_indices.next().unwrap();

        // Match a single character if possible. 
        for (c, kind) in SINGLE_CHAR_TOKENS {
            if next_char == *c {
                return Some(self.split_token(next_char.len_utf8(), *kind));
            }
        }

        // Get the character after the next char if there is one. 
        let following_char_option = char_indices.next();

        // Try to match a token that can be augmented with a possible additional equal sign. 
        for (c, without_eq, with_eq) in POSSIBLE_EQ_UPGRADE_TOKENS {
            if next_char == *c {
                match following_char_option {
                    Some('=') => return Some(self.split_token(next_char.len_utf8() + 1, *with_eq)),
                    _ => return Some(self.split_token(next_char.len_utf8(), *without_eq)),
                }   
            }
        }

        unimplemented!()
    }

}
