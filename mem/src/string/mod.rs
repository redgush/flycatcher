//! Provides a data structure similar to the standard library String.

/// A general purpose string of utf-8 characters.
pub trait FlycatcherString {

    /// Creates a FlycatcherString from a Rust string.  Iterates through all of the characters
    /// in the `src` string and writes them directly to memory.
    /// 
    /// # Arguments
    /// - `src`: The Rust string to read bytes from.
    /// 
    /// # Returns
    /// - The `FlycatcherString` that mirrored the contents of the Rust string.
    fn from_str(src: &str) -> Self;

    /// Retrieves a single character from the string's raw pointer at the index specified.  If
    /// the index provided is greater than or equal to the length of the string, `None` is
    /// returned.
    /// 
    /// # Arguments
    /// - `idx`: The index of the string where the expected character is located.
    /// 
    /// # Returns
    /// - The character found.  If the index of the character was out of bounds, `None` will
    /// be returned.  Otherwise, `Some` will be returned with the character that was found at
    /// the given `idx`.
    fn get(&self, idx: usize) -> Option<char>;

    /// Sets a single character in the string at the `idx` (index) specified.  If the index
    /// provided is out of bounds, the function will not panic, but it will do nothing.
    /// 
    /// # Arguments
    /// - `idx`: The index that `c` will be written to in the `FlycatcherString`.
    /// - `c`: The character to write at the provided `idx`.
    fn put(&mut self, idx: usize, c: char);

}

/// A general purpose string that implements the `FlycatcherString` trait.  It uses the global
/// allocator to allocate memory for the string.
pub struct AllocString {



}