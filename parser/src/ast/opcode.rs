//! Expression opcodes (such as `+` and `-`) for Flycatcher's parser.

use flycatcher_lexer::Token;

// A list of expression opcodes.
#[derive(Clone, Debug)]
pub enum Opcode {

    /// The == operator.
    EqualsEquals,

    /// The > operator.
    Greater,

    /// The < operator.
    Less,

    /// The >= operator.
    GreaterEquals,

    /// The <= operator.
    LessEquals,

    /// The + operator.
    Add,

    /// The - operator.
    Subtract,

    /// The - operator.
    Multiply,

    /// The / operator.
    Divide,

}

impl Opcode {

    /// Returns the precidence of this operator.
    pub fn precedence(&self) -> usize {
        match self {
            Opcode::EqualsEquals => 1,
            Opcode::Greater => 1,
            Opcode::GreaterEquals => 1,
            Opcode::Less => 1,
            Opcode::LessEquals => 1,
            Opcode::Add => 2,
            Opcode::Subtract => 2,
            Opcode::Multiply => 3,
            Opcode::Divide => 3
        }
    }

}

/// Returns whether or not the specified token is an operator that has an Opcode.
pub fn is_operator(tok: Token) -> bool {
    match tok {
        Token::EqualsEquals |
        Token::GreaterThan |
        Token::GreaterThanOrEqual |
        Token::LessThan |
        Token::LessThanOrEqual |
        Token::Plus |
        Token::Dash |
        Token::Star |
        Token::Slash => true,
        //Token::Percent => true,
        _ => false
    }
}

/// Converts a token to an operator.
pub fn get_operator(tok: Token) -> Option<Opcode> {
    Some(match tok {
        Token::EqualsEquals => Opcode::EqualsEquals,
        Token::GreaterThan => Opcode::Greater,
        Token::LessThan => Opcode::Less,
        Token::GreaterThanOrEqual => Opcode::GreaterEquals,
        Token::LessThanOrEqual => Opcode::LessEquals,
        Token::Plus => Opcode::Add,
        Token::Dash => Opcode::Subtract,
        Token::Star => Opcode::Multiply,
        Token::Slash => Opcode::Divide,
        _ => return None
    })
}