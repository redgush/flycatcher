//! A hand-written parser that emits an AST tree.

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