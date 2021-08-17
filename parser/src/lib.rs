//! A hand-written parser that emits an AST tree.

pub mod ast;

use codespan_reporting::diagnostic::Diagnostic;
use flycatcher_lexer::{Lexer, Logos, Token};

/// A Parser struct that takes an input string, tokenizes it and parses it into a more or less
/// readable AST tree.
pub struct Parser<'a> {

    /// The name of the input file that is being parsed.  This property helps make more precise
    /// diagnostic messages, by providing the name of the file that the diagnostic originated
    /// from.
    pub filename: &'a str,
    
    /// The string of Flycatcher input that is tokenized and parsed by the parser.  The source
    /// is also used to emit code snippets in diagnostic messages.
    pub source: &'a str,

    /// A list of diagnostics that were created during parsing.  These are not logged to the
    /// console by the parser, so they can be used to recieve information for IDEs and such.
    pub diagnostics: Vec<Diagnostic<()>>,

    /// The lexer that the parser recieves input tokens from.
    lexer: Lexer<'a, Token>,

}

impl<'a> Parser<'a> {

    /// Allocates a new parser object.  This does not start the parsing process, it only
    /// initializes a lexer and parser and returns the parser.
    /// 
    /// # Arguments
    /// - `filename`: The absolute file path to the file being parsed, if any.  If you don't
    /// have an actual file to put here, put `@anonymous`.
    /// - `source`: The string that will be tokenized and parsed by the parser that is allocated
    /// by this function.
    pub fn new(filename: &'a str, source: &'a str) -> Self {
        Self {
            filename,
            source,
            diagnostics: vec![],
            lexer: Token::lexer(source)
        }
    }

}