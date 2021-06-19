//! Infrastructure of lifting the data representation (DR) into structured
//! representation (SR).

mod storage;

use self::storage::LiftStorage;
use crate::{
    dr,
    sr::{instructions, module, ops, storage::Token, Constant, StructMember, Type},
};

use std::{borrow::Borrow, mem};

/// A structure that we associate an <id> with, containing
/// both the operation token and the result type.
struct OpInfo {
    op: Token<ops::Op>,
    ty: Option<Token<Type>>,
}

impl Borrow<Token<ops::Op>> for OpInfo {
    fn borrow(&self) -> &Token<ops::Op> {
        &self.op
    }
}

pub struct LiftContext {
    //current_block: Option<Token<module::Block>>,
    types: LiftStorage<Type>,
    constants: LiftStorage<Constant>,
    blocks: LiftStorage<module::Block>,
    ops: LiftStorage<ops::Op, OpInfo>,
}

include!("autogen_context.rs");

/// Error lifting a data representation of an operand into the structured
/// representation.
#[derive(Clone, Debug)]
pub enum OperandError {
    /// Operand has a wrong type.
    WrongType,
    /// Operand is an integer value that corresponds to a specified enum,
    /// but the given integer is not known to have a mapping.
    WrongEnumValue,
    /// Operand is missing from the list.
    Missing,
}

/// Error lifting a data representation of an instruction.
#[derive(Clone, Debug)]
pub enum InstructionError {
    /// Instruction has a wrong opcode.
    WrongOpcode,
    /// Instruction is missing a result <id> or type.
    MissingResult,
    /// One of the operands can not be lifted.
    Operand(OperandError),
}

impl From<OperandError> for InstructionError {
    fn from(error: OperandError) -> Self {
        InstructionError::Operand(error)
    }
}

/// Error that may occur during the convesion from the data representation
/// of a module into a structured representation.
#[derive(Clone, Debug)]
pub enum ConversionError {
    MissingHeader,
    MissingFunction,
    MissingFunctionType,
    MissingLabel,
    MissingTerminator,
    Instruction(InstructionError),
}

impl From<InstructionError> for ConversionError {
    fn from(error: InstructionError) -> Self {
        ConversionError::Instruction(error)
    }
}

impl LiftContext {
    /// Convert a module from the data representation into structured representation.
    pub fn convert(module: &dr::Module) -> Result<module::Module, ConversionError> {
        let mut context = LiftContext {
            types: LiftStorage::new(),
            constants: LiftStorage::new(),
            blocks: LiftStorage::new(),
            ops: LiftStorage::new(),
        };
        let mut functions = Vec::new();
        let entry_points = Vec::new();

        for inst in module.types_global_values.iter() {
            match context.lift_type(inst) {
                Ok(value) => {
                    if let Some(id) = inst.result_id {
                        context.types.append_id(id, value);
                    }
                    continue;
                }
                Err(InstructionError::WrongOpcode) => {}
                Err(e) => panic!("Type lift error: {:?}", e),
            }
            match context.lift_constant(inst) {
                Ok(value) => {
                    if let Some(id) = inst.result_id {
                        context.constants.append_id(id, value);
                    }
                    continue;
                }
                Err(InstructionError::WrongOpcode) => {}
                Err(e) => panic!("Constant lift error: {:?}", e),
            }
        }

        for fun in module.functions.iter() {
            let def =
                context.lift_function(fun.def.as_ref().ok_or(ConversionError::MissingFunction)?)?;
            //TODO: lift function type instruction

            for block in fun.blocks.iter() {
                let mut arguments = Vec::new();
                for inst in &block.instructions {
                    match inst.class.opcode {
                        spirv::Op::Line => {} // skip line decorations
                        spirv::Op::Phi => {
                            let ty = context.types.lookup_token(
                                inst.result_type.ok_or(InstructionError::MissingResult)?,
                            );
                            arguments.push(ty);

                            // Sanity-check if all source variables are of the same type
                            for op in inst.operands.iter().step_by(2) {
                                match op {
                                    dr::Operand::IdRef(id) => {
                                        if let Some((_, info)) = context.ops.lookup_safe(*id) {
                                            assert_eq!(Some(ty), info.ty);
                                        } else {
                                            // let (v, info) =
                                            //     context.constants.lookup_safe(*id).unwrap();
                                            // TODO: Can't convert Constant back to their lowered type yet!
                                            // assert_eq!(Some(ty), info.ty.as_ref());
                                        }
                                    }
                                    _ => {
                                        return Err(ConversionError::Instruction(
                                            InstructionError::Operand(OperandError::Missing),
                                        ))
                                    }
                                };
                            }
                        }
                        _ => {
                            if let Some(id) = inst.result_id {
                                let op = context.lift_op(inst)?;
                                let types = &context.types;
                                let (token, entry) = context.ops.append(id, op);
                                entry.insert(OpInfo {
                                    op: token,
                                    ty: inst.result_type.map(|ty| *types.lookup(ty).1),
                                });
                            }
                        }
                    }
                }

                let terminator = context.lift_terminator(
                    block
                        .instructions
                        .last()
                        .ok_or(ConversionError::MissingTerminator)?,
                )?;

                context.blocks.append_id(
                    block.label.as_ref().unwrap().result_id.unwrap(),
                    module::Block {
                        arguments,
                        ops: Vec::new(),
                        terminator,
                    },
                );
            }

            let start_label = fun.blocks[0].label.as_ref().unwrap().result_id.unwrap();
            let start_block = context.blocks.lookup_token(start_label);
            let blocks = mem::replace(&mut context.blocks, LiftStorage::new()).unwrap();

            functions.push(module::Function {
                control: def.function_control,
                result: context.types.append_id(1, Type::Void), //TODO: fty.return_type,
                parameters: Vec::new(),
                blocks,
                start_block,
            });
        }

        Ok(module::Module {
            version: match module.header {
                Some(ref header) => header.version,
                None => return Err(ConversionError::MissingHeader),
            },
            capabilities: module
                .capabilities
                .iter()
                .map(|cap| context.lift_capability(cap).map(|cap| cap.capability))
                .collect::<Result<_, InstructionError>>()?,
            extensions: Vec::new(),
            ext_inst_imports: Vec::new(),
            memory_model: match module.memory_model {
                Some(ref mm) => context.lift_memory_model(mm)?,
                None => return Err(ConversionError::MissingHeader),
            },
            entry_points,
            types: context.types.unwrap(),
            constants: context.constants.unwrap(),
            ops: context.ops.unwrap(),
            functions,
        })
    }

