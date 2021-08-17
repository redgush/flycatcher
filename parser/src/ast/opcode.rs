//! Expression opcodes (such as `+` and `-`) for Flycatcher's parser.

// A list of expression opcodes.
#[derive(Debug)]
pub enum Opcode {

    /// The + operator.
    Add,

    /// The - operator.
    Subtract,

    /// The - operator.
    Multiply,

    /// The / operator.
    Divide,

}