use std::ops::Range;

/// The reason why a string is invalid.
#[derive(Clone, Debug, PartialEq)]
pub enum InvalidStrType {
    /// The end of the file was found before a closing quote.
    UnclosedEOF,

    /// The end of the line was found before a closing quote.
    UnclosedLine,

    /// There was no opening brace in a Unicode character code escape: `\x{0000}`.
    NoOpeningBraceUnicodeEscape,
}

/// Types of tokens that may be emitted by the Flycatcher lexer.  At this phase, tokens consist of
/// white spaces, line breaks, comments, punctuators and literals.  Keywords do not exist in this
/// phase, they are just identifiers.
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    /// An invalid token, which matches when no other token matches.  It is essentially a low-priority
    /// catchall that catches invalid characters.
    Invalid,

    /// A Unicode white space character.  Flycatcher does not include *all* Unicode white space
    /// characters, as some Unicode white spaces are line terminating, which are seperate tokens.
    WhiteSpace,

    /// A Unicode line terminating character.
    LineTerm,

    /// A single line comment.  Single line comments have no functional value, and any text inside of a
    /// line comment is discarded by the parser.
    LineComment,

    /// A single line documentation comment.  Documentation comments do not have any semantic meaning,
    /// but they may be used for the automatic generation of documentation for an item.
    DocComment,

    /// A punctuator, such as a mathematic operator.
    Punctuator,

    /// A string literal.  Strings may start and end with `"` or `'`.
    Str {
        /// The prefix before the string literal, such as:
        /// 
        /// ```flycatcher
        /// r'Hello, world!'
        /// ```
        prefix: Option<String>,
    },

    /// An string literal which either never ends, or does not end on the same line.
    InvalidStr {
        /// The reason why the string is invalid.
        ty: InvalidStrType,

        /// The location where the error occurred.
        error_loc: Range<usize>,
    },
}

impl Token {
    /// Returns `true` if this [`Token`] is a white space character.
    #[inline]
    pub fn is_white_space(&self) -> bool {
        match self {
            Self::WhiteSpace => true,
            _ => false,
        }
    }

    /// Returns `true` if this [`Token`] is a line terminating white space character.
    #[inline]
    pub fn is_line_term(&self) -> bool {
        match self {
            Self::LineTerm => true,
            _ => false,
        }
    }

    /// Returns `true` if this [`Token`] is either a line comment or a documentation comment.
    #[inline]
    pub fn is_comment(&self) -> bool {
        match self {
            Self::LineComment => true,
            Self::DocComment => true,
            _ => false,
        }
    }

    /// Returns `true` if this [`Token`] is a punctuator.
    pub fn is_punctuator(&self) -> bool {
        match self {
            Self::Punctuator => true,
            _ => false
        }
    }
}