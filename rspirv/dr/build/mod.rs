#![allow(clippy::too_many_arguments)]

use crate::dr;
use crate::spirv;

use super::Error;
use std::result;

type BuildResult<T> = result::Result<T, Error>;

/// The data representation builder.
///
/// Constructs a [`Module`](struct.Module.html) by aggregating results from
/// method calls for various instructions.
///
/// This builder is designed to be low level; its build methods' signatures
/// basically reflects the layout of the corresponding SPIR-V instructions
/// faithfully.
///
/// If a SPIR-V instruction generates a result id and the result id can be
/// forward referenced, the build method will take an optional `result_id`
/// parameter. Filling it with `Some(val)` will instruct the builder to use
/// the given `val` as the result id. For other cases, an unused result id
/// will be automatically assigned from the builder.
///
/// So for instructions forward referencing an id, to avoid id collision,
/// you can either
///
/// * first append the target instruction generating that id and then append the
///   forward referencing instruction; or
/// * use the `id()` method to get an unused id from the builder, use it in the
///   forward referencing instruction, and then later fill the optional
///   `result_id` parameter of the target instruction with the same id.
///
/// Instructions belonging to the module (e.g., `OpDecorate`) can be appended
/// at any time, no matter that a block is currently under construction
/// or not. Intructions that can appear both in the module and block
/// (e.g., `OpVariable`) will be inserted to the current block under
/// construction first, if any.
///
/// # Errors
///
/// Methods in the builder implement little sanity check; only appending
/// instructions that violates the module structure is guarded. So methods
/// possibly returning errors are basically those related to function and
/// block construction (e.g., `OpFunction` and `OpLabel`).
///
/// Errors returned are enumerants related to function structure from the
/// [`Error`](enum.Error.html) enum.
///
/// # Examples
///
/// ```
/// extern crate rspirv;
/// extern crate spirv_headers as spirv;
///
/// use rspirv::binary::Disassemble;
///
/// fn main() {
///     let mut b = rspirv::dr::Builder::new();
///     b.set_version(1, 0);
///     b.memory_model(spirv::AddressingModel::Logical, spirv::MemoryModel::Simple);
///     let void = b.type_void();
///     let voidf = b.type_function(void, vec![void]);
///     b.begin_function(void,
///                      None,
///                      (spirv::FunctionControl::DONT_INLINE |
///                       spirv::FunctionControl::CONST),
///                      voidf)
///      .unwrap();
///     b.begin_block(None).unwrap();
///     b.ret().unwrap();
///     b.end_function().unwrap();
///
///     assert_eq!(b.module().disassemble(),
///                "; SPIR-V\n\
///                 ; Version: 1.0\n\
///                 ; Generator: rspirv\n\
///                 ; Bound: 5\n\
///                 OpMemoryModel Logical Simple\n\
///                 %1 = OpTypeVoid\n\
///                 %2 = OpTypeFunction %1 %1\n\
///                 %3 = OpFunction  %1  DontInline|Const %2\n\
///                 %4 = OpLabel\n\
///                 OpReturn\n\
///                 OpFunctionEnd");
/// }
/// ```
#[derive(Default)]
pub struct Builder {
    module: dr::Module,
    next_id: u32,
    selected_function: Option<usize>,
    selected_block: Option<usize>,
}

pub enum InsertPoint {
    Begin,
    End,
    FromBegin(usize),
    FromEnd(usize),
}

impl Builder {
    /// Creates a new empty builder.
    pub fn new() -> Builder {
        Builder {
            module: dr::Module::new(),
            next_id: 1,
            selected_function: None,
            selected_block: None,
        }
    }

    /// Create a new builder from an existing module
    pub fn new_from_module(module: dr::Module) -> Builder {
        let next_id = module
            .header
            .as_ref()
            .map(|h| h.bound)
            .expect("Expecting ModuleHeader with valid bound");

        Builder {
            module,
            next_id,
            selected_function: None,
            selected_block: None,
        }
    }

    pub fn insert_into_block(
        &mut self,
        insert_point: InsertPoint,
        inst: dr::Instruction,
    ) -> BuildResult<()> {
        let (selected_function, selected_block) =
            match (self.selected_function, self.selected_block) {
                (Some(f), Some(b)) => (f, b),
                _ => return Err(Error::DetachedInstruction(Some(inst))),
            };

        let block = &mut self.module.functions[selected_function].blocks[selected_block];

        match insert_point {
            InsertPoint::End => block.instructions.push(inst),
            InsertPoint::Begin => block.instructions.insert(0, inst),
            InsertPoint::FromEnd(offset) => {
                let end = block.instructions.len();
                block.instructions.insert(end - offset, inst)
            }
            InsertPoint::FromBegin(offset) => block.instructions.insert(offset, inst),
        }

        Ok(())
    }

    pub fn insert_types_global_values(&mut self, insert_point: InsertPoint, inst: dr::Instruction) {
        match insert_point {
            InsertPoint::End => self.module.types_global_values.push(inst),
            InsertPoint::Begin => self.module.types_global_values.insert(0, inst),
            InsertPoint::FromEnd(offset) => {
                let end = self.module.types_global_values.len();
                self.module.types_global_values.insert(end - offset, inst)
            }
            InsertPoint::FromBegin(offset) => self.module.types_global_values.insert(offset, inst),
        }
    }

