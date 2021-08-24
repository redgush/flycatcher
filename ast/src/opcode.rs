use flycatcher_lexer::Token;

/// A list of opcodes that may be used in binary expressions.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Opcode {
    Comma,
    GreaterGreater,
    LessLess,
    EqualsEquals,
    ExclamationEquals,
    GreaterEquals,
    LessEquals,
    AndAnd,
    OrOr,
    Greater,
    Less,
    And,
    Or,
    Caret,
    Not,
    Equals,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent,
}

impl Opcode {
    /// Returns the opcode associated with a token, if any.
    pub fn from_token(tok: Token) -> Option<Opcode> {
        match tok {
            Token::Comma => Some(Opcode::Comma),
            Token::GreaterGreater => Some(Opcode::GreaterGreater),
            Token::LessLess => Some(Opcode::LessLess),
            Token::EqualsEquals => Some(Opcode::EqualsEquals),
            Token::ExclamationEquals => Some(Opcode::ExclamationEquals),
            Token::GreaterEquals => Some(Opcode::GreaterEquals),
            Token::LessEquals => Some(Opcode::LessEquals),
            Token::AndAnd => Some(Opcode::AndAnd),
            Token::OrOr => Some(Opcode::OrOr),
            Token::Caret => Some(Opcode::Caret),
            Token::Not => Some(Opcode::Not),
            Token::Equals => Some(Opcode::Equals),
            Token::Plus => Some(Opcode::Plus),
            Token::Minus => Some(Opcode::Minus),
            Token::Asterisk => Some(Opcode::Asterisk),
            Token::Slash => Some(Opcode::Slash),
            Token::Percent => Some(Opcode::Percent),
            _ => None,
        }
    }

    /// Calculates the precedence of this binary operator.
    pub fn precedence(&self) -> usize {
        match self {
            Self::Not => 99,
            Self::Asterisk => 98,
            Self::Slash => 98,
            Self::Percent => 98,
            Self::Plus => 97,
            Self::Minus => 97,
            Self::GreaterGreater => 96,
            Self::LessLess => 96,
            Self::Greater => 95,
            Self::Less => 95,
            Self::GreaterEquals => 95,
            Self::LessEquals => 95,
            Self::EqualsEquals => 94,
            Self::ExclamationEquals => 94,
            Self::And => 93,
            Self::Caret => 92,
            Self::Or => 91,
            Self::AndAnd => 90,
            Self::OrOr => 89,
            Self::Equals => 88,
            Self::Comma => 87,
        }
    }
}
