//! Provides types for Flycatcher typechecking, across all levels of Flycatcher.

mod align;
mod construct;
mod func;
mod named;
mod cstruct;

pub use align::round;
pub use construct::{Construct, ConstructProperty};
pub use cstruct::CStruct;
pub use func::{Function, FunctionSignature};
pub use named::Named;

/// Different basic types that can be derived from in the compiler.
#[derive(Clone, Debug, PartialEq)]
pub enum FlycatcherType {

    /// A "void" type that has an align of `0`.
    Void,

    /// A boolean type whos value must be either 1 (`true`) or 0 (`false`).
    Bool,

    /// An 8-bit unsigned integer.
    Uint8,

    /// An 16-bit unsigned integer.
    Uint16,

    /// An 32-bit unsigned integer.
    Uint32,

    /// An 64-bit unsigned integer.
    Uint64,
    
    /// An unsigned integer that scales to the target architecture.  For example, on 32-bit archectures,
    /// this would be a 32-bit unsigned integer.  On 64-bit architectures, this would be a 64-bit unsigned
    /// integer.
    Usize,

    /// 8-bit signed integer.
    Int8,

    /// 16-bit signed integer.
    Int16,

    /// 32-bit signed integer.
    Int32,

    /// 64-bit signed integer.
    Int64,

    /// A signed integer that scales to the target architecture.  For example, on 32-bit archectures,
    /// this would be a 32-bit signed integer.  On 64-bit architectures, this would be a 64-bit signed
    /// integer.
    Size,

    /// A 32-bit floating point number.
    Float32,

    /// A 64-bit floating point number.
    Float64,

    /// A Flycatcher function pointer.
    Func(Function),

    /// A Flycatcher construct.
    Construct(Construct),

    /// A C-ABI struct.
    CStruct(CStruct),

}

impl FlycatcherType {

    /// Returns the align of this type on 32-bit target architectures (in bytes).
    pub fn get_32bit_align(&self) -> usize {
        match self {
            FlycatcherType::Void => 0,
            FlycatcherType::Bool => 1,
            FlycatcherType::Uint8 => 1,
            FlycatcherType::Uint16 => 2,
            FlycatcherType::Uint32 => 4,
            FlycatcherType::Uint64 => 8,
            FlycatcherType::Usize => 4, // `usize` is 4 bytes on 32-bit architectures.
            FlycatcherType::Int8 => 1,
            FlycatcherType::Int16 => 2,
            FlycatcherType::Int32 => 4,
            FlycatcherType::Int64 => 8,
            FlycatcherType::Size => 4, // `size` is 4 bytes on 32-bit architectures.
            FlycatcherType::Float32 => 4,
            FlycatcherType::Float64 => 8,
            FlycatcherType::Func(_) => 4, // Function pointers are 4 bytes on 32-bit.
            FlycatcherType::Construct(c) => c.calculate_32bit_align(),
            FlycatcherType::CStruct(c) => c.calculate_32bit_align(),
        }
    }

    /// Returns the align of this type on 64-bit target architectures (in bytes).
    pub fn get_64bit_align(&self) -> usize {
        match self {
            FlycatcherType::Void => 0,
            FlycatcherType::Bool => 1,
            FlycatcherType::Uint8 => 1,
            FlycatcherType::Uint16 => 2,
            FlycatcherType::Uint32 => 4,
            FlycatcherType::Uint64 => 8,
            FlycatcherType::Usize => 8, // `usize` is 8 bytes on 64-bit architectures.
            FlycatcherType::Int8 => 1,
            FlycatcherType::Int16 => 2,
            FlycatcherType::Int32 => 4,
            FlycatcherType::Int64 => 8,
            FlycatcherType::Size => 8, // `size` is 8 bytes on 64-bit architectures.
            FlycatcherType::Float32 => 4,
            FlycatcherType::Float64 => 8,
            FlycatcherType::Func(_) => 8, // Function pointers are 8 bytes on 64-bit.
            FlycatcherType::Construct(c) => c.calculate_64bit_align(),
            FlycatcherType::CStruct(c) => c.calculate_64bit_align(),
        }
    }

    /// Returns the size of this type on 32-bit target architectures (in bytes).
    pub fn get_32bit_size(&self) -> usize {
        match self {
            FlycatcherType::Construct(c) => c.calculate_32bit_size(),
            FlycatcherType::CStruct(c) => c.calculate_32bit_size(),
            _ => self.get_32bit_align()
        }
    }

    /// Returns the size of this type on 64-bit target architectures (in bytes).
    pub fn get_64bit_size(&self) -> usize {
        match self {
            FlycatcherType::Construct(c) => c.calculate_64bit_size(),
            FlycatcherType::CStruct(c) => c.calculate_64bit_size(),
            _ => self.get_64bit_align()
        }
    }

}