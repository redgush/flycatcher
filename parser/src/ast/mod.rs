//! Tools for generating AST trees.

pub mod meta;
pub mod opcode;

pub use meta::AstMeta;
pub use opcode::Opcode;

/// A list of possible AST expressions.
#[derive(Clone, Debug)]
pub enum Ast {

    NullLiteral,

    /// A boolean literal, either `true` or `false`.
    BooleanLiteral(bool),

    /// A 64-bit floating point number literal.
    FloatLiteral(f64),

    /// A 64-bit integer literal.
    IntegerLiteral(i64),

    /// An identifier literal.
    IdentifierLiteral(String),

    /// A string literal.
    StringLiteral(String),

    /// A literal array of items, such as `[item1, 2, 3, true, false]`.
    ListLiteral(Vec<AstMeta>),

    /// A function call with the given arguments.
    FunctionCall(Box<AstMeta>, Vec<AstMeta>),

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

    /// Negates a logical expression, the `!` operator.  For example, (`!1 == 1`).
    NotUnary(Box<AstMeta>),

    /// A preprocessor statement with a given name and arguments.
    PreprocessorStatement(Box<AstMeta>, Vec<AstMeta>),

}