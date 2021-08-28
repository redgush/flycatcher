/// A "named" identifier, used as the full names of types.  The first `String` is the topmost item, and
/// the `Vec<String>` is a list of following properties.  `named.property.property2` would be equal to
/// 
/// ```ast
/// Named("named", ["property", "property2"])
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Named(pub String, pub Vec<String>);