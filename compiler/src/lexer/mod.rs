//! Provides utilities for tokenizing, or lexing, Flycatcher source.

mod chars;
mod token;

pub use chars::{is_iden_continue, is_iden_start, is_line_term, is_punctuator, is_white_space};
use std::ops::Range;
pub use token::{InvalidStrType, Token};

/// A lexer for Flycatcher source.  This lexer leverages the Unicode character set standard, and allows
/// any valid Unicode text as source.
pub struct Lexer {
    /// A list of characters that are tokenized.  The characters of the source string are obtained
    /// by collecting the items that the `String::chars` method outputs.
    chars: Vec<char>,

    /// The originating String that this lexer reads tokens from.  This string is used to recieve the
    /// slices of tokens.
    source: String,

    /// The range, in the source string, which the current token resides in.  This is used by the lexer
    /// to keep track of where the next token should be in the source string.  It is also used by the
    /// [`Self::slice`] method, to efficiently calculate the slice string when needed.
    loc: Range<usize>,
}

impl Lexer {
    /// Initializes a new Flycatcher Lexer.  Collects the 32-bit Unicode characters (UTF-32) from the
    /// source string provided, into a [`Vec<char>`].  Additionally also stores the source string, to
    /// allow the lazy calculation of token slices.
    pub fn new(source: String) -> Self {
        Self {
            chars: source.chars().collect(),
            source,
            loc: 0..0,
        }
    }

    /// Returns the range, in characters, of the current token.  Lexers are initialized with a
    /// [`Range<usize>`] of `0..0`.  This means that if the lexer has not had atleast one iteration,
    /// this function will return the default location.
    pub fn loc(&self) -> Range<usize> {
        self.loc.clone()
    }

    /// Returns the slice of the current token.  This function uses the [`Self::loc`] method to
    /// calculate the location of the current token, and uses that information to get the current span.
    /// If the lexer's token stream (iterator) has ran out of tokens, this function will return an
    /// empty string.
    pub fn slice(&self) -> &str {
        let span = self.loc();

        if span.end > self.chars.len() {
            return "";
        }

        &self.source[self.loc()]
    }
}

impl Iterator for Lexer {
    type Item = Token;

