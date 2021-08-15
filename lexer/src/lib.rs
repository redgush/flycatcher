//! An efficient hand-written lexer that tokenizes any valid or invalid Flycatcher source.

/// A list of tokens that the lexer may output given an input string.
pub enum Token {
    
    /// Matches any character that doesn't match any other token types.  This is recognized as
    /// an error at parsing time and will always throw an error.  The only time this error isn't
    /// thrown is in a string, any character may be matched in a string.
    Invalid,

    /// This is returned if the current index of the lexer is greater than or equal to the
    /// length of the input string, which means the token is an EndOfFile token.
    EndOfFile,

}

/// A Flycatcher lexer that takes an input string and tokenizes it.
pub struct Lexer<'a> {

    /// The input (source) string of characters that the lexer reads tokens from.
    pub source: &'a str,

    /// The index in the string of which the current token in the lexer starts.
    pub start: usize,

    /// The index in the input string where the current token ends.
    pub end: usize,

    /// The current token, taken out of the input string, if any.  If there is not a current
    /// token, then this is merely an empty string of characters.
    pub slice: &'a str,

    /// The current index in the `source` string that the lexer is reading tokens from.  This is
    /// advanced by the `next()` method.
    index: usize,

}

/// The functionality for the lexer.
impl<'a> Lexer<'a> {

    /// Allocates a lexer that will tokenize the provided string of characters.  This does not
    /// tokenize any tokens, it merely initializes a Lexer struct and returns it.
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            start: 0,
            end: 0,
            slice: "",
            index: 0,
        }
    }

    /// Returns whether or not the character provided counts as an alphabetical character.
    pub fn is_alpha(c: char) -> bool {
        match c {
            'a'..='z' |
            'A'..='Z' => true,
            _ => false
        }
    }

    /// Returns whether or not the provided character is part of the identifier start character
    /// set.
    /// 
    /// - `a-z`
    /// - `A-Z`
    /// - `_`
    /// - `$`
    pub fn is_iden_start(c: char) -> bool {
        match c {
            'a'..='z' |
            'A'..='Z' |
            '_' |
            '$' => true,
            _ => false
        }
    }

    /// Returns whether or not the provided character is part of the identifier continue
    /// character set.
    /// 
    /// - `a-z`
    /// - `A-Z`
    /// - `0-9`
    /// - `_`
    /// - `$`
    pub fn is_iden_continue(c: char) -> bool {
        match c {
            'a'..='z' |
            'A'..='Z' |
            '0'..='9' |
            '_' |
            '$' => true,
            _ => false,
        }
    }

    /// Returns whether or not a character counts as a numerical digit.
    pub fn is_digit(c: char) -> bool {
        match c {
            '0'..='9' => true,
            _ => false,
        }
    }

    /// Reads a token from current index of this lexer, returning what type of token was found.
    /// Advances the lexer's current position to the position of the next token.
    pub fn next(&mut self) -> Token {
        if self.index >= self.source.len() {
            // There is no input string left to read, so we need to return the `EndOfFile`
            // token.
            return Token::EndOfFile;
        }
        return Token::EndOfFile;
    }

}