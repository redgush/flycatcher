//! Flycatcher's lexer crate.
//!
//! This crate takes an input string and converts it into more machine-understandable tokens, such as
//! identifiers, strings, numbers, etc.
//!
//! Of course, the lexer is one of the simplest parts of Flycatcher's implementation, especially thanks
//! to the `logos` lexer generator library!

mod token;

use logos::{Lexer as LogosLexer, Logos};
use std::ops::Range;
pub use token::Token;

/// A wrapper around the logos lexer, allowing for ease of peeking and the ability to catch errors while
/// lexing.
pub struct Lexer<'a> {
    /// This is the string that is being tokenized.  Not only is this used to find tokens, but it is
    /// also used in diagnostic messages to display what went wrong & where.
    pub input: &'a str,

    /// The Logos lexer that this struct wraps.  This will be used for recieving tokens and verifying
    /// them.
    lexer: LogosLexer<'a, Token>,
}

impl<'a> Lexer<'a> {
    /// Allocates a new lexer, initializing it as an iterator from the string provided.  The string will
    /// be stored and used by the lexer to find tokens.
    pub fn new(input: &'a str) -> Self {
        let lexer = Token::lexer(input);
        Self { input, lexer }
    }

    /// Returns the current token as a string value.
    pub fn slice(&mut self) -> &'a str {
        self.lexer.slice()
    }

    /// Returns the starting and ending positions of the current token.
    pub fn span(&mut self) -> Range<usize> {
        self.lexer.span()
    }

    /// "Peeks" at the next token in the input string, if any.  This is done by cloning the lexer's
    /// iterator and recieving the next value from it.
    pub fn peek(&self) -> Option<Token> {
        self.lexer.clone().next()
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.lexer.next()
    }
}
