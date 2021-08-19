use crate::FlycatcherType;

/// Different types of variables, which may use their own FlycatcherTypes.
pub enum VariableType {

    /// A variable that is declared, but not yet defined.  This is used for the first phase of
    /// Flycatcher's compiler, which only resolves the basic symbols and their signatures.
    Declared(FlycatcherType),

    /// A variable that has been defined.  The first `usize` argument is the amount of times the
    /// variable was referenced, and the second `usize` argument is the index in the HIR vector
    /// that the variable's definition is at.
    /// 
    /// The second `usize` is used to remove variable definitions that aren't used for anything.
    Defined(FlycatcherType, usize, usize),

}