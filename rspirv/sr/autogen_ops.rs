// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

use crate::sr::{module::Jump, storage::Token, Type};
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Branch {
    Branch {
        target_label: spirv::Word,
    },
    BranchConditional {
        condition: spirv::Word,
        true_label: spirv::Word,
        false_label: spirv::Word,
        branch_weights: Vec<u32>,
    },
    Switch {
        selector: spirv::Word,
        default: spirv::Word,
        target: Vec<(u32, Jump)>,
    },
    Return,
    ReturnValue {
        value: spirv::Word,
    },
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Terminator {
    Branch(Branch),
    Kill,
    Unreachable,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Op {
    Nop,
    Undef,
    SourceContinued {
        continued_source: String,
    },
    Source {
        source_language: spirv::SourceLanguage,
        version: u32,
        file: Option<spirv::Word>,
        source: Option<String>,
    },
    SourceExtension {
        extension: String,
    },
    String {
        string: String,
    },
    Line {
        file: spirv::Word,
        line: u32,
        column: u32,
    },
    ExtInst {
        set: spirv::Word,
        instruction: u32,
        operand_1_operand_2: Vec<spirv::Word>,
    },
    FunctionCall {
        function: spirv::Word,
        argument_0_argument_1: Vec<spirv::Word>,
    },
    Variable {
        storage_class: spirv::StorageClass,
        initializer: Option<spirv::Word>,
    },
    ImageTexelPointer {
        image: spirv::Word,
        coordinate: spirv::Word,
        sample: spirv::Word,
    },
    Load {
        pointer: spirv::Word,
        memory_access: Option<spirv::MemoryAccess>,
    },
    Store {
        pointer: spirv::Word,
        object: spirv::Word,
        memory_access: Option<spirv::MemoryAccess>,
    },
    CopyMemory {
        target: spirv::Word,
        source: spirv::Word,
        memory_access: Option<spirv::MemoryAccess>,
    },
    CopyMemorySized {
        target: spirv::Word,
        source: spirv::Word,
        size: spirv::Word,
        memory_access: Option<spirv::MemoryAccess>,
    },
    AccessChain {
        base: spirv::Word,
        indexes: Vec<spirv::Word>,
    },
    InBoundsAccessChain {
        base: spirv::Word,
        indexes: Vec<spirv::Word>,
    },
    PtrAccessChain {
        base: spirv::Word,
        element: spirv::Word,
        indexes: Vec<spirv::Word>,
    },
    ArrayLength {
        structure: spirv::Word,
        array_member: u32,
    },
    GenericPtrMemSemantics {
        pointer: spirv::Word,
    },
    InBoundsPtrAccessChain {
        base: spirv::Word,
        element: spirv::Word,
        indexes: Vec<spirv::Word>,
    },
    Decorate {
        target: spirv::Word,
        decoration: spirv::Decoration,
    },
    MemberDecorate {
        structure_type: Token<Type>,
        member: u32,
        decoration: spirv::Decoration,
    },
    DecorationGroup,
    GroupDecorate {
        decoration_group: spirv::Word,
        targets: Vec<spirv::Word>,
    },
    GroupMemberDecorate {
        decoration_group: spirv::Word,
        targets: Vec<(Jump, u32)>,
    },
    VectorExtractDynamic {
        vector: spirv::Word,
        index: spirv::Word,
    },
    VectorInsertDynamic {
        vector: spirv::Word,
        component: spirv::Word,
        index: spirv::Word,
    },
    VectorShuffle {
        vector_1: spirv::Word,
        vector_2: spirv::Word,
        components: Vec<u32>,
    },
    CompositeConstruct {
        constituents: Vec<spirv::Word>,
    },
    CompositeExtract {
        composite: spirv::Word,
        indexes: Vec<u32>,
    },
    CompositeInsert {
        object: spirv::Word,
        composite: spirv::Word,
        indexes: Vec<u32>,
    },
    CopyObject {
        operand: spirv::Word,
    },
    Transpose {
        matrix: spirv::Word,
    },
    SampledImage {
        image: spirv::Word,
        sampler: spirv::Word,
    },
    ImageSampleImplicitLod {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
    },
    ImageSampleExplicitLod {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: spirv::ImageOperands,
    },
    ImageSampleDrefImplicitLod {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        d_ref: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
    },
    ImageSampleDrefExplicitLod {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        d_ref: spirv::Word,
        image_operands: spirv::ImageOperands,
    },
    ImageSampleProjImplicitLod {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
    },
    ImageSampleProjExplicitLod {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: spirv::ImageOperands,
    },
    ImageSampleProjDrefImplicitLod {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        d_ref: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
    },
    ImageSampleProjDrefExplicitLod {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        d_ref: spirv::Word,
        image_operands: spirv::ImageOperands,
    },
    ImageFetch {
        image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
    },
    ImageGather {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        component: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
    },
    ImageDrefGather {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        d_ref: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
    },
    ImageRead {
        image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
    },
    ImageWrite {
        image: spirv::Word,
        coordinate: spirv::Word,
        texel: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
    },
    Image {
        sampled_image: spirv::Word,
    },
    ImageQueryFormat {
        image: spirv::Word,
    },
    ImageQueryOrder {
        image: spirv::Word,
    },
    ImageQuerySizeLod {
        image: spirv::Word,
        level_of_detail: spirv::Word,
    },
    ImageQuerySize {
        image: spirv::Word,
    },
    ImageQueryLod {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
    },
    ImageQueryLevels {
        image: spirv::Word,
    },
    ImageQuerySamples {
        image: spirv::Word,
    },
    ConvertFToU {
        float_value: spirv::Word,
    },
    ConvertFToS {
        float_value: spirv::Word,
    },
    ConvertSToF {
        signed_value: spirv::Word,
    },
    ConvertUToF {
        unsigned_value: spirv::Word,
    },
    UConvert {
        unsigned_value: spirv::Word,
    },
    SConvert {
        signed_value: spirv::Word,
    },
    FConvert {
        float_value: spirv::Word,
    },
    QuantizeToF16 {
        value: spirv::Word,
    },
    ConvertPtrToU {
        pointer: spirv::Word,
    },
    SatConvertSToU {
        signed_value: spirv::Word,
    },
    SatConvertUToS {
        unsigned_value: spirv::Word,
    },
    ConvertUToPtr {
        integer_value: spirv::Word,
    },
    PtrCastToGeneric {
        pointer: spirv::Word,
    },
    GenericCastToPtr {
        pointer: spirv::Word,
    },
    GenericCastToPtrExplicit {
        pointer: spirv::Word,
        storage: spirv::StorageClass,
    },
    Bitcast {
        operand: spirv::Word,
    },
    SNegate {
        operand: spirv::Word,
    },
    FNegate {
        operand: spirv::Word,
    },
    IAdd {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    FAdd {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    ISub {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    FSub {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    IMul {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    FMul {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    UDiv {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    SDiv {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    FDiv {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    UMod {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    SRem {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    SMod {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    FRem {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    FMod {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    VectorTimesScalar {
        vector: spirv::Word,
        scalar: spirv::Word,
    },
    MatrixTimesScalar {
        matrix: spirv::Word,
        scalar: spirv::Word,
    },
    VectorTimesMatrix {
        vector: spirv::Word,
        matrix: spirv::Word,
    },
    MatrixTimesVector {
        matrix: spirv::Word,
        vector: spirv::Word,
    },
    MatrixTimesMatrix {
        left_matrix: spirv::Word,
        right_matrix: spirv::Word,
    },
    OuterProduct {
        vector_1: spirv::Word,
        vector_2: spirv::Word,
    },
    Dot {
        vector_1: spirv::Word,
        vector_2: spirv::Word,
    },
    IAddCarry {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    ISubBorrow {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    UMulExtended {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    SMulExtended {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    Any {
        vector: spirv::Word,
    },
    All {
        vector: spirv::Word,
    },
    IsNan {
        x: spirv::Word,
    },
    IsInf {
        x: spirv::Word,
    },
    IsFinite {
        x: spirv::Word,
    },
    IsNormal {
        x: spirv::Word,
    },
    SignBitSet {
        x: spirv::Word,
    },
    LessOrGreater {
        x: spirv::Word,
        y: spirv::Word,
    },
    Ordered {
        x: spirv::Word,
        y: spirv::Word,
    },
    Unordered {
        x: spirv::Word,
        y: spirv::Word,
    },
    LogicalEqual {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    LogicalNotEqual {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    LogicalOr {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    LogicalAnd {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    LogicalNot {
        operand: spirv::Word,
    },
    Select {
        condition: spirv::Word,
        object_1: spirv::Word,
        object_2: spirv::Word,
    },
    IEqual {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    INotEqual {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    UGreaterThan {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    SGreaterThan {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    UGreaterThanEqual {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    SGreaterThanEqual {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    ULessThan {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    SLessThan {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    ULessThanEqual {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    SLessThanEqual {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    FOrdEqual {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    FUnordEqual {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    FOrdNotEqual {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    FUnordNotEqual {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    FOrdLessThan {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    FUnordLessThan {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    FOrdGreaterThan {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    FUnordGreaterThan {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    FOrdLessThanEqual {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    FUnordLessThanEqual {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    FOrdGreaterThanEqual {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    FUnordGreaterThanEqual {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    ShiftRightLogical {
        base: spirv::Word,
        shift: spirv::Word,
    },
    ShiftRightArithmetic {
        base: spirv::Word,
        shift: spirv::Word,
    },
    ShiftLeftLogical {
        base: spirv::Word,
        shift: spirv::Word,
    },
    BitwiseOr {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    BitwiseXor {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    BitwiseAnd {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    Not {
        operand: spirv::Word,
    },
    BitFieldInsert {
        base: spirv::Word,
        insert: spirv::Word,
        offset: spirv::Word,
        count: spirv::Word,
    },
    BitFieldSExtract {
        base: spirv::Word,
        offset: spirv::Word,
        count: spirv::Word,
    },
    BitFieldUExtract {
        base: spirv::Word,
        offset: spirv::Word,
        count: spirv::Word,
    },
    BitReverse {
        base: spirv::Word,
    },
    BitCount {
        base: spirv::Word,
    },
    DPdx {
        p: spirv::Word,
    },
    DPdy {
        p: spirv::Word,
    },
    Fwidth {
        p: spirv::Word,
    },
    DPdxFine {
        p: spirv::Word,
    },
    DPdyFine {
        p: spirv::Word,
    },
    FwidthFine {
        p: spirv::Word,
    },
    DPdxCoarse {
        p: spirv::Word,
    },
    DPdyCoarse {
        p: spirv::Word,
    },
    FwidthCoarse {
        p: spirv::Word,
    },
    EmitVertex,
    EndPrimitive,
    EmitStreamVertex {
        stream: spirv::Word,
    },
    EndStreamPrimitive {
        stream: spirv::Word,
    },
    ControlBarrier {
        execution: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
    },
    MemoryBarrier {
        memory: spirv::Word,
        semantics: spirv::Word,
    },
    AtomicLoad {
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
    },
    AtomicStore {
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    },
    AtomicExchange {
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    },
    AtomicCompareExchange {
        pointer: spirv::Word,
        memory: spirv::Word,
        equal: spirv::Word,
        unequal: spirv::Word,
        value: spirv::Word,
        comparator: spirv::Word,
    },
    AtomicCompareExchangeWeak {
        pointer: spirv::Word,
        memory: spirv::Word,
        equal: spirv::Word,
        unequal: spirv::Word,
        value: spirv::Word,
        comparator: spirv::Word,
    },
    AtomicIIncrement {
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
    },
    AtomicIDecrement {
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
    },
    AtomicIAdd {
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    },
    AtomicISub {
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    },
    AtomicSMin {
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    },
    AtomicUMin {
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    },
    AtomicSMax {
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    },
    AtomicUMax {
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    },
    AtomicAnd {
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    },
    AtomicOr {
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    },
    AtomicXor {
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
        value: spirv::Word,
    },
    LoopMerge {
        merge_block: spirv::Word,
        continue_target: spirv::Word,
        loop_control: spirv::LoopControl,
    },
    SelectionMerge {
        merge_block: spirv::Word,
        selection_control: spirv::SelectionControl,
    },
    LifetimeStart {
        pointer: spirv::Word,
        size: u32,
    },
    LifetimeStop {
        pointer: spirv::Word,
        size: u32,
    },
    GroupAsyncCopy {
        execution: spirv::Word,
        destination: spirv::Word,
        source: spirv::Word,
        num_elements: spirv::Word,
        stride: spirv::Word,
        event: spirv::Word,
    },
    GroupWaitEvents {
        execution: spirv::Word,
        num_events: spirv::Word,
        events_list: spirv::Word,
    },
    GroupAll {
        execution: spirv::Word,
        predicate: spirv::Word,
    },
    GroupAny {
        execution: spirv::Word,
        predicate: spirv::Word,
    },
    GroupBroadcast {
        execution: spirv::Word,
        value: spirv::Word,
        local_id: spirv::Word,
    },
    GroupIAdd {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    },
    GroupFAdd {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    },
    GroupFMin {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    },
    GroupUMin {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    },
    GroupSMin {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    },
    GroupFMax {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    },
    GroupUMax {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    },
    GroupSMax {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    },
    ReadPipe {
        pipe: spirv::Word,
        pointer: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    },
    WritePipe {
        pipe: spirv::Word,
        pointer: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    },
    ReservedReadPipe {
        pipe: spirv::Word,
        reserve_id: spirv::Word,
        index: spirv::Word,
        pointer: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    },
    ReservedWritePipe {
        pipe: spirv::Word,
        reserve_id: spirv::Word,
        index: spirv::Word,
        pointer: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    },
    ReserveReadPipePackets {
        pipe: spirv::Word,
        num_packets: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    },
    ReserveWritePipePackets {
        pipe: spirv::Word,
        num_packets: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    },
    CommitReadPipe {
        pipe: spirv::Word,
        reserve_id: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    },
    CommitWritePipe {
        pipe: spirv::Word,
        reserve_id: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    },
    IsValidReserveId {
        reserve_id: spirv::Word,
    },
    GetNumPipePackets {
        pipe: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    },
    GetMaxPipePackets {
        pipe: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    },
    GroupReserveReadPipePackets {
        execution: spirv::Word,
        pipe: spirv::Word,
        num_packets: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    },
    GroupReserveWritePipePackets {
        execution: spirv::Word,
        pipe: spirv::Word,
        num_packets: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    },
    GroupCommitReadPipe {
        execution: spirv::Word,
        pipe: spirv::Word,
        reserve_id: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    },
    GroupCommitWritePipe {
        execution: spirv::Word,
        pipe: spirv::Word,
        reserve_id: spirv::Word,
        packet_size: spirv::Word,
        packet_alignment: spirv::Word,
    },
    EnqueueMarker {
        queue: spirv::Word,
        num_events: spirv::Word,
        wait_events: spirv::Word,
        ret_event: spirv::Word,
    },
    EnqueueKernel {
        queue: spirv::Word,
        flags: spirv::Word,
        nd_range: spirv::Word,
        num_events: spirv::Word,
        wait_events: spirv::Word,
        ret_event: spirv::Word,
        invoke: spirv::Word,
        param: spirv::Word,
        param_size: spirv::Word,
        param_align: spirv::Word,
        local_size: Vec<spirv::Word>,
    },
    GetKernelNDrangeSubGroupCount {
        nd_range: spirv::Word,
        invoke: spirv::Word,
        param: spirv::Word,
        param_size: spirv::Word,
        param_align: spirv::Word,
    },
    GetKernelNDrangeMaxSubGroupSize {
        nd_range: spirv::Word,
        invoke: spirv::Word,
        param: spirv::Word,
        param_size: spirv::Word,
        param_align: spirv::Word,
    },
    GetKernelWorkGroupSize {
        invoke: spirv::Word,
        param: spirv::Word,
        param_size: spirv::Word,
        param_align: spirv::Word,
    },
    GetKernelPreferredWorkGroupSizeMultiple {
        invoke: spirv::Word,
        param: spirv::Word,
        param_size: spirv::Word,
        param_align: spirv::Word,
    },
    RetainEvent {
        event: spirv::Word,
    },
    ReleaseEvent {
        event: spirv::Word,
    },
    CreateUserEvent,
    IsValidEvent {
        event: spirv::Word,
    },
    SetUserEventStatus {
        event: spirv::Word,
        status: spirv::Word,
    },
    CaptureEventProfilingInfo {
        event: spirv::Word,
        profiling_info: spirv::Word,
        value: spirv::Word,
    },
    GetDefaultQueue,
    BuildNDRange {
        global_work_size: spirv::Word,
        local_work_size: spirv::Word,
        global_work_offset: spirv::Word,
    },
    ImageSparseSampleImplicitLod {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
    },
    ImageSparseSampleExplicitLod {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: spirv::ImageOperands,
    },
    ImageSparseSampleDrefImplicitLod {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        d_ref: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
    },
    ImageSparseSampleDrefExplicitLod {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        d_ref: spirv::Word,
        image_operands: spirv::ImageOperands,
    },
    ImageSparseSampleProjImplicitLod {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
    },
    ImageSparseSampleProjExplicitLod {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: spirv::ImageOperands,
    },
    ImageSparseSampleProjDrefImplicitLod {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        d_ref: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
    },
    ImageSparseSampleProjDrefExplicitLod {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        d_ref: spirv::Word,
        image_operands: spirv::ImageOperands,
    },
    ImageSparseFetch {
        image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
    },
    ImageSparseGather {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        component: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
    },
    ImageSparseDrefGather {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        d_ref: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
    },
    ImageSparseTexelsResident {
        resident_code: spirv::Word,
    },
    NoLine,
    AtomicFlagTestAndSet {
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
    },
    AtomicFlagClear {
        pointer: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
    },
    ImageSparseRead {
        image: spirv::Word,
        coordinate: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
    },
    SizeOf {
        pointer: spirv::Word,
    },
    CreatePipeFromPipeStorage {
        pipe_storage: spirv::Word,
    },
    GetKernelLocalSizeForSubgroupCount {
        subgroup_count: spirv::Word,
        invoke: spirv::Word,
        param: spirv::Word,
        param_size: spirv::Word,
        param_align: spirv::Word,
    },
    GetKernelMaxNumSubgroups {
        invoke: spirv::Word,
        param: spirv::Word,
        param_size: spirv::Word,
        param_align: spirv::Word,
    },
    NamedBarrierInitialize {
        subgroup_count: spirv::Word,
    },
    MemoryNamedBarrier {
        named_barrier: spirv::Word,
        memory: spirv::Word,
        semantics: spirv::Word,
    },
    ModuleProcessed {
        process: String,
    },
    DecorateId {
        target: spirv::Word,
        decoration: spirv::Decoration,
    },
    GroupNonUniformElect {
        execution: spirv::Word,
    },
    GroupNonUniformAll {
        execution: spirv::Word,
        predicate: spirv::Word,
    },
    GroupNonUniformAny {
        execution: spirv::Word,
        predicate: spirv::Word,
    },
    GroupNonUniformAllEqual {
        execution: spirv::Word,
        value: spirv::Word,
    },
    GroupNonUniformBroadcast {
        execution: spirv::Word,
        value: spirv::Word,
        id: spirv::Word,
    },
    GroupNonUniformBroadcastFirst {
        execution: spirv::Word,
        value: spirv::Word,
    },
    GroupNonUniformBallot {
        execution: spirv::Word,
        predicate: spirv::Word,
    },
    GroupNonUniformInverseBallot {
        execution: spirv::Word,
        value: spirv::Word,
    },
    GroupNonUniformBallotBitExtract {
        execution: spirv::Word,
        value: spirv::Word,
        index: spirv::Word,
    },
    GroupNonUniformBallotBitCount {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
    },
    GroupNonUniformBallotFindLSB {
        execution: spirv::Word,
        value: spirv::Word,
    },
    GroupNonUniformBallotFindMSB {
        execution: spirv::Word,
        value: spirv::Word,
    },
    GroupNonUniformShuffle {
        execution: spirv::Word,
        value: spirv::Word,
        id: spirv::Word,
    },
    GroupNonUniformShuffleXor {
        execution: spirv::Word,
        value: spirv::Word,
        mask: spirv::Word,
    },
    GroupNonUniformShuffleUp {
        execution: spirv::Word,
        value: spirv::Word,
        delta: spirv::Word,
    },
    GroupNonUniformShuffleDown {
        execution: spirv::Word,
        value: spirv::Word,
        delta: spirv::Word,
    },
    GroupNonUniformIAdd {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    },
    GroupNonUniformFAdd {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    },
    GroupNonUniformIMul {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    },
    GroupNonUniformFMul {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    },
    GroupNonUniformSMin {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    },
    GroupNonUniformUMin {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    },
    GroupNonUniformFMin {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    },
    GroupNonUniformSMax {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    },
    GroupNonUniformUMax {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    },
    GroupNonUniformFMax {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    },
    GroupNonUniformBitwiseAnd {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    },
    GroupNonUniformBitwiseOr {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    },
    GroupNonUniformBitwiseXor {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    },
    GroupNonUniformLogicalAnd {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    },
    GroupNonUniformLogicalOr {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    },
    GroupNonUniformLogicalXor {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        value: spirv::Word,
        cluster_size: Option<spirv::Word>,
    },
    GroupNonUniformQuadBroadcast {
        execution: spirv::Word,
        value: spirv::Word,
        index: spirv::Word,
    },
    GroupNonUniformQuadSwap {
        execution: spirv::Word,
        value: spirv::Word,
        direction: spirv::Word,
    },
    CopyLogical {
        operand: spirv::Word,
    },
    PtrEqual {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    PtrNotEqual {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    PtrDiff {
        operand_1: spirv::Word,
        operand_2: spirv::Word,
    },
    SubgroupBallotKHR {
        predicate: spirv::Word,
    },
    SubgroupFirstInvocationKHR {
        value: spirv::Word,
    },
    SubgroupAllKHR {
        predicate: spirv::Word,
    },
    SubgroupAnyKHR {
        predicate: spirv::Word,
    },
    SubgroupAllEqualKHR {
        predicate: spirv::Word,
    },
    SubgroupReadInvocationKHR {
        value: spirv::Word,
        index: spirv::Word,
    },
    GroupIAddNonUniformAMD {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    },
    GroupFAddNonUniformAMD {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    },
    GroupFMinNonUniformAMD {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    },
    GroupUMinNonUniformAMD {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    },
    GroupSMinNonUniformAMD {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    },
    GroupFMaxNonUniformAMD {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    },
    GroupUMaxNonUniformAMD {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    },
    GroupSMaxNonUniformAMD {
        execution: spirv::Word,
        operation: spirv::GroupOperation,
        x: spirv::Word,
    },
    FragmentMaskFetchAMD {
        image: spirv::Word,
        coordinate: spirv::Word,
    },
    FragmentFetchAMD {
        image: spirv::Word,
        coordinate: spirv::Word,
        fragment_index: spirv::Word,
    },
    ImageSampleFootprintNV {
        sampled_image: spirv::Word,
        coordinate: spirv::Word,
        granularity: spirv::Word,
        coarse: spirv::Word,
        image_operands: Option<spirv::ImageOperands>,
    },
    GroupNonUniformPartitionNV {
        value: spirv::Word,
    },
    WritePackedPrimitiveIndices4x8NV {
        index_offset: spirv::Word,
        packed_indices: spirv::Word,
    },
    ReportIntersectionNV {
        hit: spirv::Word,
        hit_kind: spirv::Word,
    },
    IgnoreIntersectionNV,
    TerminateRayNV,
    TraceNV {
        accel: spirv::Word,
        ray_flags: spirv::Word,
        cull_mask: spirv::Word,
        sbt_offset: spirv::Word,
        sbt_stride: spirv::Word,
        miss_index: spirv::Word,
        ray_origin: spirv::Word,
        ray_tmin: spirv::Word,
        ray_direction: spirv::Word,
        ray_tmax: spirv::Word,
        payload_id: spirv::Word,
    },
    TypeAccelerationStructureNV,
    ExecuteCallableNV {
        sbt_index: spirv::Word,
        callable_data_id: spirv::Word,
    },
    TypeCooperativeMatrixNV {
        component_type: Token<Type>,
        execution: spirv::Word,
        rows: spirv::Word,
        columns: spirv::Word,
    },
    CooperativeMatrixLoadNV {
        pointer: spirv::Word,
        stride: spirv::Word,
        column_major: spirv::Word,
        memory_access: Option<spirv::MemoryAccess>,
    },
    CooperativeMatrixStoreNV {
        pointer: spirv::Word,
        object: spirv::Word,
        stride: spirv::Word,
        column_major: spirv::Word,
        memory_access: Option<spirv::MemoryAccess>,
    },
    CooperativeMatrixMulAddNV {
        a: spirv::Word,
        b: spirv::Word,
        c: spirv::Word,
    },
    CooperativeMatrixLengthNV {
        ty: spirv::Word,
    },
    SubgroupShuffleINTEL {
        data: spirv::Word,
        invocation_id: spirv::Word,
    },
    SubgroupShuffleDownINTEL {
        current: spirv::Word,
        next: spirv::Word,
        delta: spirv::Word,
    },
    SubgroupShuffleUpINTEL {
        previous: spirv::Word,
        current: spirv::Word,
        delta: spirv::Word,
    },
    SubgroupShuffleXorINTEL {
        data: spirv::Word,
        value: spirv::Word,
    },
    SubgroupBlockReadINTEL {
        ptr: spirv::Word,
    },
    SubgroupBlockWriteINTEL {
        ptr: spirv::Word,
        data: spirv::Word,
    },
    SubgroupImageBlockReadINTEL {
        image: spirv::Word,
        coordinate: spirv::Word,
    },
    SubgroupImageBlockWriteINTEL {
        image: spirv::Word,
        coordinate: spirv::Word,
        data: spirv::Word,
    },
    SubgroupImageMediaBlockReadINTEL {
        image: spirv::Word,
        coordinate: spirv::Word,
        width: spirv::Word,
        height: spirv::Word,
    },
    SubgroupImageMediaBlockWriteINTEL {
        image: spirv::Word,
        coordinate: spirv::Word,
        width: spirv::Word,
        height: spirv::Word,
        data: spirv::Word,
    },
    DecorateString {
        target: spirv::Word,
        decoration: spirv::Decoration,
    },
    MemberDecorateStringGOOGLE {
        struct_type: Token<Type>,
        member: u32,
        decoration: spirv::Decoration,
    },
}
