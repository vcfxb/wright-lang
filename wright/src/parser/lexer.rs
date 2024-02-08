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

/// The number of rows of the generated prefix table. 
pub const PREFIX_TABLE_ROWS: usize = {
    SINGLE_CHAR_TOKENS.len() 
    + 2 * POSSIBLE_EQ_UPGRADE_TOKENS.len()
    + 3 * POSSIBLE_EQ_OR_DOUBLED_UPGRADE_TOKENS.len()
    + 3 * POSSIBLE_EQ_OR_ARROW_UPGRADE_TOKENS.len()
};

/// A relationship between a prefix and the token that should be generated when that prefix matches. 
#[derive(Copy, Clone, Debug)]
pub struct PrefixToToken {
    /// An array of two chars. In single char tokens, the second one should be a null character (`'\0'`). 
    /// the char_length field will be used to slice this buffer to get the actual prefix. 
    pub char_buffer: [char; 2],
    /// The byte length of this prefix and all generated tokens by this prefix. 
    pub byte_len: usize,
    /// The kind of [Token] generated when this prefix matches. 
    pub kind: TokenTy,
}

/// A full table generated at compile time using all the token tables 
/// ([SINGLE_CHAR_TOKENS], [POSSIBLE_EQ_UPGRADE_TOKENS], [POSSIBLE_EQ_OR_DOUBLED_UPGRADE_TOKENS], 
/// [POSSIBLE_EQ_OR_ARROW_UPGRADE_TOKENS]). 
/// 
/// This table can be iterated on in order when trying to match a token at the start of a fragment of source code. 
pub const PREFIX_TABLE: [PrefixToToken; PREFIX_TABLE_ROWS] = {
    // Make a mutable table with dummy values to replace with actual values. 
    let mut table: [PrefixToToken; PREFIX_TABLE_ROWS] = 
        [PrefixToToken { char_buffer: ['\0'; 2], byte_len: 0, kind: TokenTy::Unknown }; PREFIX_TABLE_ROWS];

    // Current index to insert into table at.
    let mut write_index: usize = 0;

    // Index used for reading from various tables. 
    let mut read_index: usize = 0;

    // Iterate first over all the single char tokens. 
    while read_index < SINGLE_CHAR_TOKENS.len() {
        // Get row from source table.
        let (c, token_kind) = SINGLE_CHAR_TOKENS[read_index];

        // Put row in destination table.
        table[write_index] = PrefixToToken {
            char_buffer: [c, '\0'],
            byte_len: c.len_utf8(),
            kind: token_kind,
        };

        // Increment both indices. 
        read_index += 1;
        write_index += 1;
    }

    // Then do all the tokens that can be upgraded with an equals sign. 
    // Add the row for the token with the equals sign first so that when we iterate over this table in order,
    // the version without the equals sign does not match prematurely. 
    read_index = 0;
    while read_index < POSSIBLE_EQ_UPGRADE_TOKENS.len() {
        let (c, without_eq, with_eq) = POSSIBLE_EQ_UPGRADE_TOKENS[read_index];

        table[write_index] = PrefixToToken {
            char_buffer: [c, '='],
            byte_len: c.len_utf8() + '='.len_utf8(),
            kind: with_eq,
        };

        write_index += 1;
        table[write_index] = PrefixToToken {
            char_buffer: [c, '\0'],
            byte_len: c.len_utf8(),
            kind: without_eq,
        };

        read_index += 1;
        write_index += 1;
    }

    // Do the same for the tokens that can be upgraded with an equals sign or doubled. 
    read_index = 0;
    while read_index < POSSIBLE_EQ_OR_DOUBLED_UPGRADE_TOKENS.len() {
        let (c, without_eq, with_eq, doubled) = POSSIBLE_EQ_OR_DOUBLED_UPGRADE_TOKENS[read_index];

        table[write_index] = PrefixToToken {
            char_buffer: [c, c],
            byte_len: 2 * c.len_utf8(),
            kind: doubled,
        };

        write_index += 1;
        table[write_index] = PrefixToToken {
            char_buffer: [c, '='],
            byte_len: c.len_utf8() + '='.len_utf8(),
            kind: with_eq,
        };

        write_index += 1;
        table[write_index] = PrefixToToken {
            char_buffer: [c, '\0'],
            byte_len: c.len_utf8(),
            kind: without_eq,
        };

        read_index += 1;
        write_index += 1;
    }

    // Do the same for possible eq or arrow upgrades.
    read_index = 0;
    while read_index < POSSIBLE_EQ_OR_ARROW_UPGRADE_TOKENS.len() {
        let (c, without_eq, with_eq, with_arrow) = POSSIBLE_EQ_OR_ARROW_UPGRADE_TOKENS[read_index];

        table[write_index] = PrefixToToken {
            char_buffer: [c, '>'],
            byte_len: c.len_utf8() + '>'.len_utf8(),
            kind: with_arrow,
        };

        write_index += 1;
        table[write_index] = PrefixToToken {
            char_buffer: [c, '='],
            byte_len: c.len_utf8() + '='.len_utf8(),
            kind: with_eq,
        };

        write_index += 1;
        table[write_index] = PrefixToToken {
            char_buffer: [c, '\0'],
            byte_len: c.len_utf8(),
            kind: without_eq,
        };

        read_index += 1;
        write_index += 1;
    }

    table
};


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

    /// See if the remaining fragment in this [Lexer] starts with a given [str] prefix and if so,
    /// split off a token of the length of this prefix with the given variant. 
    fn match_str_prefix(&mut self, prefix: &str, token_kind: TokenTy) -> Option<Token<'src>> {
        if self.remaining.inner.starts_with(prefix) {
            Some(self.split_token(prefix.len(), token_kind))
        } else {
            None
        }
    }


    /// Get the next token from the lexer.
    pub fn next_token(&mut self) -> Option<Token<'src>> {
        // If the remaining input is empty, there is no token. 
        if self.remaining.is_empty() {
            return None;
        }
        
        // To attempt to match a token from the prefix table, make a char iterator
        // and get two chars from it to test equality. None of the tokens start with a
        // null character so use that as a single of an unavailable char.
        let mut char_iter = self.remaining.chars();
        let char_array: [char; 2] = [
            // Just unwrap here since we know there's at least one char. 
            char_iter.next().unwrap(), 
            char_iter.next().unwrap_or('\0')
        ];

        // Next iterate through the prefix table to try to get any tokens that are covered there.
        for prefix_meta in PREFIX_TABLE.iter() {
            if &prefix_meta.char_buffer == &char_array {
                return Some(self.split_token(prefix_meta.byte_len, prefix_meta.kind));
            }
        }

        unimplemented!()
    }

}

#[cfg(test)]
mod tests {
    use crate::parser::lexer::TokenTy;

    use super::Lexer;
    use super::PREFIX_TABLE;

    #[test]
    #[ignore = "this test is just used for debugging the prefix table"]
    /// Run this with `cargo test manual_debug_prefix_table -- --nocapture --ignored`.
    fn manual_debug_prefix_table() {
        dbg!(PREFIX_TABLE);
    }

    #[test]
    fn plus_and_plus_eq_tokens() {
        let mut plus = Lexer::new("+");
        let mut plus_eq = Lexer::new("+=");

        let plus_token = plus.next_token().unwrap();
        let plus_eq_token = plus_eq.next_token().unwrap();

        assert_eq!(plus.bytes_remaining(), 0);
        assert_eq!(plus_eq.bytes_remaining(), 0);
        assert_eq!(plus_token.variant, TokenTy::Plus);
        assert_eq!(plus_eq_token.variant, TokenTy::PlusEq);
    }
}
