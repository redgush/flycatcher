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
            }

            self.successful = false;

            return false;
        }

        false
    }
}
