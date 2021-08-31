use crate::{HirConstruct, HirFunction};

/// Different symbol types used by Flycatcher HIR.
pub enum HirSymbol {
    /// A function that was declared using the `declare` keyword.
    ExternalFunction(HirFunction),

    /// A function declared inside of the module.
    Function(HirFunction),

    /// A construct declared inside of the module.
    Construct(HirConstruct),
}

pub struct HirModule {

    /// A list of symbols declared in this module.
    pub symbols: Vec<HirSymbol>,

}