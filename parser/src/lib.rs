//! A hand-written parser that emits an AST tree.

pub mod ast;
pub mod error;

use ast::{Ast, AstMeta};
use ast::opcode::{get_operator, is_operator};
use codespan_reporting::diagnostic::{Diagnostic, Label};
use error::ErrorKind;
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

    /// Parses a list of Flycatcher values.  The token provided is the token used to close the
    /// list.
    fn parse_list(&mut self, close: Token) -> Result<Vec<AstMeta>, ErrorKind> {
        let mut state = 0; // 0 = value, 1 = ,
        let mut args = vec![]; // list of items in the list.

        let start = self.lexer.span().start;

        loop {
            let mut peekable = self.lexer.clone();

            if let Some(tok) = peekable.next() {
                if tok == close {
                    self.lexer.next();
                    break;
                }
            }

            if state == 0 {
                state = 1;
                match self.parse_expression() {
                    Ok(ast) => {
                        args.push(ast);
                    },
                    Err(e) => {
                        if e == ErrorKind::EndOfFile {
                            let label = Label::primary((), start..self.lexer.span().end)
                                .with_message(format!("this here array never closes."));
                            
                            let help = Label::secondary((), self.lexer.span())
                                .with_message("try inserting a ']' here.");
                                        
                            let diagnostic = Diagnostic::error()
                                .with_code("FC0011")
                                .with_labels(vec![label, help])
                                .with_message(format!("array never closes."));
                                        
                            self.diagnostics.push(diagnostic);
                    
                            return Err(ErrorKind::SyntaxError);
                        }

                        return Err(e);
                    }
                }
            } else if state == 1 {
                state = 0;

                if let Some(tok) = self.lexer.next() {
                    if tok != Token::Comma {
                        let label = Label::primary((), self.lexer.span())
                            .with_message(format!("expected a comma here."));
                                            
                        let diagnostic = Diagnostic::error()
                            .with_code("FC0012")
                            .with_labels(vec![label])
                            .with_message(format!("expected comma in array, got '{}'", self.lexer.slice()));
                                            
                        self.diagnostics.push(diagnostic);
                        
                        return Err(ErrorKind::SyntaxError);
                    }
                } else {
                    let label = Label::primary((), start..self.lexer.span().end)
                        .with_message(format!("this here array never closes."));

                    let help = Label::secondary((), self.lexer.span())
                        .with_message("try inserting a '[' here.");
                                        
                    let diagnostic = Diagnostic::error()
                        .with_code("FC0011")
                        .with_labels(vec![label, help])
                        .with_message(format!("array never closes."));
                                        
                    self.diagnostics.push(diagnostic);
                    
                    return Err(ErrorKind::SyntaxError);
                }
            }
        }

        Ok(args)
    }

    /// Parses a single literal token from the lexer.
    fn parse_literal(&mut self) -> Result<AstMeta, ErrorKind> {
        if let Some(tok) = self.lexer.next() {
            if tok == Token::Identifier {
                // At this phase, certain keywords don't exist, like `true`, `false` and `null`,
                // so we'll need to implement them here.

                let slice = self.lexer.slice();

                if slice == "true" {
                    return Ok(
                        AstMeta::new(
                            self.lexer.span(),
                            Ast::BooleanLiteral(true)
                        )
                    );
                } else if slice == "false" {
                    return Ok(
                        AstMeta::new(
                            self.lexer.span(),
                            Ast::BooleanLiteral(false)
                        )
                    );
                } else if slice == "null" {
                    return Ok(
                        AstMeta::new(
                            self.lexer.span(),
                            Ast::NullLiteral
                        )
                    );
                }

                Ok(AstMeta::new(
                    self.lexer.span(),
                    Ast::IdentifierLiteral(slice.into())
                ))
            } else if tok == Token::Number {
                let slice = self.lexer.slice().to_string();

                if slice.contains('.') || slice.contains('e') || slice.contains('E') {
                    let f = slice.parse::<f64>().unwrap();
                    Ok(AstMeta::new(
                        self.lexer.span(),
                        Ast::FloatLiteral(f)
                    ))
                } else {
                    let i = slice.parse::<i64>().unwrap();
                    Ok(AstMeta::new(
                        self.lexer.span(),
                        Ast::IntegerLiteral(i)
                    ))
                }
                /*
                let f = self.lexer.slice().parse::<f64>().unwrap();
                Ok(AstMeta::new(
                    self.lexer.span(),
                    Ast::NumberLiteral(f)
                ))*/
            } else if tok == Token::String {
                let str = &self.lexer.slice()[1..self.lexer.slice().len() - 1];
                Ok(AstMeta::new(
                    self.lexer.span(),
                    Ast::StringLiteral(str.into())
                ))
            } else if tok == Token::OBrack {
                // Array literal.
                let start = self.lexer.span().start;
                match self.parse_list(Token::CBrack) {
                    Ok(ast) => {
                        Ok(
                            AstMeta::new(
                                start..self.lexer.span().end,
                                Ast::ListLiteral(ast)
                            )
                        )
                    }
                    Err(e) => Err(e),
                }
            } else {
                // The token is unrecognized, so we have to give the correct error message.
                if tok == Token::Invalid {
                    let label = Label::primary((), self.lexer.span())
                        .with_message("this character is unrecognized by flycatcher.");
                    
                    let diagnostic = Diagnostic::error()
                        .with_code("FC0001")
                        .with_labels(vec![label])
                        .with_message(format!("invalid character '{}'", self.lexer.slice()));
                    
                    self.diagnostics.push(diagnostic);

                    Err(ErrorKind::SyntaxError)
                } else {
                    let label = Label::primary((), self.lexer.span())
                        .with_message("expected a proper value here");
                    
                    let diagnostic = Diagnostic::error()
                        .with_code("FC0002")
                        .with_labels(vec![label])
                        .with_message(format!("expected a value here, got '{}'", self.lexer.slice()));
                    
                    self.diagnostics.push(diagnostic);

                    Err(ErrorKind::SyntaxError)
                }
            }
        } else {
            // No token was found, so we return ErrorKind::EndOfFile
            Err(ErrorKind::EndOfFile)
        }
    }

    /// Parses an index statement, such as `item1.item2` or `item["item2"]`.
    fn parse_index(&mut self, first: AstMeta) -> Result<AstMeta, ErrorKind> {
        // This is the state of the parser, basically what the parser expects.  1 means that the
        // parser expects either a `.` or a `["key"]`.  0 means that the parser expects an
        // identifier.
        let mut state = 1;
        let start = first.range.start;

        let mut items = vec![first];

        loop {
            // Clone the lexer to not disturb its original state, in the case that the index
            // statement ends.
            let mut peekable = self.lexer.clone();

            if let Some(tok) = peekable.next() {
                if state == 0 {
                    state = 1;

                    if tok == Token::Identifier {
                        self.lexer.next();
                        items.push(
                            AstMeta::new(
                                self.lexer.span(),
                                Ast::IdentifierLiteral(self.lexer.slice().into())
                            )
                        );
                        // Move on to the next token.
                        //self.lexer.next();
                    } else {
                        // Trying to use a `.` to index with anything other than an identifier
                        // always results in an error.
                        self.lexer.next();
                        let label = Label::primary((), start..self.lexer.span().end)
                            .with_message(format!("expected a property name, got '{}'", self.lexer.slice()));
    
                        let help = Label::secondary((), self.lexer.span())
                            .with_message(format!("try wrapping it in []: '[{}]'", self.lexer.slice()));
                        
                        let diagnostic = Diagnostic::error()
                            .with_code("FC0004")
                            .with_labels(vec![label, help])
                            .with_message(format!("you indexed with a '.', expected a property name."));
                        
                        self.diagnostics.push(diagnostic);
    
                        return Err(ErrorKind::SyntaxError);
                    }
                } else if state == 1 {
                    if tok == Token::Dot {
                        // It's a `.`, so we can set the state to 0 and skip over it.
                        state = 0;
                        self.lexer.next();
                    } else if tok == Token::OBrack {
                        // Uses a recursive call to `parse_expression` inside of the opened
                        // bracket, to recieve the inners ;)
                        self.lexer.next();
                        let start = self.lexer.span().start;
                        match self.parse_expression() {
                            Ok(index) => {
                                // Check if there's a closing bracket

                                if let Some(tok) = self.lexer.next() {
                                    if tok == Token::CBrack {
                                        items.push(
                                            AstMeta::new(
                                                start..self.lexer.span().end,
                                                Ast::BracketIndex(
                                                    index.as_box()
                                                )
                                            )
                                        )
                                    } else {
                                        let label = Label::primary((), start..self.lexer.span().end)
                                            .with_message(format!("expected a closing bracket before this."));
                                        
                                        let diagnostic = Diagnostic::error()
                                            .with_code("FC0006")
                                            .with_labels(vec![label])
                                            .with_message(format!("expected a closing bracket instead of '{}'", self.lexer.slice()));
                                        
                                        self.diagnostics.push(diagnostic);
                    
                                        return Err(ErrorKind::SyntaxError);
                                    }
                                } else {
                                    let label = Label::primary((), start..self.lexer.span().end)
                                        .with_message(format!("unclosed brackets here."));
                                    
                                    let diagnostic = Diagnostic::error()
                                        .with_code("FC0005")
                                        .with_labels(vec![label])
                                        .with_message(format!("you indexed an object with unclosed brackets."));
                                    
                                    self.diagnostics.push(diagnostic);
                
                                    return Err(ErrorKind::SyntaxError);
                                }
                            },
                            Err(e) => {
                                // We need to check if an error message has been sent, if not,
                                // we'll need to send our own.
                                if e == ErrorKind::EndOfFile {
                                    // No error was emitted.
                                    let label = Label::primary((), self.lexer.span())
                                        .with_message(format!("unclosed brackets here."));
                                    
                                    let diagnostic = Diagnostic::error()
                                        .with_code("FC0005")
                                        .with_labels(vec![label])
                                        .with_message(format!("you indexed an object with unclosed brackets."));
                                    
                                    self.diagnostics.push(diagnostic);
                
                                    return Err(ErrorKind::SyntaxError);
                                }

                                return Err(e);
                            }
                        }
                    } else {
                        break;
                    }
                }
            } else {
                // There was no token, if the state was 1, everything should be fine.
                // Otherwise, the rule of no open index statements (such as `item1.item2.`)
                // is broken.
                if state == 0 {
                    self.lexer.next();
                    let label = Label::primary((), start..self.lexer.span().end)
                        .with_message("this index expression is unclosed.");

                    let help = Label::secondary((), self.lexer.span())
                        .with_message("there is an extra period here.");
                    
                    let diagnostic = Diagnostic::error()
                        .with_code("FC0003")
                        .with_labels(vec![label, help])
                        .with_message(format!("unclosed index expression"));
                    
                    self.diagnostics.push(diagnostic);

                    return Err(ErrorKind::SyntaxError);
                } else {
                    // No actual syntax errors occurred, so we break the loop.
                    break;
                }
            }
        }

        Ok(
            AstMeta::new(
                start..self.lexer.span().end, 
                Ast::IndexExpression(items)
            )
        )
    }

    /// Parses an expression operand.
    fn parse_secondary(&mut self) -> Result<AstMeta, ErrorKind> {
        let mut peekable = self.lexer.clone();
        
        if let Some(tok) = peekable.next() {
            if tok == Token::Dash {
                self.lexer.next();

                let start = self.lexer.span().start;
                let end = self.lexer.span().end;

                match self.parse_primary() {
                    Ok(ast) => {
                        return Ok(
                            AstMeta::new(
                                start..self.lexer.span().end,
                                Ast::NegativeUnary(
                                    ast.as_box()
                                )
                            )
                        );
                    },
                    Err(e) => {
                        if e == ErrorKind::EndOfFile {
                            self.lexer.next();
                            let label = Label::primary((), start..end)
                                .with_message("expression starts here");
        
                            let help = Label::secondary((), self.lexer.span())
                                .with_message("expected a value here");
                            
                            let diagnostic = Diagnostic::error()
                                .with_code("FC0007")
                                .with_labels(vec![label, help])
                                .with_message(format!("no value found in `-` expression."));
                            
                            self.diagnostics.push(diagnostic);
        
                            return Err(ErrorKind::SyntaxError);
                        }

                        return Err(e);
                    }
                }
            } if tok == Token::Plus {
                self.lexer.next();

                let start = self.lexer.span().start;
                let end = self.lexer.span().end;

                match self.parse_primary() {
                    Ok(ast) => {
                        return Ok(
                            AstMeta::new(
                                start..self.lexer.span().end,
                                Ast::PositiveUnary(
                                    ast.as_box()
                                )
                            )
                        );
                    },
                    Err(e) => {
                        if e == ErrorKind::EndOfFile {
                            self.lexer.next();
                            let label = Label::primary((), start..end)
                                .with_message("expression starts here");
        
                            let help = Label::secondary((), self.lexer.span())
                                .with_message("expected a value here");
                            
                            let diagnostic = Diagnostic::error()
                                .with_code("FC0007")
                                .with_labels(vec![label, help])
                                .with_message(format!("no value found in `+` expression."));
                            
                            self.diagnostics.push(diagnostic);
        
                            return Err(ErrorKind::SyntaxError);
                        }

                        return Err(e);
                    }
                }
            } else {
                return self.parse_literal()
            }
        }

        return Err(ErrorKind::EndOfFile);
    }

    /// Parses a binary expression with operator precedence, providing a minimum precedence for
    /// operators.
    fn parse_binary(&mut self, mut left: AstMeta, min: usize) -> Result<AstMeta, ErrorKind> {
        let mut tok = self.lexer.clone().next();

        while let Some(lookahead) = tok {
            if let Some(op) = get_operator(lookahead) {
                if op.precedence() >= min {
                    self.lexer.next();
                    let mut right = match self.parse_primary() {
                        Ok(ast) => ast,
                        Err(e) => {
                            if e == ErrorKind::EndOfFile {
                                // No error was emitted.
                                let label = Label::primary((), self.lexer.span())
                                    .with_message(format!("expected right hand side of expression here."));
                                
                                let diagnostic = Diagnostic::error()
                                    .with_code("FC0010")
                                    .with_labels(vec![label])
                                    .with_message(format!("expected right hand side of expression."));
                                
                                self.diagnostics.push(diagnostic);
        
                                return Err(ErrorKind::SyntaxError);
                            }
        
                            return Err(e);
                        }
                    };
        
                    tok = self.lexer.clone().next();
        
                    while let Some(lookahead2) = tok {
                        if let Some(op2) = get_operator(lookahead2) {
                            if op2.precedence() >= op.precedence() {
                                right = match self.parse_binary(right, min + 1) {
                                    Ok(ast) => ast,
                                    Err(e) => {
                                        if e == ErrorKind::EndOfFile {
                                            // No error was emitted.
                                            let label = Label::primary((), self.lexer.span())
                                                .with_message(format!("expected right hand side of expression here."));
                                            
                                            let diagnostic = Diagnostic::error()
                                                .with_code("FC0010")
                                                .with_labels(vec![label])
                                                .with_message(format!("expected right hand side of expression."));
                                            
                                            self.diagnostics.push(diagnostic);
                    
                                            return Err(ErrorKind::SyntaxError);
                                        }
                    
                                        return Err(e);
                                    }
                                };
        
                                tok = self.lexer.clone().next();
                            } else {
                                break;
                            }

                        } else {
                            break;
                        }
                    }


                    left = AstMeta::new(
                        left.range.start..self.lexer.span().end,
                        Ast::BinaryExpression(
                            op,
                            left.as_box(),
                            right.as_box()
                        )
                    );
                    tok = self.lexer.clone().next();
                } else {
                    self.lexer.next();
                    break;
                }
            } else {
                break;
            }
        }
        
        Ok(left)
        /*
        loop {
            let mut peekable = self.lexer.clone();

            if let Some(tok) = peekable.next() {
                if let Some(op) = get_operator(tok) {
                    if op.precedence() >= min {

                    } else {
                        break;
                    }
                } else {
                    // The next token isn't an operator, which means the expression is ending.
                    break;
                }
            }
        }*/
        /*
        let peekable = self.lexer.clone();

        if let Some(lookahead) = peekable.next() {

        }*/
    }

    /// Recursively parses function calls and object indexes.
    fn parse_opt_ending(&mut self, left: AstMeta) -> Result<AstMeta, ErrorKind> {
        if let Some(tok) = self.lexer.clone().next() {
            if tok == Token::OParen {
                self.lexer.next();
                match self.parse_list(Token::CParen) {
                    Ok(args) => {
                        return self.parse_opt_ending(
                            AstMeta::new(
                                left.range.start..self.lexer.span().end,
                                Ast::FunctionCall(
                                    left.as_box(),
                                    args
                                )
                            )
                        )
                    },
                    Err(e) => {
                        if e == ErrorKind::EndOfFile {
                            // Throw our own diagnostic messages.
                            // No error was emitted.
                            let label = Label::primary((), left.range.start..self.lexer.span().end)
                                .with_message(format!("this function call's argument list is never closed."));
                            
                            let help = Label::secondary((), self.lexer.span())
                                .with_message("try inserting a ')' here.");
                                        
                            let diagnostic = Diagnostic::error()
                                .with_code("FC0013")
                                .with_labels(vec![label, help])
                                .with_message(format!("argument list never closes."));
                                        
                            self.diagnostics.push(diagnostic);
                    
                            return Err(ErrorKind::SyntaxError);
                        }

                        return Err(e);
                    }
                }
            } else if tok == Token::Dot || tok == Token::OBrack {
                let l = left.clone();
                match self.parse_index(left) {
                    Ok(args) => {
                        return self.parse_opt_ending(args)
                    },
                    Err(e) => {
                        if e == ErrorKind::EndOfFile {
                            // Throw our own diagnostic messages.
                            // No error was emitted.
                            let start = l.range.start;
                            let label = Label::primary((), start..self.lexer.span().end)
                                .with_message(format!("this function call's argument list is never closed."));
                            
                            let help = Label::secondary((), self.lexer.span())
                                .with_message("try inserting a ')' here.");
                                        
                            let diagnostic = Diagnostic::error()
                                .with_code("FC0013")
                                .with_labels(vec![label, help])
                                .with_message(format!("argument list never closes."));
                                        
                            self.diagnostics.push(diagnostic);
                    
                            return Err(ErrorKind::SyntaxError);
                        }

                        return Err(e);
                    }
                }
            }
        }

        Ok(left)
    }

    /// Wraps the parse_literal function and allows function calls.
    fn parse_primary(&mut self) -> Result<AstMeta, ErrorKind> {
        match self.parse_secondary() {
            Ok(ast) => {
                self.parse_opt_ending(ast)
            },
            Err(e) => Err(e),
        }
    }

    /// Parses a single expression from the lexer, returning a single AST object that represents
    /// it, or an ErrorKind enum object describing how it ended.  If `Err(ErrorKind::EndOfFile)`
    /// was returned, that only means that there was no expression left, not that an actual
    /// error occurred.
    pub fn parse_expression(&mut self) -> Result<AstMeta, ErrorKind> {
        match self.parse_primary() {
            Ok(ast) => {
                self.parse_binary(ast, 0)
            },
            Err(e) => {
                return Err(e);
            }
        }
    }

}