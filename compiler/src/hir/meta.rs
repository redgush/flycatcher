use crate::Hir;
use std::ops::Range;

/// Provides metadata about a HIR object, such as where it was found in the input file, what the
/// name of the file that it was found in is, etc.
#[derive(Clone, Debug)]
pub struct HirMeta<'a> {

    /// The range of characters in the source file that this HIR item was found in.
    pub range: Range<usize>,

    /// The name of the file that the HIR item was found in.
    pub filename: &'a str,

    /// The HIR item that this struct wraps.
    pub item: Hir<'a>,

}

impl<'a> HirMeta<'a> {

    /// Creates a new HirMeta instance.
    pub fn new(range: Range<usize>, filename: &'a str, item: Hir<'a>) -> Self {
        Self {
            range,
            filename,
            item
        }
    }

    /// Creates a new boxed HirMeta instance.
    pub fn boxed(range: Range<usize>, filename: &'a str, item: Hir<'a>) -> Box<Self> {
        Box::new(HirMeta::new(range, filename, item))
    }

    /// Converts the current HirMeta instance into a boxed HirMeta instance.
    pub fn into_box(self) -> Box<Self> {
        Box::new(self)
    }

}