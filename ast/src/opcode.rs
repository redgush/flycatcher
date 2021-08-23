/// A list of opcodes that may be used in binary expressions.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BinaryOpcode {
    GreaterGreater,
    LessLess,
    EqualsEquals,
    ExclamationEquals,
    GreaterEquals,
    LessEquals,
    Greater,
    Less,
    And,
    Or,
    Not,
    Equals,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent,
}