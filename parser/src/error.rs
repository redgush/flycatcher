//! Exposes error types for the parser.

/// A list of possible errors that may occur during parsing.
#[derive(PartialEq)]
pub enum ErrorKind {

    /// A general syntax error, if this error is thrown, it is guaranteed that the parser will
    /// have emitted a diagnostic message.
    SyntaxError,

    /// Returned when there is no tokens left in the lexer that the parser uses.  The parser
    /// will not throw a diagnostic message if this error is found.
    EndOfFile,

}