    /// Calculates the next [`Token`] in the lexer.  This function only returns the *type* of the next
    /// token.  The value, or *slice*, of the next token can be obtained with the [`Lexer::slice`]
    /// method.  To get the location of the next token, you can use the [`Lexer::loc`] method.
    fn next(&mut self) -> Option<Self::Item> {
        // This is the starting index of the next token in the lexer.  Because of the way Rust Ranges
        // work, the `end` property is essentially the length of the next token, plus the starting
        // index of it.  This means that the end index is one greater than the actual ending character
        // in the string, which allows us to directly use the `end` property of the last token as the
        // start index of the next token.
        let start_index = self.loc.end;

        // Before we do any lexing magic, we need to make sure that the `start_index` is within the
        // range of the source string.  Otherwise, there will be an unwanted overflow panic.
        if start_index >= self.chars.len() {
            // `None` in an iterator is returned when there is nothing left to iterate.
            return None;
        }

        // Since the above `if` statement used a `return` statement instead of a semicolonless
        // statement, we can omit the `else` statement.
        //
        // The first thing we are going to check, is if the current character is a white space
        // character.  Functionality to do this check was provided by the `chars` module.
        //
        // We should allocate a variable for the starting character of the token, as this character
        // may be compared several times.  Preferably, we just want to avoid the constant getting of
        // the exact same character.
        let start_char = self.chars[start_index];

        if is_white_space(start_char) {
            // As mentioned above, the current token is a white space token.
            //
            // The process for setting the location of the current token is equivalent to
            // [Token Start Index]..([Token Start Index] + [Token Length]).
            //
            // Here, the length is confirmed to be one UTF-32 character, so there is no need to
            // calculate the length of the token.
            self.loc = start_index..start_index + 1;
            
            // White space tokens have no arguments, so we can simply return a `Some` value.
            return Some(Token::WhiteSpace);
        } else if is_line_term(start_char) {
            // If the program lands here, the current token is a line terminator.  The process for line
            // termination is exactly the same as the process for white space characters, as seen above.
            self.loc = start_index..start_index + 1;
            return Some(Token::LineTerm);
        } else if is_punctuator(start_char) {
            // We do punctuators next.  Punctuators are essentially symbols that have some sort of
            // semantic use.
            //
            // The thing is, comments start with the `/` character, which is a punctuator.  Comments
            // start with two slash characters, so we can test if a slash is a comment by checking the
            // next character.
            
            if start_char == '/' {
                // We need to check if there is a next character in the `chars` vector, to prevent an
                // array overflow panic.
                if start_index >= self.chars.len() {
                    // There is not a character left in the string, so the token must be a punctuator.

                    self.loc = start_index..start_index + 1;
                    return Some(Token::Punctuator);
                }

                let next_char = self.chars[start_index + 1];

                if next_char == '/' {
                    // The character after the starting character was indeed a slash, so this token is
                    // a comment.

                    let mut pos = start_index + 2; // the position of the current character of the
                                                   // token.

                    // Next, we need to see if the type is a documentation comment or a line comment.
                    // We can do so by checking if the next character is a slash as well.
                    let next_char = self.chars[pos];
                    let mut ty = Token::LineComment; // This is the type of the token.  We will set
                                                     // this to Token::DocComment if the below if
                                                     // statement is triggered.
                    
                    if next_char == '/' {
                        ty = Token::DocComment;
                        pos += 1; // skip over the third slash.
                    }

                    // This just loops until the end of the file or a line terminating white space is
                    // found.
                    while pos < self.chars.len()
                        && !is_line_term(self.chars[pos]) {
                        pos += 1;
                    }

                    self.loc = start_index..pos;
                    return Some(ty);
                }
            }

            // If we end up here, the token was not a comment and we can just return a punctuator
            // token.
            self.loc = start_index..start_index + 1;
            return Some(Token::Punctuator);
        } else if start_char == '"' || start_char == '\'' {
            // Alright, if the program lands here, the current token is a string.  We'll use the
            // `start_char` to find the end of the string.
            let mut pos = start_index + 1;

            while pos < self.chars.len() {
                // In this loop, we need to check if the current character is the correct character to
                // end the string.  We also need to skip over escaped characters.

                let str_char = self.chars[pos];

                if str_char == start_char {
                    // The string has ended.

                    pos += 1;

                    self.loc = start_index..pos;
                    return Some(Token::Str {
                        prefix: None
                    });
                } else if is_line_term(str_char) {
                    // If we land here, the string did not end before a new line character was found.
                    // This makes the string invalid.

                    self.loc = start_index..pos;
                    return Some(Token::InvalidStr {
                        ty: InvalidStrType::UnclosedLine,
                        error_loc: pos - 1..pos,
                    });
                } else if str_char == '\\' {
                    // The current character in the string is escaped, but we'll need to see if it is a
                    // Unicode escape, or a normal escaped character.

                    pos += 1; // move to the escaped character.

                    // Before we do anything, we need to confirm that the string is still valid, and
                    // the next character (the character code) exists.
                    if pos >= self.chars.len() {
                        // There wasn't a closing quote before the file ended.
                        self.loc = start_index..pos;
                        return Some(Token::InvalidStr {
                            ty: InvalidStrType::UnclosedEOF,
                            error_loc: pos - 1..pos,
                        });
                    } else if is_line_term(self.chars[pos]) {
                        // The string doesn't end on the line that it starts.
                        self.loc = start_index..pos;
                        return Some(Token::InvalidStr {
                            ty: InvalidStrType::UnclosedLine,
                            error_loc: pos - 1..pos,
                        });
                    }

                    // At this phase in the language, we don't have to actually calculate any of the
                    // character codes, we can simply skip over the next character.  The loop will
                    // verify that the string is valid.
                    pos += 1;
                } else {
                    // The current character is just a normal string character.
                    pos += 1;
                }
            }

            self.loc = start_index..pos;

            // If we get here, the string never ended.
            return Some(Token::InvalidStr {
                ty: InvalidStrType::UnclosedEOF,
                error_loc: pos - 1..pos,
            });
        } else if is_iden_start(start_char) {
            // Alright, the next thing we need to tokenize is identifiers.  Identifiers must start with
            // a Unicode XID character, or an underscore.  An identifier ends when the next character
            // is no longer an XID continuing character.

            let mut pos = start_index + 1;

            while pos < self.chars.len() {
                // This checks if the identifier ends at this character or not.

                let iden_char = self.chars[pos];

                if is_iden_continue(iden_char) {
                    // The current character is an XID continuing character, so we may continue the
                    // loop.
                    pos += 1;
                } else if iden_char == '"' || iden_char == '\'' {
                    // It looks like the identifier was a string prefix.  String prefixes are simply
                    // identifiers directly before a string, with no spaces.
                    //
                    // This means that we will need to tokenize a string, similar to the process above.
                }
            }
        }

        // If the program lands here, we can safely assume that no valid token was found.  This means
        // that we can return an `Invalid` token.
        //
        // We don't know how long the token was intended to be, but the invalid character may only be
        // up to one character in length.  We will use this as the length to calculate the location of
        // the invalid token.
        self.loc = start_index..start_index + 1;
        return Some(Token::Invalid);
    }
}

#[test]
fn test() {
    // This test prints out all tokens in the lexer, which is initialized below.
    let mut lexer = Lexer::new("/// Hello, world!\n".to_string());

    loop {
        let item = lexer.next();

        if item == None {
            // If there is no token left in the lexer, then we must end the loop.
            break;
        }

        // Print the token type, starting and ending location, and the slice of the current token.
        // Currently, this may have some formatting issues with struct enum items.
        let loc = lexer.loc();
        println!("{:#?}@{}:{} '{}'", item, loc.start, loc.end, lexer.slice());
    }
}