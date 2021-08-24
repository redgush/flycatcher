use crate::Ast;
use std::ops::Range;

/// Provides "metadata" about an AST item.  This includes where the AST item was found in a source file,
/// the document comments before it, whether there's a semicolon after it, and so on.
pub struct AstMeta {
    /// The absolute indexes where the AST item starts and ends in the source file.
    pub range: Range<usize>,

    /// Whether or not there was a semicolon after the AST item that this struct wraps.
    pub semicolon: bool,

    /// A list of document comments that were defined before this AST item.
    pub comments: Vec<String>,

    /// The AST item that this struct wraps.
    pub item: Ast,

    /// Whether or not the AST item is wrapped in parenthesis.
    pub parenthesis: bool,
}

impl AstMeta {
    /// Creates an AST metadata struct.  `semicolon` defaults to `false`, which may be changed later.
    pub fn new(range: Range<usize>, item: Ast) -> Self {
        Self {
            range,
            semicolon: false,
            comments: vec![],
            item,
            parenthesis: false,
        }
    }

    /// Creates an AST metadata object and immediately boxes it.
    pub fn boxed(range: Range<usize>, item: Ast) -> Box<Self> {
        Box::new(Self {
            range,
            semicolon: false,
            comments: vec![],
            item,
            parenthesis: false,
        })
    }

    /// Sets the list of comments for this AST item.
    pub fn with_comments(mut self, comments: Vec<String>) -> Self {
        self.comments = comments;
        self
    }

    /// Sets the value of `semicolon` to `true`.
    pub fn semicolon(&mut self) {
        self.semicolon = true;
    }

    /// Sets the value of `parenthesis` to `true`.
    pub fn parenthesis(&mut self) {
        self.parenthesis = true;
    }
}
