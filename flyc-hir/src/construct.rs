use crate::func::HirFunction;
use crate::Hir;
use flyc_types::Named;

/// A property for a construct.
pub struct HirConstructProperty {
    /// The name of the property.
    pub name: String,

    /// The type declaration for the item.
    pub ty: Option<Hir>,

    /// The default value for the item.
    pub value: Option<Hir>,
}

/// A Flycatcher construct declaration.
pub struct HirConstruct {
    /// The name of the construct used to declare this construct.
    pub construct: String,

    /// A list of methods in the construct.
    pub methods: Vec<HirFunction>,

    /// A list of properties defined in the construct.
    pub properties: Vec<HirConstructProperty>,

    /// The name of this construct.
    pub name: String,

    /// The absolute path to the construct.
    pub full_name: Named,
}
