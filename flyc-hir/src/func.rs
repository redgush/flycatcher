use crate::symbol::Access;
use crate::Hir;
use flyc_types::Named;

/// A parameter for the function.
#[derive(Clone, Debug, PartialEq)]
pub struct FunctionArgument {
    /// The name of the argument.
    pub name: String,

    /// The type annotation provided for the parameter.
    pub ty: Hir,
}

/// An overload for a function.
#[derive(Clone, Debug, PartialEq)]
pub struct HirFunctionOverload {
    /// The code associated with the overload.
    pub hir: Vec<Hir>,

    /// A list of function arguments.
    pub args: Vec<FunctionArgument>,

    /// The optional return type annotation of the function overload.
    pub returns: Option<Hir>,
}

/// A Function declaration.
#[derive(Clone, Debug, PartialEq)]
pub struct HirFunction {
    /// The construct used to declare the function.
    pub construct: String,

    /// The name of the function.
    pub name: String,

    /// The absolute path to the function.
    pub full_name: Named,

    /// The allowed access of the function.
    pub access: Access,

    /// A list of overloads for this function.  There must be atleast 1 overload, which is the default
    /// function to use if there are no other overloads.
    pub overloads: Vec<HirFunctionOverload>,
}
