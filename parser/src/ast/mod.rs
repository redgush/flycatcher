//! Tools for generating AST trees.

pub mod meta;
pub mod opcode;

pub use meta::AstMeta;
pub use opcode::Opcode;

/// A list of possible AST expressions.
#[derive(Debug)]
pub enum Ast {

    NullLiteral,

    /// A boolean literal, either `true` or `false`.
    BooleanLiteral(bool),

    /// An identifier literal.
    IdentifierLiteral(String),

    /// A binary expression with two operands.
    /// 
    /// ```flycatcher
    /// 1 + 1
    /// ```
    BinaryExpression(Opcode, Box<AstMeta>, Box<AstMeta>),

    /// An expression made with an unlimited amount of indexes, for example,
    /// `item1["item2"].item3[item4()]`
    IndexExpression(Vec<AstMeta>),

    /// An index for an IndexExpression that may contain something other than an identifier.
    /// 
    /// ```flycatcher
    /// item1["BracketIndexHere"]
    /// ```
    BracketIndex(Box<AstMeta>),

    /// This is caused by using the `+` operator at the start of an operand, such as `+10`.
    PositiveUnary(Box<AstMeta>),

    /// This is caused by using the `-` operator at the start of an operand, such as `-10`.
    NegativeUnary(Box<AstMeta>),

}