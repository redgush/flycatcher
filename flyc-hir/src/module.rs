use crate::{HirConstruct, HirFunction};

/// Different symbol types used by Flycatcher HIR.
#[derive(Clone, Debug, PartialEq)]
pub enum HirSymbol {
    /// A function that was declared using the `declare` keyword.
    ExternalFunction(HirFunction),

    /// A function declared inside of the module.
    Function(HirFunction),

    /// A construct declared inside of the module.
    Construct(HirConstruct),
}

/// A module which stores Flycatcher HIR objects.
#[derive(Clone, Debug, PartialEq)]
pub struct HirModule {

    /// A list of symbols declared in this module.
    pub symbols: Vec<HirSymbol>,

}