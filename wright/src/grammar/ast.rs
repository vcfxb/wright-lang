use crate::grammar::model::Fragment;

/// Numerical literal in wright source code.
/// i.e. `10`, `0xCa1a0`, `0b0101_0101`, `100_000`
#[derive(Copy, Clone, Debug)]
pub struct NumLit<'s> {
    /// Associated fragment of source code.
    pub frag: Fragment<'s>,
    /// Represented value.
    pub inner: u128,
}

/// Character literal in wright source code.
/// i.e 'a', '\n', '\u{01f441}', '\x00', '♦'
/// see [this page](https://doc.rust-lang.org/reference/tokens.html#ascii-escapes) for escape
/// information.
#[derive(Copy, Clone, Debug)]
pub struct CharLit<'s> {
    /// Associated fragment of source code.
    pub frag: Fragment<'s>,
    /// Represented Value.
    pub inner: char,
}
