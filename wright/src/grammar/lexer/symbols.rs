use codespan::{Span, ByteIndex, ByteOffset};

#[allow(missing_docs)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SymTy {
    Num(char),
    Letter(char),
    Symbol(char),
    Whitespace(char),
    LCurly, RCurly,
    LBracket, RBracket,
    LParen, RParen,
    LT, GT, // may transform into left angle and right angle.
    Quote, DoubleQuote,
    Plus, Minus, Equals,
    Slash, Star, Dot, Comma,
    Bang, Hash, At, Cash, Mod,
    Carrot, And, Bar, UnderScore,
    Backslash, Tilda, Backtick, Question,
}

impl SymTy {
    /// The source code character of this symbol variant.
    pub fn to_char(self) -> char {
        use SymTy::*;
        match self {
            Num(c) | Letter(c) | Symbol(c) | Whitespace(c)
                => c,
            LCurly => '{',
            RCurly => '}',
            LBracket => '[',
            RBracket => ']',
            LParen => '(',
            RParen => ')',
            LT => '<',
            GT => '>',
            Quote => '\'',
            DoubleQuote => '"',
            Plus => '+',
            Minus => '-',
            Equals => '=',
            Slash => '/',
            Star => '*',
            Dot => '.',
            Comma => ',',
            Bang => '!',
            Hash => '#',
            At => '@',
            Cash => '$',
            Mod => '%',
            Carrot => '^',
            And => '&',
            Bar => '|',
            UnderScore => '_',
            Backslash => '\\',
            Tilda => '~',
            Backtick => '`',
            Question => '?',
        }
    }

    /// The symbolic variant of a sourcecode char.
    pub fn from_char(c: char) -> Self {
        use SymTy::*;
        match c {
            '{' => LCurly,
            '}' => RCurly,
            '[' => LBracket,
            ']' => RBracket,
            '(' => LParen,
            ')' => RParen,
            '<' => LT,
            '>' => GT,
            '\'' => Quote,
            '"' => DoubleQuote,
            '+' => Plus,
            '-' => Minus,
            '=' => Equals,
            '*' => Star,
            '/' => Slash,
            '.' => Dot,
            ',' => Comma,
            '!' => Bang,
            '@' => At,
            '#' => Hash,
            '$' => Cash,
            '%' => Mod,
            '^' => Carrot,
            '&' => And,
            '|' => Bar,
            '_' => UnderScore,
            '?' => Question,
            '\\' => Backslash,
            '~' => Tilda,
            '`' => Backtick,
            c => {
                if c.is_ascii_digit() {Num(c)}
                else if c.is_ascii_alphabetic() {Letter(c)}
                else if c.is_ascii_whitespace() {Whitespace(c)}
                else {Symbol(c)}
            }
        }
    }
}

/// A single character in source code.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Sym {
    span: Span,
    inner: SymTy,
}

impl Sym {
    /// Construct a new token.
    pub fn new(span: Span, c: char) -> Self {
        Sym {
            span,
            inner: SymTy::from_char(c)
        }
    }

    /// Get this symbol's span.
    pub fn get_span(&self) -> Span {self.span}

    /// Get this symbol's type.
    pub fn get_ty(&self) -> SymTy {self.inner}
}

/// Translate a stream of input characters into a vector of first-pass symbols.
pub fn do_pass(iter: impl Iterator<Item=char>, span: Span) -> Vec<Sym> {
    iter.map(|c| (c, c.len_utf8()))
        .scan(span.start().to_usize(), |state, (c, len)| {
            let sp = Span::new(*state as u32, (*state+len) as u32);
            *state += len;
            Some(Sym::new(sp, c))
        })
        .collect()
}

impl std::fmt::Display for Sym {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.inner.to_char())
    }
}