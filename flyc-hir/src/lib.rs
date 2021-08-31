//! Crate for generating Flycatcher HIR.

mod access;
mod construct;
mod func;
mod meta;
mod module;

pub use access::Access;
pub use construct::HirConstruct;
pub use func::{HirFunction, HirFunctionArgument};
pub use meta::HirMeta;
pub use module::HirModule;

/// The Flycatcher HIR instructions that are possible.
#[derive(Clone, Debug, PartialEq)]
pub enum Hir {

}