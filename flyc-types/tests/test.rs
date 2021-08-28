extern crate flyc_types;

use flyc_types::{Construct, ConstructProperty, FlycatcherType, Named};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test() {
        let mut test_const = Construct {
            name: "test".into(),
            full_name: Named("test".into(), vec![]),
            properties: vec![
                ConstructProperty {
                    name: "prop1".into(),
                    ty: FlycatcherType::Uint8
                },
                ConstructProperty {
                    name: "prop2".into(),
                    ty: FlycatcherType::Uint8
                },
                ConstructProperty {
                    name: "prop3".into(),
                    ty: FlycatcherType::Uint16
                },
            ],
            methods: vec![]
        };

        assert_eq!(test_const.calculate_64bit_size(), 4);
    }
}