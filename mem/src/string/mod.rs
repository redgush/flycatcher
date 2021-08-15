//! Provides a data structure similar to the standard library String.

extern crate alloc;
extern crate core;

use alloc::alloc::{alloc, dealloc, Layout, realloc};
use core::mem::size_of;

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

    /// The raw pointer to the memory where the string's characters are stored.  Raw pointers
    /// are used to prevent the use of the standard library, as well as to help with dynamic
    /// sizing.
    ptr: *mut char,

    /// This is the count of characters in the string.  The `AllocString` isn't null terminated,
    /// rather, it uses this value to keep track of the length of the string.
    len: usize,

}

/// The FlycatcherString implementation for AllocString.
impl FlycatcherString for AllocString {

    fn from_str(src: &str) -> Self {
        // Convert the string to a list of characters that can be iterated through.
        let chars = src.as_bytes();
        let len = chars.len(); // this will be the length of the AllocString object.

        // Allocate the memory to the size of the string, so it doesn't have to be reallocated
        // to fit the source string later.
        let ptr = unsafe {
            let layout = Layout::from_size_align_unchecked(len, size_of::<char>() * len);
            alloc(layout) as *mut char
        };

        // Now we need to load the characters from `chars` into the heap, via the allocated
        // pointer defined above.
        for (i, c) in chars.iter().enumerate() {
            // `i` is the index of the character and `c` is the character itself.

            // We need to directly access the raw pointer to write to it, but that's okay.
            unsafe {
                *ptr.add(i) = *c as char;
            }
        }

        // Initialize and return the object.
        Self {
            ptr,
            len
        }
    }

    fn get(&self, idx: usize) -> Option<char> {
        // First, we need to check if the index provided is in memory.
        if idx >= self.len {
            return None;
        }

        // If we get here, that means that the index is in bounds, meaning we can return the
        // value directly.
        unsafe {
            Some(*(&*self.ptr.add(idx)))
        }
    }

    fn put(&mut self, idx: usize, c: char) {
        // `set` is implemented similarly to the `get` method, except it writes directly to
        // memory instead of reading directly from memory.

        // First, we need to check if the index provided is in memory.
        if idx >= self.len {
            return;
        }

        // If we get here, that means that the index is in bounds, meaning we can return the
        // value directly.
        unsafe {
            *self.ptr.add(idx) = c;
        }
    }

}

/// Memory management for AllocString.
impl Drop for AllocString {

    fn drop(&mut self) {
        unsafe {
            let layout = Layout::from_size_align_unchecked(self.len, size_of::<char>() * self.len);
            dealloc(self.ptr as *mut u8, layout);
        }
    }

}