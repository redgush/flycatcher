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
#[derive(Clone, Copy, Debug, Logos, PartialEq)]
pub enum Token {

    /// The equal comparison operator.
    /// 
    /// ```
    /// use flycatcher_lexer::Logos;
    /// use flycatcher_lexer::Token;
    /// 
    /// let mut lexer = Token::lexer("21 == 21");
    /// assert_eq!(lexer.next(), Some(Token::Number));
    /// assert_eq!(lexer.slice(), "21");
    /// assert_eq!(lexer.next(), Some(Token::EqualsEquals));
    /// assert_eq!(lexer.slice(), "==");
    /// assert_eq!(lexer.next(), Some(Token::Number));
    /// assert_eq!(lexer.slice(), "21");
    /// ```
    #[token("==")]
    EqualsEquals,

    /// The equal comparison operator.
    /// 
    /// ```
    /// use flycatcher_lexer::Logos;
    /// use flycatcher_lexer::Token;
    /// 
    /// let mut lexer = Token::lexer("21 != 21");
    /// assert_eq!(lexer.next(), Some(Token::Number));
    /// assert_eq!(lexer.slice(), "21");
    /// assert_eq!(lexer.next(), Some(Token::ExclaimationEquals));
    /// assert_eq!(lexer.slice(), "!=");
    /// assert_eq!(lexer.next(), Some(Token::Number));
    /// assert_eq!(lexer.slice(), "21");
    /// ```
    #[token("!=")]
    ExclaimationEquals,

    /// The greater than or equal comparison operator.
    /// 
    /// ```
    /// use flycatcher_lexer::Logos;
    /// use flycatcher_lexer::Token;
    /// 
    /// let mut lexer = Token::lexer("21 >= 21");
    /// assert_eq!(lexer.next(), Some(Token::Number));
    /// assert_eq!(lexer.slice(), "21");
    /// assert_eq!(lexer.next(), Some(Token::GreaterThanOrEqual));
    /// assert_eq!(lexer.slice(), ">");
    /// assert_eq!(lexer.next(), Some(Token::Number));
    /// assert_eq!(lexer.slice(), "21");
    /// ```
    #[token(">=")]
    GreaterThanOrEqual,

    /// The less than or equal comparison operator.
    /// 
    /// ```
    /// use flycatcher_lexer::Logos;
    /// use flycatcher_lexer::Token;
    /// 
    /// let mut lexer = Token::lexer("21 <= 21");
    /// assert_eq!(lexer.next(), Some(Token::Number));
    /// assert_eq!(lexer.slice(), "21");
    /// assert_eq!(lexer.next(), Some(Token::GreaterThanOrEqual));
    /// assert_eq!(lexer.slice(), "<=");
    /// assert_eq!(lexer.next(), Some(Token::Number));
    /// assert_eq!(lexer.slice(), "21");
    /// ```
    #[token("<=")]
    LessThanOrEqual,

    /// The greater than comparison operator.
    /// 
    /// ```
    /// use flycatcher_lexer::Logos;
    /// use flycatcher_lexer::Token;
    /// 
    /// let mut lexer = Token::lexer("21 > 21");
    /// assert_eq!(lexer.next(), Some(Token::Number));
    /// assert_eq!(lexer.slice(), "21");
    /// assert_eq!(lexer.next(), Some(Token::GreaterThan));
    /// assert_eq!(lexer.slice(), ">");
    /// assert_eq!(lexer.next(), Some(Token::Number));
    /// assert_eq!(lexer.slice(), "21");
    /// ```
    #[token(">")]
    GreaterThan,

    /// The less than comparison operator.
    /// 
    /// ```
    /// use flycatcher_lexer::Logos;
    /// use flycatcher_lexer::Token;
    /// 
    /// let mut lexer = Token::lexer("21 < 21");
    /// assert_eq!(lexer.next(), Some(Token::Number));
    /// assert_eq!(lexer.slice(), "21");
    /// assert_eq!(lexer.next(), Some(Token::LessThan));
    /// assert_eq!(lexer.slice(), "<");
    /// assert_eq!(lexer.next(), Some(Token::Number));
    /// assert_eq!(lexer.slice(), "21");
    /// ```
    #[token("<")]
    LessThan,

