extern crate flycatcher_mem;

use flycatcher_mem::string::{AllocString, FlycatcherString};

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn string_index() {
        // Initialize a FlycatcherString.
        let str = AllocString::from_str("Hello, world!");

        assert_eq!(str.get(0), Some('H'));
        assert_eq!(str.get(1), Some('e'));
        assert_eq!(str.get(2), Some('l'));
        assert_eq!(str.get(3), Some('l'));
        assert_eq!(str.get(4), Some('o'));
        assert_eq!(str.get(5), Some(','));
        assert_eq!(str.get(6), Some(' '));
        assert_eq!(str.get(7), Some('w'));
        assert_eq!(str.get(8), Some('o'));
        assert_eq!(str.get(9), Some('r'));
        assert_eq!(str.get(10), Some('l'));
        assert_eq!(str.get(11), Some('d'));
        assert_eq!(str.get(12), Some('!'));
    }

}