// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

impl Builder {
    #[doc = "Appends an OpTypeVoid instruction and returns the result id."]
    pub fn type_void(&mut self) -> spirv::Word {
        let id = self.id();
        self.module.types_global_values.push(dr::Instruction::new(
            spirv::Op::TypeVoid,
            None,
            Some(id),
            vec![],
        ));
        id
    }
    #[doc = "Appends an OpTypeBool instruction and returns the result id."]
    pub fn type_bool(&mut self) -> spirv::Word {
        let id = self.id();
        self.module.types_global_values.push(dr::Instruction::new(
            spirv::Op::TypeBool,
            None,
            Some(id),
            vec![],
        ));
        id
    }
    #[doc = "Appends an OpTypeInt instruction and returns the result id."]
    pub fn type_int(&mut self, width: u32, signedness: u32) -> spirv::Word {
        let id = self.id();
        self.module.types_global_values.push(dr::Instruction::new(
            spirv::Op::TypeInt,
            None,
            Some(id),
            vec![
                dr::Operand::LiteralInt32(width),
                dr::Operand::LiteralInt32(signedness),
            ],
        ));
        id
    }
    #[doc = "Appends an OpTypeFloat instruction and returns the result id."]
    pub fn type_float(&mut self, width: u32) -> spirv::Word {
        let id = self.id();
        self.module.types_global_values.push(dr::Instruction::new(
            spirv::Op::TypeFloat,
            None,
            Some(id),
            vec![dr::Operand::LiteralInt32(width)],
        ));
        id
    }
    #[doc = "Appends an OpTypeVector instruction and returns the result id."]
    pub fn type_vector(
        &mut self,
        component_type: spirv::Word,
        component_count: u32,
    ) -> spirv::Word {
        let id = self.id();
        self.module.types_global_values.push(dr::Instruction::new(
            spirv::Op::TypeVector,
            None,
            Some(id),
            vec![
                dr::Operand::IdRef(component_type),
                dr::Operand::LiteralInt32(component_count),
            ],
        ));
        id
    }
    #[doc = "Appends an OpTypeMatrix instruction and returns the result id."]
    pub fn type_matrix(&mut self, column_type: spirv::Word, column_count: u32) -> spirv::Word {
        let id = self.id();
        self.module.types_global_values.push(dr::Instruction::new(
            spirv::Op::TypeMatrix,
            None,
            Some(id),
            vec![
                dr::Operand::IdRef(column_type),
                dr::Operand::LiteralInt32(column_count),
            ],
        ));
        id
    }
    #[doc = "Appends an OpTypeImage instruction and returns the result id."]
    pub fn type_image(
        &mut self,
        sampled_type: spirv::Word,
        dim: spirv::Dim,
        depth: u32,
        arrayed: u32,
        ms: u32,
        sampled: u32,
        image_format: spirv::ImageFormat,
        access_qualifier: Option<spirv::AccessQualifier>,
    ) -> spirv::Word {
        let id = self.id();
        self.module.types_global_values.push(dr::Instruction::new(
            spirv::Op::TypeImage,
            None,
            Some(id),
            vec![
                dr::Operand::IdRef(sampled_type),
                dr::Operand::Dim(dim),
                dr::Operand::LiteralInt32(depth),
                dr::Operand::LiteralInt32(arrayed),
                dr::Operand::LiteralInt32(ms),
                dr::Operand::LiteralInt32(sampled),
                dr::Operand::ImageFormat(image_format),
            ],
        ));
        if let Some(v) = access_qualifier {
            #[allow(clippy::identity_conversion)]
            self.module
                .types_global_values
                .last_mut()
                .expect("interal error")
                .operands
                .push(dr::Operand::AccessQualifier(v.into()));
        }
        id
    }
    #[doc = "Appends an OpTypeSampler instruction and returns the result id."]
    pub fn type_sampler(&mut self) -> spirv::Word {
        let id = self.id();
        self.module.types_global_values.push(dr::Instruction::new(
            spirv::Op::TypeSampler,
            None,
            Some(id),
            vec![],
        ));
        id
    }
    #[doc = "Appends an OpTypeSampledImage instruction and returns the result id."]
    pub fn type_sampled_image(&mut self, image_type: spirv::Word) -> spirv::Word {
        let id = self.id();
        self.module.types_global_values.push(dr::Instruction::new(
            spirv::Op::TypeSampledImage,
            None,
            Some(id),
            vec![dr::Operand::IdRef(image_type)],
        ));
        id
    }
    #[doc = "Appends an OpTypeArray instruction and returns the result id."]
    pub fn type_array(&mut self, element_type: spirv::Word, length: spirv::Word) -> spirv::Word {
        let id = self.id();
        self.module.types_global_values.push(dr::Instruction::new(
            spirv::Op::TypeArray,
            None,
            Some(id),
            vec![dr::Operand::IdRef(element_type), dr::Operand::IdRef(length)],
        ));
        id
    }
    #[doc = "Appends an OpTypeRuntimeArray instruction and returns the result id."]
    pub fn type_runtime_array(&mut self, element_type: spirv::Word) -> spirv::Word {
        let id = self.id();
        self.module.types_global_values.push(dr::Instruction::new(
            spirv::Op::TypeRuntimeArray,
            None,
            Some(id),
            vec![dr::Operand::IdRef(element_type)],
        ));
        id
    }
    #[doc = "Appends an OpTypeStruct instruction and returns the result id."]
    pub fn type_struct<T: AsRef<[spirv::Word]>>(&mut self, members: T) -> spirv::Word {
        let id = self.id();
        self.module.types_global_values.push(dr::Instruction::new(
            spirv::Op::TypeStruct,
            None,
            Some(id),
            vec![],
        ));
        for v in members.as_ref() {
            self.module
                .types_global_values
                .last_mut()
                .expect("interal error")
                .operands
                .push(dr::Operand::IdRef(*v));
        }
        id
    }
    #[doc = "Appends an OpTypeFunction instruction and returns the result id."]
    pub fn type_function<T: AsRef<[spirv::Word]>>(
        &mut self,
        return_type: spirv::Word,
        parameter_0_type_parameter_1_type: T,
    ) -> spirv::Word {
        let id = self.id();
        self.module.types_global_values.push(dr::Instruction::new(
            spirv::Op::TypeFunction,
            None,
            Some(id),
            vec![dr::Operand::IdRef(return_type)],
        ));
        for v in parameter_0_type_parameter_1_type.as_ref() {
            self.module
                .types_global_values
                .last_mut()
                .expect("interal error")
                .operands
                .push(dr::Operand::IdRef(*v));
        }
        id
    }
    #[doc = "Appends an OpTypeEvent instruction and returns the result id."]
    pub fn type_event(&mut self) -> spirv::Word {
        let id = self.id();
        self.module.types_global_values.push(dr::Instruction::new(
            spirv::Op::TypeEvent,
            None,
            Some(id),
            vec![],
        ));
        id
    }
    #[doc = "Appends an OpTypeDeviceEvent instruction and returns the result id."]
    pub fn type_device_event(&mut self) -> spirv::Word {
        let id = self.id();
        self.module.types_global_values.push(dr::Instruction::new(
            spirv::Op::TypeDeviceEvent,
            None,
            Some(id),
            vec![],
        ));
        id
    }
    #[doc = "Appends an OpTypeReserveId instruction and returns the result id."]
    pub fn type_reserve_id(&mut self) -> spirv::Word {
        let id = self.id();
        self.module.types_global_values.push(dr::Instruction::new(
            spirv::Op::TypeReserveId,
            None,
            Some(id),
            vec![],
        ));
        id
    }
    #[doc = "Appends an OpTypeQueue instruction and returns the result id."]
    pub fn type_queue(&mut self) -> spirv::Word {
        let id = self.id();
        self.module.types_global_values.push(dr::Instruction::new(
            spirv::Op::TypeQueue,
            None,
            Some(id),
            vec![],
        ));
        id
    }
    #[doc = "Appends an OpTypePipe instruction and returns the result id."]
    pub fn type_pipe(&mut self, qualifier: spirv::AccessQualifier) -> spirv::Word {
        let id = self.id();
        self.module.types_global_values.push(dr::Instruction::new(
            spirv::Op::TypePipe,
            None,
            Some(id),
            vec![dr::Operand::AccessQualifier(qualifier)],
        ));
        id
    }
    #[doc = "Appends an OpTypePipeStorage instruction and returns the result id."]
    pub fn type_pipe_storage(&mut self) -> spirv::Word {
        let id = self.id();
        self.module.types_global_values.push(dr::Instruction::new(
            spirv::Op::TypePipeStorage,
            None,
            Some(id),
            vec![],
        ));
        id
    }
    #[doc = "Appends an OpTypeNamedBarrier instruction and returns the result id."]
    pub fn type_named_barrier(&mut self) -> spirv::Word {
        let id = self.id();
        self.module.types_global_values.push(dr::Instruction::new(
            spirv::Op::TypeNamedBarrier,
            None,
            Some(id),
            vec![],
        ));
        id
    }
}
