use flycatcher_lexer::Token;

/// A list of opcodes that may be used in binary expressions.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Opcode {
    /// The `Period` is used for member accessing, such as `my_variable.my_access`
    Period,

    /// The `Subscript` operator is used for indexing, such as `my_variable[0]`
    Subscript,

    /// `Call` operators are `()` argument lists after a name, such as `my_function()`.
    Call,
    Colon,
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
    Exclamation,
}

impl Opcode {
    /// Returns the opcode associated with a token, if any.
    pub fn from_token(tok: Token) -> Option<Opcode> {
        match tok {
            Token::Colon => Some(Opcode::Colon),
            Token::Period => Some(Opcode::Period),
            Token::LBrack => Some(Opcode::Subscript),
            Token::LParen => Some(Opcode::Call),
            Token::EqualsEquals => Some(Opcode::EqualsEquals),
            Token::Exclamation => Some(Opcode::Exclamation),
            Token::ExclamationEquals => Some(Opcode::ExclamationEquals),
            Token::GreaterEquals => Some(Opcode::GreaterEquals),
            Token::LessEquals => Some(Opcode::LessEquals),
            Token::Less => Some(Opcode::Less),
            Token::Greater => Some(Opcode::Greater),
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
    pub fn infix_precedence(&self) -> Option<(usize, usize)> {
        Some(match self {
            Self::Period => (100, 99),
            Self::Not => (93, 94),
            Self::Asterisk => (91, 92),
            Self::Slash => (91, 92),
            Self::Percent => (91, 92),
            Self::Plus => (89, 90),
            Self::Minus => (89, 90),
            Self::GreaterGreater => (87, 88),
            Self::LessLess => (87, 88),
            Self::Greater => (85, 86),
            Self::Less => (85, 86),
            Self::GreaterEquals => (85, 86),
            Self::LessEquals => (85, 86),
            Self::EqualsEquals => (83, 84),
            Self::ExclamationEquals => (83, 84),
            Self::And => (81, 82),
            Self::Caret => (79, 80),
            Self::Or => (77, 78),
            Self::AndAnd => (75, 76),
            Self::OrOr => (73, 74),
            Self::Colon => (71, 72),
            Self::Equals => (69, 70),
            _ => return None,
        })
    }

    /// Returns the postfix binding power of this operator, if applicable.
    pub fn postfix_precedence(&self) -> Option<usize> {
        Some(match self {
            Self::Subscript => 100,
            Self::Call => 98,
            _ => return None,
        })
    }

    /// Returns the prefix binding power of this operator, if applicable.
    pub fn prefix_precedence(&self) -> Option<usize> {
        Some(match self {
            Self::Exclamation => 98,
            Self::Plus => 97,
            Self::Minus => 97,
            _ => return None,
        })
    }

    /// Returns the binding power of the operator if it is an infix operator.  This returns the binding
    /// power of the opcode for *types*, which allows template declarations.
    pub fn type_infix_precedence(&self) -> Option<(usize, usize)> {
        Some(match self {
            Self::Period => (100, 99),
            Self::Plus => (97, 98),
            Self::Colon => (95, 96),
            _ => return None,
        })
    }

    /// Returns the type binding power of the operator, if it is a postfix.
    pub fn type_postfix_precedence(&self) -> Option<usize> {
        Some(match self {
            Self::Subscript => 100,
            // The `<` token starts a template declaration.
            Self::Less => 99,
            _ => return None,
        })
    }
}
