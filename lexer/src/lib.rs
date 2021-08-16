//! An efficient lexer for Flycatcher source powered by Logos.

pub use logos::{Lexer, Logos};

/// A list of tokens that may be matched by the lexer.  Because of the wonderful Logos crate,
/// this enum also acts as a lexer, for example:
/// 
/// ```
/// // The `Logos` trait is required to use the lexer.
/// use flycatcher_lexer::Logos;
/// use flycatcher_lexer::Token;
/// 
/// let mut lexer = Token::lexer("'Hello, world!'");
/// assert_eq!(lexer.next(), Some(Token::String));
/// assert_eq!(lexer.slice(), "'Hello, world!'");
/// ```
#[derive(Clone, Debug, Logos, PartialEq)]
pub enum Token {

    /// A Flycatcher style string literal, which may start and end with either `'` or `"`.  It
    /// allows escaping characters, but those are not parsed here.
    /// 
    /// ```
    /// use flycatcher_lexer::Logos;
    /// use flycatcher_lexer::Token;
    /// 
    /// let mut lexer = Token::lexer("'Hello, world!' \"Hello, world!\"");
    /// assert_eq!(lexer.next(), Some(Token::String));
    /// assert_eq!(lexer.slice(), "'Hello, world!'");
    /// assert_eq!(lexer.next(), Some(Token::String));
    /// assert_eq!(lexer.slice(), "\"Hello, world!\"");
    /// ```
    #[regex("\"([^\"\\\\]*(\\.[^\"\\\\]*)*)\"|'([^'\\\\]*(\\.[^'\\\\]*)*)'")]
    String,

    /// A Flycatcher style identifier literal.  An identifier must start with one of
    /// `a-z`/`A-Z`, `_` or `$`.  Any character after that must be one of `a-z`/`A-Z`, `_`, `$`
    /// or `0-9`.
    /// 
    /// ```
    /// use flycatcher_lexer::Logos;
    /// use flycatcher_lexer::Token;
    /// 
    /// let mut lexer = Token::lexer("Hello");
    /// assert_eq!(lexer.next(), Some(Token::Identifier));
    /// assert_eq!(lexer.slice(), "Hello");
    /// ```
    #[regex(r"[a-zA-Z_$][a-zA-Z0-9]*")]
    Identifier,

    /// This token matches any whitespace character, including regular whitespaces, tabs and
    /// line breaks/new lines.  It is ignored at lexing time and Logos will pass over it if it
    /// is found.
    /// 
    /// ```
    /// use flycatcher_lexer::Logos;
    /// use flycatcher_lexer::Token;
    /// 
    /// let mut lexer = Token::lexer(" \t\n");
    /// assert_eq!(lexer.next(), None);
    /// ```
    #[regex(r"\s", logos::skip)]
    Whitespace,

    /// The `Invalid` token matches any character that doesn't match any other token types, it's
    /// basically a catchall.  This is recognized as an error at parsing time and will always
    /// throw an error.  The only time that it's impossible to find this token is inside of a
    /// string, any characters may be matched in a string, provided they are UTF characters.
    #[error]
    Invalid,

}