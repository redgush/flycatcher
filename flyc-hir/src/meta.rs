//! Tools for mapping Flycatcher HIR to a source file.

use crate::Hir;
use std::ops::Range;

/// Provides "metadata" about an HIR item.  This includes where the HIR item was found in a source file.
#[derive(Clone)]
pub struct HirMeta {
    /// The absolute indexes where the HIR item starts and ends in the source file.
    pub range: Range<usize>,

    /// The HIR item that this struct wraps.
    pub item: Hir,
}

impl HirMeta {
    /// Creates an HIR metadata struct.
    pub fn new(range: Range<usize>, item: Hir) -> Self {
        Self { range, item }
    }

    /// Creates an HIR metadata object and immediately boxes it.
    pub fn boxed(range: Range<usize>, item: Hir) -> Box<Self> {
        Box::new(Self { range, item })
    }

    /// (Destructively) converts this HIR metadata struct into a boxed HIR metadata struct.
    pub fn into_box(self) -> Box<Self> {
        Box::new(self)
    }
}

impl std::fmt::Debug for HirMeta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.item.fmt(f)
    }
}
