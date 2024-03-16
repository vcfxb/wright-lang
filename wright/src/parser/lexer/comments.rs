//! Implementation of comment token lexing.

use super::{token::TokenTy, Lexer};

/// The pattern that begins any single line comments (including doc comments).
pub const SINGLE_LINE_COMMENT_PREFIX: &str = "//";

/// The pattern that starts any multi-line comments (including doc comments).
pub const MULTI_LINE_COMMENT_START: &str = "/*";

/// The pattern that ends any multi-line comments (including doc comments).
pub const MULTI_LINE_COMMENT_END: &str = "*/";

/// Attempt to match a sinlgle line comment from the start of the [Lexer::remaining] fragment.
/// Return a [usize] and optionally a [TokenTy]. The [usize] indicates how many bytes were in the comment.
/// The [TokenTy] (if it's not [None]) should be either [TokenTy::InnerDocComment] or [TokenTy::OuterDocComment].
///
/// If the [TokenTy] is not [None], the lexer should consume the specified number of bytes (by the [usize]) and
/// Produce a token with the [variant](super::token::Token::variant) from this function.
///
/// Generally I'm trying to follow the [rust comment spec] here.
///
/// [rust comment spec]: https://doc.rust-lang.org/reference/comments.html
pub fn try_match_single_line_comment(lexer: &Lexer) -> (usize, Option<TokenTy>) {
    // Fork the lexer so we can do all the parsing on the fork without worrying about modifying the original
    // unnecessarily.
    let mut fork: Lexer = lexer.fork();

    // Try to consume the single line comment prefix from the fork.
    if fork.consume(SINGLE_LINE_COMMENT_PREFIX) {
        // We consumed it successfully, read through a newline or the end of the forked lexer if we get there.

        // First determine if this is a doc comment of some kind.
        let is_inner_doc: bool = fork.matches("/") && !fork.matches("//");
        let is_outer_doc: bool = fork.matches("!");

        // The consume until a newline, carraige return, or the end of the source fragment.
        while !fork.remaining.is_empty() && !fork.matches("\r") && !fork.matches("\n") {
            fork.consume_any();
        }

        // Determine the kind of token to produce (if any).
        let variant: Option<TokenTy> = match (is_inner_doc, is_outer_doc) {
            (true, false) => Some(TokenTy::InnerDocComment),
            (false, true) => Some(TokenTy::OuterDocComment),
            (false, false) => None,
            (true, true) => unreachable!("It is impossible for the `remaining` fragment to start with an `!` and a `/` simultaneously.")
        };

        // Return the number of bytes consumed and the type of token to
        // produce if any.
        return (fork.offset_from(lexer), variant);
    }

    // If the single line comment prefix was not immediately available, there is no comment.
    (0, None)
}

/// Attempt to match a block comment from the start of the [Lexer::remaining] fragment.
/// Return a [usize] and optionally a [TokenTy]. The [usize] indicates how many bytes were in the comment.
/// The [TokenTy] (if it's not [None]) should be [TokenTy::InnerBlockDocComment], [TokenTy::OuterBlockDocComment], or
/// [TokenTy::UnterminatedBlockComment].
///
/// If the [TokenTy] is not [None], the lexer should consume the specified number of bytes (by the [usize]) and
/// Produce a token with the [variant](super::token::Token::variant) from this function.
pub fn try_match_block_comment(lexer: &Lexer) -> (usize, Option<TokenTy>) {
    // Handle corner cases here so we don't have to below.
    // These are both considered empty non-documenting comments.
    if lexer.matches("/***/") {
        return (5, None);
    }

    if lexer.matches("/**/") {
        return (4, None);
    }

    // Make a fork of the lexer to avoid modifying this lexer if we fail to parse.
    let mut fork: Lexer = lexer.fork();

    // Try to parse the start of a multi-line comment.
    if fork.consume(MULTI_LINE_COMMENT_START) {
        // Check if this is a doc comment.
        let is_outer_doc: bool = fork.matches("!");
        // Use this to indicate that more than one following asterix is not a doc comment.
        let is_inner_doc: bool = fork.matches("*") && !fork.matches("**");

        // Consume until we see the end of the doc comment. If we run out of characters, consider the
        // comment unterminated.
        while !fork.matches(MULTI_LINE_COMMENT_END) {
            // Handle nested comments here:
            if fork.matches(MULTI_LINE_COMMENT_START) {
                // Discard the output -- don't care about doc comments in other comments.
                let (nested_comment_bytes, _) = try_match_block_comment(&fork);

                // SAFETY: the return from this function should never be on a char boundary or out of bounds.
                // This is because the return value is always either 0 or calculated using `offset_from`.
                unsafe { fork.advance_unchecked(nested_comment_bytes) };

                // Restart the loop to keep consuming this comment.
                continue;
            }

            // Handle unterminated comments here.
            if fork.remaining.is_empty() {
                // If we have not hit a "*/" before the end of the input, return an unterminated block comment.
                let bytes_consumed: usize = fork.offset_from(lexer);
                return (bytes_consumed, Some(TokenTy::UnterminatedBlockComment));
            }

            // If there's still input, and not a nested comment, consume it.
            fork.consume_any();
        }

        // If we get here, the comment was terminated. Consume the terminating characters, and return.
        // Use debug assert here to make sure that the comment is actually terminated.
        let consumed_comment_terminator: bool = fork.consume(MULTI_LINE_COMMENT_END);
        debug_assert!(consumed_comment_terminator, "comment is actually terminated");

        // Determine the kind of token to produce (if any).
        let variant: Option<TokenTy> = match (is_inner_doc, is_outer_doc) {
            (true, false) => Some(TokenTy::InnerBlockDocComment),
            (false, true) => Some(TokenTy::OuterBlockDocComment),
            (false, false) => None,
            (true, true) => {
                unreachable!("Lexer should not match multiple comment types at once.")
            }
        };

        return (fork.offset_from(lexer), variant);
    }

    (0, None)
}

#[cfg(test)]
mod tests {
    use super::Lexer;

    #[test]
    fn ignored_single_line_comment() {
        let mut lexer = Lexer::new("// test comment ");
        assert!(lexer.next_token().is_none());
        assert_eq!(lexer.remaining.len(), 0);
    }
}