    /// A period, used for indexing objects.
    /// 
    /// ```
    /// use flycatcher_lexer::Logos;
    /// use flycatcher_lexer::Token;
    /// 
    /// let mut lexer = Token::lexer("item1.item2");
    /// assert_eq!(lexer.next(), Some(Token::Identifier));
    /// assert_eq!(lexer.slice(), "item1");
    /// assert_eq!(lexer.next(), Some(Token::Dot));
    /// assert_eq!(lexer.slice(), ".");
    /// assert_eq!(lexer.next(), Some(Token::Identifier));
    /// assert_eq!(lexer.slice(), "item2");
    /// ```
    #[token(".")]
    Dot,

    /// An opening bracket character (`[`).
    /// 
    /// ```
    /// use flycatcher_lexer::Logos;
    /// use flycatcher_lexer::Token;
    /// 
    /// let mut lexer = Token::lexer("item1[item2]");
    /// assert_eq!(lexer.next(), Some(Token::Identifier));
    /// assert_eq!(lexer.slice(), "item1");
    /// assert_eq!(lexer.next(), Some(Token::OBrack));
    /// assert_eq!(lexer.slice(), "[");
    /// assert_eq!(lexer.next(), Some(Token::Identifier));
    /// assert_eq!(lexer.slice(), "item2");
    /// assert_eq!(lexer.next(), Some(Token::CBrack));
    /// assert_eq!(lexer.slice(), "]");
    /// ```
    #[token("[")]
    OBrack,

    /// A closing bracket character (`[`).
    /// 
    /// ```
    /// use flycatcher_lexer::Logos;
    /// use flycatcher_lexer::Token;
    /// 
    /// let mut lexer = Token::lexer("item1[item2]");
    /// assert_eq!(lexer.next(), Some(Token::Identifier));
    /// assert_eq!(lexer.slice(), "item1");
    /// assert_eq!(lexer.next(), Some(Token::OBrack));
    /// assert_eq!(lexer.slice(), "[");
    /// assert_eq!(lexer.next(), Some(Token::Identifier));
    /// assert_eq!(lexer.slice(), "item2");
    /// assert_eq!(lexer.next(), Some(Token::CBrack));
    /// assert_eq!(lexer.slice(), "]");
    /// ```
    #[token("]")]
    CBrack,

    /// An opening parenthesis character (`(`).
    /// 
    /// ```
    /// use flycatcher_lexer::Logos;
    /// use flycatcher_lexer::Token;
    /// 
    /// let mut lexer = Token::lexer("func(1)");
    /// assert_eq!(lexer.next(), Some(Token::Identifier));
    /// assert_eq!(lexer.slice(), "func");
    /// assert_eq!(lexer.next(), Some(Token::OParen));
    /// assert_eq!(lexer.slice(), "(");
    /// assert_eq!(lexer.next(), Some(Token::Number));
    /// assert_eq!(lexer.slice(), "1");
    /// assert_eq!(lexer.next(), Some(Token::CParen));
    /// assert_eq!(lexer.slice(), ")");
    /// ```
    #[token("(")]
    OParen,

    /// An opening parenthesis character (`)`).
    /// 
    /// ```
    /// use flycatcher_lexer::Logos;
    /// use flycatcher_lexer::Token;
    /// 
    /// let mut lexer = Token::lexer("func(1)");
    /// assert_eq!(lexer.next(), Some(Token::Identifier));
    /// assert_eq!(lexer.slice(), "func");
    /// assert_eq!(lexer.next(), Some(Token::OParen));
    /// assert_eq!(lexer.slice(), "(");
    /// assert_eq!(lexer.next(), Some(Token::Number));
    /// assert_eq!(lexer.slice(), "1");
    /// assert_eq!(lexer.next(), Some(Token::CParen));
    /// assert_eq!(lexer.slice(), ")");
    /// ```
    #[token(")")]
    CParen,

