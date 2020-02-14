//! Infrastructure of lifting the data representation (DR) into structured
//! representation (SR).

mod storage;

use self::storage::LiftStorage;
use crate::{
    dr,
    sr::{
        instructions,
        module,
        ops,
        storage::Token,
        Constant, ConstEnum, StructMember, Type, TypeEnum,
    },
};

use spirv;

use std::{borrow::Borrow, mem};

/// A structure that we associate an <id> with, containing
/// both the operation token and the result type.
struct OpInfo {
    op: Token<module::Operation>,
    ty: Option<Token<Type>>,
}

impl Borrow<Token<module::Operation>> for OpInfo {
    fn borrow(&self) -> &Token<module::Operation> {
        &self.op
    }
}

pub struct LiftContext {
    //current_block: Option<Token<module::Block>>,
    types: LiftStorage<Type>,
    constants: LiftStorage<Constant>,
    blocks: LiftStorage<module::Block>,
    ops: LiftStorage<module::Operation, OpInfo>,
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
                        context.types.append_id(id, Type {
                            raw: value,
                            name: String::new(),
                        });
                    }
                    continue
                }
                Err(InstructionError::WrongOpcode) => {},
                Err(e) => panic!("Type lift error: {:?}", e),
            }
            match context.lift_constant(inst) {
                Ok(value) => {
                    if let Some(id) = inst.result_id {
                        context.constants.append_id(id, Constant {
                            raw: value,
                            name: String::new(),
                        });
                    }
                    continue
                }
                Err(InstructionError::WrongOpcode) => {},
                Err(e) => panic!("Constant lift error: {:?}", e),
            }
        }

        for fun in module.functions.iter() {
            let def = context.lift_function(
                fun.def
                    .as_ref()
                    .ok_or(ConversionError::MissingFunction)?
            )?;

            for block in fun.blocks.iter() {
                let mut arguments = Vec::new();
                let mut ops = Vec::new();
                if block.instructions.is_empty() {
                    return Err(ConversionError::MissingTerminator);
                }

                for inst in &block.instructions[.. block.instructions.len() - 1] {
                    match inst.class.opcode {
                        spirv::Op::Line => {} // skip line decorations
                        spirv::Op::Phi => {
                            match inst.operands[0] {
                                dr::Operand::IdRef(id) => {
                                    let (_, info) = context.ops.lookup(id);
                                    arguments.push(info.ty
                                        .ok_or(InstructionError::MissingResult)?
                                    );
                                }
                                _ => return Err(ConversionError::Instruction(
                                    InstructionError::Operand(OperandError::Missing)
                                )),
                            };
                        }
                        spirv::Op::Name => {
                            let target = match inst.operands[0] {
                                dr::Operand::IdRef(id) => id,
                                _ => return Err(ConversionError::Instruction(
                                    InstructionError::Operand(OperandError::Missing)
                                )),
                            };
                            let name = match inst.operands[1] {
                                dr::Operand::LiteralString(ref name) => name.clone(),
                                _ => return Err(ConversionError::Instruction(
                                    InstructionError::Operand(OperandError::Missing)
                                )),
                            };
                            if let Some(op) = context.ops.try_lookup_mut(target) {
                                op.name = name;
                            } else
                            if let Some(ty) = context.types.try_lookup_mut(target) {
                                ty.name = name;
                            } else
                            if let Some(con) = context.constants.try_lookup_mut(target) {
                                con.name = name;
                            } else {
                                //log::warn!("Unable to assign name {} -> {}", target, name);
                            }
                        }
                        spirv::Op::MemberName => {
                            let target = match inst.operands[0] {
                                dr::Operand::IdRef(id) => id,
                                _ => return Err(ConversionError::Instruction(
                                    InstructionError::Operand(OperandError::Missing)
                                )),
                            };
                            let member = match inst.operands[1] {
                                dr::Operand::LiteralInt32(index) => index,
                                _ => return Err(ConversionError::Instruction(
                                    InstructionError::Operand(OperandError::Missing)
                                )),
                            };
                            let name = match inst.operands[2] {
                                dr::Operand::LiteralString(ref name) => name.clone(),
                                _ => return Err(ConversionError::Instruction(
                                    InstructionError::Operand(OperandError::Missing)
                                )),
                            };
                            match context.types.try_lookup_mut(target) {
                                Some(Type { raw: TypeEnum::Struct { ref mut members }, .. }) => {
                                    match members.get_mut(member as usize) {
                                        Some(sm) => {
                                            sm.name = name;
                                        }
                                        None => {
                                            return Err(ConversionError::Instruction(
                                                InstructionError::Operand(OperandError::WrongType)
                                            ));
                                        }
                                    }
                                }
                                Some(_) => {
                                    return Err(ConversionError::Instruction(
                                        InstructionError::Operand(OperandError::WrongType)
                                    ));
                                }
                                None => {
                                    //TODO: better error here
                                    return Err(ConversionError::Instruction(
                                        InstructionError::Operand(OperandError::Missing)
                                    ));
                                }
                            }
                        }
                        _ => {
                            let op = module::Operation {
                                raw: context.lift_op(inst)?,
                                name: String::new(),
                            };
                            let token = match inst.result_id {
                                Some(id) => {
                                    let types = &context.types;
                                    let (token, entry) = context.ops.append(id, op);
                                    entry.insert(OpInfo {
                                        op: token,
                                        ty: inst.result_type.map(|ty| *types.lookup(ty).1),
                                    });
                                    token
                                }
                                None => {
                                    context.ops.append_noid(op)
                                }
                            };
                            ops.push(token);
                        }
                    }
                }

                let terminator = context.lift_terminator(
                    block.instructions
                        .last()
                        .ok_or(ConversionError::MissingTerminator)?
                )?;

                context.blocks.append_id(
                    block.label.as_ref().unwrap().result_id.unwrap(),
                    module::Block {
                        arguments,
                        ops,
                        terminator,
                    },
                );
            }

            let start_label = fun.blocks[0].label.as_ref().unwrap().result_id.unwrap();
            let start_block = context.blocks.lookup_token(start_label);
            let blocks = mem::replace(&mut context.blocks, LiftStorage::new())
                .unwrap();

            functions.push(module::Function {
                control: def.function_control,
                result: context.types.lookup_token(def.function_type),
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
            capabilities: module.capabilities
                .iter()
                .map(|cap| context.lift_capability(cap).map(|cap| cap.capability))
                .collect::<Result<_, InstructionError>>()?,
            extensions: module.extensions
                .iter()
                .map(|ext| context.lift_extension(ext).map(|ext| ext.name))
                .collect::<Result<_, InstructionError>>()?,
            ext_inst_imports: module.extensions
                .iter()
                .map(|ext| context.lift_ext_inst_import(ext).map(|ext| ext.name))
                .collect::<Result<_, InstructionError>>()?,
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

    fn lift_constant(
        &self, inst: &dr::Instruction
    ) -> Result<ConstEnum, InstructionError> {
        match inst.class.opcode {
            spirv::Op::ConstantTrue => Ok(ConstEnum::Bool(true)),
            spirv::Op::ConstantFalse => Ok(ConstEnum::Bool(false)),
            spirv::Op::Constant => {
                match inst.result_type {
                    Some(id) => {
                        let oper = inst.operands
                            .first()
                            .ok_or(InstructionError::Operand(OperandError::Missing))?;
                        let (value, width) = match self.types.lookup(id).0.raw {
                            TypeEnum::Int { signedness: 0, width } => match *oper {
                                dr::Operand::LiteralInt32(v) => (ConstEnum::UInt(v), width),
                                _ => return Err(InstructionError::Operand(OperandError::WrongType)),
                            },
                            TypeEnum::Int { width, .. } => match *oper {
                                dr::Operand::LiteralInt32(v) => (ConstEnum::Int(v as i32), width),
                                _ => return Err(InstructionError::Operand(OperandError::WrongType)),
                            },
                            TypeEnum::Float { width } => match *oper {
                                dr::Operand::LiteralFloat32(v) => (ConstEnum::Float(v), width),
                                _ => return Err(InstructionError::Operand(OperandError::WrongType)),
                            },
                            _ => return Err(InstructionError::MissingResult),
                        };
                        if width > 32 {
                            //log::warn!("Constant <id> {} doesn't fit in 32 bits", id);
                        }
                        Ok(value)
                    }
                    _ => return Err(InstructionError::MissingResult),
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
                Ok(ConstEnum::Composite(vec))
            }
            spirv::Op::ConstantSampler => {
                if inst.operands.len() < 3 {
                    return Err(InstructionError::Operand(OperandError::Missing))
                }
                Ok(ConstEnum::Sampler {
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
            spirv::Op::ConstantNull => Ok(ConstEnum::Null),
            _ => Err(InstructionError::WrongOpcode)
        }
    }
}