    pub fn pop_instruction(&mut self) -> BuildResult<dr::Instruction> {
        let (selected_function, selected_block) =
            match (self.selected_function, self.selected_block) {
                (Some(f), Some(b)) => (f, b),
                _ => return Err(Error::DetachedInstruction(None)),
            };

        let block = &mut self.module.functions[selected_function].blocks[selected_block];

        block.instructions.pop().ok_or(Error::EmptyInstructionList)
    }

    /// Sets the SPIR-V version to the given major.minor version.
    ///
    /// If this method is not called, the generated SPIR-V will be set as the newest version
    /// supported.
    pub fn set_version(&mut self, major: u8, minor: u8) {
        if self.module.header.is_none() {
            // The bound will be fixed up when module() is called.
            self.module.header = Some(dr::ModuleHeader::new(0));
        }
        self.module
            .header
            .as_mut()
            .unwrap()
            .set_version(major, minor);
    }

    /// Get the SPIR-V version as a (major, minor) tuple
    pub fn version(&self) -> Option<(u8, u8)> {
        self.module.header.as_ref().map(|h| h.version())
    }

    /// Returns the `Module` under construction.
    pub fn module(self) -> dr::Module {
        let mut module = self.module;

        match &mut module.header {
            Some(header) => header.bound = self.next_id,
            None => module.header = Some(dr::ModuleHeader::new(self.next_id)),
        }

        module
    }

    /// Returns the `Module` under construction as a reference. Note that header.bound will be inaccurate.
    pub fn module_ref(&self) -> &dr::Module {
        &self.module
    }

    /// Returns the `Module` under construction as a mutable reference. Note that header.bound will be inaccurate.
    pub fn module_mut(&mut self) -> &mut dr::Module {
        &mut self.module
    }

    pub fn selected_function(&self) -> Option<usize> {
        self.selected_function
    }

    pub fn selected_block(&self) -> Option<usize> {
        self.selected_block
    }

