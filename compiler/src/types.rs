/// The types of the Flycatcher compiler.  This includes the inbuilt primitive types such as 
/// uint64 and boolean.
#[derive(Clone, Copy, PartialEq)]
pub enum FlycatcherType {

    /// A boolean type, which is a single bit which may be either 1 (true) or 0 (false), wrapped
    /// in a single byte.
    Boolean,

    /// The `uint8` type, an 8-bit unsigned integer.
    Uint8,

    /// The `uint16` type, an 16-bit unsigned integer.
    Uint16,
    
    /// The `uint32` type, an 32-bit unsigned integer.
    Uint32,

    /// The `uint64` type, an 64-bit unsigned integer.
    Uint64,

    /// The `usize` type, which scales according to the target ISA's architecture, 32 bit is a
    /// 32-bit unsigned integer, 64-bit is a 64-bit unsigned integer.
    Usize,

    /// The `int8` type, an 8-bit signed integer.
    Int8,

    /// The `int16` type, an 16-bit signed integer.
    Int16,

    /// The `int32` type, an 32-bit signed integer.
    Int32,

    /// The `int64` type, an 64-bit signed integer.
    Int64,

    /// The `size` type, which scales according to the target ISA's architecture, 32 bit is a
    /// 32-bit signed integer, 64-bit is a 64-bit signed integer.
    Size,

    /// A 32-bit floating point number.
    Float32,

    /// A 64-bit floating point number.
    Float64,

}

impl<'a> Into<&'a str> for FlycatcherType {

    fn into(self) -> &'a str {
        match self {
            FlycatcherType::Boolean => "boolean",
            FlycatcherType::Uint8 => "uint8",
            FlycatcherType::Uint16 => "uint16",
            FlycatcherType::Uint32 => "uint32",
            FlycatcherType::Uint64 => "uint64",
            FlycatcherType::Usize => "usize",
            FlycatcherType::Int8 => "int8",
            FlycatcherType::Int16 => "int16",
            FlycatcherType::Int32 => "int32",
            FlycatcherType::Int64 => "int64",
            FlycatcherType::Size => "size",
            FlycatcherType::Float32 => "float32",
            FlycatcherType::Float64 => "float64",
        }
    }

}