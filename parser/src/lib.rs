//! Flycatcher's parser, which uses the lexer behind the scenes to convert an input string into a
//! Flycatcher AST tree.

use flycatcher_ast::{Ast, AstMeta, Opcode};
use flycatcher_diagnostic::{Context, Diagnostic, Label};
use flycatcher_lexer::{Lexer, Token};

/// A parser which translates a string into a list of AST items.
pub struct Parser<'a> {
    /// A list of diagnostics emitted by the parser.
    pub context: &'a mut Context<'a>,

    /// A list of document comments before an AST item.
    comments: Vec<String>,

    /// Whether or not the Parser has thrown an error yet.  This defaults to `true`.
    successful: bool,

    /// The lexer that this parser uses.
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    /// Initializes a parser that will parse the source string from the provided context..  A parser
    /// emits a Flycatcher AST tree, which can be used to compile to a binary or perform analyses of the
    /// source string.
    pub fn new(context: &'a mut Context<'a>) -> Self {
        // NOTE: we need to use `context.source` as a seperate variable because of Rust's borrow
        // checking system.  If we did otherwise, Rust would think that the context is being borrowed
        // mutably and immutably at the same time, causing a compilation error.
        //  ↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓
        let str = context.source;

        Self {
            context,
            comments: vec![],
            successful: true,
            lexer: Lexer::new(str),
        }
    }

    /// Consumes a single token from the lexer.  If the next token doesn't match, it will emit one or
    /// more diagnostic messages to the `diagnostics` vector.
    ///
    /// This method ignores whitespaces, line breaks and comments.
    ///
    /// If `doc` is true, it pushes any document comments to the comments table.  Otherwise, it will
    /// throw an error if any diagnostic messages are found.
    fn eat(&mut self, expect: Token, doc: bool) -> bool {
        let mut next_token = self.lexer.next();

        // Since this function needs to ignore unnecessary tokens, such as white spaces and comments, we
        // must loop until we find a non-skipped token.
        //
        // This loop also verifies whether or not the token is valid (a.k.a. whether or not the
        // next token matches the `expect`ed token).
        //    ↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓
        while let Some(tok) = next_token {
            if tok == expect {
                // As the statement `tok == expect` seems, the token matches what the parser wanted.
                // This means we can return `true`, meaning the process was successful.
                return true;
            } else if tok == Token::DocComment {
                //           ↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑
                // The current token is a documentation comment.  This means that we need to push the
                // comment into the comments vector.  This is only if the `doc` parameter is `true`.

                if !doc {
                    // An error must be thrown as the document comment isn't allowed here.  We won't
                    // break the loop because of this though.
                    //
                    // NOTE: there's more information about labels and diagnostics below.

                    let label = Label::primary((), self.lexer.span())
                        .with_message("document comments aren't allowed here.");

                    let diagnostic = Diagnostic::error()
                        .with_code("E0004")
                        .with_labels(vec![label])
                        .with_message("invalid place for a document comment.");

                    self.successful = false;
                    self.context.diagnostics.push(diagnostic);
                }

                // Of course, we need to remove the leading slashes and the first leading space, if any.
                let mut slice = self.lexer.slice().trim_start_matches('/');

                if slice.starts_with(' ') {
                    slice = &slice[1..];
                    //      ↑↑↑↑↑↑↑↑↑↑↑ This truncates the string, removing the first character.  We
                    //                  have confirmed that this first character is a space character,
                    //                  so we may ignore it.
                }

                // Now that we've removed the extra characters, we can push the comment to the comments
                // vector.
                self.comments.push(slice.into());
                //                 ↑↑↑↑↑↑↑↑↑↑↑↑ `slice` is a `&str`, which has the `into()` method.
                //                              in this context, the compiler interprets that the
                //                              `into()` method used should return a `String`.

                next_token = self.lexer.next();
                continue; // Skip to the next token, as document comments should still be ignored.
            } else if tok == Token::Whitespace || tok == Token::Linebreak {
                // Line breaks (2) and whitespaces (1) will be completely ignored by the parser in this
                // function, so we'll iterate to the next token in the lexer and continue the loop.
                next_token = self.lexer.next();
                continue;
            }

            // Alright, the token isn't a document comment, whitespace or the token that we expected,
            // which means a *syntax error occurred!*
            //
            // The parent parser likely won't throw an error from this, so we have to throw one here.
            // We should check if the token is invalid here to see if we should throw an error other
            // than "unexpected token."
            if tok == Token::InvalidString {
                //    ↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑
                // `InvalidString`s are strings that opened, but don't have a matching closing quote on
                // the same line.

                let span = self.lexer.span();

                // We use multiple blocks because this token in this context emits multiple diagnostics.
                {
                    // Labels are used to show where an error occurred, as well as more information
                    // about an error.  Primary errors talk about the source of the problem, rather than
                    // how to fix it.
                    //           ↓↓↓↓↓↓↓↓↓↓↓↓↓↓
                    let label1 = Label::primary((), span.clone())
                        //                          ↑↑↑↑↑↑↑↑↑↑↑↑
                        // This is the range of characters where the error occurred.  This is used in
                        // diagnostic messages for displaying samples of the offending code.
                        //
                        // We need to use `.clone()` to prevent Rust's very aggressive borrow/move
                        // checker from throwing an error.
                        // As for the message, this seems pretty self explanatory.  It's the message
                        // displayed on the label.  If we don't provide a message, `codespan-reporting`
                        // will simply display an error squiggle where the error occurred, without any
                        // helpful message.
                        .with_message("this string must be closed on the same line that it starts.");

                    let label2 = Label::secondary((), span.start..span.start + 1)
                        //                            ↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑ This is the range of
                        //                                                       the starting quote.
                        .with_message("no matching closing quote for this quote.");

                    // This creates an "error" diagnostic.  This means that something went wrong that
                    // wasn't correctable.  These diagnostics will be printed to the `stderr` stream,
                    // which should tell any parent processes that the compilation process wasn't
                    // successful.
                    //               ↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓
                    let diagnostic = Diagnostic::error()
                        .with_code("E0001")
                        .with_labels(vec![label1, label2])
                        .with_message("unclosed string.");

                    self.context.diagnostics.push(diagnostic);
                }

                if expect != Token::String {
                    //       ↑↑↑↑↑↑↑↑↑↑↑↑↑ This diagnostic message only throws if the parser wasn't
                    //                     expecting a string.
                    let label = Label::primary((), span).with_message("unexpected string.");

                    let diagnostic = Diagnostic::error()
                        .with_code("E0002")
                        .with_labels(vec![label])
                        .with_message(if let Some(s) = expect.as_string() {
                            //                         ↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑
                            // There is a string constant for the token, meaning it is likely a keyword
                            // or operator was expected.  Either way, we can use that in the label here.
                            format!("expected '{}', found string.", s)
                        } else {
                            // Otherwise, we can check whether or not the token is provided a human
                            // friendly name.  If not, we just send off the most generic label possible.
                            if let Some(s) = expect.as_name() {
                                //           ↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑
                                // This gets the name of the object, such as "a boolean" or "a string".
                                // Usually, this is used in place of the `as_string()` method if no
                                // string was returned.
                                format!("expected {}, found string.", s)
                            } else {
                                // The very generic default.  I don't believe it should ever get here,
                                // but if it does, you'll see "uNeXpEcTeD sTrInG" show up on your
                                // terminal.
                                "unexpected string".into()
                            }
                        });

                    self.context.diagnostics.push(diagnostic);
                }
            } else if tok == Token::Invalid {
                //           ↑↑↑↑↑↑↑↑↑↑↑↑↑↑
                // "What defines an invalid token?" you may ask.  An invalid token is a character that
                // didn't match any other tokens, such as any UTF-8 character (outside of a string, of
                // course).

                let label1 = Label::primary((), self.lexer.span()).with_message(format!(
                    "unexpected '{}' (invalid character)",
                    self.lexer.slice()
                ));

                let mut labels = vec![label1];

                // If the expected token has a valid name, then we will add an additional label stating
                // what the parser expected.
                if let Some(s) = expect.as_string() {
                    let label2 = Label::secondary((), self.lexer.span())
                        .with_message(format!("expected '{}'", s));
                    labels.push(label2);
                } else {
                    if let Some(s) = expect.as_name() {
                        let label2 = Label::secondary((), self.lexer.span())
                            .with_message(format!("expected {}", s));
                        labels.push(label2);
                    }
                }

                let diagnostic = Diagnostic::error()
                    .with_code("E0005")
                    .with_labels(labels)
                    .with_message(format!("invalid character: '{}'", self.lexer.slice()));

                self.context.diagnostics.push(diagnostic);
            } else {
                // The token matched was technically valid, just not in this context as it doesn't match
                // the expected token type.

                let label = Label::primary((), self.lexer.span())
                    .with_message(format!("unexpected '{}'", self.lexer.slice()));

                let diagnostic = Diagnostic::error()
                    .with_code("E0006")
                    .with_labels(vec![label])
                    .with_message(if let Some(s) = expect.as_string() {
                        format!("expected '{}', found '{}'.", s, self.lexer.slice())
                    } else {
                        if let Some(s) = expect.as_name() {
                            format!("expected {}, found '{}'.", s, self.lexer.slice())
                        } else {
                            format!("unexpected '{}'", self.lexer.slice())
                        }
                    });

                self.context.diagnostics.push(diagnostic);
            }

            // Tell the rest of the parser that the eating process was unsuccessful.
            //   ↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓
            self.successful = false;

            // Return `false` by default, as we already know an error occurred.  We don't want the loop
            // to go through another iteration as we have not set the `next_token` variable to the next
            // token, meaning a Rusty move checker error occurs.
            //     ↓↓↓↓↓
            return false;
        }

        // Alright, the lexer ran out of tokens and we didn't find the expected token.  We must throw
        // an error in this case.
        let label = Label::primary((), self.lexer.span()).with_message("unexpected end of file.");

        let diagnostic = Diagnostic::error()
            .with_code("E0003")
            .with_labels(vec![label])
            .with_message(if let Some(s) = expect.as_string() {
                format!("expected '{}', instead we found the end of the file.", s)
            } else {
                if let Some(s) = expect.as_name() {
                    format!("expected {}, instead, we found the end of the file.", s)
                } else {
                    "unexpected end of file.".into()
                }
            });

        self.context.diagnostics.push(diagnostic);
        self.successful = false;

        // We default to false as an error must have occurred, since the loop didn't provide any
        // results.
        false
    }

    #[allow(dead_code)]
    /// Consumes a token of the given type, if possible.  Otherwise, this returns `false`.
    ///
    /// If `doc` is `true`, this function will push any document
    fn eat_optional(&mut self, expect: Token, doc: bool) -> bool {
        let mut next_token = self.lexer.peek();
        //                   ↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑
        // Unlike the `eat` method, this method peeks at the next token rather than reading it directly
        // from the lexer.
        //
        // It iterates past any whitespaces, line breaks and document comments (if document comments
        // are enabled for this eat call).
        //
        // This method only throws diagnostics for invalid tokens that were found.

        while let Some(tok) = next_token {
            if tok == expect {
                // `tok` matches the expected type, so we may return a `true` boolean saying so.  BUT!
                // First we must iterate to the next token, since we only peeked for this token.
                self.lexer.next();

                //     ↓↓↓↓ Again, `true` means that we successfully found the token.
                return true;
            } else if tok == Token::DocComment {
                //    ↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑
                // As seen above, the current token is a documentation comment.  If doc comments are
                // allowed, we can push the value of the comment to the `comments` table.

                // Above we did: let mut next_token = self.lexer.peek();
                //                                    ^^^^^^^^^^^^^^^^^
                // This doesn't actually advance to the next token, this only returns what token is
                // next, without iterating.  This means we have yet to *iterate to the doc comment!*
                self.lexer.next();

                //      If this is false, document comments aren't allowed before the expected token.
                // ↓↓↓↓ This is used for function definitions, classes, etc.
                if !doc {
                    // Document comments aren't allowed here, so we must throw another diagnostic.  This
                    // should be the same error as above, `E0004`.

                    let label = Label::primary((), self.lexer.span())
                        .with_message("document comments aren't allowed here.");

                    let diagnostic = Diagnostic::error()
                        .with_code("E0004")
                        .with_labels(vec![label])
                        .with_message("invalid place for a document comment.");

                    self.successful = false;
                    self.context.diagnostics.push(diagnostic);
                }

                // Remove the first 3 (and any more) leading slashes of the comment.
                let mut slice = self.lexer.slice().trim_start_matches('/');

                if slice.starts_with(' ') {
                    // There is an extra space at the start (presumably) that is ignored by the parser.
                    slice = &slice[1..];
                }

                self.comments.push(slice.into());
                next_token = self.lexer.next();

                continue;
            } else if tok == Token::Whitespace || tok == Token::Linebreak {
                self.lexer.next(); // skip over the whitespace/line break
                next_token = self.lexer.next();

                continue;
            }

            // We can safely assume the token was either invalid or didn't match the `expect`ed token.
            // This method ignores any tokens that don't match the expected token, but we still need to
            // verify that the token found was value.

            if tok == Token::InvalidString {
                //    ↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑ The token found is an invalid string.  Even though this
                //                         method ignores any tokens that aren't valid, it still must
                //                         report errors.

                //   ↓↓↓↓↓↓↓↓↓↓↓↓ Iterate to the invalid string token.
                self.lexer.next();

                // You should hopefully understand what's going on here by now, so I won't commentate
                // over most of the diagnostic emitting part.
                let span = self.lexer.span();
                {
                    let label1 = Label::primary((), span.clone())
                        .with_message("this string must be closed on the same line that it starts.");

                    let label2 = Label::secondary((), span.start..span.start + 1)
                        .with_message("no matching closing quote for this quote.");

                    let diagnostic = Diagnostic::error()
                        .with_code("E0001")
                        .with_labels(vec![label1, label2])
                        .with_message("unclosed string.");

                    self.context.diagnostics.push(diagnostic);
                }

                //                         Before we return, we check if the parser even expected a
                // ↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓ string.
                if expect != Token::String {
                    let label = Label::primary((), span).with_message("unexpected string.");

                    let diagnostic = Diagnostic::error()
                        .with_code("E0002")
                        .with_labels(vec![label])
                        .with_message(if let Some(s) = expect.as_string() {
                            format!("expected '{}', found string.", s)
                        } else {
                            if let Some(s) = expect.as_name() {
                                format!("expected {}, found string.", s)
                            } else {
                                "unexpected string".into()
                            }
                        });

                    self.context.diagnostics.push(diagnostic);
                }
            } else if tok == Token::Invalid {
                self.lexer.next();

                let label1 = Label::primary((), self.lexer.span()).with_message(format!(
                    "unexpected '{}' (invalid character)",
                    self.lexer.slice()
                ));

                let mut labels = vec![label1];

                // If the expected token has a valid name, then we will add an additional label stating
                // what the parser expected.
                if let Some(s) = expect.as_string() {
                    let label2 = Label::secondary((), self.lexer.span())
                        .with_message(format!("expected '{}'", s));
                    labels.push(label2);
                } else {
                    if let Some(s) = expect.as_name() {
                        let label2 = Label::secondary((), self.lexer.span())
                            .with_message(format!("expected {}", s));
                        labels.push(label2);
                    }
                }

                let diagnostic = Diagnostic::error()
                    .with_code("E0005")
                    .with_labels(labels)
                    .with_message(format!("invalid character: '{}'", self.lexer.slice()));

                self.context.diagnostics.push(diagnostic);
                self.successful = false;
            }

            return false;
        }

        // We default to false and return since a matching token is optional.
        false
    }

    /// Iterates over the skipped tokens such as whitespaces and line breaks, peeking at the token that
    /// is found.
    fn peek_token(&mut self) -> Option<Token> {
        loop {
            if let Some(t) = self.lexer.peek() {
                // ↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑ We have found a token.  If it's a whitespace, line break
                //                             or comment, we should skip over it and continue the loop.
                if t == Token::Whitespace || t == Token::Linebreak || t == Token::DocComment {
                    self.lexer.next();
                    continue;
                } else {
                    return Some(t);
                }
            } else {
                return None;
            }
        }
    }

    /// Parses a list until the specified closing token is found.
    fn parse_list(&mut self, close: Token) -> Option<Vec<AstMeta>> {
        // This is the "state" of the parser.  Basically, what the parser is currently expecting.  The
        // parser may expect a value (0) or a delimiter ',' (1).
        //  ↓↓↓↓↓↓↓↓↓
        let mut state = 0;

        // This list of items to return.
        let mut items = vec![];

        loop {
            if let Some(tok) = self.peek_token() {
                if tok == close {
                    // End the loop, we've found the closing token.
                    self.lexer.next();
                    break;
                }

                if state == 0 {
                    self.eat(Token::Comma, false);
                    state = 1;
                } else if state == 0 {
                    if let Some(val) = self.parse_expression() {
                        items.push(val);
                        state = 0;
                    } else {
                        if self.successful {
                            let label = Label::primary((), self.lexer.span()).with_message(format!(
                                "expected a closing '{}', found end of file",
                                close.as_name().unwrap()
                            ));
            
                            let diagnostic = Diagnostic::error()
                                .with_code("E0014")
                                .with_labels(vec![label])
                                .with_message(format!(
                                    "expected a closing '{}'",
                                    close.as_name().unwrap()
                                ));
            
                            self.context.diagnostics.push(diagnostic);
                        }

                        return None;
                    }
                }
            } else {
                // The list must not have ended yet (since we are still in the loop), so we have to
                // throw an error here.
                let label = Label::primary((), self.lexer.span()).with_message(format!(
                    "expected a closing '{}', found end of file",
                    close.as_name().unwrap()
                ));

                let diagnostic = Diagnostic::error()
                    .with_code("E0014")
                    .with_labels(vec![label])
                    .with_message(format!(
                        "expected a closing '{}'",
                        close.as_name().unwrap()
                    ));

                self.context.diagnostics.push(diagnostic);
                self.successful = false;
            }
        }

        Some(items)
    }

    /// Parses a "primary" token.  Primary tokens are identifiers, numbers, booleans, etc. and may be
    /// used in any expression.
    fn parse_primary(&mut self) -> Option<AstMeta> {
        if let Some(tok) = self.peek_token() {
            if tok == Token::Identifier {
                self.lexer.next();
                return Some(AstMeta::new(
                    self.lexer.span(),
                    Ast::IdentifierLiteral(self.lexer.slice().into()),
                ));
            } else if tok == Token::TrueKeyword {
                self.lexer.next();
                return Some(AstMeta::new(self.lexer.span(), Ast::BooleanLiteral(true)));
            } else if tok == Token::FalseKeyword {
                self.lexer.next();
                return Some(AstMeta::new(self.lexer.span(), Ast::BooleanLiteral(false)));
            } else if tok == Token::String {
                self.lexer.next();
                let slice = self.lexer.slice();
                //          ↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑
                // We use this variable for slightly better efficiency, rather than calling
                // `self.lexer.slice()` multiple times below.

                return Some(AstMeta::new(
                    self.lexer.span(),
                    Ast::StringLiteral(slice[1..slice.len() - 1].into()),
                    /*                 ↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑ This removes the starting and
                     * ending quotes
                     * from the string. */
                ));
            } else if tok == Token::Number {
                //           ↑↑↑↑↑↑↑↑↑↑↑↑↑
                // As you can tell (I'm sure you can), the token found was a number.  This means that we
                // need to convert the number to the correct AST item.

                self.lexer.next();

                let slice = self.lexer.slice();

                if slice.contains('e') || slice.contains('E') || slice.contains('.') {
                    // The token found must have been a floating point number.
                    return Some(AstMeta::new(
                        self.lexer.span(),
                        // We need to convert the slice into a float, this is possible with Rust's
                        // `parse` method.
                        //                ↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓
                        Ast::FloatLiteral(slice.parse::<f64>().unwrap()),
                    ));
                } else {
                    // The token is an integer literal, so we need to confirm that the number is small
                    // enough to fit into a `u64`.

                    if let Ok(item) = slice.parse::<u64>() {
                        // ↑↑↑↑↑↑↑↑ This if statement checks if the number was small enough or not. If
                        // we reach this block, then the number must have been valid.

                        return Some(AstMeta::new(self.lexer.span(), Ast::IntegerLiteral(item)));
                    } else {
                        // The u64 parsing process was unsuccessful, so we should throw a diagnostic
                        // saying so.

                        let label = Label::primary((), self.lexer.span())
                            .with_message("this number is too large to handle.");
                        //                                ↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑ ;)

                        let diagnostic = Diagnostic::error()
                            .with_code("E0007")
                            .with_labels(vec![label])
                            .with_message("invalid number (too large).");

                        self.context.diagnostics.push(diagnostic);
                        self.successful = false;

                        // We can fall through because be default, this function returns `None`.
                    }
                }
            } else if tok == Token::IfKeyword {
                // The expression is an `if` operation.
                //
                // First of all, we need to find the expression that the if statement uses.

                let start = self.lexer.span().start;
                self.lexer.next();

                let ifblock;
                let expr;

                if let Some(e) = self.parse_expression() {
                    if self.eat(Token::LCurly, false) {
                        if let Some(block) = self.parse_block() {
                            ifblock = block;
                            expr = e;
                        } else {
                            // `parse_block()` should have thrown an error had anything gone wrong.
                            return None;
                        }
                    } else {
                        return None;
                    }
                } else {
                    if self.successful {
                        // No expression was found in the `if` statement, so we must throw an error, no
                        // error was thrown, we should throw an error here.
                        let label = Label::primary((), self.lexer.span())
                            .with_message("expected expression in 'if' statement (here).");

                        let diagnostic = Diagnostic::error()
                            .with_code("E0010")
                            .with_labels(vec![label])
                            .with_message("expected expression in 'if' statement.");

                        self.context.diagnostics.push(diagnostic);
                        self.successful = false;
                    }

                    return None;
                }

                let mut branches = vec![];

                // Now, we need to parse the other branches of the if statement.  We do this by looping
                // until there is no more `else if` or `else` statements at the end of a block.
                loop {
                    if let Some(tok) = self.peek_token() {
                        if tok == Token::ElseKeyword {
                            // It could be either an `else` statement or an `else if` statement.  We
                            // have yet to check whether there is an `if`
                            // after the `else` keyword.

                            let span = self.lexer.span();
                            self.lexer.next();

                            if let Some(tok) = self.peek_token() {
                                if tok == Token::IfKeyword {
                                    // It's an `else if` statement.

                                    self.lexer.next();

                                    println!("TESTasdfasdfa");
                                    if let Some(e) = self.parse_expression() {
                                        if self.eat(Token::LCurly, false) {
                                            if let Some(block) = self.parse_block() {
                                                // Push the `else if` statement to the branches vector.
                                                branches.push(AstMeta::new(
                                                    span.start..self.lexer.span().end,
                                                    Ast::IfStmnt {
                                                        block: block,
                                                        branches: vec![],
                                                        expr: e.into_box(),
                                                    },
                                                ))
                                            } else {
                                                // `parse_block()` should have thrown an error had
                                                // anything
                                                // gone wrong.
                                                return None;
                                            }
                                        } else {
                                            return None;
                                        }
                                    } else {
                                        if self.successful {
                                            // No expression was found in the `if` statement, so we must
                                            // throw an error, no
                                            // error was thrown, we should throw an error here.
                                            let label = Label::primary((), self.lexer.span())
                                                .with_message(
                                                    "expected expression in 'if' statement (here).",
                                                );

                                            let diagnostic = Diagnostic::error()
                                                .with_code("E0010")
                                                .with_labels(vec![label])
                                                .with_message("expected expression in 'if' statement.");

                                            self.context.diagnostics.push(diagnostic);
                                            self.successful = false;
                                        }

                                        return None;
                                    }
                                } else if tok == Token::LCurly {
                                    // It's an `else` statement
                                    self.lexer.next();
                                    if let Some(block) = self.parse_block() {
                                        branches.push(AstMeta::new(
                                            span.start..self.lexer.span().end,
                                            Ast::Block(block),
                                        ));
                                    } else {
                                        return None;
                                    }
                                }
                            } else {
                                // There is no `if` keyword, and there isn't a block either.  This is
                                // indeed a syntax error.
                                let label = Label::primary((), span).with_message(
                                    "expected one of '{' or 'if', found the end of the file.",
                                );

                                let diagnostic = Diagnostic::error()
                                    .with_code("E0012")
                                    .with_labels(vec![label])
                                    .with_message("expected '{' or 'if'");

                                self.context.diagnostics.push(diagnostic);
                                self.successful = false;

                                return None;
                            }
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }

                return Some(AstMeta::new(
                    start..self.lexer.span().end,
                    Ast::IfStmnt {
                        expr: expr.into_box(),
                        block: ifblock,
                        branches,
                    },
                ));
            } else if tok == Token::WhileKeyword {
                let start = self.lexer.span().start;
                self.lexer.next();

                let whileblock;
                let expr;

                if let Some(e) = self.parse_expression() {
                    if self.eat(Token::LCurly, false) {
                        if let Some(block) = self.parse_block() {
                            whileblock = block;
                            expr = e;
                        } else {
                            // `parse_block()` should have thrown an error had anything gone wrong.
                            return None;
                        }
                    } else {
                        return None;
                    }
                } else {
                    if self.successful {
                        // No expression was found in the `if` statement, so we must throw an error, no
                        // error was thrown, we should throw an error here.
                        let label = Label::primary((), self.lexer.span())
                            .with_message("expected expression in 'if' statement (here).");

                        let diagnostic = Diagnostic::error()
                            .with_code("E0010")
                            .with_labels(vec![label])
                            .with_message("expected expression in 'if' statement.");

                        self.context.diagnostics.push(diagnostic);
                        self.successful = false;
                    }

                    return None;
                }

                return Some(AstMeta::new(
                    start..self.lexer.span().end,
                    Ast::WhileStmnt {
                        expr: expr.into_box(),
                        block: whileblock,
                    },
                ));
            } else if tok == Token::LBrack {
                self.lexer.next();
                let start = self.lexer.span().start;

                if let Some(t) = self.parse_list(Token::RBrack) {
                    return Some(AstMeta::new(
                        start..self.lexer.span().end,
                        Ast::ArrayLiteral(t),
                    ));
                }
                
                return None;
            } else {
                self.lexer.next();
                let label = Label::primary((), self.lexer.span())
                    .with_message(format!("expected a value here, got {}", self.lexer.slice()));

                let diagnostic = Diagnostic::error()
                    .with_code("E0013")
                    .with_labels(vec![label])
                    .with_message("expected a value.");

                self.context.diagnostics.push(diagnostic);
                self.successful = false;
            }
        }

        // Default to a None value.
        None
    }

    /// Parses a binary expression, if possible.  This function uses a basic (yet efficient) Pratt
    /// parser.
    pub fn parse_binary(&mut self, min: usize) -> Option<AstMeta> {
        // This function doesn't use the `eat` methods for higher efficiency.

        // This is the left side of the operation, which is determined below.
        let mut left;

        // First, we see if the next token is a prefix, if so, we use a recursive call to `parse_binary`
        // and use that as the left side of the operation.  Otherwise, we use the return value of
        // `parse_primary` as the left side of the operation.
        //               ↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓
        if let Some(l) = self.peek_token() {
            // ↑↑↑↑↑↑↑↑↑↑↑↑↑ Here, we know that there is a token left in the lexer.  We need to see if
            //               it is a prefix operator.

            if let Some(o) = Opcode::from_token(l.clone()) {
                //           ↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑ There *is* an operator that matches this token, but is
                //                              it a prefix operator?  Let's find out with the
                //                              `prefix_precedence` method of `o`.

                if let Some(prec) = o.prefix_precedence() {
                    // The token was a prefix!  We need to get the operand of the prefix and use it as
                    // the left side of the operation.

                    self.lexer.next(); // iterate to the prefix
                    let start = self.lexer.span().start; // this is the starting character of the prefix

                    // Here, we recieve the operand of the prefix with the recursive call to
                    // `parse_binary`.  We use the `prec` variable as the minimum binding power for the
                    // operand.  For example, this allows `-my_iden.my_other_iden + 2` since the
                    // binding power of `.` is greater than that of the `-` prefix.  The tree generated
                    // from that looks like this:
                    //
                    //     (+
                    //         (-  // the `-` prefix
                    //             (.
                    //                 my_iden,
                    //                 my_other_iden
                    //             )
                    //         ),
                    //         2
                    //     )
                    //
                    //                 ↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓
                    if let Some(rhs) = self.parse_binary(prec) {
                        left = AstMeta::new(
                            start..self.lexer.span().end,
                            Ast::UnaryExpr(o, rhs.into_box()),
                        )
                    } else {
                        // There was no operand for the prefix operator, which means an error probably
                        // occurred.

                        if self.successful {
                            // No error was thrown, we should throw an error here.
                            let label = Label::primary((), self.lexer.span()).with_message(format!(
                                "expected expression [here] after '{}' operator",
                                l.as_string().unwrap()
                            ));

                            let diagnostic = Diagnostic::error()
                                .with_code("E0008")
                                .with_labels(vec![label])
                                .with_message("expected expression after operator");

                            self.context.diagnostics.push(diagnostic);
                            self.successful = false;
                        }

                        // Return None, because whether the parser was successful or not, an error
                        // occurred.
                        return None;
                    }
                } else {
                    // The token isn't a prefix operator, so we should just use the value of a call to
                    // `parse_primary`.
                    if let Some(t) = self.parse_primary() {
                        left = t;
                    } else {
                        // The file must have ended, or an error occurred.
                        return None;
                    }
                }
            } else {
                // The token isn't a prefix operator, so we should just use the value of a call to
                // `parse_primary`.
                if let Some(t) = self.parse_primary() {
                    left = t;
                } else {
                    // The file must have ended, or an error occurred.
                    return None;
                }
            }
        } else {
            // No tokens are left in the lexer, so we should just return None instead of starting a
            // loop.
            return None;
        }

        loop {
            if let Some(next_op) = self.peek_token() {
                if let Some(op) = Opcode::from_token(next_op.clone()) {
                    // There is an operator in the token stream, now let's see what kind of operator it
                    // is.

                    if let Some(prec) = op.postfix_precedence() {
                        // If the precedence of this postfix operator is less than the minimum
                        // precedence, we should break the loop.
                        // ↓↓↓↓↓↓↓↓↓↓
                        if prec < min {
                            break;
                        }

                        // Iterate over the operator, so we can get a possible value of the operator,
                        // if the operator is a subscript or call operator.
                        self.lexer.next();

                        if op == Opcode::Subscript {
                            // We need to get the value of the subscript (if any), like so:
                            if let Some(t) = self.peek_token() {
                                if t == Token::RBrack {
                                    self.lexer.next();

                                    // There is no value in the subscript.
                                    left = AstMeta::new(
                                        left.range.start..self.lexer.span().end,
                                        Ast::SubscriptExpr(left.into_box(), None),
                                    );
                                } else {
                                    // The subscript has a right operand, so we can use a recursive call
                                    // to recieve the operand.
                                    if let Some(right) = self.parse_expression() {
                                        // eat the closing bracket
                                        self.eat(Token::RBrack, false);

                                        left = AstMeta::new(
                                            left.range.start..self.lexer.span().end,
                                            Ast::SubscriptExpr(left.into_box(), Some(right.into_box())),
                                        );
                                    } else {
                                        if self.successful {
                                            let label = Label::primary((), self.lexer.span())
                                                .with_message("expected a ']' here");

                                            let diagnostic = Diagnostic::error()
                                                .with_code("E0009")
                                                .with_labels(vec![label])
                                                .with_message("expected expression after operator");

                                            self.context.diagnostics.push(diagnostic);
                                            self.successful = false;
                                            return None;
                                        }
                                        return None;
                                    }
                                }
                            } else {
                                // It is guaranteed that there is no closing `]` here, so we must throw
                                // an error stating this.
                                // No error was thrown, we should throw an error here.
                                let label = Label::primary((), self.lexer.span())
                                    .with_message("expected a ']' here");

                                let diagnostic = Diagnostic::error()
                                    .with_code("E0009")
                                    .with_labels(vec![label])
                                    .with_message("expected expression after operator");

                                self.context.diagnostics.push(diagnostic);
                                self.successful = false;
                                return None;
                            }
                        }
                    } else if let Some((lp, rp)) = op.infix_precedence() {
                        // The token is an infix operator, which means an operator that takes both left
                        // and right operands.  These are known as "binary expressions," since they
                        // take two operands.
                        // An example of this includes `21 + 21`, which equates to the tree:
                        //
                        //     (+
                        //         21,
                        //         21
                        //     )
                        //

                        if lp < min {
                            break;
                        }

                        self.lexer.next();

                        if let Some(rhs) = self.parse_binary(rp) {
                            left = AstMeta::new(
                                left.range.start..self.lexer.span().end,
                                Ast::BinaryExpr(op, left.into_box(), rhs.into_box()),
                            );
                        } else {
                            // We expected a right hand side operand after the operator, but there was
                            // nothing.
                            if self.successful {
                                // No error was thrown, we should throw an error here.
                                let label =
                                    Label::primary((), self.lexer.span()).with_message(format!(
                                        "expected expression [here] after '{}' operator",
                                        next_op.as_string().unwrap()
                                    ));

                                let diagnostic = Diagnostic::error()
                                    .with_code("E0008")
                                    .with_labels(vec![label])
                                    .with_message("expected expression after operator");

                                self.context.diagnostics.push(diagnostic);
                                self.successful = false;
                            }
                        }
                    } else {
                        break;
                    }
                } else {
                    // We assume the expression ended because there was not an operator.
                    break;
                }
            } else {
                // No tokens left, break the loop.
                break;
            }
        }

        // Now that we've finished the main loop, we can return the left hand statement, which may or
        // may not have been modified by an expression.
        Some(left)
    }

    /// Parses a single expression at the current index of the lexer.
    pub fn parse_expression(&mut self) -> Option<AstMeta> {
        // self.parse_binary(0)
        if let Some(tok) = self.peek_token() {}

        self.parse_binary(0)
    }

    /// Parses expressions until a `}` is found.
    fn parse_block(&mut self) -> Option<Vec<AstMeta>> {
        let mut ast = vec![];

        loop {
            if let Some(tok) = self.peek_token() {
                if tok == Token::RCurly {
                    self.lexer.next();
                    break;
                }
                if tok == Token::Semicolon {
                    self.lexer.next();
                    continue;
                }

                if let Some(mut expr) = self.parse_expression() {
                    if let Some(tok2) = self.peek_token() {
                        if tok2 == Token::Semicolon {
                            self.lexer.next();
                            expr.semicolon();
                        }
                    }

                    ast.push(expr);
                } else {
                    if self.successful {
                        // End of file found before the closing curly bracket.
                        self.lexer.next();

                        let label = Label::primary((), self.lexer.span())
                            .with_message("expected a closing '}' here.");

                        let diagnostic = Diagnostic::error()
                            .with_code("E0011")
                            .with_labels(vec![label])
                            .with_message("unclosed block statement.");

                        self.context.diagnostics.push(diagnostic);
                        self.successful = false;
                    }

                    return None;
                }
            } else {
                // The file ended; meaning no closing bracket was found.
                self.lexer.next();

                let label =
                    Label::primary((), self.lexer.span()).with_message("expected a closing '}' here.");

                let diagnostic = Diagnostic::error()
                    .with_code("E0011")
                    .with_labels(vec![label])
                    .with_message("unclosed block statement.");

                self.context.diagnostics.push(diagnostic);
                self.successful = false;
                return None;
            }
        }

        Some(ast)
    }

    /// Parses all items in the source file.
    pub fn parse(&mut self) -> Option<Vec<AstMeta>> {
        let mut ast = vec![];

        loop {
            if let Some(tok) = self.peek_token() {
                if tok == Token::RCurly {
                    self.lexer.next();
                    break;
                }

                if let Some(mut expr) = self.parse_expression() {
                    if let Some(tok2) = self.peek_token() {
                        if tok2 == Token::Semicolon {
                            self.lexer.next();
                            expr.semicolon();
                        }
                    }

                    ast.push(expr);
                } else {
                    if self.successful {
                        // End of file found before the closing curly bracket.
                        self.lexer.next();

                        let label = Label::primary((), self.lexer.span())
                            .with_message("expected a closing '}' here.");

                        let diagnostic = Diagnostic::error()
                            .with_code("E0011")
                            .with_labels(vec![label])
                            .with_message("unclosed block statement.");

                        self.context.diagnostics.push(diagnostic);
                        self.successful = false;
                    }

                    return None;
                }
            } else {
                // The file ended; meaning no closing bracket was found.
                self.lexer.next();

                let label =
                    Label::primary((), self.lexer.span()).with_message("expected a closing '}' here.");

                let diagnostic = Diagnostic::error()
                    .with_code("E0011")
                    .with_labels(vec![label])
                    .with_message("unclosed block statement.");

                self.context.diagnostics.push(diagnostic);
                self.successful = false;
                return None;
            }
        }

        Some(ast)
    }
}
