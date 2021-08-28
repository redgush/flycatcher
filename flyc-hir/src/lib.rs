//! This crate provides tools for HIR generation from AST trees.
//!
//! Though, this crate is not the crate that lowers an AST tree into Flycatcher HIR, it just provides
//! tools for doing so.

pub mod construct;
pub mod func;
pub mod meta;
pub mod module;
pub mod symbol;

/// The main Flycatcher HIR items that are used by the compiler.
#[derive(Clone, Debug, PartialEq)]
pub enum Hir {}
