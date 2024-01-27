//! First pass lexer that gets run on the source code and returns a series of tokens with their associated [Fragment]s.
//! 
//! Note that this will strip out comments and whitespace, returning only fragments that match one of the paterns 
//! defined for tokens. 

use super::fragment::Fragment;

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
    Minus, MinusEq,
    Star, StarEq,
    Div, DivEq,
    Xor, XorEq,
    Mod, ModEq,
    Bang, BangEq,
    Eq, EqEq,

    Lt, LtEq, LtLt,
    Gt, GtEq, GtGt,
    And, AndEq, AndAnd,
    Or, OrEq, OrOr,
    
    Colon, ColonColon,

    At,
    Tilde,
    Underscore,
    Semi,
    Dot,
    Comma,
    Hash,

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

    /// Try to match a single character to a single character token if possible. 
    #[rustfmt::skip]
    const fn single_char_tokens(c: char) -> Option<TokenTy> {
        use TokenTy::*;

        match c {
            '{' => Some(LeftCurly),
            '}' => Some(RightCurly),
            '[' => Some(LeftBracket),
            ']' => Some(RightBracket),
            '(' => Some(LeftParen),
            ')' => Some(RightParen),
            
            '@' => Some(At),
            '~' => Some(Tilde),
            '_' => Some(Underscore),
            '.' => Some(Dot),
            ',' => Some(Comma),
            ';' => Some(Semi),
            '#' => Some(Hash),

            _ => None,
        }
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

}