    /// Returns the next unused id.
    pub fn id(&mut self) -> spirv::Word {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    /// Insert a OpType instruction, deduplicate it if needed and either return the existing ID
    /// or a new unused ID if we can't find find the instruction already. Useful to uphold
    /// the SPIR-V rule that non-aggregate types can't be duplicates.
    pub fn dedup_insert_type(&mut self, inst: &dr::Instruction) -> Option<spirv::Word> {
        for ty in &self.module.types_global_values {
            if ty.is_type_identical(&inst) {
                if let Some(id) = ty.result_id {
                    return Some(id);
                }
            }
        }

        None
    }

    /// Find all blocks that end in OpReturn
    pub fn find_return_block_indices(&self) -> Vec<usize> {
        let mut result = vec![];

        if let Some(sel_fn) = self.selected_function {
            let func = &self.module.functions[sel_fn];

            for (idx, blk) in func.blocks.iter().enumerate() {
                // OpReturn must be the last instruction in a block
                let last_instr = blk.instructions.last().unwrap();

                match last_instr.class.opcode {
                    spirv::Op::Return | spirv::Op::ReturnValue => {
                        result.push(idx);
                    }
                    _ => {}
                }
            }
        }

        result
    }

    /// Select a function to insert instructions into by name
    pub fn select_function_by_name(&mut self, name: &str) -> BuildResult<()> {
        for dbg in &self.module.debug_names {
            if dbg.class.opcode == spirv::Op::Name {
                if let dr::Operand::IdRef(target_id) = dbg.operands[0] {
                    if let dr::Operand::LiteralString(found_name) = &dbg.operands[1] {
                        if found_name == name {
                            for (idx, func) in self.module.functions.iter().enumerate() {
                                if func.def.as_ref().unwrap().result_id.unwrap() == target_id {
                                    return self.select_function(Some(idx));
                                }
                            }
                        }
                    }
                }
            }
        }

        Err(Error::FunctionNotFound)
    }

    /// Select a function to insert instructions into by index (indexed into self.module.functions), or unselect if None
    pub fn select_function(&mut self, idx: Option<usize>) -> BuildResult<()> {
        match idx {
            Some(idx) => {
                if idx < self.module.functions.len() {
                    self.selected_function = Some(idx);
                    Ok(())
                } else {
                    Err(Error::FunctionNotFound)
                }
            }
            None => {
                // make sure to unselect block too
                self.selected_block = None;
                self.selected_function = None;
                Ok(())
            }
        }
    }

    /// Select a basic block (by index) to insert instructions into, indexed off of self.modules.functions[self.selected_function].blocks[idx], or unselect if None
    pub fn select_block(&mut self, idx: Option<usize>) -> BuildResult<()> {
        match idx {
            Some(idx) => {
                let selected_function = match self.selected_function {
                    Some(f) => f,
                    None => return Err(Error::DetachedBlock),
                };
                if idx < self.module.functions[selected_function].blocks.len() {
                    self.selected_block = Some(idx);
                    Ok(())
                } else {
                    Err(Error::BlockNotFound)
                }
            }
            None => {
                self.selected_block = None;
                Ok(())
            }
        }
    }

    /// Begins building of a new function.
    ///
    /// If `function_id` is `Some(val)`, then `val` will be used as the result
    /// id of the function under construction; otherwise, an unused result id
    /// will be automatically assigned.
    pub fn begin_function(
        &mut self,
        return_type: spirv::Word,
        function_id: Option<spirv::Word>,
        control: spirv::FunctionControl,
        function_type: spirv::Word,
    ) -> BuildResult<spirv::Word> {
        if self.selected_function.is_some() {
            return Err(Error::NestedFunction);
        }

        let id = match function_id {
            Some(v) => v,
            None => self.id(),
        };

        let mut f = dr::Function::new();
        f.def = Some(dr::Instruction::new(
            spirv::Op::Function,
            Some(return_type),
            Some(id),
            vec![
                dr::Operand::FunctionControl(control),
                dr::Operand::IdRef(function_type),
            ],
        ));
        self.module.functions.push(f);
        self.selected_function = Some(self.module.functions.len() - 1);
        Ok(id)
    }

    /// Ends building of the current function.
    pub fn end_function(&mut self) -> BuildResult<()> {
        let selected_function = match self.selected_function {
            Some(f) => f,
            None => return Err(Error::MismatchedFunctionEnd),
        };

        self.module.functions[selected_function].end = Some(dr::Instruction::new(
            spirv::Op::FunctionEnd,
            None,
            None,
            vec![],
        ));
        self.selected_function = None;
        Ok(())
    }

    /// Declares a formal parameter for the current function.
    pub fn function_parameter(&mut self, result_type: spirv::Word) -> BuildResult<spirv::Word> {
        let selected_function = match self.selected_function {
            Some(f) => f,
            None => return Err(Error::DetachedFunctionParameter),
        };
        let id = self.id();
        let inst = dr::Instruction::new(
            spirv::Op::FunctionParameter,
            Some(result_type),
            Some(id),
            vec![],
        );
        self.module.functions[selected_function]
            .parameters
            .push(inst);
        Ok(id)
    }

    /// Begins building of a new block.
    ///
    /// If `label_id` is `Some(val)`, then `val` will be used as the result
    /// id for the `OpLabel` instruction begining this block; otherwise,
    /// a unused result id will be automatically assigned.
    pub fn begin_block(&mut self, label_id: Option<spirv::Word>) -> BuildResult<spirv::Word> {
        let selected_function = match self.selected_function {
            Some(f) => f,
            None => return Err(Error::DetachedBlock),
        };
        if self.selected_block.is_some() {
            return Err(Error::NestedBlock);
        }

        let id = match label_id {
            Some(v) => v,
            None => self.id(),
        };

        let mut bb = dr::Block::new();
        bb.label = Some(dr::Instruction::new(
            spirv::Op::Label,
            None,
            Some(id),
            vec![],
        ));

        let blocks = &mut self.module.functions[selected_function].blocks;
        blocks.push(bb);
        self.selected_block = Some(blocks.len() - 1);
        Ok(id)
    }

    /// Begins building of a new block.
    ///
    /// Counter to `begin_block` that always generates a new OpLabel at the beginning of a block - in some cases
    /// this is undesirable (such as when constructing a branch).
    pub fn begin_block_no_label(
        &mut self,
        label_id: Option<spirv::Word>,
    ) -> BuildResult<spirv::Word> {
        let selected_function = match self.selected_function {
            Some(f) => f,
            None => return Err(Error::DetachedBlock),
        };
        if self.selected_block.is_some() {
            return Err(Error::NestedBlock);
        }

        let id = match label_id {
            Some(v) => v,
            None => self.id(),
        };

        let bb = dr::Block::new();
        let blocks = &mut self.module.functions[selected_function].blocks;
        blocks.push(bb);
        self.selected_block = Some(blocks.len() - 1);
        Ok(id)
    }

    fn end_block(&mut self, inst: dr::Instruction) -> BuildResult<()> {
        self.insert_end_block(InsertPoint::End, inst)
    }

    fn insert_end_block(
        &mut self,
        insert_point: InsertPoint,
        inst: dr::Instruction,
    ) -> BuildResult<()> {
        if self.selected_block.is_some() {
            self.insert_into_block(insert_point, inst)?;
            self.selected_block = None;
            return Ok(());
        }

        Err(Error::MismatchedTerminator)
    }

    /// Appends an OpCapability instruction.
    pub fn capability(&mut self, capability: spirv::Capability) {
        let inst = dr::Instruction::new(
            spirv::Op::Capability,
            None,
            None,
            vec![dr::Operand::Capability(capability)],
        );
        self.module.capabilities.push(inst);
    }

    /// Appends an OpExtension instruction.
    pub fn extension(&mut self, extension: impl Into<String>) {
        let inst = dr::Instruction::new(
            spirv::Op::Extension,
            None,
            None,
            vec![dr::Operand::LiteralString(extension.into())],
        );
        self.module.extensions.push(inst);
    }

    /// Appends an OpExtInstImport instruction and returns the result id.
    pub fn ext_inst_import(&mut self, extended_inst_set: impl Into<String>) -> spirv::Word {
        let id = self.id();
        let inst = dr::Instruction::new(
            spirv::Op::ExtInstImport,
            None,
            Some(id),
            vec![dr::Operand::LiteralString(extended_inst_set.into())],
        );
        self.module.ext_inst_imports.push(inst);
        id
    }

    /// Appends an OpMemoryModel instruction.
    pub fn memory_model(
        &mut self,
        addressing_model: spirv::AddressingModel,
        memory_model: spirv::MemoryModel,
    ) {
        let inst = dr::Instruction::new(
            spirv::Op::MemoryModel,
            None,
            None,
            vec![
                dr::Operand::AddressingModel(addressing_model),
                dr::Operand::MemoryModel(memory_model),
            ],
        );
        self.module.memory_model = Some(inst);
    }

    /// Appends an OpEntryPoint instruction.
    pub fn entry_point(
        &mut self,
        execution_model: spirv::ExecutionModel,
        entry_point: spirv::Word,
        name: impl Into<String>,
        interface: impl AsRef<[spirv::Word]>,
    ) {
        let mut operands = vec![
            dr::Operand::ExecutionModel(execution_model),
            dr::Operand::IdRef(entry_point),
            dr::Operand::LiteralString(name.into()),
        ];
        for v in interface.as_ref() {
            operands.push(dr::Operand::IdRef(*v));
        }

        let inst = dr::Instruction::new(spirv::Op::EntryPoint, None, None, operands);
        self.module.entry_points.push(inst);
    }

    /// Appends an OpExecutionMode instruction.
    pub fn execution_mode(
        &mut self,
        entry_point: spirv::Word,
        execution_mode: spirv::ExecutionMode,
        params: impl AsRef<[u32]>,
    ) {
        let mut operands = vec![
            dr::Operand::IdRef(entry_point),
            dr::Operand::ExecutionMode(execution_mode),
        ];
        for v in params.as_ref() {
            operands.push(dr::Operand::LiteralInt32(*v));
        }

        let inst = dr::Instruction::new(spirv::Op::ExecutionMode, None, None, operands);
        self.module.execution_modes.push(inst);
    }

    pub fn ext_inst(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        extension_set: spirv::Word,
        instruction: spirv::Word,
        operands: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<spirv::Word> {
        let mut ops = vec![
            dr::Operand::IdRef(extension_set),
            dr::Operand::LiteralExtInstInteger(instruction),
        ];
        ops.extend(operands);
        let _id = result_id.unwrap_or_else(|| self.id());
        let inst = dr::Instruction::new(spirv::Op::ExtInst, Some(result_type), Some(_id), ops);
        self.insert_into_block(InsertPoint::End, inst)?;
        Ok(_id)
    }

    /// Appends an `OpLine` instruction.
    ///
    /// If a block is currently selected, the `OpLine` is inserted into that block. If no block is
    /// currently selected, the `OpLine` is inserted into `types_global_values`.
    pub fn line(&mut self, file: spirv::Word, line: u32, column: u32) {
        let inst = dr::Instruction::new(
            spirv::Op::Line,
            None,
            None,
            vec![
                dr::Operand::IdRef(file),
                dr::Operand::LiteralInt32(line),
                dr::Operand::LiteralInt32(column),
            ],
        );
        if self.selected_block.is_some() {
            self.insert_into_block(InsertPoint::End, inst)
                .expect("Internal error: insert_into_block failed when selected_block was Some");
        } else {
            // types_global_values is the only valid section (other than functions) that
            // OpLine/OpNoLine can be placed in, so put it there.
            self.module.types_global_values.push(inst);
        }
    }

    /// Appends an `OpNoLine` instruction.
    ///
    /// If a block is currently selected, the `OpNoLine` is inserted into that block. If no block
    /// is currently selected, the `OpNoLine` is inserted into `types_global_values`.
    pub fn no_line(&mut self) {
        let inst = dr::Instruction::new(spirv::Op::NoLine, None, None, vec![]);
        if self.selected_block.is_some() {
            self.insert_into_block(InsertPoint::End, inst)
                .expect("Internal error: insert_into_block failed when selected_block was Some");
        } else {
            // types_global_values is the only valid section (other than functions) that
            // OpLine/OpNoLine can be placed in, so put it there.
            self.module.types_global_values.push(inst);
        }
    }
}

include!("autogen_type.rs");
include!("autogen_constant.rs");
include!("autogen_annotation.rs");
include!("autogen_terminator.rs");
include!("autogen_debug.rs");

impl Builder {
    /// Appends an OpDecorationGroup instruction and returns the result id.
    pub fn decoration_group(&mut self) -> spirv::Word {
        let id = self.id();
        self.module.annotations.push(dr::Instruction::new(
            spirv::Op::DecorationGroup,
            None,
            Some(id),
            vec![],
        ));
        id
    }

    pub fn string(&mut self, s: impl Into<String>) -> spirv::Word {
        let id = self.id();
        self.module.debug_string_source.push(dr::Instruction::new(
            spirv::Op::String,
            None,
            Some(id),
            vec![dr::Operand::LiteralString(s.into())],
        ));
        id
    }
}

impl Builder {
    /// Appends an OpTypeForwardPointer instruction.
    pub fn type_forward_pointer(
        &mut self,
        pointer_type: spirv::Word,
        storage_class: spirv::StorageClass,
    ) {
        self.module.types_global_values.push(dr::Instruction::new(
            spirv::Op::TypeForwardPointer,
            None,
            None,
            vec![
                dr::Operand::IdRef(pointer_type),
                dr::Operand::StorageClass(storage_class),
            ],
        ));
    }

    /// Appends an OpTypePointer instruction and returns the result id, or return the existing id if the instruction was already present.
    pub fn type_pointer(
        &mut self,
        result_id: Option<spirv::Word>,
        storage_class: spirv::StorageClass,
        pointee_type: spirv::Word,
    ) -> spirv::Word {
        let mut inst = dr::Instruction::new(
            spirv::Op::TypePointer,
            None,
            result_id,
            vec![
                dr::Operand::StorageClass(storage_class),
                dr::Operand::IdRef(pointee_type),
            ],
        );
        if let Some(result_id) = result_id {
            // An explicit ID was provided, emit it no matter what.
            self.module.types_global_values.push(inst);
            result_id
        } else if let Some(id) = self.dedup_insert_type(&inst) {
            // No ID was provided, and the type has already been declared.
            id
        } else {
            // No ID was provided, it didn't already exist, so generate a new ID and emit it.
            let new_id = self.id();
            inst.result_id = Some(new_id);
            self.module.types_global_values.push(inst);
            new_id
        }
    }

    /// Appends an OpTypeOpaque instruction and returns the result id.
    pub fn type_opaque(&mut self, type_name: impl Into<String>) -> spirv::Word {
        let id = self.id();
        self.module.types_global_values.push(dr::Instruction::new(
            spirv::Op::TypeOpaque,
            None,
            Some(id),
            vec![dr::Operand::LiteralString(type_name.into())],
        ));
        id
    }

    /// Appends an OpConstant instruction with the given 32-bit float `value`.
    /// or the module if no block is under construction.
    pub fn constant_f32(&mut self, result_type: spirv::Word, value: f32) -> spirv::Word {
        let id = self.id();
        let inst = dr::Instruction::new(
            spirv::Op::Constant,
            Some(result_type),
            Some(id),
            vec![dr::Operand::LiteralFloat32(value)],
        );
        self.module.types_global_values.push(inst);
        id
    }

    /// Appends an OpConstant instruction with the given 64-bit float `value`.
    pub fn constant_f64(&mut self, result_type: spirv::Word, value: f64) -> spirv::Word {
        let id = self.id();
        let inst = dr::Instruction::new(
            spirv::Op::Constant,
            Some(result_type),
            Some(id),
            vec![dr::Operand::LiteralFloat64(value)],
        );
        self.module.types_global_values.push(inst);
        id
    }

    /// Appends an OpConstant instruction with the given 32-bit integer `value`.
    /// or the module if no block is under construction.
    pub fn constant_u32(&mut self, result_type: spirv::Word, value: u32) -> spirv::Word {
        let id = self.id();
        let inst = dr::Instruction::new(
            spirv::Op::Constant,
            Some(result_type),
            Some(id),
            vec![dr::Operand::LiteralInt32(value)],
        );
        self.module.types_global_values.push(inst);
        id
    }

    /// Appends an OpConstant instruction with the given 64-bit integer `value`.
    pub fn constant_u64(&mut self, result_type: spirv::Word, value: u64) -> spirv::Word {
        let id = self.id();
        let inst = dr::Instruction::new(
            spirv::Op::Constant,
            Some(result_type),
            Some(id),
            vec![dr::Operand::LiteralInt64(value)],
        );
        self.module.types_global_values.push(inst);
        id
    }

    /// Appends an OpSpecConstant instruction with the given 32-bit float `value`.
    /// or the module if no block is under construction.
    pub fn spec_constant_f32(&mut self, result_type: spirv::Word, value: f32) -> spirv::Word {
        let id = self.id();
        let inst = dr::Instruction::new(
            spirv::Op::SpecConstant,
            Some(result_type),
            Some(id),
            vec![dr::Operand::LiteralFloat32(value)],
        );
        self.module.types_global_values.push(inst);
        id
    }

    /// Appends an OpSpecConstant instruction with the given 64-bit float `value`.
    pub fn spec_constant_f64(&mut self, result_type: spirv::Word, value: f64) -> spirv::Word {
        let id = self.id();
        let inst = dr::Instruction::new(
            spirv::Op::SpecConstant,
            Some(result_type),
            Some(id),
            vec![dr::Operand::LiteralFloat64(value)],
        );
        self.module.types_global_values.push(inst);
        id
    }

    /// Appends an OpSpecConstant instruction with the given 32-bit integer `value`.
    /// or the module if no block is under construction.
    pub fn spec_constant_u32(&mut self, result_type: spirv::Word, value: u32) -> spirv::Word {
        let id = self.id();
        let inst = dr::Instruction::new(
            spirv::Op::SpecConstant,
            Some(result_type),
            Some(id),
            vec![dr::Operand::LiteralInt32(value)],
        );
        self.module.types_global_values.push(inst);
        id
    }

    /// Appends an OpSpecConstant instruction with the given 32-bit integer `value`.
    pub fn spec_constant_u64(&mut self, result_type: spirv::Word, value: u64) -> spirv::Word {
        let id = self.id();
        let inst = dr::Instruction::new(
            spirv::Op::SpecConstant,
            Some(result_type),
            Some(id),
            vec![dr::Operand::LiteralInt64(value)],
        );
        self.module.types_global_values.push(inst);
        id
    }

    /// Appends an OpVariable instruction to either the current block
    /// or the module if no block is under construction.
    pub fn variable(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
        storage_class: spirv::StorageClass,
        initializer: Option<spirv::Word>,
    ) -> spirv::Word {
        let id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        let mut operands = vec![dr::Operand::StorageClass(storage_class)];
        if let Some(val) = initializer {
            operands.push(dr::Operand::IdRef(val));
        }
        let inst = dr::Instruction::new(spirv::Op::Variable, Some(result_type), Some(id), operands);

        match (self.selected_function, self.selected_block) {
            (Some(selected_function), Some(selected_block)) => {
                self.module.functions[selected_function].blocks[selected_block]
                    .instructions
                    .push(inst)
            }
            _ => self.module.types_global_values.push(inst),
        }
        id
    }

    /// Appends an OpUndef instruction to either the current block
    /// or the module if no block is under construction.
    pub fn undef(
        &mut self,
        result_type: spirv::Word,
        result_id: Option<spirv::Word>,
    ) -> spirv::Word {
        let id = match result_id {
            Some(v) => v,
            None => self.id(),
        };
        let inst = dr::Instruction::new(spirv::Op::Undef, Some(result_type), Some(id), vec![]);

        match (self.selected_function, self.selected_block) {
            (Some(selected_function), Some(selected_block)) => {
                self.module.functions[selected_function].blocks[selected_block]
                    .instructions
                    .push(inst)
            }
            _ => self.module.types_global_values.push(inst),
        }
        id
    }
}

include!("autogen_norm_insts.rs");

#[cfg(test)]
mod tests {
    use crate::dr;
    use crate::spirv;

    use super::Builder;
    use std::f32;

    use crate::binary::Disassemble;

    fn has_only_one_global_inst(module: &dr::Module) -> bool {
        if !module.functions.is_empty() {
            return false;
        }
        (module.capabilities.len()
            + module.extensions.len()
            + module.ext_inst_imports.len()
            + module.entry_points.len()
            + module.types_global_values.len()
            + module.execution_modes.len()
            + module.debug_string_source.len()
            + module.debug_names.len()
            + module.debug_module_processed.len()
            + module.annotations.len())
            + (if module.memory_model.is_some() { 1 } else { 0 })
            == 1
    }

    #[test]
    fn test_spirv_version() {
        let mut b = Builder::new();
        b.set_version(1, 2);
        let m = b.module();
        let header = &m.header;
        assert!(header.is_some());
        assert_eq!((1, 2), header.as_ref().unwrap().version());
    }

    #[test]
    fn test_memory_model() {
        let mut b = Builder::new();
        b.memory_model(spirv::AddressingModel::Logical, spirv::MemoryModel::Simple);
        let m = b.module();
        assert!(m.memory_model.is_some());
        let inst = m.memory_model.as_ref().unwrap();
        assert!(has_only_one_global_inst(&m));
        assert_eq!("MemoryModel", inst.class.opname);
        assert_eq!(2, inst.operands.len());
        assert_eq!(
            dr::Operand::from(spirv::AddressingModel::Logical),
            inst.operands[0]
        );
        assert_eq!(
            dr::Operand::from(spirv::MemoryModel::Simple),
            inst.operands[1]
        );
    }

    #[test]
    fn test_decoration_no_additional_params() {
        let mut b = Builder::new();
        b.member_decorate(1, 0, spirv::Decoration::RelaxedPrecision, vec![]);
        let m = b.module();
        assert!(has_only_one_global_inst(&m));
        let inst = m.annotations.last().unwrap();
        assert_eq!("MemberDecorate", inst.class.opname);
        assert_eq!(3, inst.operands.len());
        assert_eq!(dr::Operand::IdRef(1), inst.operands[0]);
        assert_eq!(dr::Operand::from(0u32), inst.operands[1]);
        assert_eq!(
            dr::Operand::from(spirv::Decoration::RelaxedPrecision),
            inst.operands[2]
        );
    }

    #[test]
    fn test_decoration_with_additional_params() {
        let mut b = Builder::new();
        b.decorate(
            1,
            spirv::Decoration::LinkageAttributes,
            vec![
                dr::Operand::from("name"),
                dr::Operand::from(spirv::LinkageType::Export),
            ],
        );
        let m = b.module();
        assert!(has_only_one_global_inst(&m));
        let inst = m.annotations.last().unwrap();
        assert_eq!("Decorate", inst.class.opname);
        assert_eq!(4, inst.operands.len());
        assert_eq!(dr::Operand::IdRef(1), inst.operands[0]);
        assert_eq!(
            dr::Operand::from(spirv::Decoration::LinkageAttributes),
            inst.operands[1]
        );
        assert_eq!(dr::Operand::from("name"), inst.operands[2]);
        assert_eq!(
            dr::Operand::from(spirv::LinkageType::Export),
            inst.operands[3]
        );
    }

    #[test]
    fn test_constant_f32() {
        let mut b = Builder::new();
        let float = b.type_float(32);
        // Normal numbers
        b.constant_f32(float, 3.14);
        b.constant_f32(float, 2e-10);
        // Zero
        b.constant_f32(float, 0.);
        // Inf
        b.constant_f32(float, f32::NEG_INFINITY);
        // Subnormal numbers
        b.constant_f32(float, -1.0e-40_f32);
        // Nan
        b.constant_f32(float, f32::NAN);
        let m = b.module();
        assert_eq!(7, m.types_global_values.len());

        let inst = &m.types_global_values[1];
        assert_eq!(spirv::Op::Constant, inst.class.opcode);
        assert_eq!(Some(1), inst.result_type);
        assert_eq!(Some(2), inst.result_id);
        assert_eq!(dr::Operand::from(3.14f32), inst.operands[0]);

        let inst = &m.types_global_values[2];
        assert_eq!(spirv::Op::Constant, inst.class.opcode);
        assert_eq!(Some(1), inst.result_type);
        assert_eq!(Some(3), inst.result_id);
        assert_eq!(dr::Operand::from(2e-10_f32), inst.operands[0]);

        let inst = &m.types_global_values[3];
        assert_eq!(spirv::Op::Constant, inst.class.opcode);
        assert_eq!(Some(1), inst.result_type);
        assert_eq!(Some(4), inst.result_id);
        assert_eq!(dr::Operand::from(0.0f32), inst.operands[0]);

        let inst = &m.types_global_values[4];
        assert_eq!(spirv::Op::Constant, inst.class.opcode);
        assert_eq!(Some(1), inst.result_type);
        assert_eq!(Some(5), inst.result_id);
        assert_eq!(dr::Operand::from(f32::NEG_INFINITY), inst.operands[0]);

        let inst = &m.types_global_values[5];
        assert_eq!(spirv::Op::Constant, inst.class.opcode);
        assert_eq!(Some(1), inst.result_type);
        assert_eq!(Some(6), inst.result_id);
        assert_eq!(dr::Operand::from(-1.0e-40_f32), inst.operands[0]);

        let inst = &m.types_global_values[6];
        assert_eq!(spirv::Op::Constant, inst.class.opcode);
        assert_eq!(Some(1), inst.result_type);
        assert_eq!(Some(7), inst.result_id);
        // NaN != NaN
        match inst.operands[0] {
            dr::Operand::LiteralFloat32(f) => assert!(f.is_nan()),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_spec_constant_f32() {
        let mut b = Builder::new();
        let float = b.type_float(32);
        // Normal numbers
        b.spec_constant_f32(float, 10.);
        // Zero
        b.spec_constant_f32(float, -0.);
        // Inf
        b.spec_constant_f32(float, f32::INFINITY);
        // Subnormal numbers
        b.spec_constant_f32(float, 1.0e-40_f32);
        // Nan
        b.spec_constant_f32(float, f32::NAN);
        let m = b.module();
        assert_eq!(6, m.types_global_values.len());

        let inst = &m.types_global_values[1];
        assert_eq!(spirv::Op::SpecConstant, inst.class.opcode);
        assert_eq!(Some(1), inst.result_type);
        assert_eq!(Some(2), inst.result_id);
        assert_eq!(dr::Operand::from(10.0f32), inst.operands[0]);

        let inst = &m.types_global_values[2];
        assert_eq!(spirv::Op::SpecConstant, inst.class.opcode);
        assert_eq!(Some(1), inst.result_type);
        assert_eq!(Some(3), inst.result_id);
        assert_eq!(dr::Operand::from(-0.0f32), inst.operands[0]);

        let inst = &m.types_global_values[3];
        assert_eq!(spirv::Op::SpecConstant, inst.class.opcode);
        assert_eq!(Some(1), inst.result_type);
        assert_eq!(Some(4), inst.result_id);
        assert_eq!(dr::Operand::from(f32::INFINITY), inst.operands[0]);

        let inst = &m.types_global_values[4];
        assert_eq!(spirv::Op::SpecConstant, inst.class.opcode);
        assert_eq!(Some(1), inst.result_type);
        assert_eq!(Some(5), inst.result_id);
        assert_eq!(dr::Operand::from(1.0e-40_f32), inst.operands[0]);

        let inst = &m.types_global_values[5];
        assert_eq!(spirv::Op::SpecConstant, inst.class.opcode);
        assert_eq!(Some(1), inst.result_type);
        assert_eq!(Some(6), inst.result_id);
        // NaN != NaN
        match inst.operands[0] {
            dr::Operand::LiteralFloat32(f) => assert!(f.is_nan()),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_forward_ref_pointer_type() {
        let mut b = Builder::new();
        let float = b.type_float(32); // 1
                                      // Let builder generate
        let p1 = b.type_pointer(None, spirv::StorageClass::Input, float); // 2
                                                                          // We supply
        let pointee = b.id(); // 3
        b.type_forward_pointer(pointee, spirv::StorageClass::Output);
        let p2 = b.type_pointer(Some(pointee), spirv::StorageClass::Output, float);
        let m = b.module();
        assert_eq!(1, float);
        assert_eq!(2, p1);
        assert_eq!(pointee, p2); // Return the same id
        assert_eq!(4, m.types_global_values.len());

        let inst = &m.types_global_values[0];
        assert_eq!(spirv::Op::TypeFloat, inst.class.opcode);
        assert_eq!(None, inst.result_type);
        assert_eq!(Some(1), inst.result_id);
        assert_eq!(vec![dr::Operand::LiteralInt32(32)], inst.operands);

        let inst = &m.types_global_values[1];
        assert_eq!(spirv::Op::TypePointer, inst.class.opcode);
        assert_eq!(None, inst.result_type);
        assert_eq!(Some(2), inst.result_id);
        assert_eq!(
            vec![
                dr::Operand::from(spirv::StorageClass::Input),
                dr::Operand::IdRef(1),
            ],
            inst.operands
        );

        let inst = &m.types_global_values[2];
        assert_eq!(spirv::Op::TypeForwardPointer, inst.class.opcode);
        assert_eq!(None, inst.result_type);
        assert_eq!(None, inst.result_id);
        assert_eq!(
            vec![
                dr::Operand::IdRef(3),
                dr::Operand::from(spirv::StorageClass::Output),
            ],
            inst.operands
        );

        let inst = &m.types_global_values[3];
        assert_eq!(spirv::Op::TypePointer, inst.class.opcode);
        assert_eq!(None, inst.result_type);
        assert_eq!(Some(3), inst.result_id);
        assert_eq!(
            vec![
                dr::Operand::from(spirv::StorageClass::Output),
                dr::Operand::IdRef(1),
            ],
            inst.operands
        );
    }

    #[test]
    fn test_forward_ref_phi() {
        let mut b = Builder::new();

        let float = b.type_float(32);
        assert_eq!(1, float);
        let f32ff32 = b.type_function(float, vec![float]);
        assert_eq!(2, f32ff32);
        let c0 = b.constant_f32(float, 0.0f32);
        assert_eq!(3, c0);

        let fid = b
            .begin_function(float, None, spirv::FunctionControl::NONE, f32ff32)
            .unwrap();
        assert_eq!(4, fid);

        let epid = b.begin_block(None).unwrap(); // Entry block id
        assert_eq!(5, epid);
        let target1 = b.id();
        assert_eq!(6, target1);
        assert!(b.branch(target1).is_ok());

        let pbid = b.begin_block(Some(target1)).unwrap(); // Phi block id
        assert_eq!(target1, pbid);
        let target2 = b.id();
        assert_eq!(7, target2);
        let fr_add = b.id();
        assert_eq!(8, fr_add);
        // OpPhi can forward reference ids for both labels and results
        let phi = b
            .phi(
                float,
                None,
                // From above, from this, from below
                vec![(c0, epid), (fr_add, pbid), (c0, target2)],
            )
            .unwrap();
        assert_eq!(9, phi);
        let res_add = b.f_add(float, Some(fr_add), c0, c0).unwrap();
        assert_eq!(res_add, fr_add);
        assert!(b.branch(target2).is_ok());

        let exid = b.begin_block(Some(target2)).unwrap(); // Exit block id
        assert_eq!(exid, target2);
        assert!(b.ret_value(c0).is_ok());

        assert!(b.end_function().is_ok());

        let m = b.module();
        assert_eq!(1, m.functions.len());
        assert_eq!(
            m.functions.first().unwrap().disassemble(),
            "%4 = OpFunction  %1  None %2\n\
                    %5 = OpLabel\n\
                    OpBranch %6\n\
                    %6 = OpLabel\n\
                    %9 = OpPhi  %1  %3 %5 %8 %6 %3 %7\n\
                    %8 = OpFAdd  %1  %3 %3\n\
                    OpBranch %7\n\
                    %7 = OpLabel\n\
                    OpReturnValue %3\n\
                    OpFunctionEnd"
        );
    }

    #[test]
    fn test_build_variables() {
        let mut b = Builder::new();

        let void = b.type_void();
        assert_eq!(1, void);
        let float = b.type_float(32);
        assert_eq!(2, float);
        let ifp = b.type_pointer(None, spirv::StorageClass::Input, float);
        assert_eq!(3, ifp);
        let ffp = b.type_pointer(None, spirv::StorageClass::Function, float);
        assert_eq!(4, ffp);
        let voidfvoid = b.type_function(void, vec![void]);
        assert_eq!(5, voidfvoid);

        // Global variable
        let v1 = b.variable(ifp, None, spirv::StorageClass::Input, None);
        assert_eq!(6, v1);

        let f = b
            .begin_function(void, None, spirv::FunctionControl::NONE, voidfvoid)
            .unwrap();
        assert_eq!(7, f);
        let bb = b.begin_block(None).unwrap();
        assert_eq!(8, bb);
        // Local variable
        let v2 = b.variable(ffp, None, spirv::StorageClass::Function, None);
        assert_eq!(9, v2);
        assert!(b.ret().is_ok());
        assert!(b.end_function().is_ok());

        // Global variable again
        let v3 = b.variable(ifp, None, spirv::StorageClass::Input, None);
        assert_eq!(10, v3);

        assert_eq!(
            b.module().disassemble(),
            "; SPIR-V\n; Version: 1.5\n; Generator: rspirv\n; Bound: 11\n\
                    %1 = OpTypeVoid\n\
                    %2 = OpTypeFloat 32\n\
                    %3 = OpTypePointer Input %2\n\
                    %4 = OpTypePointer Function %2\n\
                    %5 = OpTypeFunction %1 %1\n\
                    %6 = OpVariable  %3  Input\n\
                    %10 = OpVariable  %3  Input\n\
                    %7 = OpFunction  %1  None %5\n\
                    %8 = OpLabel\n\
                    %9 = OpVariable  %4  Function\n\
                    OpReturn\n\
                    OpFunctionEnd"
        );
    }

    #[test]
    fn test_build_undefs() {
        let mut b = Builder::new();

        let void = b.type_void();
        assert_eq!(1, void);
        let float = b.type_float(32);
        assert_eq!(2, float);
        let voidfvoid = b.type_function(void, vec![void]);
        assert_eq!(3, voidfvoid);

        // Global undef
        let v1 = b.undef(float, None);
        assert_eq!(4, v1);

        let f = b
            .begin_function(void, None, spirv::FunctionControl::NONE, voidfvoid)
            .unwrap();
        assert_eq!(5, f);
        let bb = b.begin_block(None).unwrap();
        assert_eq!(6, bb);
        // Local undef
        let v2 = b.undef(float, None);
        assert_eq!(7, v2);
        assert!(b.ret().is_ok());
        assert!(b.end_function().is_ok());

        // Global undef again
        let v3 = b.undef(float, None);
        assert_eq!(8, v3);

        assert_eq!(
            b.module().disassemble(),
            "; SPIR-V\n; Version: 1.5\n; Generator: rspirv\n; Bound: 9\n\
                    %1 = OpTypeVoid\n\
                    %2 = OpTypeFloat 32\n\
                    %3 = OpTypeFunction %1 %1\n\
                    %4 = OpUndef  %2\n\
                    %8 = OpUndef  %2\n\
                    %5 = OpFunction  %1  None %3\n\
                    %6 = OpLabel\n\
                    %7 = OpUndef  %2\n\
                    OpReturn\n\
                    OpFunctionEnd"
        );
    }
}
