//! Exposes the `AllocString` type.

extern crate alloc;
extern crate core;

use crate::string::FlycatcherString;
use alloc::alloc::{alloc, dealloc, Layout, realloc};
use core::mem::size_of;

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

    fn push(&mut self, c: char) {
        unsafe {
            let layout = Layout::from_size_align_unchecked(self.len + 1, size_of::<char>() * (self.len + 1));
            realloc(self.ptr as *mut u8, layout, layout.size());

            *self.ptr.add(self.len) = c;
        }
        self.len += 1;
    }

    fn len(&self) -> usize {
        self.len
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