    /// The exclaimation mark operator.
    /// 
    /// ```
    /// use flycatcher_lexer::Logos;
    /// use flycatcher_lexer::Token;
    /// 
    /// let mut lexer = Token::lexer("!1");
    /// assert_eq!(lexer.next(), Some(Token::Exclaimation));
    /// assert_eq!(lexer.slice(), "!");
    /// assert_eq!(lexer.next(), Some(Token::Number));
    /// assert_eq!(lexer.slice(), "1");
    /// ```
    #[token("!")]
    Exclaimation,

    /// The plus (`+`) operator.
    /// 
    /// ```
    /// use flycatcher_lexer::Logos;
    /// use flycatcher_lexer::Token;
    /// 
    /// let mut lexer = Token::lexer("21 + 21");
    /// assert_eq!(lexer.next(), Some(Token::Number));
    /// assert_eq!(lexer.slice(), "21");
    /// assert_eq!(lexer.next(), Some(Token::Plus));
    /// assert_eq!(lexer.slice(), "+");
    /// assert_eq!(lexer.next(), Some(Token::Number));
    /// assert_eq!(lexer.slice(), "21");
    /// ```
    #[token("+")]
    Plus,

    /// The dash/hyphen/minus (`-`) operator.
    /// 
    /// ```
    /// use flycatcher_lexer::Logos;
    /// use flycatcher_lexer::Token;
    /// 
    /// let mut lexer = Token::lexer("21 - 21");
    /// assert_eq!(lexer.next(), Some(Token::Number));
    /// assert_eq!(lexer.slice(), "21");
    /// assert_eq!(lexer.next(), Some(Token::Dash));
    /// assert_eq!(lexer.slice(), "-");
    /// assert_eq!(lexer.next(), Some(Token::Number));
    /// assert_eq!(lexer.slice(), "21");
    /// ```
    #[token("-")]
    Dash,

    /// The star/asterisk/multiply (`*`) operator.
    /// 
    /// ```
    /// use flycatcher_lexer::Logos;
    /// use flycatcher_lexer::Token;
    /// 
    /// let mut lexer = Token::lexer("21 * 21");
    /// assert_eq!(lexer.next(), Some(Token::Number));
    /// assert_eq!(lexer.slice(), "21");
    /// assert_eq!(lexer.next(), Some(Token::Star));
    /// assert_eq!(lexer.slice(), "*");
    /// assert_eq!(lexer.next(), Some(Token::Number));
    /// assert_eq!(lexer.slice(), "21");
    /// ```
    #[token("*")]
    Star,

    /// The forward slash/divide (`/`) operator.
    /// 
    /// ```
    /// use flycatcher_lexer::Logos;
    /// use flycatcher_lexer::Token;
    /// 
    /// let mut lexer = Token::lexer("42 / 2");
    /// assert_eq!(lexer.next(), Some(Token::Number));
    /// assert_eq!(lexer.slice(), "42");
    /// assert_eq!(lexer.next(), Some(Token::Slash));
    /// assert_eq!(lexer.slice(), "/");
    /// assert_eq!(lexer.next(), Some(Token::Number));
    /// assert_eq!(lexer.slice(), "2");
    /// ```
    #[token("/")]
    Slash,

    /// The percent/modulus (`%`) operator.
    /// 
    /// ```
    /// use flycatcher_lexer::Logos;
    /// use flycatcher_lexer::Token;
    /// 
    /// let mut lexer = Token::lexer("21 % 2");
    /// assert_eq!(lexer.next(), Some(Token::Number));
    /// assert_eq!(lexer.slice(), "21");
    /// assert_eq!(lexer.next(), Some(Token::Percent));
    /// assert_eq!(lexer.slice(), "%");
    /// assert_eq!(lexer.next(), Some(Token::Number));
    /// assert_eq!(lexer.slice(), "2");
    /// ```
    #[token("%")]
    Percent,

