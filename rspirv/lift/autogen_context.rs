// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

impl LiftContext {
    pub fn lift_branch(&mut self, raw: &dr::Instruction) -> Result<ops::Branch, InstructionError> {
        let mut operands = raw.operands.iter();
        match raw.class.opcode as u32 {
            249u32 => Ok(ops::Branch::Branch {
                target_label: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            250u32 => Ok(ops::Branch::BranchConditional {
                condition: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                true_label: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                false_label: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                branch_weights: {
                    let mut vec = Vec::new();
                    while let Some(item) = match operands.next() {
                        Some(&dr::Operand::LiteralInt32(ref value)) => Some(*value),
                        Some(_) => Err(OperandError::WrongType)?,
                        None => None,
                    } {
                        vec.push(item);
                    }
                    vec
                },
            }),
            251u32 => Ok(ops::Branch::Switch {
                selector: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                default: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                target: {
                    let mut vec = Vec::new();
                    while let Some(item) = match (operands.next(), operands.next()) {
                        (
                            Some(&dr::Operand::LiteralInt32(first)),
                            Some(&dr::Operand::IdRef(second)),
                        ) => Some((first, self.lookup_jump(second))),
                        (None, None) => None,
                        _ => Err(OperandError::WrongType)?,
                    } {
                        vec.push(item);
                    }
                    vec
                },
            }),
            253u32 => Ok(ops::Branch::Return),
            254u32 => Ok(ops::Branch::ReturnValue {
                value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            _ => Err(InstructionError::WrongOpcode),
        }
    }
    pub fn lift_terminator(
        &mut self,
        raw: &dr::Instruction,
    ) -> Result<ops::Terminator, InstructionError> {
        match raw.class.opcode as u32 {
            252u32 => Ok(ops::Terminator::Kill),
            255u32 => Ok(ops::Terminator::Unreachable),
            _ => self.lift_branch(raw).map(ops::Terminator::Branch),
        }
    }
    pub fn lift_op(&mut self, raw: &dr::Instruction) -> Result<ops::Op, InstructionError> {
        let mut operands = raw.operands.iter();
        match raw.class.opcode as u32 {
            0u32 => Ok(ops::Op::Nop),
            1u32 => Ok(ops::Op::Undef),
            2u32 => Ok(ops::Op::SourceContinued {
                continued_source: (match operands.next() {
                    Some(&dr::Operand::LiteralString(ref value)) => Some(value.clone()),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            3u32 => Ok(ops::Op::Source {
                source_language: (match operands.next() {
                    Some(&dr::Operand::SourceLanguage(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                version: (match operands.next() {
                    Some(&dr::Operand::LiteralInt32(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                file: match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                },
                source: match operands.next() {
                    Some(&dr::Operand::LiteralString(ref value)) => Some(value.clone()),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                },
            }),
            4u32 => Ok(ops::Op::SourceExtension {
                extension: (match operands.next() {
                    Some(&dr::Operand::LiteralString(ref value)) => Some(value.clone()),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            7u32 => Ok(ops::Op::String {
                string: (match operands.next() {
                    Some(&dr::Operand::LiteralString(ref value)) => Some(value.clone()),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            8u32 => Ok(ops::Op::Line {
                file: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                line: (match operands.next() {
                    Some(&dr::Operand::LiteralInt32(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                column: (match operands.next() {
                    Some(&dr::Operand::LiteralInt32(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            12u32 => Ok(ops::Op::ExtInst {
                set: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                instruction: (match operands.next() {
                    Some(&dr::Operand::LiteralExtInstInteger(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_1_operand_2: {
                    let mut vec = Vec::new();
                    while let Some(item) = match operands.next() {
                        Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                        Some(_) => Err(OperandError::WrongType)?,
                        None => None,
                    } {
                        vec.push(item);
                    }
                    vec
                },
            }),
            57u32 => Ok(ops::Op::FunctionCall {
                function: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                argument_0_argument_1: {
                    let mut vec = Vec::new();
                    while let Some(item) = match operands.next() {
                        Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                        Some(_) => Err(OperandError::WrongType)?,
                        None => None,
                    } {
                        vec.push(item);
                    }
                    vec
                },
            }),
            59u32 => Ok(ops::Op::Variable {
                storage_class: (match operands.next() {
                    Some(&dr::Operand::StorageClass(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                initializer: match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                },
            }),
            60u32 => Ok(ops::Op::ImageTexelPointer {
                image: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                sample: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            61u32 => Ok(ops::Op::Load {
                pointer: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory_access: match operands.next() {
                    Some(&dr::Operand::MemoryAccess(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                },
            }),
            62u32 => Ok(ops::Op::Store {
                pointer: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                object: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory_access: match operands.next() {
                    Some(&dr::Operand::MemoryAccess(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                },
            }),
            63u32 => Ok(ops::Op::CopyMemory {
                target: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                source: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory_access: match operands.next() {
                    Some(&dr::Operand::MemoryAccess(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                },
            }),
            64u32 => Ok(ops::Op::CopyMemorySized {
                target: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                source: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                size: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory_access: match operands.next() {
                    Some(&dr::Operand::MemoryAccess(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                },
            }),
            65u32 => Ok(ops::Op::AccessChain {
                base: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                indexes: {
                    let mut vec = Vec::new();
                    while let Some(item) = match operands.next() {
                        Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                        Some(_) => Err(OperandError::WrongType)?,
                        None => None,
                    } {
                        vec.push(item);
                    }
                    vec
                },
            }),
            66u32 => Ok(ops::Op::InBoundsAccessChain {
                base: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                indexes: {
                    let mut vec = Vec::new();
                    while let Some(item) = match operands.next() {
                        Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                        Some(_) => Err(OperandError::WrongType)?,
                        None => None,
                    } {
                        vec.push(item);
                    }
                    vec
                },
            }),
            67u32 => Ok(ops::Op::PtrAccessChain {
                base: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                element: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                indexes: {
                    let mut vec = Vec::new();
                    while let Some(item) = match operands.next() {
                        Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                        Some(_) => Err(OperandError::WrongType)?,
                        None => None,
                    } {
                        vec.push(item);
                    }
                    vec
                },
            }),
            68u32 => Ok(ops::Op::ArrayLength {
                structure: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                array_member: (match operands.next() {
                    Some(&dr::Operand::LiteralInt32(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            69u32 => Ok(ops::Op::GenericPtrMemSemantics {
                pointer: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            70u32 => Ok(ops::Op::InBoundsPtrAccessChain {
                base: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                element: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                indexes: {
                    let mut vec = Vec::new();
                    while let Some(item) = match operands.next() {
                        Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                        Some(_) => Err(OperandError::WrongType)?,
                        None => None,
                    } {
                        vec.push(item);
                    }
                    vec
                },
            }),
            71u32 => Ok(ops::Op::Decorate {
                target: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                decoration: (match operands.next() {
                    Some(&dr::Operand::Decoration(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            72u32 => Ok(ops::Op::MemberDecorate {
                structure_type: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(self.types.lookup_token(*value)),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                member: (match operands.next() {
                    Some(&dr::Operand::LiteralInt32(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                decoration: (match operands.next() {
                    Some(&dr::Operand::Decoration(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            73u32 => Ok(ops::Op::DecorationGroup),
            74u32 => Ok(ops::Op::GroupDecorate {
                decoration_group: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                targets: {
                    let mut vec = Vec::new();
                    while let Some(item) = match operands.next() {
                        Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                        Some(_) => Err(OperandError::WrongType)?,
                        None => None,
                    } {
                        vec.push(item);
                    }
                    vec
                },
            }),
            75u32 => Ok(ops::Op::GroupMemberDecorate {
                decoration_group: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                targets: {
                    let mut vec = Vec::new();
                    while let Some(item) = match (operands.next(), operands.next()) {
                        (
                            Some(&dr::Operand::IdRef(first)),
                            Some(&dr::Operand::LiteralInt32(second)),
                        ) => Some((self.lookup_jump(first), second)),
                        (None, None) => None,
                        _ => Err(OperandError::WrongType)?,
                    } {
                        vec.push(item);
                    }
                    vec
                },
            }),
            77u32 => Ok(ops::Op::VectorExtractDynamic {
                vector: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                index: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            78u32 => Ok(ops::Op::VectorInsertDynamic {
                vector: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                component: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                index: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            79u32 => Ok(ops::Op::VectorShuffle {
                vector_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                vector_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                components: {
                    let mut vec = Vec::new();
                    while let Some(item) = match operands.next() {
                        Some(&dr::Operand::LiteralInt32(ref value)) => Some(*value),
                        Some(_) => Err(OperandError::WrongType)?,
                        None => None,
                    } {
                        vec.push(item);
                    }
                    vec
                },
            }),
            80u32 => Ok(ops::Op::CompositeConstruct {
                constituents: {
                    let mut vec = Vec::new();
                    while let Some(item) = match operands.next() {
                        Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                        Some(_) => Err(OperandError::WrongType)?,
                        None => None,
                    } {
                        vec.push(item);
                    }
                    vec
                },
            }),
            81u32 => Ok(ops::Op::CompositeExtract {
                composite: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                indexes: {
                    let mut vec = Vec::new();
                    while let Some(item) = match operands.next() {
                        Some(&dr::Operand::LiteralInt32(ref value)) => Some(*value),
                        Some(_) => Err(OperandError::WrongType)?,
                        None => None,
                    } {
                        vec.push(item);
                    }
                    vec
                },
            }),
            82u32 => Ok(ops::Op::CompositeInsert {
                object: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                composite: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                indexes: {
                    let mut vec = Vec::new();
                    while let Some(item) = match operands.next() {
                        Some(&dr::Operand::LiteralInt32(ref value)) => Some(*value),
                        Some(_) => Err(OperandError::WrongType)?,
                        None => None,
                    } {
                        vec.push(item);
                    }
                    vec
                },
            }),
            83u32 => Ok(ops::Op::CopyObject {
                operand: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            84u32 => Ok(ops::Op::Transpose {
                matrix: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            86u32 => Ok(ops::Op::SampledImage {
                image: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                sampler: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            87u32 => Ok(ops::Op::ImageSampleImplicitLod {
                sampled_image: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(&dr::Operand::ImageOperands(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                },
            }),
            88u32 => Ok(ops::Op::ImageSampleExplicitLod {
                sampled_image: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: (match operands.next() {
                    Some(&dr::Operand::ImageOperands(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            89u32 => Ok(ops::Op::ImageSampleDrefImplicitLod {
                sampled_image: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                d_ref: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(&dr::Operand::ImageOperands(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                },
            }),
            90u32 => Ok(ops::Op::ImageSampleDrefExplicitLod {
                sampled_image: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                d_ref: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: (match operands.next() {
                    Some(&dr::Operand::ImageOperands(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            91u32 => Ok(ops::Op::ImageSampleProjImplicitLod {
                sampled_image: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(&dr::Operand::ImageOperands(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                },
            }),
            92u32 => Ok(ops::Op::ImageSampleProjExplicitLod {
                sampled_image: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: (match operands.next() {
                    Some(&dr::Operand::ImageOperands(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            93u32 => Ok(ops::Op::ImageSampleProjDrefImplicitLod {
                sampled_image: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                d_ref: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(&dr::Operand::ImageOperands(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                },
            }),
            94u32 => Ok(ops::Op::ImageSampleProjDrefExplicitLod {
                sampled_image: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                d_ref: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: (match operands.next() {
                    Some(&dr::Operand::ImageOperands(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            95u32 => Ok(ops::Op::ImageFetch {
                image: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(&dr::Operand::ImageOperands(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                },
            }),
            96u32 => Ok(ops::Op::ImageGather {
                sampled_image: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                component: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(&dr::Operand::ImageOperands(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                },
            }),
            97u32 => Ok(ops::Op::ImageDrefGather {
                sampled_image: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                d_ref: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(&dr::Operand::ImageOperands(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                },
            }),
            98u32 => Ok(ops::Op::ImageRead {
                image: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(&dr::Operand::ImageOperands(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                },
            }),
            99u32 => Ok(ops::Op::ImageWrite {
                image: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                texel: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(&dr::Operand::ImageOperands(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                },
            }),
            100u32 => Ok(ops::Op::Image {
                sampled_image: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            101u32 => Ok(ops::Op::ImageQueryFormat {
                image: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            102u32 => Ok(ops::Op::ImageQueryOrder {
                image: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            103u32 => Ok(ops::Op::ImageQuerySizeLod {
                image: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                level_of_detail: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            104u32 => Ok(ops::Op::ImageQuerySize {
                image: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            105u32 => Ok(ops::Op::ImageQueryLod {
                sampled_image: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            106u32 => Ok(ops::Op::ImageQueryLevels {
                image: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            107u32 => Ok(ops::Op::ImageQuerySamples {
                image: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            109u32 => Ok(ops::Op::ConvertFToU {
                float_value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            110u32 => Ok(ops::Op::ConvertFToS {
                float_value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            111u32 => Ok(ops::Op::ConvertSToF {
                signed_value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            112u32 => Ok(ops::Op::ConvertUToF {
                unsigned_value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            113u32 => Ok(ops::Op::UConvert {
                unsigned_value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            114u32 => Ok(ops::Op::SConvert {
                signed_value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            115u32 => Ok(ops::Op::FConvert {
                float_value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            116u32 => Ok(ops::Op::QuantizeToF16 {
                value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            117u32 => Ok(ops::Op::ConvertPtrToU {
                pointer: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            118u32 => Ok(ops::Op::SatConvertSToU {
                signed_value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            119u32 => Ok(ops::Op::SatConvertUToS {
                unsigned_value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            120u32 => Ok(ops::Op::ConvertUToPtr {
                integer_value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            121u32 => Ok(ops::Op::PtrCastToGeneric {
                pointer: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            122u32 => Ok(ops::Op::GenericCastToPtr {
                pointer: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            123u32 => Ok(ops::Op::GenericCastToPtrExplicit {
                pointer: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                storage: (match operands.next() {
                    Some(&dr::Operand::StorageClass(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            124u32 => Ok(ops::Op::Bitcast {
                operand: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            126u32 => Ok(ops::Op::SNegate {
                operand: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            127u32 => Ok(ops::Op::FNegate {
                operand: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            128u32 => Ok(ops::Op::IAdd {
                operand_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            129u32 => Ok(ops::Op::FAdd {
                operand_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            130u32 => Ok(ops::Op::ISub {
                operand_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            131u32 => Ok(ops::Op::FSub {
                operand_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            132u32 => Ok(ops::Op::IMul {
                operand_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            133u32 => Ok(ops::Op::FMul {
                operand_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            134u32 => Ok(ops::Op::UDiv {
                operand_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            135u32 => Ok(ops::Op::SDiv {
                operand_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            136u32 => Ok(ops::Op::FDiv {
                operand_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            137u32 => Ok(ops::Op::UMod {
                operand_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            138u32 => Ok(ops::Op::SRem {
                operand_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            139u32 => Ok(ops::Op::SMod {
                operand_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            140u32 => Ok(ops::Op::FRem {
                operand_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            141u32 => Ok(ops::Op::FMod {
                operand_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            142u32 => Ok(ops::Op::VectorTimesScalar {
                vector: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                scalar: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            143u32 => Ok(ops::Op::MatrixTimesScalar {
                matrix: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                scalar: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            144u32 => Ok(ops::Op::VectorTimesMatrix {
                vector: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                matrix: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            145u32 => Ok(ops::Op::MatrixTimesVector {
                matrix: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                vector: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            146u32 => Ok(ops::Op::MatrixTimesMatrix {
                left_matrix: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                right_matrix: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            147u32 => Ok(ops::Op::OuterProduct {
                vector_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                vector_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            148u32 => Ok(ops::Op::Dot {
                vector_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                vector_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            149u32 => Ok(ops::Op::IAddCarry {
                operand_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            150u32 => Ok(ops::Op::ISubBorrow {
                operand_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            151u32 => Ok(ops::Op::UMulExtended {
                operand_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            152u32 => Ok(ops::Op::SMulExtended {
                operand_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            154u32 => Ok(ops::Op::Any {
                vector: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            155u32 => Ok(ops::Op::All {
                vector: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            156u32 => Ok(ops::Op::IsNan {
                x: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            157u32 => Ok(ops::Op::IsInf {
                x: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            158u32 => Ok(ops::Op::IsFinite {
                x: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            159u32 => Ok(ops::Op::IsNormal {
                x: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            160u32 => Ok(ops::Op::SignBitSet {
                x: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            161u32 => Ok(ops::Op::LessOrGreater {
                x: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                y: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            162u32 => Ok(ops::Op::Ordered {
                x: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                y: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            163u32 => Ok(ops::Op::Unordered {
                x: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                y: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            164u32 => Ok(ops::Op::LogicalEqual {
                operand_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            165u32 => Ok(ops::Op::LogicalNotEqual {
                operand_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            166u32 => Ok(ops::Op::LogicalOr {
                operand_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            167u32 => Ok(ops::Op::LogicalAnd {
                operand_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            168u32 => Ok(ops::Op::LogicalNot {
                operand: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            169u32 => Ok(ops::Op::Select {
                condition: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                object_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                object_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            170u32 => Ok(ops::Op::IEqual {
                operand_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            171u32 => Ok(ops::Op::INotEqual {
                operand_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            172u32 => Ok(ops::Op::UGreaterThan {
                operand_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            173u32 => Ok(ops::Op::SGreaterThan {
                operand_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            174u32 => Ok(ops::Op::UGreaterThanEqual {
                operand_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            175u32 => Ok(ops::Op::SGreaterThanEqual {
                operand_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            176u32 => Ok(ops::Op::ULessThan {
                operand_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            177u32 => Ok(ops::Op::SLessThan {
                operand_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            178u32 => Ok(ops::Op::ULessThanEqual {
                operand_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            179u32 => Ok(ops::Op::SLessThanEqual {
                operand_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            180u32 => Ok(ops::Op::FOrdEqual {
                operand_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            181u32 => Ok(ops::Op::FUnordEqual {
                operand_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            182u32 => Ok(ops::Op::FOrdNotEqual {
                operand_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            183u32 => Ok(ops::Op::FUnordNotEqual {
                operand_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            184u32 => Ok(ops::Op::FOrdLessThan {
                operand_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            185u32 => Ok(ops::Op::FUnordLessThan {
                operand_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            186u32 => Ok(ops::Op::FOrdGreaterThan {
                operand_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            187u32 => Ok(ops::Op::FUnordGreaterThan {
                operand_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            188u32 => Ok(ops::Op::FOrdLessThanEqual {
                operand_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            189u32 => Ok(ops::Op::FUnordLessThanEqual {
                operand_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            190u32 => Ok(ops::Op::FOrdGreaterThanEqual {
                operand_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            191u32 => Ok(ops::Op::FUnordGreaterThanEqual {
                operand_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            194u32 => Ok(ops::Op::ShiftRightLogical {
                base: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                shift: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            195u32 => Ok(ops::Op::ShiftRightArithmetic {
                base: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                shift: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            196u32 => Ok(ops::Op::ShiftLeftLogical {
                base: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                shift: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            197u32 => Ok(ops::Op::BitwiseOr {
                operand_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            198u32 => Ok(ops::Op::BitwiseXor {
                operand_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            199u32 => Ok(ops::Op::BitwiseAnd {
                operand_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            200u32 => Ok(ops::Op::Not {
                operand: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            201u32 => Ok(ops::Op::BitFieldInsert {
                base: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                insert: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                offset: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                count: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            202u32 => Ok(ops::Op::BitFieldSExtract {
                base: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                offset: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                count: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            203u32 => Ok(ops::Op::BitFieldUExtract {
                base: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                offset: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                count: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            204u32 => Ok(ops::Op::BitReverse {
                base: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            205u32 => Ok(ops::Op::BitCount {
                base: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            207u32 => Ok(ops::Op::DPdx {
                p: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            208u32 => Ok(ops::Op::DPdy {
                p: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            209u32 => Ok(ops::Op::Fwidth {
                p: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            210u32 => Ok(ops::Op::DPdxFine {
                p: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            211u32 => Ok(ops::Op::DPdyFine {
                p: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            212u32 => Ok(ops::Op::FwidthFine {
                p: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            213u32 => Ok(ops::Op::DPdxCoarse {
                p: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            214u32 => Ok(ops::Op::DPdyCoarse {
                p: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            215u32 => Ok(ops::Op::FwidthCoarse {
                p: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            218u32 => Ok(ops::Op::EmitVertex),
            219u32 => Ok(ops::Op::EndPrimitive),
            220u32 => Ok(ops::Op::EmitStreamVertex {
                stream: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            221u32 => Ok(ops::Op::EndStreamPrimitive {
                stream: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            224u32 => Ok(ops::Op::ControlBarrier {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(&dr::Operand::IdMemorySemantics(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            225u32 => Ok(ops::Op::MemoryBarrier {
                memory: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(&dr::Operand::IdMemorySemantics(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            227u32 => Ok(ops::Op::AtomicLoad {
                pointer: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(&dr::Operand::IdMemorySemantics(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            228u32 => Ok(ops::Op::AtomicStore {
                pointer: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(&dr::Operand::IdMemorySemantics(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            229u32 => Ok(ops::Op::AtomicExchange {
                pointer: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(&dr::Operand::IdMemorySemantics(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            230u32 => Ok(ops::Op::AtomicCompareExchange {
                pointer: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                equal: (match operands.next() {
                    Some(&dr::Operand::IdMemorySemantics(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                unequal: (match operands.next() {
                    Some(&dr::Operand::IdMemorySemantics(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                comparator: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            231u32 => Ok(ops::Op::AtomicCompareExchangeWeak {
                pointer: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                equal: (match operands.next() {
                    Some(&dr::Operand::IdMemorySemantics(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                unequal: (match operands.next() {
                    Some(&dr::Operand::IdMemorySemantics(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                comparator: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            232u32 => Ok(ops::Op::AtomicIIncrement {
                pointer: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(&dr::Operand::IdMemorySemantics(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            233u32 => Ok(ops::Op::AtomicIDecrement {
                pointer: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(&dr::Operand::IdMemorySemantics(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            234u32 => Ok(ops::Op::AtomicIAdd {
                pointer: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(&dr::Operand::IdMemorySemantics(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            235u32 => Ok(ops::Op::AtomicISub {
                pointer: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(&dr::Operand::IdMemorySemantics(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            236u32 => Ok(ops::Op::AtomicSMin {
                pointer: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(&dr::Operand::IdMemorySemantics(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            237u32 => Ok(ops::Op::AtomicUMin {
                pointer: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(&dr::Operand::IdMemorySemantics(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            238u32 => Ok(ops::Op::AtomicSMax {
                pointer: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(&dr::Operand::IdMemorySemantics(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            239u32 => Ok(ops::Op::AtomicUMax {
                pointer: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(&dr::Operand::IdMemorySemantics(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            240u32 => Ok(ops::Op::AtomicAnd {
                pointer: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(&dr::Operand::IdMemorySemantics(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            241u32 => Ok(ops::Op::AtomicOr {
                pointer: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(&dr::Operand::IdMemorySemantics(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            242u32 => Ok(ops::Op::AtomicXor {
                pointer: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(&dr::Operand::IdMemorySemantics(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            246u32 => Ok(ops::Op::LoopMerge {
                merge_block: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                continue_target: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                loop_control: (match operands.next() {
                    Some(&dr::Operand::LoopControl(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            247u32 => Ok(ops::Op::SelectionMerge {
                merge_block: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                selection_control: (match operands.next() {
                    Some(&dr::Operand::SelectionControl(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            256u32 => Ok(ops::Op::LifetimeStart {
                pointer: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                size: (match operands.next() {
                    Some(&dr::Operand::LiteralInt32(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            257u32 => Ok(ops::Op::LifetimeStop {
                pointer: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                size: (match operands.next() {
                    Some(&dr::Operand::LiteralInt32(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            259u32 => Ok(ops::Op::GroupAsyncCopy {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                destination: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                source: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                num_elements: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                stride: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                event: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            260u32 => Ok(ops::Op::GroupWaitEvents {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                num_events: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                events_list: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            261u32 => Ok(ops::Op::GroupAll {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                predicate: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            262u32 => Ok(ops::Op::GroupAny {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                predicate: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            263u32 => Ok(ops::Op::GroupBroadcast {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                local_id: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            264u32 => Ok(ops::Op::GroupIAdd {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&dr::Operand::GroupOperation(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            265u32 => Ok(ops::Op::GroupFAdd {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&dr::Operand::GroupOperation(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            266u32 => Ok(ops::Op::GroupFMin {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&dr::Operand::GroupOperation(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            267u32 => Ok(ops::Op::GroupUMin {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&dr::Operand::GroupOperation(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            268u32 => Ok(ops::Op::GroupSMin {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&dr::Operand::GroupOperation(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            269u32 => Ok(ops::Op::GroupFMax {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&dr::Operand::GroupOperation(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            270u32 => Ok(ops::Op::GroupUMax {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&dr::Operand::GroupOperation(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            271u32 => Ok(ops::Op::GroupSMax {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&dr::Operand::GroupOperation(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            274u32 => Ok(ops::Op::ReadPipe {
                pipe: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                pointer: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_size: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_alignment: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            275u32 => Ok(ops::Op::WritePipe {
                pipe: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                pointer: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_size: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_alignment: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            276u32 => Ok(ops::Op::ReservedReadPipe {
                pipe: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                reserve_id: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                index: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                pointer: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_size: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_alignment: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            277u32 => Ok(ops::Op::ReservedWritePipe {
                pipe: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                reserve_id: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                index: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                pointer: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_size: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_alignment: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            278u32 => Ok(ops::Op::ReserveReadPipePackets {
                pipe: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                num_packets: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_size: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_alignment: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            279u32 => Ok(ops::Op::ReserveWritePipePackets {
                pipe: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                num_packets: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_size: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_alignment: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            280u32 => Ok(ops::Op::CommitReadPipe {
                pipe: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                reserve_id: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_size: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_alignment: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            281u32 => Ok(ops::Op::CommitWritePipe {
                pipe: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                reserve_id: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_size: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_alignment: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            282u32 => Ok(ops::Op::IsValidReserveId {
                reserve_id: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            283u32 => Ok(ops::Op::GetNumPipePackets {
                pipe: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_size: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_alignment: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            284u32 => Ok(ops::Op::GetMaxPipePackets {
                pipe: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_size: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_alignment: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            285u32 => Ok(ops::Op::GroupReserveReadPipePackets {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                pipe: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                num_packets: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_size: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_alignment: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            286u32 => Ok(ops::Op::GroupReserveWritePipePackets {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                pipe: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                num_packets: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_size: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_alignment: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            287u32 => Ok(ops::Op::GroupCommitReadPipe {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                pipe: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                reserve_id: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_size: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_alignment: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            288u32 => Ok(ops::Op::GroupCommitWritePipe {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                pipe: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                reserve_id: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_size: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packet_alignment: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            291u32 => Ok(ops::Op::EnqueueMarker {
                queue: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                num_events: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                wait_events: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                ret_event: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            292u32 => Ok(ops::Op::EnqueueKernel {
                queue: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                flags: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                nd_range: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                num_events: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                wait_events: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                ret_event: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                invoke: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                param: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                param_size: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                param_align: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                local_size: {
                    let mut vec = Vec::new();
                    while let Some(item) = match operands.next() {
                        Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                        Some(_) => Err(OperandError::WrongType)?,
                        None => None,
                    } {
                        vec.push(item);
                    }
                    vec
                },
            }),
            293u32 => Ok(ops::Op::GetKernelNDrangeSubGroupCount {
                nd_range: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                invoke: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                param: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                param_size: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                param_align: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            294u32 => Ok(ops::Op::GetKernelNDrangeMaxSubGroupSize {
                nd_range: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                invoke: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                param: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                param_size: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                param_align: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            295u32 => Ok(ops::Op::GetKernelWorkGroupSize {
                invoke: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                param: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                param_size: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                param_align: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            296u32 => Ok(ops::Op::GetKernelPreferredWorkGroupSizeMultiple {
                invoke: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                param: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                param_size: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                param_align: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            297u32 => Ok(ops::Op::RetainEvent {
                event: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            298u32 => Ok(ops::Op::ReleaseEvent {
                event: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            299u32 => Ok(ops::Op::CreateUserEvent),
            300u32 => Ok(ops::Op::IsValidEvent {
                event: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            301u32 => Ok(ops::Op::SetUserEventStatus {
                event: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                status: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            302u32 => Ok(ops::Op::CaptureEventProfilingInfo {
                event: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                profiling_info: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            303u32 => Ok(ops::Op::GetDefaultQueue),
            304u32 => Ok(ops::Op::BuildNDRange {
                global_work_size: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                local_work_size: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                global_work_offset: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            305u32 => Ok(ops::Op::ImageSparseSampleImplicitLod {
                sampled_image: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(&dr::Operand::ImageOperands(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                },
            }),
            306u32 => Ok(ops::Op::ImageSparseSampleExplicitLod {
                sampled_image: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: (match operands.next() {
                    Some(&dr::Operand::ImageOperands(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            307u32 => Ok(ops::Op::ImageSparseSampleDrefImplicitLod {
                sampled_image: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                d_ref: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(&dr::Operand::ImageOperands(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                },
            }),
            308u32 => Ok(ops::Op::ImageSparseSampleDrefExplicitLod {
                sampled_image: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                d_ref: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: (match operands.next() {
                    Some(&dr::Operand::ImageOperands(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            309u32 => Ok(ops::Op::ImageSparseSampleProjImplicitLod {
                sampled_image: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(&dr::Operand::ImageOperands(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                },
            }),
            310u32 => Ok(ops::Op::ImageSparseSampleProjExplicitLod {
                sampled_image: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: (match operands.next() {
                    Some(&dr::Operand::ImageOperands(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            311u32 => Ok(ops::Op::ImageSparseSampleProjDrefImplicitLod {
                sampled_image: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                d_ref: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(&dr::Operand::ImageOperands(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                },
            }),
            312u32 => Ok(ops::Op::ImageSparseSampleProjDrefExplicitLod {
                sampled_image: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                d_ref: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: (match operands.next() {
                    Some(&dr::Operand::ImageOperands(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            313u32 => Ok(ops::Op::ImageSparseFetch {
                image: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(&dr::Operand::ImageOperands(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                },
            }),
            314u32 => Ok(ops::Op::ImageSparseGather {
                sampled_image: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                component: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(&dr::Operand::ImageOperands(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                },
            }),
            315u32 => Ok(ops::Op::ImageSparseDrefGather {
                sampled_image: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                d_ref: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(&dr::Operand::ImageOperands(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                },
            }),
            316u32 => Ok(ops::Op::ImageSparseTexelsResident {
                resident_code: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            317u32 => Ok(ops::Op::NoLine),
            318u32 => Ok(ops::Op::AtomicFlagTestAndSet {
                pointer: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(&dr::Operand::IdMemorySemantics(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            319u32 => Ok(ops::Op::AtomicFlagClear {
                pointer: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(&dr::Operand::IdMemorySemantics(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            320u32 => Ok(ops::Op::ImageSparseRead {
                image: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(&dr::Operand::ImageOperands(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                },
            }),
            321u32 => Ok(ops::Op::SizeOf {
                pointer: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            324u32 => Ok(ops::Op::CreatePipeFromPipeStorage {
                pipe_storage: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            325u32 => Ok(ops::Op::GetKernelLocalSizeForSubgroupCount {
                subgroup_count: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                invoke: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                param: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                param_size: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                param_align: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            326u32 => Ok(ops::Op::GetKernelMaxNumSubgroups {
                invoke: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                param: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                param_size: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                param_align: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            328u32 => Ok(ops::Op::NamedBarrierInitialize {
                subgroup_count: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            329u32 => Ok(ops::Op::MemoryNamedBarrier {
                named_barrier: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                semantics: (match operands.next() {
                    Some(&dr::Operand::IdMemorySemantics(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            330u32 => Ok(ops::Op::ModuleProcessed {
                process: (match operands.next() {
                    Some(&dr::Operand::LiteralString(ref value)) => Some(value.clone()),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            332u32 => Ok(ops::Op::DecorateId {
                target: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                decoration: (match operands.next() {
                    Some(&dr::Operand::Decoration(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            333u32 => Ok(ops::Op::GroupNonUniformElect {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            334u32 => Ok(ops::Op::GroupNonUniformAll {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                predicate: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            335u32 => Ok(ops::Op::GroupNonUniformAny {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                predicate: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            336u32 => Ok(ops::Op::GroupNonUniformAllEqual {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            337u32 => Ok(ops::Op::GroupNonUniformBroadcast {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                id: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            338u32 => Ok(ops::Op::GroupNonUniformBroadcastFirst {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            339u32 => Ok(ops::Op::GroupNonUniformBallot {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                predicate: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            340u32 => Ok(ops::Op::GroupNonUniformInverseBallot {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            341u32 => Ok(ops::Op::GroupNonUniformBallotBitExtract {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                index: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            342u32 => Ok(ops::Op::GroupNonUniformBallotBitCount {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&dr::Operand::GroupOperation(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            343u32 => Ok(ops::Op::GroupNonUniformBallotFindLSB {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            344u32 => Ok(ops::Op::GroupNonUniformBallotFindMSB {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            345u32 => Ok(ops::Op::GroupNonUniformShuffle {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                id: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            346u32 => Ok(ops::Op::GroupNonUniformShuffleXor {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                mask: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            347u32 => Ok(ops::Op::GroupNonUniformShuffleUp {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                delta: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            348u32 => Ok(ops::Op::GroupNonUniformShuffleDown {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                delta: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            349u32 => Ok(ops::Op::GroupNonUniformIAdd {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&dr::Operand::GroupOperation(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                cluster_size: match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                },
            }),
            350u32 => Ok(ops::Op::GroupNonUniformFAdd {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&dr::Operand::GroupOperation(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                cluster_size: match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                },
            }),
            351u32 => Ok(ops::Op::GroupNonUniformIMul {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&dr::Operand::GroupOperation(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                cluster_size: match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                },
            }),
            352u32 => Ok(ops::Op::GroupNonUniformFMul {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&dr::Operand::GroupOperation(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                cluster_size: match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                },
            }),
            353u32 => Ok(ops::Op::GroupNonUniformSMin {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&dr::Operand::GroupOperation(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                cluster_size: match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                },
            }),
            354u32 => Ok(ops::Op::GroupNonUniformUMin {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&dr::Operand::GroupOperation(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                cluster_size: match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                },
            }),
            355u32 => Ok(ops::Op::GroupNonUniformFMin {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&dr::Operand::GroupOperation(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                cluster_size: match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                },
            }),
            356u32 => Ok(ops::Op::GroupNonUniformSMax {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&dr::Operand::GroupOperation(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                cluster_size: match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                },
            }),
            357u32 => Ok(ops::Op::GroupNonUniformUMax {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&dr::Operand::GroupOperation(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                cluster_size: match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                },
            }),
            358u32 => Ok(ops::Op::GroupNonUniformFMax {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&dr::Operand::GroupOperation(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                cluster_size: match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                },
            }),
            359u32 => Ok(ops::Op::GroupNonUniformBitwiseAnd {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&dr::Operand::GroupOperation(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                cluster_size: match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                },
            }),
            360u32 => Ok(ops::Op::GroupNonUniformBitwiseOr {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&dr::Operand::GroupOperation(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                cluster_size: match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                },
            }),
            361u32 => Ok(ops::Op::GroupNonUniformBitwiseXor {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&dr::Operand::GroupOperation(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                cluster_size: match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                },
            }),
            362u32 => Ok(ops::Op::GroupNonUniformLogicalAnd {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&dr::Operand::GroupOperation(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                cluster_size: match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                },
            }),
            363u32 => Ok(ops::Op::GroupNonUniformLogicalOr {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&dr::Operand::GroupOperation(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                cluster_size: match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                },
            }),
            364u32 => Ok(ops::Op::GroupNonUniformLogicalXor {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&dr::Operand::GroupOperation(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                cluster_size: match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                },
            }),
            365u32 => Ok(ops::Op::GroupNonUniformQuadBroadcast {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                index: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            366u32 => Ok(ops::Op::GroupNonUniformQuadSwap {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                direction: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            400u32 => Ok(ops::Op::CopyLogical {
                operand: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            401u32 => Ok(ops::Op::PtrEqual {
                operand_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            402u32 => Ok(ops::Op::PtrNotEqual {
                operand_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            403u32 => Ok(ops::Op::PtrDiff {
                operand_1: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operand_2: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            4421u32 => Ok(ops::Op::SubgroupBallotKHR {
                predicate: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            4422u32 => Ok(ops::Op::SubgroupFirstInvocationKHR {
                value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            4428u32 => Ok(ops::Op::SubgroupAllKHR {
                predicate: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            4429u32 => Ok(ops::Op::SubgroupAnyKHR {
                predicate: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            4430u32 => Ok(ops::Op::SubgroupAllEqualKHR {
                predicate: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            4432u32 => Ok(ops::Op::SubgroupReadInvocationKHR {
                value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                index: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5000u32 => Ok(ops::Op::GroupIAddNonUniformAMD {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&dr::Operand::GroupOperation(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5001u32 => Ok(ops::Op::GroupFAddNonUniformAMD {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&dr::Operand::GroupOperation(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5002u32 => Ok(ops::Op::GroupFMinNonUniformAMD {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&dr::Operand::GroupOperation(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5003u32 => Ok(ops::Op::GroupUMinNonUniformAMD {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&dr::Operand::GroupOperation(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5004u32 => Ok(ops::Op::GroupSMinNonUniformAMD {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&dr::Operand::GroupOperation(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5005u32 => Ok(ops::Op::GroupFMaxNonUniformAMD {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&dr::Operand::GroupOperation(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5006u32 => Ok(ops::Op::GroupUMaxNonUniformAMD {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&dr::Operand::GroupOperation(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5007u32 => Ok(ops::Op::GroupSMaxNonUniformAMD {
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                operation: (match operands.next() {
                    Some(&dr::Operand::GroupOperation(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                x: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5011u32 => Ok(ops::Op::FragmentMaskFetchAMD {
                image: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5012u32 => Ok(ops::Op::FragmentFetchAMD {
                image: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                fragment_index: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5283u32 => Ok(ops::Op::ImageSampleFootprintNV {
                sampled_image: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                granularity: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coarse: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_operands: match operands.next() {
                    Some(&dr::Operand::ImageOperands(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                },
            }),
            5296u32 => Ok(ops::Op::GroupNonUniformPartitionNV {
                value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5299u32 => Ok(ops::Op::WritePackedPrimitiveIndices4x8NV {
                index_offset: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                packed_indices: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5334u32 => Ok(ops::Op::ReportIntersectionNV {
                hit: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                hit_kind: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5335u32 => Ok(ops::Op::IgnoreIntersectionNV),
            5336u32 => Ok(ops::Op::TerminateRayNV),
            5337u32 => Ok(ops::Op::TraceNV {
                accel: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                ray_flags: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                cull_mask: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                sbt_offset: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                sbt_stride: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                miss_index: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                ray_origin: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                ray_tmin: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                ray_direction: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                ray_tmax: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                payload_id: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5341u32 => Ok(ops::Op::TypeAccelerationStructureNV),
            5344u32 => Ok(ops::Op::ExecuteCallableNV {
                sbt_index: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                callable_data_id: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5358u32 => Ok(ops::Op::TypeCooperativeMatrixNV {
                component_type: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(self.types.lookup_token(*value)),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                execution: (match operands.next() {
                    Some(&dr::Operand::IdScope(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                rows: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                columns: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5359u32 => Ok(ops::Op::CooperativeMatrixLoadNV {
                pointer: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                stride: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                column_major: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory_access: match operands.next() {
                    Some(&dr::Operand::MemoryAccess(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                },
            }),
            5360u32 => Ok(ops::Op::CooperativeMatrixStoreNV {
                pointer: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                object: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                stride: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                column_major: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                memory_access: match operands.next() {
                    Some(&dr::Operand::MemoryAccess(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                },
            }),
            5361u32 => Ok(ops::Op::CooperativeMatrixMulAddNV {
                a: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                b: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                c: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5362u32 => Ok(ops::Op::CooperativeMatrixLengthNV {
                ty: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5571u32 => Ok(ops::Op::SubgroupShuffleINTEL {
                data: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                invocation_id: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5572u32 => Ok(ops::Op::SubgroupShuffleDownINTEL {
                current: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                next: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                delta: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5573u32 => Ok(ops::Op::SubgroupShuffleUpINTEL {
                previous: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                current: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                delta: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5574u32 => Ok(ops::Op::SubgroupShuffleXorINTEL {
                data: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                value: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5575u32 => Ok(ops::Op::SubgroupBlockReadINTEL {
                ptr: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5576u32 => Ok(ops::Op::SubgroupBlockWriteINTEL {
                ptr: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                data: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5577u32 => Ok(ops::Op::SubgroupImageBlockReadINTEL {
                image: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5578u32 => Ok(ops::Op::SubgroupImageBlockWriteINTEL {
                image: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                data: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5580u32 => Ok(ops::Op::SubgroupImageMediaBlockReadINTEL {
                image: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                width: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                height: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5581u32 => Ok(ops::Op::SubgroupImageMediaBlockWriteINTEL {
                image: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                coordinate: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                width: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                height: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                data: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5632u32 => Ok(ops::Op::DecorateString {
                target: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                decoration: (match operands.next() {
                    Some(&dr::Operand::Decoration(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            5633u32 => Ok(ops::Op::MemberDecorateStringGOOGLE {
                struct_type: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(self.types.lookup_token(*value)),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                member: (match operands.next() {
                    Some(&dr::Operand::LiteralInt32(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                decoration: (match operands.next() {
                    Some(&dr::Operand::Decoration(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            _ => Err(InstructionError::WrongOpcode),
        }
    }
    pub fn lift_type(&mut self, raw: &dr::Instruction) -> Result<TypeEnum, InstructionError> {
        let mut operands = raw.operands.iter();
        match raw.class.opcode as u32 {
            19u32 => Ok(TypeEnum::Void),
            20u32 => Ok(TypeEnum::Bool),
            21u32 => Ok(TypeEnum::Int {
                width: (match operands.next() {
                    Some(&dr::Operand::LiteralInt32(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                signedness: (match operands.next() {
                    Some(&dr::Operand::LiteralInt32(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            22u32 => Ok(TypeEnum::Float {
                width: (match operands.next() {
                    Some(&dr::Operand::LiteralInt32(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            23u32 => Ok(TypeEnum::Vector {
                component_type: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(self.types.lookup_token(*value)),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                component_count: (match operands.next() {
                    Some(&dr::Operand::LiteralInt32(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            24u32 => Ok(TypeEnum::Matrix {
                column_type: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(self.types.lookup_token(*value)),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                column_count: (match operands.next() {
                    Some(&dr::Operand::LiteralInt32(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            25u32 => Ok(TypeEnum::Image {
                sampled_type: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(self.types.lookup_token(*value)),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                dim: (match operands.next() {
                    Some(&dr::Operand::Dim(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                depth: (match operands.next() {
                    Some(&dr::Operand::LiteralInt32(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                arrayed: (match operands.next() {
                    Some(&dr::Operand::LiteralInt32(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                ms: (match operands.next() {
                    Some(&dr::Operand::LiteralInt32(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                sampled: (match operands.next() {
                    Some(&dr::Operand::LiteralInt32(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                image_format: (match operands.next() {
                    Some(&dr::Operand::ImageFormat(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                access_qualifier: match operands.next() {
                    Some(&dr::Operand::AccessQualifier(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                },
            }),
            26u32 => Ok(TypeEnum::Sampler),
            27u32 => Ok(TypeEnum::SampledImage {
                image_type: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(self.types.lookup_token(*value)),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            28u32 => Ok(TypeEnum::Array {
                element_type: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(self.types.lookup_token(*value)),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                length: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => {
                        Some(self.constants.lookup_token(*value))
                    }
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            29u32 => Ok(TypeEnum::RuntimeArray {
                element_type: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(self.types.lookup_token(*value)),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            30u32 => Ok(TypeEnum::Struct {
                members: {
                    let mut vec = Vec::new();
                    while let Some(item) = match operands.next() {
                        Some(&dr::Operand::IdRef(ref value)) => {
                            Some(StructMember::new(self.types.lookup_token(*value)))
                        }
                        Some(_) => Err(OperandError::WrongType)?,
                        None => None,
                    } {
                        vec.push(item);
                    }
                    vec
                },
            }),
            31u32 => Ok(TypeEnum::Opaque {
                type_name: (match operands.next() {
                    Some(&dr::Operand::LiteralString(ref value)) => Some(value.clone()),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            32u32 => Ok(TypeEnum::Pointer {
                storage_class: (match operands.next() {
                    Some(&dr::Operand::StorageClass(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                ty: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            33u32 => Ok(TypeEnum::Function {
                return_type: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(self.types.lookup_token(*value)),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                parameter_0_type_parameter_1_type: {
                    let mut vec = Vec::new();
                    while let Some(item) = match operands.next() {
                        Some(&dr::Operand::IdRef(ref value)) => {
                            Some(self.types.lookup_token(*value))
                        }
                        Some(_) => Err(OperandError::WrongType)?,
                        None => None,
                    } {
                        vec.push(item);
                    }
                    vec
                },
            }),
            34u32 => Ok(TypeEnum::Event),
            35u32 => Ok(TypeEnum::DeviceEvent),
            36u32 => Ok(TypeEnum::ReserveId),
            37u32 => Ok(TypeEnum::Queue),
            38u32 => Ok(TypeEnum::Pipe {
                qualifier: (match operands.next() {
                    Some(&dr::Operand::AccessQualifier(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            39u32 => Ok(TypeEnum::ForwardPointer {
                pointer_type: (match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(self.types.lookup_token(*value)),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
                storage_class: (match operands.next() {
                    Some(&dr::Operand::StorageClass(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                })
                .ok_or(OperandError::Missing)?,
            }),
            322u32 => Ok(TypeEnum::PipeStorage),
            327u32 => Ok(TypeEnum::NamedBarrier),
            _ => Err(InstructionError::WrongOpcode),
        }
    }
    #[allow(unused)]
    pub fn lift_extension(
        &mut self,
        raw: &dr::Instruction,
    ) -> Result<instructions::Extension, InstructionError> {
        if raw.class.opcode as u32 != 10u32 {
            return Err(InstructionError::WrongOpcode);
        }
        let mut operands = raw.operands.iter();
        Ok(instructions::Extension {
            name: (match operands.next() {
                Some(&dr::Operand::LiteralString(ref value)) => Some(value.clone()),
                Some(_) => Err(OperandError::WrongType)?,
                None => None,
            })
            .ok_or(OperandError::Missing)?,
        })
    }
    #[allow(unused)]
    pub fn lift_ext_inst_import(
        &mut self,
        raw: &dr::Instruction,
    ) -> Result<instructions::ExtInstImport, InstructionError> {
        if raw.class.opcode as u32 != 11u32 {
            return Err(InstructionError::WrongOpcode);
        }
        let mut operands = raw.operands.iter();
        Ok(instructions::ExtInstImport {
            name: (match operands.next() {
                Some(&dr::Operand::LiteralString(ref value)) => Some(value.clone()),
                Some(_) => Err(OperandError::WrongType)?,
                None => None,
            })
            .ok_or(OperandError::Missing)?,
        })
    }
    #[allow(unused)]
    pub fn lift_memory_model(
        &mut self,
        raw: &dr::Instruction,
    ) -> Result<instructions::MemoryModel, InstructionError> {
        if raw.class.opcode as u32 != 14u32 {
            return Err(InstructionError::WrongOpcode);
        }
        let mut operands = raw.operands.iter();
        Ok(instructions::MemoryModel {
            addressing_model: (match operands.next() {
                Some(&dr::Operand::AddressingModel(ref value)) => Some(*value),
                Some(_) => Err(OperandError::WrongType)?,
                None => None,
            })
            .ok_or(OperandError::Missing)?,
            memory_model: (match operands.next() {
                Some(&dr::Operand::MemoryModel(ref value)) => Some(*value),
                Some(_) => Err(OperandError::WrongType)?,
                None => None,
            })
            .ok_or(OperandError::Missing)?,
        })
    }
    #[allow(unused)]
    pub fn lift_entry_point(
        &mut self,
        raw: &dr::Instruction,
    ) -> Result<instructions::EntryPoint, InstructionError> {
        if raw.class.opcode as u32 != 15u32 {
            return Err(InstructionError::WrongOpcode);
        }
        let mut operands = raw.operands.iter();
        Ok(instructions::EntryPoint {
            execution_model: (match operands.next() {
                Some(&dr::Operand::ExecutionModel(ref value)) => Some(*value),
                Some(_) => Err(OperandError::WrongType)?,
                None => None,
            })
            .ok_or(OperandError::Missing)?,
            entry_point: (match operands.next() {
                Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                Some(_) => Err(OperandError::WrongType)?,
                None => None,
            })
            .ok_or(OperandError::Missing)?,
            name: (match operands.next() {
                Some(&dr::Operand::LiteralString(ref value)) => Some(value.clone()),
                Some(_) => Err(OperandError::WrongType)?,
                None => None,
            })
            .ok_or(OperandError::Missing)?,
            interface: {
                let mut vec = Vec::new();
                while let Some(item) = match operands.next() {
                    Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                    Some(_) => Err(OperandError::WrongType)?,
                    None => None,
                } {
                    vec.push(item);
                }
                vec
            },
        })
    }
    #[allow(unused)]
    pub fn lift_execution_mode(
        &mut self,
        raw: &dr::Instruction,
    ) -> Result<instructions::ExecutionMode, InstructionError> {
        if raw.class.opcode as u32 != 16u32 {
            return Err(InstructionError::WrongOpcode);
        }
        let mut operands = raw.operands.iter();
        Ok(instructions::ExecutionMode {
            entry_point: (match operands.next() {
                Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                Some(_) => Err(OperandError::WrongType)?,
                None => None,
            })
            .ok_or(OperandError::Missing)?,
            mode: (match operands.next() {
                Some(&dr::Operand::ExecutionMode(ref value)) => Some(*value),
                Some(_) => Err(OperandError::WrongType)?,
                None => None,
            })
            .ok_or(OperandError::Missing)?,
        })
    }
    #[allow(unused)]
    pub fn lift_capability(
        &mut self,
        raw: &dr::Instruction,
    ) -> Result<instructions::Capability, InstructionError> {
        if raw.class.opcode as u32 != 17u32 {
            return Err(InstructionError::WrongOpcode);
        }
        let mut operands = raw.operands.iter();
        Ok(instructions::Capability {
            capability: (match operands.next() {
                Some(&dr::Operand::Capability(ref value)) => Some(*value),
                Some(_) => Err(OperandError::WrongType)?,
                None => None,
            })
            .ok_or(OperandError::Missing)?,
        })
    }
    #[allow(unused)]
    pub fn lift_function(
        &mut self,
        raw: &dr::Instruction,
    ) -> Result<instructions::Function, InstructionError> {
        if raw.class.opcode as u32 != 54u32 {
            return Err(InstructionError::WrongOpcode);
        }
        let mut operands = raw.operands.iter();
        Ok(instructions::Function {
            function_control: (match operands.next() {
                Some(&dr::Operand::FunctionControl(ref value)) => Some(*value),
                Some(_) => Err(OperandError::WrongType)?,
                None => None,
            })
            .ok_or(OperandError::Missing)?,
            function_type: (match operands.next() {
                Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                Some(_) => Err(OperandError::WrongType)?,
                None => None,
            })
            .ok_or(OperandError::Missing)?,
        })
    }
    #[allow(unused)]
    pub fn lift_function_parameter(
        &mut self,
        raw: &dr::Instruction,
    ) -> Result<instructions::FunctionParameter, InstructionError> {
        if raw.class.opcode as u32 != 55u32 {
            return Err(InstructionError::WrongOpcode);
        }
        Ok(instructions::FunctionParameter {})
    }
    #[allow(unused)]
    pub fn lift_function_end(
        &mut self,
        raw: &dr::Instruction,
    ) -> Result<instructions::FunctionEnd, InstructionError> {
        if raw.class.opcode as u32 != 56u32 {
            return Err(InstructionError::WrongOpcode);
        }
        Ok(instructions::FunctionEnd {})
    }
    #[allow(unused)]
    pub fn lift_label(
        &mut self,
        raw: &dr::Instruction,
    ) -> Result<instructions::Label, InstructionError> {
        if raw.class.opcode as u32 != 248u32 {
            return Err(InstructionError::WrongOpcode);
        }
        Ok(instructions::Label {})
    }
    #[allow(unused)]
    pub fn lift_execution_mode_id(
        &mut self,
        raw: &dr::Instruction,
    ) -> Result<instructions::ExecutionModeId, InstructionError> {
        if raw.class.opcode as u32 != 331u32 {
            return Err(InstructionError::WrongOpcode);
        }
        let mut operands = raw.operands.iter();
        Ok(instructions::ExecutionModeId {
            entry_point: (match operands.next() {
                Some(&dr::Operand::IdRef(ref value)) => Some(*value),
                Some(_) => Err(OperandError::WrongType)?,
                None => None,
            })
            .ok_or(OperandError::Missing)?,
            mode: (match operands.next() {
                Some(&dr::Operand::ExecutionMode(ref value)) => Some(*value),
                Some(_) => Err(OperandError::WrongType)?,
                None => None,
            })
            .ok_or(OperandError::Missing)?,
        })
    }
}
