use logos::Logos;

/// A list of tokens that Flycatcher's lexer may use.  This is powered by Logos and its derive macro,
/// making this the lexer itself, not just a mere list of tokens.
#[derive(Clone, Debug, Logos, PartialEq)]
pub enum Token {
    /// A semicolon token. `;`
    #[token(";")]
    Semicolon,

    /// A left curly bracket.  `{`.
    #[token("{")]
    LCurly,

    /// A left curly bracket.  `}`.
    #[token("}")]
    RCurly,

    /// A left square bracket.  `[`.
    #[token("[")]
    LBrack,

    /// A left square bracket.  `]`.
    #[token("]")]
    RBrack,

    /// A left parenthesis.  `[`.
    #[token("(")]
    LParen,

    /// A left parenthesis.  `]`.
    #[token(")")]
    RParen,

    /// The `>>` bitwise right bit shift operator.
    #[token(">>")]
    GreaterGreater,

    /// The `<<` bitwise left bit shift operator.
    #[token("<<")]
    LessLess,

    /// The `==` comparison operator.
    #[token("==")]
    EqualsEquals,

    /// The `!=` comparison operator.
    #[token("!=")]
    ExclamationEquals,

    /// The `>=` comparison operator.
    #[token(">=")]
    GreaterEquals,

    /// The `<=` comparison operator.
    #[token("<=")]
    LessEquals,

    /// The `&&` comparison operator.
    #[token("&&")]
    AndAnd,

    /// The `||` comparison operator.
    #[token("||")]
    OrOr,

    /// The `>` operator.
    #[token(">")]
    Greater,

    /// The `<` operator.
    #[token("<")]
    Less,

    /// A `.` character.
    #[token(".")]
    Period,

    /// A `,` character.
    #[token(",")]
    Comma,

    /// The `:` operator.
    #[token(":")]
    Colon,

    /// An `!` operator.
    #[token("!")]
    Exclamation,

    /// The bitwise AND operator (`&`).
    #[token("&")]
    And,

    /// The bitwise OR operator (`|`).
    #[token("|")]
    Or,

    /// The `^` operator, which is the bitwise XOR operator in most cases.
    #[token("^")]
    Caret,

    /// The `~` bitwise NOT operator.
    #[token("~")]
    Not,

    /// An `=` operator.
    #[token("=")]
    Equals,

    /// A `+` operator, which adds two number primitives together.  If the first operand isn't a number
    /// primitive but it does have a `plus` operator overload, it must accept the right side of the
    /// expression as an argument.
    #[token("+")]
    Plus,

    /// The `-` operator.
    #[token("-")]
    Minus,

    /// The `*` operator.
    #[token("*")]
    Asterisk,

    /// The `/` operator.
    #[token("/")]
    Slash,

    /// The `%` operator.
    #[token("%")]
    Percent,

    /// The `true` keyword is a boolean value equal to 1.  Booleans are used in logical operations
    /// often used to compare two sides of an expression.  For example, `1 == 1` would be equal to
    /// `true`, and `1 == 2` would be 'false'.
    #[token("true")]
    TrueKeyword,

    /// `false` is equal to 0 in logical operations, it is the opposite of `true`.
    #[token("false")]
    FalseKeyword,

    /// The `as` keyword, which is used as an operator to convert one type to another.
    #[token("as")]
    AsKeyword,

    /// `declare`s an external function that can be linked at compile time.
    #[token("declare")]
    DeclareKeyword,

    /// The `pub` keyword makes a field of a module or construct accessible to the public.  This is
    /// ignored for imports using the `#include` preprocessor.
    #[token("pub")]
    PubKeyword,

    /// `priv` makes a field of a module or construct private.
    #[token("priv")]
    PrivKeyword,

    /// The `if` keyword, which is used in logical operations.
    ///
    /// ```flycatcher
    /// if expression {
    ///     // ...
    /// } else {
    ///     // ...
    /// }
    /// ```
    #[token("if")]
    IfKeyword,

    /// The `else` keyword.
    #[token("else")]
    ElseKeyword,

    /// The `while` keyword, which is somewhat similar to the `if` keyword.  It repeats a block of code
    /// until the provided expression is equal to false.
    #[token("while")]
    WhileKeyword,

    /// A number literal in Flycatcher may be a floating point number, or it may be an integer.  This
    /// token also matches an optional exponent/mantissa, like so:
    ///
    /// ```flycatcher
    /// 42
    /// 4.2
    /// 4.2e1
    /// 4.2e+1
    /// .42
    /// ```
    #[regex("[0-9]*\\.?[0-9]+([eE][-+]?[0-9]+)?")]
    Number,

