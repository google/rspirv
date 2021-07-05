// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

impl Builder {
    #[doc = "Appends an OpLoopMerge instruction and ends the current block."]
    pub fn loop_merge(
        &mut self,
        merge_block: spirv::Word,
        continue_target: spirv::Word,
        loop_control: spirv::LoopControl,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::LoopMerge,
            None,
            None,
            vec![
                dr::Operand::IdRef(merge_block),
                dr::Operand::IdRef(continue_target),
                dr::Operand::LoopControl(loop_control),
            ],
        );
        inst.operands.extend(additional_params);
        self.end_block(inst)
    }
    #[doc = "Insert an OpLoopMerge instruction and ends the current block."]
    pub fn insert_loop_merge(
        &mut self,
        insert_point: InsertPoint,
        merge_block: spirv::Word,
        continue_target: spirv::Word,
        loop_control: spirv::LoopControl,
        additional_params: impl IntoIterator<Item = dr::Operand>,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::LoopMerge,
            None,
            None,
            vec![
                dr::Operand::IdRef(merge_block),
                dr::Operand::IdRef(continue_target),
                dr::Operand::LoopControl(loop_control),
            ],
        );
        inst.operands.extend(additional_params);
        self.insert_end_block(insert_point, inst)
    }
    #[doc = "Appends an OpBranch instruction and ends the current block."]
    pub fn branch(&mut self, target_label: spirv::Word) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Branch,
            None,
            None,
            vec![dr::Operand::IdRef(target_label)],
        );
        self.end_block(inst)
    }
    #[doc = "Insert an OpBranch instruction and ends the current block."]
    pub fn insert_branch(
        &mut self,
        insert_point: InsertPoint,
        target_label: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Branch,
            None,
            None,
            vec![dr::Operand::IdRef(target_label)],
        );
        self.insert_end_block(insert_point, inst)
    }
    #[doc = "Appends an OpBranchConditional instruction and ends the current block."]
    pub fn branch_conditional(
        &mut self,
        condition: spirv::Word,
        true_label: spirv::Word,
        false_label: spirv::Word,
        branch_weights: impl IntoIterator<Item = u32>,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::BranchConditional,
            None,
            None,
            vec![
                dr::Operand::IdRef(condition),
                dr::Operand::IdRef(true_label),
                dr::Operand::IdRef(false_label),
            ],
        );
        inst.operands
            .extend(branch_weights.into_iter().map(dr::Operand::LiteralInt32));
        self.end_block(inst)
    }
    #[doc = "Insert an OpBranchConditional instruction and ends the current block."]
    pub fn insert_branch_conditional(
        &mut self,
        insert_point: InsertPoint,
        condition: spirv::Word,
        true_label: spirv::Word,
        false_label: spirv::Word,
        branch_weights: impl IntoIterator<Item = u32>,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::BranchConditional,
            None,
            None,
            vec![
                dr::Operand::IdRef(condition),
                dr::Operand::IdRef(true_label),
                dr::Operand::IdRef(false_label),
            ],
        );
        inst.operands
            .extend(branch_weights.into_iter().map(dr::Operand::LiteralInt32));
        self.insert_end_block(insert_point, inst)
    }
    #[doc = "Appends an OpSwitch instruction and ends the current block."]
    pub fn switch(
        &mut self,
        selector: spirv::Word,
        default: spirv::Word,
        target: impl IntoIterator<Item = (dr::Operand, spirv::Word)>,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Switch,
            None,
            None,
            vec![dr::Operand::IdRef(selector), dr::Operand::IdRef(default)],
        );
        for v in target {
            inst.operands.push(v.0);
            inst.operands.push(dr::Operand::IdRef(v.1));
        }
        self.end_block(inst)
    }
    #[doc = "Insert an OpSwitch instruction and ends the current block."]
    pub fn insert_switch(
        &mut self,
        insert_point: InsertPoint,
        selector: spirv::Word,
        default: spirv::Word,
        target: impl IntoIterator<Item = (dr::Operand, spirv::Word)>,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::Switch,
            None,
            None,
            vec![dr::Operand::IdRef(selector), dr::Operand::IdRef(default)],
        );
        for v in target {
            inst.operands.push(v.0);
            inst.operands.push(dr::Operand::IdRef(v.1));
        }
        self.insert_end_block(insert_point, inst)
    }
    #[doc = "Appends an OpKill instruction and ends the current block."]
    pub fn kill(&mut self) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(spirv::Op::Kill, None, None, vec![]);
        self.end_block(inst)
    }
    #[doc = "Insert an OpKill instruction and ends the current block."]
    pub fn insert_kill(&mut self, insert_point: InsertPoint) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(spirv::Op::Kill, None, None, vec![]);
        self.insert_end_block(insert_point, inst)
    }
    #[doc = "Appends an OpReturn instruction and ends the current block."]
    pub fn ret(&mut self) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(spirv::Op::Return, None, None, vec![]);
        self.end_block(inst)
    }
    #[doc = "Insert an OpReturn instruction and ends the current block."]
    pub fn insert_ret(&mut self, insert_point: InsertPoint) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(spirv::Op::Return, None, None, vec![]);
        self.insert_end_block(insert_point, inst)
    }
    #[doc = "Appends an OpReturnValue instruction and ends the current block."]
    pub fn ret_value(&mut self, value: spirv::Word) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ReturnValue,
            None,
            None,
            vec![dr::Operand::IdRef(value)],
        );
        self.end_block(inst)
    }
    #[doc = "Insert an OpReturnValue instruction and ends the current block."]
    pub fn insert_ret_value(
        &mut self,
        insert_point: InsertPoint,
        value: spirv::Word,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::ReturnValue,
            None,
            None,
            vec![dr::Operand::IdRef(value)],
        );
        self.insert_end_block(insert_point, inst)
    }
    #[doc = "Appends an OpUnreachable instruction and ends the current block."]
    pub fn unreachable(&mut self) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(spirv::Op::Unreachable, None, None, vec![]);
        self.end_block(inst)
    }
    #[doc = "Insert an OpUnreachable instruction and ends the current block."]
    pub fn insert_unreachable(&mut self, insert_point: InsertPoint) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(spirv::Op::Unreachable, None, None, vec![]);
        self.insert_end_block(insert_point, inst)
    }
    #[doc = "Appends an OpLifetimeStart instruction and ends the current block."]
    pub fn lifetime_start(&mut self, pointer: spirv::Word, size: u32) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::LifetimeStart,
            None,
            None,
            vec![dr::Operand::IdRef(pointer), dr::Operand::LiteralInt32(size)],
        );
        self.end_block(inst)
    }
    #[doc = "Insert an OpLifetimeStart instruction and ends the current block."]
    pub fn insert_lifetime_start(
        &mut self,
        insert_point: InsertPoint,
        pointer: spirv::Word,
        size: u32,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::LifetimeStart,
            None,
            None,
            vec![dr::Operand::IdRef(pointer), dr::Operand::LiteralInt32(size)],
        );
        self.insert_end_block(insert_point, inst)
    }
    #[doc = "Appends an OpLifetimeStop instruction and ends the current block."]
    pub fn lifetime_stop(&mut self, pointer: spirv::Word, size: u32) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::LifetimeStop,
            None,
            None,
            vec![dr::Operand::IdRef(pointer), dr::Operand::LiteralInt32(size)],
        );
        self.end_block(inst)
    }
    #[doc = "Insert an OpLifetimeStop instruction and ends the current block."]
    pub fn insert_lifetime_stop(
        &mut self,
        insert_point: InsertPoint,
        pointer: spirv::Word,
        size: u32,
    ) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(
            spirv::Op::LifetimeStop,
            None,
            None,
            vec![dr::Operand::IdRef(pointer), dr::Operand::LiteralInt32(size)],
        );
        self.insert_end_block(insert_point, inst)
    }
    #[doc = "Appends an OpTerminateInvocation instruction and ends the current block."]
    pub fn terminate_invocation(&mut self) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(spirv::Op::TerminateInvocation, None, None, vec![]);
        self.end_block(inst)
    }
    #[doc = "Insert an OpTerminateInvocation instruction and ends the current block."]
    pub fn insert_terminate_invocation(&mut self, insert_point: InsertPoint) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(spirv::Op::TerminateInvocation, None, None, vec![]);
        self.insert_end_block(insert_point, inst)
    }
    #[doc = "Appends an OpIgnoreIntersectionKHR instruction and ends the current block."]
    pub fn ignore_intersection_khr(&mut self) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(spirv::Op::IgnoreIntersectionKHR, None, None, vec![]);
        self.end_block(inst)
    }
    #[doc = "Insert an OpIgnoreIntersectionKHR instruction and ends the current block."]
    pub fn insert_ignore_intersection_khr(&mut self, insert_point: InsertPoint) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(spirv::Op::IgnoreIntersectionKHR, None, None, vec![]);
        self.insert_end_block(insert_point, inst)
    }
    #[doc = "Appends an OpTerminateRayKHR instruction and ends the current block."]
    pub fn terminate_ray_khr(&mut self) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(spirv::Op::TerminateRayKHR, None, None, vec![]);
        self.end_block(inst)
    }
    #[doc = "Insert an OpTerminateRayKHR instruction and ends the current block."]
    pub fn insert_terminate_ray_khr(&mut self, insert_point: InsertPoint) -> BuildResult<()> {
        #[allow(unused_mut)]
        let mut inst = dr::Instruction::new(spirv::Op::TerminateRayKHR, None, None, vec![]);
        self.insert_end_block(insert_point, inst)
    }
}
