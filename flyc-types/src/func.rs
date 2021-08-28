use crate::{FlycatcherType, Named};

/// A Flycatcher function which allows any amount of overloads.
#[derive(Clone, Debug, PartialEq)]
pub struct Function {

    /// The name of the function, such as `printf`.
    pub name: String,

    /// The full absolute path to the function.  This is used mostly to mangle the name at compile time,
    /// and maybe diagnostic messages.
    pub full_name: Named,

    /// A list of the signatures defined.
    pub signatures: Vec<FunctionSignature>,

}

/// Signature for type checking function calls.  There may be multiple signatures per function; known as
/// overloads.  A function, of course, must have atleast one signature.
#[derive(Clone, Debug, PartialEq)]
pub struct FunctionSignature {

    /// A list of arguments declared for the function.
    pub arguments: Vec<FlycatcherType>,

    /// The type that the function returns, if any.
    pub returns: FlycatcherType,

}

impl Function {

    /// Creates a new Function with the specified signatures.
    pub fn new(name: String, full_name: Named, signature: FunctionSignature) -> Self {
        Function {
            name,
            full_name,
            signatures: vec![signature]
        }
    }

}

impl FunctionSignature {

    /// Creates a new, empty Flycatcher function signature.
    pub fn new() -> Self {
        Self {
            arguments: vec![],
            returns: FlycatcherType::Void,
        }
    }

}