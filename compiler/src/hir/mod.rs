//! Exposes the high-level intermediate representation (the HIR).

pub mod meta;

use crate::{FlycatcherType, SymbolTable, VariableType};
pub use meta::HirMeta;

/// This is the high-level intermediate representation used by Flycatcher's compiler front end.
/// The HIR allows type verification and various other safety checks that may be implemented
/// before the flycatcher source is translated into a lower level intermediate representation,
/// and or a machine code binary.
#[derive(Clone, Debug)]
pub enum Hir<'a> {

    /// A boolean value that may either be `true` or `false`.
    Boolean(bool),

    /// A 64-bit signed integer.
    Integer(i64),

    /// A 64-bit unsigned integer.
    UnsignedInteger(u64),

    /// A 64-bit floating point number.
    Float(f64),

    /// A reference to a named variable value.
    Named(String),

    /// Adds two HIR objects together, regardless of their type, as long as the first type
    /// supports addition with the other type.
    Add(Box<HirMeta<'a>>, Box<HirMeta<'a>>),

    /// Subtracts two HIR objects together, regardless of their type, as long as the first type
    /// supports subtraction with the other type.
    Subtract(Box<HirMeta<'a>>, Box<HirMeta<'a>>),

    /// Multiplies two HIR objects together, regardless of their type, as long as the first type
    /// supports multiplication with the other type.
    Multiply(Box<HirMeta<'a>>, Box<HirMeta<'a>>),

    /// Divides two HIR objects together, regardless of their type, as long as the first type
    /// supports division with the other type.
    Divide(Box<HirMeta<'a>>, Box<HirMeta<'a>>),

}

impl<'a> Hir<'a> {

    /// Gets the default type of the current HIR object.  If the HIR object is a `Named` value,
    /// it will use the symbol table to find what the type of the variable is.
    pub fn get_type(&self, symbols: &SymbolTable) -> FlycatcherType {
        match self {
            Hir::Boolean(_) => FlycatcherType::Boolean,
            Hir::Integer(_) => FlycatcherType::Size,
            Hir::UnsignedInteger(_) => FlycatcherType::Usize,
            Hir::Float(_) => FlycatcherType::Float64,
            Hir::Named(n) => {
                let v = symbols.get(n).unwrap();
                match v {
                    VariableType::Declared(t) => *t,
                    VariableType::Defined(t, _, _) => *t
                }
            },
            Hir::Add(l, r) => l.item.get_type(symbols),
            Hir::Subtract(l, r) => l.item.get_type(symbols),
            Hir::Multiply(l, r) => l.item.get_type(symbols),
            Hir::Divide(l, r) => l.item.get_type(symbols),
        }
    }

}