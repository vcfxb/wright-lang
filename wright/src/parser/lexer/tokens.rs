use derive_more::Display;

/// Token of Wright source code.
#[derive(Clone, Copy, Debug, Display)]
#[display(fmt = "{} ({}b)", variant, length)]
pub struct Token {
    /// What type of token is it?
    pub variant: TokenTy,
    /// How many bytes of source code long is it? Note this doesn't necessarily mean how many characters long it is.
    pub length: usize,
}

/// All of the reserved words are just upper-case versions of the
/// matching source code unless otherwise stated.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Display)]
pub enum TokenTy {
    // Operators and parentheses
    LeftParen,      // (
    RightParen,     // )
    Bang,           // !
    BangEq,         // !=
    Tilde,          // ~
    TildeArrow,     // ~>
    TildeEq,        // ~=
    At,             // @
    Pound,          // #
    Dollar,         // $
    Mod,            // %
    ModEq,          // %=
    Xor,            // ^
    XorEq,          // ^=
    And,            // &
    AndEq,          // &=
    AndAnd,         // &&
    Or,             // |
    OrEq,           // |=
    OrOr,           // ||
    Star,           // *
    StarEq,         // *=
    Plus,           // +
    PlusEq,         // +=
    Minus,          // -
    MinusEq,        // -=
    SingleArrow,    // ->
    Gt,             // >
    GtEq,           // >=
    ShiftRight,     // >>
    Lt,             // <
    LtEq,           // <=
    ShiftLeft,      // <<
    Eq,             // =
    EqEq,           // ==
    DoubleArrow,    // =>
    Div,            // /
    DivEq,          // /=
    Semi,           // ;
    Colon,          // :
    ColonColon,     // ::
    ColonEq,        // :=
    Question,       // ?
    Dot,            // .
    Range,          // ..
    RangeInclusive, // ..=
    Comma,          // ,
    LeftSquare,     // [
    RightSquare,    // ]
    LeftBracket,    // {
    RightBracket,   // }

    // Reserved words
    Class,
    Struct,
    Record,
    Trait,
    Fn,
    /// Publicly visible.
    Public,
    /// Visible in the package only. 
    Package,
    Constraint,
    Constrain,
    /// Used to constrain relations between variables. 
    Relation,
    Enum,
    Union,
    Unsafe,
    /// May use similar to unsafe in Rust -- call a function or cast without checking any of the constraints.
    Unchecked,
    Import,
    Impl,
    Type,
    Const,
    Var,
    If,
    Else,
    Is,
    As,
    /// For try { } blocks.
    Try,

    /// `Self` in source code.
    #[display(fmt = "Self")]
    SelfUpper,

    /// `self` in source code.
    #[display(fmt = "self")]
    SelfLower,

    /// `mod` in source code.
    Module,

    /// Whitespace of any kind and length.
    #[display(fmt = "W")]
    Whitespace,

    /// Single line comment started with `#`. Optionally `## ` or `##! ` for documentation.
    #[display(fmt = "Single line {} comment", comment_type)]
    SingleLineComment {
        comment_type: CommentTy,
    },

    /// Multiline comment between `#*` and `*#`. Starts with `#**` or `#*!` for documentation.
    #[display(
        fmt = "Multiline {} comment (terminated = {})",
        comment_type,
        is_terminated
    )]
    MultilineComment {
        comment_type: CommentTy,
        /// Is this comment terminated? If not raise an error before parsing the tokens.
        is_terminated: bool,
    },

    /// Integer literal. This is a literal integer in source code. May include underscores after the leading digit
    /// as visual seperators. May also include a prefix such as `0x`, `0o`, or `0b` for hex, octal, or binary.
    IntegerLit,

    /// A string literal in source code.
    #[display(
        fmt = "StringLit (fmt = {}, terminated = {})",
        is_format,
        is_terminated
    )]
    StringLit {
        /// For format strings (backticks instead of double quotes)
        is_format: bool,
        /// Is this string terminated?
        is_terminated: bool,
    },

    /// A character literal in source code.
    #[display(fmt = "CharLit (terminated = {})", is_terminated)]
    CharLit {
        /// Is the char lit terminated?
        is_terminated: bool,
    },

    /// A identifier in source code (such as a variable name). At this stage keywords (such as 'struct') are
    /// also considered identifiers.
    #[display(fmt = "ID")]
    Identifier,

    /// Unknown character for the lexer.
    #[display(fmt = "?")]
    Unknown,
}

/// Different types of comments.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Display)]
pub enum CommentTy {
    /// Normal comment that does not get used in documentation.
    Normal,
    /// Documentation for a declaration in the file.
    InnerDoc,
    /// Documentation for the file itself.
    OuterDoc,
}