    fn lookup_jump(&self, destination: spirv::Word) -> module::Jump {
        let (_, block) = self.blocks.lookup(destination);
        module::Jump {
            block: *block,
            arguments: Vec::new(), //TODO
        }
    }

    fn lift_constant(&self, inst: &dr::Instruction) -> Result<Constant, InstructionError> {
        match inst.class.opcode {
            spirv::Op::ConstantTrue => Ok(Constant::Bool(true)),
            spirv::Op::ConstantFalse => Ok(Constant::Bool(false)),
            spirv::Op::Constant => {
                match inst.result_type {
                    Some(id) => {
                        let oper = inst
                            .operands
                            .first()
                            .ok_or(InstructionError::Operand(OperandError::Missing))?;
                        let (value, width) = match *self.types.lookup(id).0 {
                            Type::Int {
                                signedness: 0,
                                width,
                            } => match *oper {
                                dr::Operand::LiteralInt32(v) => (Constant::UInt(v), width),
                                _ => {
                                    return Err(InstructionError::Operand(OperandError::WrongType))
                                }
                            },
                            Type::Int { width, .. } => match *oper {
                                dr::Operand::LiteralInt32(v) => (Constant::Int(v as i32), width),
                                _ => {
                                    return Err(InstructionError::Operand(OperandError::WrongType))
                                }
                            },
                            Type::Float { width } => match *oper {
                                dr::Operand::LiteralFloat32(v) => (Constant::Float(v), width),
                                _ => {
                                    return Err(InstructionError::Operand(OperandError::WrongType))
                                }
                            },
                            _ => return Err(InstructionError::MissingResult),
                        };
                        if width > 32 {
                            //log::warn!("Constant <id> {} doesn't fit in 32 bits", id);
                        }
                        Ok(value)
                    }
                    _ => Err(InstructionError::MissingResult),
                }
            }
            spirv::Op::ConstantComposite => {
                let mut vec = Vec::with_capacity(inst.operands.len());
                for oper in inst.operands.iter() {
                    let token = match *oper {
                        dr::Operand::IdRef(v) => self.constants.lookup_token(v),
                        _ => return Err(InstructionError::Operand(OperandError::WrongType)),
                    };
                    vec.push(token);
                }
                Ok(Constant::Composite(vec))
            }
            spirv::Op::ConstantSampler => {
                if inst.operands.len() < 3 {
                    return Err(InstructionError::Operand(OperandError::Missing));
                }
                Ok(Constant::Sampler {
                    addressing_mode: match inst.operands[0] {
                        dr::Operand::SamplerAddressingMode(v) => v,
                        _ => return Err(InstructionError::Operand(OperandError::WrongType)),
                    },
                    normalized: match inst.operands[1] {
                        dr::Operand::LiteralInt32(v) => v != 0,
                        _ => return Err(InstructionError::Operand(OperandError::WrongType)),
                    },
                    filter_mode: match inst.operands[2] {
                        dr::Operand::SamplerFilterMode(v) => v,
                        _ => return Err(InstructionError::Operand(OperandError::WrongType)),
                    },
                })
            }
            spirv::Op::ConstantNull => Ok(Constant::Null),
            _ => Err(InstructionError::WrongOpcode),
        }
    }
}
