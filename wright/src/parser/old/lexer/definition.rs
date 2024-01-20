//! The lexer definition in a rust constant that tells us how to handle characters encountered and lists all the 
//! possible tokens produced. 

use super::tokens::{TokenTy};

/// A single character token matches a single character from the input, and produces a token of the length of the 
/// character exactly. 
#[derive(Clone, Copy, Debug)]
pub struct SingleCharToken {
    /// The character to match. 
    pub matching_char: char, 
    /// The token type produced. 
    pub produces: TokenTy, 
}

impl SingleCharToken {
    /// Turn a single character token into a lexer branch. 
    const fn into_lexer_branch(self) -> LexerBranch {
        LexerBranch::SingleCharToken(self)
    }
}

/// A set of posible continuations from a single char token that will form multi char tokens 
/// (i.e. going from `&` to `&&` and `&=`).  
#[derive(Clone, Copy, Debug)]
pub struct PossibleContinuations {
    /// The base single char and the token it produces when not followed by one of the other possible characters. 
    pub base: SingleCharToken,
    /// The characters that can follow this and the tokens they would produce. 
    pub continuations: &'static [(char, TokenTy)]
}

impl PossibleContinuations {
    /// Convert to a [LexerBranch].
    const fn into_lexer_branch(self) -> LexerBranch {
        LexerBranch::PossibleContinuations(self)
    }
}

/// A branch in the lexer, representing options to be tried. 
#[derive(Debug)]
pub enum LexerBranch {
    /// A single character token (such as '[') with no option for continuation. 
    SingleCharToken(SingleCharToken),
    PossibleContinuations(PossibleContinuations)

}

// Below is a variety of `const-fn`s to make generating this structure easier. 

/// Makes a [SingleCharToken]. 
const fn single(matching_char: char, produces: TokenTy) -> SingleCharToken {
    SingleCharToken { matching_char, produces }
}

/// Makes a [PossibleContinuations].
const fn pc(matching_char: char, produces: TokenTy, continuations: &'static [(char, TokenTy)]) -> PossibleContinuations {
    PossibleContinuations { base: single(matching_char, produces), continuations }
}


/// The lexer's definition, in abstract branching. 
pub const DEFINITION: &[LexerBranch] = &[
    single('(', TokenTy::LeftParen).into_lexer_branch(),
    single(')', TokenTy::RightParen).into_lexer_branch(),

    pc('+', TokenTy::Plus, &[
        ('=', TokenTy::PlusEq),
    ]).into_lexer_branch(),


];
