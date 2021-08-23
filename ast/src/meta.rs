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
}

impl AstMeta {
    pub fn new(range: Range<usize>, item: Ast) -> Self {
        Self {
            range,
            semicolon: false,
            comments: vec![],
            item,
        }
    }
}
