//! The front end for Flycatcher's compiler system.
//! 
//! This crate converts Flycatcher AST into Flycatcher HIR, which is much more optimized.  This
//! process involves type checking and other safety checks.
//! 
//! Once this process is finished, the resulting HIR may be passed to a Flycatcher compiler
//! backend, where it can be compiled into either a LIR or a binary, or both.

pub mod hir;
pub mod types;
pub mod var;

use codespan_reporting::diagnostic::{Diagnostic, Label};
use flycatcher_diagnostic::{codespan_reporting, DiagnosticEmitter};
use flycatcher_parser::ast::{Ast, AstMeta, Opcode};
pub use hir::{Hir, HirMeta};
pub use std::collections::HashMap;
pub use types::FlycatcherType;
pub use var::VariableType;

pub type SymbolTable = HashMap<String, VariableType>;

/// Flycatcher's front end for it's compiler.  This struct takes an input AST tree and converts
/// it to a slightly lower level representation, the HIR.  The HIR removes the abstractions of
/// preprocessors and such, and it's easier to compile directly to another lower level
/// intermediate representation, or just directly to machine code.
pub struct FlycatcherFrontend<'a> {

    /// The name of the file that the compiler frontend is converting into an HIR tree.  This is
    /// used for diagnostic messages.
    pub filename: &'a str,

    /// The source that relates to the provided filename.  This is also used for diagnostic
    /// messages emitted by the compiler.
    pub source: &'a str,

    /// A list of diagnostics that the frontend emitted while converting an AST tree to an HIR
    /// tree.
    pub diagnostics: Vec<Diagnostic<()>>,

    /// A list of generated HIR objects.  This should not be used if `self::successful` is equal
    /// to `false`.
    pub hir: Vec<HirMeta<'a>>,

    /// A list of variables defined in the provided AST tree.  These are used to resolve what
    /// variable names are valid and what variable names aren't.
    pub symbols: HashMap<String, VariableType>,

    /// Whether or not the compilation process was successful.  This defaults to true and is set
    /// to false if any errors occur.
    successful: bool,

}

impl<'a> FlycatcherFrontend<'a> {

    /// Creates a new Flycatcher compiler front end.  After initialization, to use this struct,
    /// you'll need to pass an AST tree to convert to Flycatcher MIR.
    pub fn new(filename: &'a str, source: &'a str) -> Self {
        Self {
            filename,
            source,
            diagnostics: vec![],
            hir: vec![],
            symbols: HashMap::new(),
            successful: true,
        }
    }

