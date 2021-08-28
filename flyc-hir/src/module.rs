use crate::symbol::SymbolType;

/// A Flycatcher module stores information about a Flycatcher source, including its imports, symbols,
/// etc.
#[derive(Clone, Debug, PartialEq)]
pub struct FlycatcherModule {
    /// A list of symbols declared in the module.
    pub symbols: Vec<SymbolType>,

    /// A list of `#import`s used by the module.
    pub imports: Vec<String>,
}
