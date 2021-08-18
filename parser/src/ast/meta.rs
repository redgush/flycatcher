//! Exposes a data structure for information about AST items.

use crate::ast::Ast;
use std::ops::Range;

/// Describes where an AST item was found in the source input string.
//#[derive(Debug)]
pub struct AstMeta {

    // The range of characters where the AST item in this metadata structure was found.
    pub range: Range<usize>,

    /// The item that this metadata structure describes.
    pub item: Ast,

}

impl AstMeta {

    /// Creates a new AST metadata data structure.
    pub fn new(range: Range<usize>, item: Ast) -> Self {
        Self {
            range,
            item
        }
    }

    /// Creates a new AST metadata object, then boxing it.
    pub fn boxed(range: Range<usize>, item: Ast) -> Box<Self> {
        Box::new(
            Self {
                range,
                item
            }
        )
    }

    /// Converts this AST metadata object into a boxed AST metadata object.
    pub fn as_box(self) -> Box<Self> {
        Box::new(self)
    }

}

impl std::fmt::Debug for AstMeta {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.item.fmt(f)
    }

}