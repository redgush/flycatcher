use crate::func::HirFunction;
use crate::Hir;
use flyc_types::Named;

/// Different accesses.
#[derive(Clone, Debug, PartialEq)]
pub enum Access {
    /// Accessable outside of the module; using the `pub` keyword.
    Public,

    /// Accessable only inside of the module, which uses the `priv` keyword.  This is the default access
    /// for all symbols.
    Private,
}

/// A static variable declaration.
#[derive(Clone, Debug, PartialEq)]
pub struct StaticVariable {
    /// The name of the variable.
    pub name: String,

    /// The absolute path to the static variable.
    pub full_name: Named,

    /// The value assigned to the static variable.
    pub val: Hir,

    /// The allowed access of the static variable.
    pub access: Access,
}

/// Different types of symbols usable by the compiler.
#[derive(Clone, Debug, PartialEq)]
pub enum SymbolType {
    /// A static variable type.
    StaticVariable(StaticVariable),

    /// A function type.
    Function(HirFunction),
}