    /// Converts an AST literal to Flycatcher HIR.  This also verifies any symbol references,
    /// and if they exist, they shall be incremented here.
    fn ast_literal(&mut self, ast: AstMeta) -> Option<HirMeta<'a>> {
        match ast.item {
            Ast::BooleanLiteral(b) => Some(HirMeta::new(
                ast.range, 
                self.filename,
                Hir::Boolean(b)
            )),
            Ast::IntegerLiteral(i) => Some(HirMeta::new(
                ast.range,
                self.filename,
                // Default to a signed integer.
                Hir::Integer(i)
            )),
            Ast::FloatLiteral(f) => Some(HirMeta::new(
                ast.range,
                self.filename,
                Hir::Float(f)
            )),
            Ast::IdentifierLiteral(n) => {
                // Verify if the identifier literal is valid.
                if self.symbols.contains_key(&n) {
                    // Check if the symbol has been declared or defined, if the variable has
                    // been declared, but not defined, we need to throw an error because the
                    // variable is not yet usable.
                    match self.symbols.get_mut(&n).unwrap() {
                        VariableType::Declared(_) => {
                            // The variable was declared but not defined.  This is an issue!
                            self.successful = false;
                    
                            // Throw an error since the symbol requested isn't defined in this scope.
                            let label = Label::primary((), ast.range)
                                .with_message("this variable is declared, but not yet given a value.");
        
                            let diagnostic = Diagnostic::error()
                                .with_code("FC0018")
                                .with_labels(vec![label])
                                .with_message("use of undefined variable.");
                            
                            self.diagnostics.push(diagnostic);
                        },
                        VariableType::Defined(_, c, _) => {
                            // Increment the reference counter.
                            *c += 1;

                            return Some(HirMeta::new(
                                ast.range,
                                self.filename,
                                Hir::Named(n)
                            ));
                        }
                    }
                } else {
                    self.successful = false;
                    
                    // Throw an error since the symbol requested isn't defined in this scope.
                    let label = Label::primary((), ast.range)
                        .with_message("this variable is undeclared in this scope.");

                    let diagnostic = Diagnostic::error()
                        .with_code("FC0017")
                        .with_labels(vec![label])
                        .with_message("use of undeclared variable.");
                    
                    self.diagnostics.push(diagnostic);
                }

                None
            }
            // If no match was found, it wasn't an error, this function can be used to check if
            // an AST item is a literal.
            _ => None,
        }
    }

    /// Converts an AST expression to Flycatcher HIR.
    fn ast_expression(&mut self, ast: AstMeta) -> Option<HirMeta<'a>> {
        if let Some(hir) = self.ast_literal(ast.clone()) {
            Some(hir)
        } else {
            if !self.successful {
                return None;
            }

            match ast.item {
                Ast::BinaryExpression(op, left, right) => {
                    if op == Opcode::Add ||
                        op == Opcode::Subtract ||
                        op == Opcode::Multiply ||
                        op == Opcode::Divide {
                        // We should verify that both sides of the expression are indeed valid.

                        // Translate both sides of the expression into HIR objects.
                        let l = match self.ast_expression(*left) {
                            Some(item) => item,
                            None => {
                                if self.successful {
                                    self.successful = false;
                    
                                    // Throw an error since the symbol requested isn't defined in this scope.
                                    let label = Label::primary((), ast.range)
                                        .with_message("invalid expression here.");
    
                                    let diagnostic = Diagnostic::error()
                                        .with_code("FC0020")
                                        .with_labels(vec![label])
                                        .with_message("invalid expression.");
                                    
                                    self.diagnostics.push(diagnostic);
                                }
                                return None;
                            }
                        };

                        let r = match self.ast_expression(*right) {
                            Some(item) => item,
                            None => {
                                if self.successful {
                                    self.successful = false;
                    
                                    // Throw an error since the symbol requested isn't defined in this scope.
                                    let label = Label::primary((), ast.range)
                                        .with_message("invalid expression here.");
    
                                    let diagnostic = Diagnostic::error()
                                        .with_code("FC0020")
                                        .with_labels(vec![label])
                                        .with_message("invalid expression.");
                                    
                                    self.diagnostics.push(diagnostic);
                                }
                                return None;
                            }
                        };

                        // Check if both types are the same.
                        let left_type = l.item.get_type(&self.symbols);
                        let right_type = r.item.get_type(&self.symbols);

                        if right_type != left_type {
                            self.successful = false;
                    
                            // Throw an error since the symbol requested isn't defined in this scope.
                            let leftt: &str = left_type.into();
                            let rightt: &str = right_type.into();

                            let label = Label::secondary((), l.range)
                                .with_message(format!("this is a(n) '{}'", leftt));

                            let label2 = Label::secondary((), r.range)
                                .with_message(format!("this is a(n) '{}'", rightt));
                            
                            let label3 = Label::primary((), ast.range)
                                .with_message(format!("both sides of this expression should be of type '{}'", leftt));

                            let diagnostic = Diagnostic::error()
                                .with_code("FC0021")
                                .with_labels(vec![label, label2, label3])
                                .with_message("cannot use two different types in expression.");
                                
                            self.diagnostics.push(diagnostic);
                            return None;
                        }

                        Some(HirMeta::new(
                            ast.range,
                            self.filename,
                            match op {
                                Opcode::Add => Hir::Add(
                                    l.into_box(),
                                    r.into_box(),
                                ),
                                Opcode::Subtract => Hir::Subtract(
                                    l.into_box(),
                                    r.into_box(),
                                ),
                                Opcode::Multiply => Hir::Multiply(
                                    l.into_box(),
                                    r.into_box(),
                                ),
                                Opcode::Divide => Hir::Divide(
                                    l.into_box(),
                                    r.into_box(),
                                ),
                                _ => panic!("Weird error occurred!"),
                            }
                        ))
                    } else if op == Opcode::Equals {
                        let n;
                        match left.item {
                            Ast::IdentifierLiteral(str) => n = str.to_string(),
                            _ => {
                                self.successful = false;
                        
                                // Throw an error since the symbol requested isn't defined in this scope.
                                let label = Label::primary((), left.range.clone())
                                    .with_message("the '=' operator may only be used on variable names.");
    
                                let diagnostic = Diagnostic::error()
                                    .with_code("FC0023")
                                    .with_labels(vec![label])
                                    .with_message("invalid set expression.");
                                
                                self.diagnostics.push(diagnostic);
    
                                return None;
                            }
                        }

                        let r = match self.ast_expression(*right) {
                            Some(item) => item,
                            None => {
                                if self.successful {
                                    self.successful = false;
                    
                                    // Throw an error since the symbol requested isn't defined in this scope.
                                    let label = Label::primary((), ast.range)
                                        .with_message("invalid expression here.");
    
                                    let diagnostic = Diagnostic::error()
                                        .with_code("FC0020")
                                        .with_labels(vec![label])
                                        .with_message("invalid expression.");
                                    
                                    self.diagnostics.push(diagnostic);
                                }
                                return None;
                            }
                        };
                        
                        let desired_type = match self.symbols.get(&n).unwrap() {
                            VariableType::Declared(t) => *t,
                            VariableType::Defined(t, ..) => *t,
                        };

                        if r.item.get_type(&self.symbols) != desired_type {
                            self.successful = false;
                            
                            let dtype: &str = desired_type.into();
                            let rtype: &str = r.item.get_type(&self.symbols).into();

                            // Throw an error since the symbol requested isn't defined in this scope.
                            let label = Label::primary((), ast.range)
                                .with_message(format!("this variable is of type '{}'", dtype));

                            let label2 = Label::primary((), r.range)
                                .with_message(format!("new value is of type '{}'", rtype));
    
                            let diagnostic = Diagnostic::error()
                                .with_code("FC0025")
                                .with_labels(vec![label, label2])
                                .with_message("variable value doesn't match variable signature.");
                                    
                            self.diagnostics.push(diagnostic);
                            return None;
                        }

                        self.symbols.insert(n.to_string(), VariableType::Defined(desired_type, 0, self.hir.len()));

                        return Some(HirMeta::new(
                            ast.range,
                            self.filename,
                            Hir::Set(
                                HirMeta::boxed(
                                    left.range,
                                    self.filename,
                                    Hir::Named(n),
                                ),
                                r.into_box()
                            )
                        ));
                    } else {
                        self.successful = false;
                    
                        // Throw an error since the symbol requested isn't defined in this scope.
                        let label = Label::primary((), ast.range)
                            .with_message("this expression isn't supported by the compiler yet.");

                        let diagnostic = Diagnostic::error()
                            .with_code("FC0019")
                            .with_labels(vec![label])
                            .with_message("unsupported expression.");
                        
                        self.diagnostics.push(diagnostic);

                        None
                    }
                },
                _ => None,
            }
        }
    }

    /// Loops through the provided AST tree, calculating which symbols are declared.
    fn resolve_symbols(&mut self, ast: &Vec<AstMeta>) {
        for item in ast {
            match &item.item {
                Ast::BinaryExpression(op, l, r) => {
                    // The expression is a binary expression, meaning it may be a variable set
                    // operation.

                    if *op != Opcode::Equals {
                        continue;
                    }

                    let n;
                    match &l.item {
                        Ast::IdentifierLiteral(str) => n = str.to_string(),
                        _ => {
                            self.successful = false;
                    
                            // Throw an error since the symbol requested isn't defined in this scope.
                            let label = Label::primary((), l.range.clone())
                                .with_message("the '=' operator may only be used on variable names.");

                            let diagnostic = Diagnostic::error()
                                .with_code("FC0023")
                                .with_labels(vec![label])
                                .with_message("invalid set expression.");
                            
                            self.diagnostics.push(diagnostic);

                            break;
                        }
                    }

                    if self.symbols.contains_key(&n) {
                        continue;
                    }

                    if let Some(t) = self.ast_expression(*r.clone()) {
                        let var_type = t.item.get_type(&self.symbols);
                        self.symbols.insert(
                            n,
                            VariableType::Declared(var_type)
                        );
                    } else {
                        if self.successful {
                            self.successful = false;
                    
                            // Throw an error since the symbol requested isn't defined in this scope.
                            let label = Label::primary((), r.range.clone())
                                .with_message("this value is invalid.");

                            let diagnostic = Diagnostic::error()
                                .with_code("FC0024")
                                .with_labels(vec![label])
                                .with_message("invalid value for variable.");
                            
                            self.diagnostics.push(diagnostic);

                            break;
                        }
                    }
                },
                _ => continue
            }
        }
    }

    /// Converts all of the items in the provided AST tree into a tree of Flycatcher HIR.
    pub fn convert(&mut self, ast: Vec<AstMeta>) {
        self.resolve_symbols(&ast);

        if !self.successful { return }

        for item in ast {
            if let Some(e) = self.ast_expression(item.clone()) {
                self.hir.push(e);
            } else {
                if self.successful {
                    self.successful = false;
                    
                    // Throw an error since the symbol requested isn't defined in this scope.
                    let label = Label::primary((), item.range)
                        .with_message("this statement isn't supported by the compiler yet.");
    
                    let diagnostic = Diagnostic::error()
                        .with_code("FC0022")
                        .with_labels(vec![label])
                        .with_message("unsupported statement.");
                            
                    self.diagnostics.push(diagnostic);
                }
            }
        }
    }

    /// Returns whether or not the compilation process was a success.
    pub fn successful(&self) -> bool {
        self.successful
    }

}