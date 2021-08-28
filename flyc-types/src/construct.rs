use crate::{round, FlycatcherType, Function, Named};

/// Properties for constructs and structs.
#[derive(Clone, Debug, PartialEq)]
pub struct ConstructProperty {
    /// The name of the property.
    pub name: String,

    /// The type of the property.
    pub ty: FlycatcherType,
}

/// A Flycatcher-style construct.
#[derive(Clone, Debug, PartialEq)]
pub struct Construct {
    /// The name of the construct.
    pub name: String,

    /// The absolute path to the construct, used in mangling.
    pub full_name: Named,

    /// A list of properties declared in the construct.
    pub properties: Vec<ConstructProperty>,

    /// A list of methods declared in the construct.
    pub methods: Vec<Function>,
}

impl Construct {
    /// Calculates the minimum align for this value.
    pub fn calculate_32bit_align(&self) -> usize {
        let mut size = 0;

        for prop in &self.properties {
            let align = prop.ty.get_32bit_align();
            if align > size {
                size = align;
            }
        }

        size
    }

    /// Calculates the minimum align for this value.
    pub fn calculate_64bit_align(&self) -> usize {
        let mut size = 0;

        for prop in &self.properties {
            let align = prop.ty.get_64bit_align();
            if align > size {
                size = align;
            }
        }

        size
    }

    /// Calculates the alignment and padding between each member of the construct.
    pub fn calculate_32bit_size(&self) -> usize {
        let mut size = 0;

        let mut i = 0;
        while i < self.properties.len() {
            let prop = &self.properties[i];

            let second_i = i + 1;
            if second_i < self.properties.len() {
                let second_prop = &self.properties[second_i];
                let first_align = prop.ty.get_32bit_align();
                let second_align = second_prop.ty.get_32bit_align();

                size += first_align;
                size = round(size, second_align);
            } else {
                size += prop.ty.get_32bit_align();
            }

            i += 1;
        }

        round(size, self.calculate_32bit_align())
    }

    /// Calculates the alignment and padding between each member of the construct.
    pub fn calculate_64bit_size(&self) -> usize {
        let mut size = 0;

        let mut i = 0;
        while i < self.properties.len() {
            let prop = &self.properties[i];

            let second_i = i + 1;
            if second_i < self.properties.len() {
                let second_prop = &self.properties[second_i];
                let first_align = prop.ty.get_64bit_align();
                let second_align = second_prop.ty.get_64bit_align();

                size += first_align;
                size = round(size, second_align);
            } else {
                size += prop.ty.get_64bit_align();
            }

            i += 1;
        }

        round(size, self.calculate_64bit_align())
    }
}
