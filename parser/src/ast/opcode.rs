//! Expression opcodes (such as `+` and `-`) for Flycatcher's parser.

use flycatcher_lexer::Token;

// A list of expression opcodes.
#[derive(Clone, Debug)]
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

impl Opcode {

    /// Returns the precidence of this operator.
    pub fn precedence(&self) -> usize {
        match self {
            Opcode::Add => 1,
            Opcode::Subtract => 1,
            Opcode::Multiply => 2,
            Opcode::Divide => 2
        }
    }

}

/// Returns whether or not the specified token is an operator that has an Opcode.
pub fn is_operator(tok: Token) -> bool {
    match tok {
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
        Token::Plus => Opcode::Add,
        Token::Dash => Opcode::Subtract,
        Token::Star => Opcode::Multiply,
        Token::Slash => Opcode::Divide,
        _ => return None
    })
}