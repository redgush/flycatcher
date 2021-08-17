//! Tools for generating AST trees.

pub mod opcode;

pub use opcode::Opcode;

/// A list of possible AST expressions.
#[derive(Debug)]
pub enum Ast {

    /// A binary expression with two operands.
    /// 
    /// ```flycatcher
    /// 1 + 1
    /// ```
    BinaryExpression(Opcode, Box<Ast>, Box<Ast>),

}