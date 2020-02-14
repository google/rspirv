use crate::spirv;

use super::storage::Token;

/// Represents a SPIR-V constant.
#[derive(Clone, Debug, PartialEq)]
pub enum ConstEnum {
    Bool(bool),
    // TODO: Change to arbitrary width datatype
    Int(i32),
    UInt(u32),
    Float(f32),
    Composite(Vec<Token<Constant>>),
    Null,
    Sampler {
        addressing_mode: spirv::SamplerAddressingMode,
        normalized: bool,
        filter_mode: spirv::SamplerFilterMode,
    },
    SpecBool(bool),
    SpecInt(i32),
    SpecUInt(u32),
    SpecFloat(f32),
    SpecComposite(Vec<Token<Constant>>),
    SpecOp(spirv::Op, Vec<Token<Constant>>),
}

pub struct Constant {
    pub raw: ConstEnum,
    pub name: String,
}

impl ConstEnum {
    pub fn is_bool_constant(&self) -> bool {
        match self {
            ConstEnum::Bool { .. } |
            ConstEnum::SpecBool { .. } => true,
            _ => false,
        }
    }

    pub fn is_int_constant(&self) -> bool {
        match self {
            ConstEnum::Int { .. } |
            ConstEnum::SpecInt { .. } => true,
            _ => false,
        }
    }

    pub fn is_uint_constant(&self) -> bool {
        match self {
            ConstEnum::UInt { .. } |
            ConstEnum::SpecUInt { .. } => true,
            _ => false,
        }
    }

    pub fn is_float_constant(&self) -> bool {
        match self {
            ConstEnum::Float { .. } |
            ConstEnum::SpecFloat { .. } => true,
            _ => false,
        }
    }

    pub fn is_composite_constant(&self) -> bool {
        match self {
            ConstEnum::Composite { .. } |
            ConstEnum::SpecComposite { .. } => true,
            _ => false,
        }
    }

    pub fn is_null_constant(&self) -> bool {
        match self {
            ConstEnum::Null { .. } => true,
            _ => false,
        }
    }

    pub fn is_sampler_constant(&self) -> bool {
        match self {
            ConstEnum::Sampler { .. } => true,
            _ => false,
        }
    }

    pub fn is_spec_constant(&self) -> bool {
        match self {
            ConstEnum::SpecBool { .. } |
            ConstEnum::SpecInt { .. } |
            ConstEnum::SpecUInt { .. } |
            ConstEnum::SpecFloat { .. } |
            ConstEnum::SpecComposite { .. } |
            ConstEnum::SpecOp { .. } => true,
            _ => false,
        }
    }

    pub fn is_spec_op_constant(&self) -> bool {
        match self {
            ConstEnum::SpecOp { .. } => true,
            _ => false,
        }
    }
}
