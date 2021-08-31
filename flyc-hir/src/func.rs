use crate::{Access, HirMeta};

/// Function parameters with a name and type annotation.
#[derive(Clone, Debug, PartialEq)]
pub struct HirFunctionArgument {
    /// The name of the argument.
    pub name: String,

    /// The type of the argument, not translated to a type object yet because we are not in the type
    /// checking pass yet.
    pub ty: HirMeta,
}

/// A Flycatcher function.
#[derive(Clone, Debug, PartialEq)]
pub struct HirFunction {

    /// The construct used to initialize the function.
    pub construct: String,

    /// The name of the function.
    pub name: String,

    /// The access allowed for the function.
    pub access: Access,

    /// A list of parameters declared in the function.
    pub args: Vec<HirFunctionArgument>,

    /// The instructions defined inside of the function.
    pub code: Vec<HirMeta>,

    /// A list of templates declared for the construct in the top level, for example:
    /// 
    /// ```flycatcher
    /// @construct template<T> {
    ///     // ...
    /// }
    /// ```
    pub templates: Vec<HirMeta>,
}