    /// The equals operator (`=`).
    /// 
    /// ```
    /// use flycatcher_lexer::Logos;
    /// use flycatcher_lexer::Token;
    /// 
    /// let mut lexer = Token::lexer("my_iden = 2");
    /// assert_eq!(lexer.next(), Some(Token::Identifier));
    /// assert_eq!(lexer.slice(), "my_iden");
    /// assert_eq!(lexer.next(), Some(Token::Equals));
    /// assert_eq!(lexer.slice(), "=");
    /// assert_eq!(lexer.next(), Some(Token::Number));
    /// assert_eq!(lexer.slice(), "2");
    /// ```
    #[token("=")]
    Equals,

    /// A comma token: `,`
    /// 
    /// ```
    /// use flycatcher_lexer::Logos;
    /// use flycatcher_lexer::Token;
    /// 
    /// let mut lexer = Token::lexer("[1, 2]");
    /// assert_eq!(lexer.next(), Some(Token::OBrack));
    /// assert_eq!(lexer.slice(), "[");
    /// assert_eq!(lexer.next(), Some(Token::Number));
    /// assert_eq!(lexer.slice(), "1");
    /// assert_eq!(lexer.next(), Some(Token::Comma));
    /// assert_eq!(lexer.slice(), ",");
    /// assert_eq!(lexer.next(), Some(Token::Number));
    /// assert_eq!(lexer.slice(), "2");
    /// assert_eq!(lexer.next(), Some(Token::CBrack));
    /// assert_eq!(lexer.slice(), "]");
    /// ```
    #[token(",")]
    Comma,

    /// A semicolon (`;`)
    /// 
    /// ```
    /// use flycatcher_lexer::Logos;
    /// use flycatcher_lexer::Token;
    /// 
    /// let mut lexer = Token::lexer("hello;");
    /// assert_eq!(lexer.next(), Some(Token::Identifier));
    /// assert_eq!(lexer.slice(), "hello");
    /// assert_eq!(lexer.next(), Some(Token::Semicolon));
    /// assert_eq!(lexer.slice(), ";");
    /// ```
    #[token(";")]
    Semicolon,

    /// A number literal that supports integers and floating point numbers, with an optional
    /// mantissa (exponent).
    /// 
    /// ```
    /// use flycatcher_lexer::Logos;
    /// use flycatcher_lexer::Token;
    /// 
    /// let mut lexer = Token::lexer("42 4.2 4.2e1");
    /// assert_eq!(lexer.next(), Some(Token::Number));
    /// assert_eq!(lexer.slice(), "42");
    /// assert_eq!(lexer.next(), Some(Token::Number));
    /// assert_eq!(lexer.slice(), "4.2");
    /// assert_eq!(lexer.next(), Some(Token::Number));
    /// assert_eq!(lexer.slice(), "4.2e1");
    /// ```
    #[regex("[0-9]*\\.?[0-9]+([eE][-+]?[0-9]+)?")]
    Number,
    
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

    /// A preprocessor identifier.  Used before compilations to preprocess the source.
    /// 
    /// ```
    /// use flycatcher_lexer::Logos;
    /// use flycatcher_lexer::Token;
    /// 
    /// let mut lexer = Token::lexer("#my_preprocessor");
    /// assert_eq!(lexer.next(), Some(Token::PreprocessorIdentifier));
    /// assert_eq!(lexer.slice(), "#my_preprocessor");
    /// ```
    #[regex(r"#[a-zA-Z_$][a-zA-Z_$0-9]*")]
    PreprocessorIdentifier,

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
    #[regex(r"[a-zA-Z_$][a-zA-Z_$0-9]*")]
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
    #[regex(r"\s+", logos::skip)]
    Whitespace,

    /// The `Invalid` token matches any character that doesn't match any other token types, it's
    /// basically a catchall.  This is recognized as an error at parsing time and will always
    /// throw an error.  The only time that it's impossible to find this token is inside of a
    /// string, any characters may be matched in a string, provided they are UTF characters.
    #[error]
    Invalid,

}