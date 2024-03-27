//! Expressions grouped in parentheses in Wright. 

use crate::parser::{ast::{expression::{Expression, ExpressionParsingError}, AstGeneratorContext, AstNode}, fragment::Fragment, lexer::token::TokenTy};

/// An expression enclosed in parentheses in wright source code. 
#[derive(Debug)]
pub struct ParensExpression<'src> {
    /// The matching fragment of source code. 
    pub fragment: Fragment<'src>,

    /// The expression enclosed in parenthesese. 
    pub expression: Box<Expression<'src>>,
}

/// Error parsing parentheses expression. 
#[derive(Debug, Clone)]
pub enum ParensParsingError<'src> {
    /// A closing parenthese was not found.
    ClosingParenNotFound {
        /// The location a closing parenthese was expected. 
        at: Fragment<'src>
    },

    /// An error occurred while parsing withing the parenthesese. 
    ErrorInParentheses(Box<ExpressionParsingError<'src>>),

    /// A parentheses expression was expected and was not found. 
    ExpectedParensExpression {
        /// The location a parentheses expression was expected. 
        at: Fragment<'src> 
    }
}

impl<'src> AstNode<'src> for ParensExpression<'src> {
    type Error = ParensParsingError<'src>;

    fn fragment(&self) -> Fragment<'src> {
        self.fragment
    }

    fn try_parse(ctx: &mut AstGeneratorContext<'src>) -> Result<Self, Self::Error>
    where
        Self: Sized 
    {
        // Fork the parser and attempt to parse on the fork. 
        let mut fork: AstGeneratorContext = ctx.fork();

        // Parse through the left paren. 
        fork
            .next_if_is(TokenTy::LeftParen)
            .ok_or_else( || ParensParsingError::ExpectedParensExpression { at: fork.peek_fragment() })?;

        // Parse the expression in the parentheseses. Error if there is not one. 
        let expr: Expression = Expression::try_parse(&mut fork)
            // Box up the error and then wrap it in the correct variant. 
            .map_err(Box::new)
            .map_err(ParensParsingError::ErrorInParentheses)?;

        // Parse the closing parentheses. 
        fork
            .next_if_is(TokenTy::RightParen)
            .ok_or_else(|| ParensParsingError::ClosingParenNotFound { at: fork.peek_fragment() })?;

        // Update the parsing context. Use the trimmed fragment since the lexer may have consumed whitespace
        // before the first paren.
        let consumed_source: Fragment = ctx.update(&fork).trimmed();
        // Return the parens expression. 
        Ok(Self { fragment: consumed_source, expression: Box::new(expr) })
    }
}
