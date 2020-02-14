//! **S**tructured **r**epresentation of various SPIR-V language constructs.

pub use self::autogen_decoration::Decoration;
pub use self::autogen_instructions as instructions;
pub use self::autogen_ops as ops;
pub use self::constants::{Constant, ConstEnum};
pub use self::types::{StructMember, Type, TypeEnum};

mod autogen_decoration;
pub mod autogen_instructions;
pub mod autogen_ops;
mod constants;
pub mod module;
pub mod storage;
mod types;
