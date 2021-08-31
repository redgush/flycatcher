use crate::{Access, HirMeta, HirFunction};

pub struct HirConstructProperty {
    /// The name of the property.
    pub name: String,

    /// The type annotation provided for the property.
    pub annotation: HirMeta,

    /// The accessability allowed by the property.
    pub access: Access
}

/// A Flycatcher style construct.
pub struct HirConstruct {
    /// The name of the construct used to define this construct.
    pub construct: String,

    /// The name of the construct.
    pub name: String,

    /// The publicity of the construct.
    pub access: Access,

    /// A list of methods declared in the construct.
    pub methods: Vec<HirFunction>,

    /// A list of properties in the construct.
    pub properties: Vec<HirConstructProperty>,

    /// A list of templates declared for the construct in the top level, for example:
    /// 
    /// ```flycatcher
    /// @construct template<T> {
    ///     // ...
    /// }
    /// ```
    pub templates: Vec<HirMeta>,
}