    /// Flycatcher's string literals are much inspired by ECMAScript's string literals.  In Flycatcher,
    /// there are no "character literals," unlike C, C++, Rust, Java, etc.  A string may start with
    /// either `"` or `'`.
    ///
    /// The string, at the lexing phase, *supports* escaped quotes, but they aren't converted to their
    /// correct characters yet.  This is done during parsing.  For example, `"\"Hello, world!\""` is
    /// the same as `"\\\"Hello, world!\\\""` in a Rust string.
    #[regex("\"(?:[^\"\\\\]|\\\\.)*\"|'(?:[^'\\\\]|\\\\.)*'")]
    String,

    /// An invalid string is similar to a `String`, but it matches any string that doesn't have a
    /// closing quote on the same line.  This is used for better diagnostic messages rather to serve an
    /// actual purpose.
    #[regex("\"(?:[^\"\\\\]|\\\\.)*|'(?:[^'\\\\]|\\\\.)*")]
    InvalidString,

    /// An identifier is used as names and can be formatted like normal human words, such as `hello`.
    /// Identifiers may start with any alphabetical character, and underscore or a dollar sign, and any
    /// characters continuing an identifier may contain a numerical digit.
    ///
    /// ```flycatcher
    /// my_identifier // Valid identifier
    /// _my_identifier // Valid identifier
    /// $my_identifier // Valid identifier
    /// $ // Valid Identifier
    /// my_0_identifier // Valid Identifier
    /// 0_identifier // INVALID Identifier.  Identifiers may not start with a digit.
    /// ```
    #[regex("[a-zA-Z_$][a-zA-Z_$0-9]*")]
    Identifier,

    /// A construct identifier, which extends the functionality of the `Identifier` token.  A construct
    /// identifier must start with an at sign (`@`).
    #[regex("@[a-zA-Z_$][a-zA-Z_$0-9]*")]
    ConstructIdentifier,

    /// Preprocessor identifiers are, once again, similar to identifiers.  They must start with a
    /// hashtag (`#`) character.
    #[regex("#[a-zA-Z_$][a-zA-Z_$0-9]*")]
    PreprocessorIdentifier,

    /// Flycatcher document comments are similar to Rust's, they use three slashes (`///`) at the start
    /// and they end at the end of the line.
    #[regex("///.*")]
    DocComment,

    /// A Flycatcher comment, which is ignored by the lexer.
    #[regex("//.*", logos::skip)]
    Comment,

    /// A line break character that matches `\n` and `\r`.
    #[regex("[\n\r]+", logos::skip)]
    Linebreak,

    /// Matches any whitespace that isn't a line break, such as tabs and normal white spaces.
    #[regex("\\s+", logos::skip, priority = 2)]
    Whitespace,

    /// `Invalid` is a token provided by Logos, which is just a token that doesn't match any other
    /// token's signature.  This doesn't actually pass as an "error" while lexing, it's up to the
    /// parser to throw an error if this token was found.
    #[error]
    Invalid,
}

impl Token {
    /// Displays a constant token as a string.  For example, if this token is a `TrueKeyword`, this
    /// function will return `true`.  This function will return `None` if this token is not a constant
    /// token, such as a string or number.
    pub fn as_string(&self) -> Option<String> {
        match self {
            Token::LCurly => Some("{".into()),
            Token::RCurly => Some("}".into()),
            Token::LBrack => Some("[".into()),
            Token::RBrack => Some("]".into()),
            Token::LParen => Some("(".into()),
            Token::RParen => Some(")".into()),
            Token::EqualsEquals => Some("==".into()),
            Token::ExclamationEquals => Some("!=".into()),
            Token::GreaterEquals => Some(">=".into()),
            Token::LessEquals => Some("<=".into()),
            Token::Greater => Some(">".into()),
            Token::Less => Some("<".into()),
            Token::Period => Some(".".into()),
            Token::Comma => Some(",".into()),
            Token::Colon => Some(":".into()),
            Token::Exclamation => Some("!".into()),
            Token::Equals => Some("=".into()),
            Token::Plus => Some("+".into()),
            Token::Minus => Some("-".into()),
            Token::Asterisk => Some("*".into()),
            Token::Slash => Some("/".into()),
            Token::Percent => Some("%".into()),
            Token::Caret => Some("^".into()),
            Token::TrueKeyword => Some("true".into()),
            Token::FalseKeyword => Some("false".into()),
            _ => None,
        }
    }

    /// Gets the name of the token, as a string.  For example, `true` and `false` are "a boolean".
    pub fn as_name(&self) -> Option<String> {
        match self {
            Token::TrueKeyword | Token::FalseKeyword => Some("a boolean".into()),
            Token::Identifier => Some("a name".into()),
            Token::ConstructIdentifier => Some("a construct name".into()),
            Token::PreprocessorIdentifier => Some("a preprocessor name".into()),
            Token::String => Some("a string".into()),
            Token::Number => Some("a number".into()),
            Token::DeclareKeyword => Some("'declare'".into()),
            _ => {
                // If there was a success finding the string related to the token, we wrap it in
                // quotes.
                // ↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓
                if let Some(n) = self.as_string() {
                    Some("'".to_string() + &n + "'")
                } else {
                    None
                }
            }
        }
    }
}
