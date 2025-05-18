use crate::jolt::{
    instruction::{
        add::ADDInstruction, and::ANDInstruction, beq::BEQInstruction, bge::BGEInstruction,
        bgeu::BGEUInstruction, bne::BNEInstruction, mul::MULInstruction, mulhu::MULHUInstruction,
        mulu::MULUInstruction, or::ORInstruction, sll::SLLInstruction, slt::SLTInstruction,
        sltu::SLTUInstruction, sra::SRAInstruction, srl::SRLInstruction, sub::SUBInstruction,
        virtual_advice::ADVICEInstruction,
        virtual_assert_halfword_alignment::AssertHalfwordAlignmentInstruction,
        virtual_assert_lte::ASSERTLTEInstruction,
        virtual_assert_valid_div0::AssertValidDiv0Instruction,
        virtual_assert_valid_signed_remainder::AssertValidSignedRemainderInstruction,
        virtual_assert_valid_unsigned_remainder::AssertValidUnsignedRemainderInstruction,
        virtual_move::MOVEInstruction, virtual_movsign::MOVSIGNInstruction,
        virtual_pow2::POW2Instruction, virtual_right_shift_padding::RightShiftPaddingInstruction,
        xor::XORInstruction,
    },
    vm::rv32i_vm::{RV32I, WORD_SIZE, WORD_SIZE_1},
};
use common::rv_trace::{ELFInstruction, RVTraceRow, WASM};

impl TryFrom<&ELFInstruction> for RV32I {
    type Error = &'static str;

    // TODO: DOUBLE CHECK ALL NECESSARY INSTRUCTIONS ARE IMPLEMENTED
    #[rustfmt::skip] // keep matches pretty
    fn try_from(instruction: &ELFInstruction) -> Result<Self, Self::Error> {
        match instruction.opcode {
            WASM::I32ADD  => Ok(ADDInstruction::<WORD_SIZE>::default().into()),
            WASM::I32SUB  => Ok(SUBInstruction::<WORD_SIZE>::default().into()),
            WASM::I32MUL  => Ok(MULInstruction::<WORD_SIZE>::default().into()),
            WASM::I32XOR  => Ok(XORInstruction::<WORD_SIZE>::default().into()),
            WASM::I32OR   => Ok(ORInstruction::<WORD_SIZE>::default().into()),
            WASM::I32AND  => Ok(ANDInstruction::<WORD_SIZE>::default().into()),
            WASM::I32SHL  => Ok(SLLInstruction::<WORD_SIZE>::default().into()),
            WASM::I32SHRU  => Ok(SRLInstruction::<WORD_SIZE>::default().into()),
            WASM::I32SHRS  => Ok(SRAInstruction::<WORD_SIZE>::default().into()),

            // immediates
            WASM::I32MULI => Ok(MULInstruction::<WORD_SIZE>::default().into()),
            WASM::I32ADDI  => Ok(ADDInstruction::<WORD_SIZE>::default().into()),
            WASM::I32XORI  => Ok(XORInstruction::<WORD_SIZE>::default().into()),
            WASM::I32ORI   => Ok(ORInstruction::<WORD_SIZE>::default().into()),
            WASM::I32ANDI  => Ok(ANDInstruction::<WORD_SIZE>::default().into()),

            WASM::I64ADD  => Ok(ADDInstruction::<WORD_SIZE_1>::default().into()),
            WASM::I64SUB  => Ok(SUBInstruction::<WORD_SIZE_1>::default().into()),
            WASM::I64MUL  => Ok(MULInstruction::<WORD_SIZE_1>::default().into()),
            WASM::I64XOR  => Ok(XORInstruction::<WORD_SIZE_1>::default().into()),
            WASM::I64OR   => Ok(ORInstruction::<WORD_SIZE_1>::default().into()),
            WASM::I64AND  => Ok(ANDInstruction::<WORD_SIZE_1>::default().into()),

            // i64 immediates
            WASM::I64MULI => Ok(MULInstruction::<WORD_SIZE_1>::default().into()),
            WASM::I64ADDI  => Ok(ADDInstruction::<WORD_SIZE_1>::default().into()),
            WASM::I64XORI  => Ok(XORInstruction::<WORD_SIZE_1>::default().into()),
            WASM::I64ORI   => Ok(ORInstruction::<WORD_SIZE_1>::default().into()),
            WASM::I64ANDI  => Ok(ANDInstruction::<WORD_SIZE_1>::default().into()),

            WASM::I64SHL  => Ok(SLLInstruction::<WORD_SIZE_1>::default().into()),
            WASM::I64SHRU => Ok(SRLInstruction::<WORD_SIZE_1>::default().into()),
            WASM::I64SHRS => Ok(SRAInstruction::<WORD_SIZE_1>::default().into()),

            // WASM::SLL  => Ok(SLLInstruction::default().into()),
            // WASM::SRL  => Ok(SRLInstruction::default().into()),
            // WASM::SRA  => Ok(SRAInstruction::default().into()),
            // WASM::SLT  => Ok(SLTInstruction::default().into()),
            // WASM::SLTU => Ok(SLTUInstruction::default().into()),


            // WASM::SLLI  => Ok(SLLInstruction::default().into()),
            // WASM::SRLI  => Ok(SRLInstruction::default().into()),
            // WASM::SRAI  => Ok(SRAInstruction::default().into()),
            // WASM::SLTI  => Ok(SLTInstruction::default().into()),
            // WASM::SLTIU => Ok(SLTUInstruction::default().into()),

            // WASM::BEQ  => Ok(BEQInstruction::default().into()),
            // WASM::BNE  => Ok(BNEInstruction::default().into()),
            // WASM::BLT  => Ok(SLTInstruction::default().into()),
            // WASM::BLTU => Ok(SLTUInstruction::default().into()),
            // WASM::BGE  => Ok(BGEInstruction::default().into()),
            // WASM::BGEU => Ok(BGEUInstruction::default().into()),

            WASM::JAL   => Ok(ADDInstruction::<WORD_SIZE>::default().into()),
            WASM::JALR  => Ok(ADDInstruction::<WORD_SIZE>::default().into()),
            WASM::AUIPC => Ok(ADDInstruction::<WORD_SIZE>::default().into()),
            // WASM::LUI => Ok(ADVICEInstruction::default().into()),

            // WASM::MUL => Ok(MULInstruction::default().into()),
            WASM::MULU => Ok(MULUInstruction::<WORD_SIZE>::default().into()),
            WASM::VIRTUAL_ASSERT_EQ => Ok(BEQInstruction::<WORD_SIZE>::default().into()),
            WASM::VIRTUAL_ASSERT_LTE => Ok(ASSERTLTEInstruction::<WORD_SIZE>::default().into()),
            WASM::VIRTUAL_ASSERT_VALID_UNSIGNED_REMAINDER => Ok(AssertValidUnsignedRemainderInstruction::<WORD_SIZE>::default().into()),
            WASM::VIRTUAL_ASSERT_VALID_DIV0 => Ok(AssertValidDiv0Instruction::<WORD_SIZE>::default().into()),
            WASM::VIRTUAL_ADVICE => Ok(ADVICEInstruction::<WORD_SIZE>::default().into()),
            WASM::VIRTUAL_MOVE => Ok(MOVEInstruction::<WORD_SIZE>::default().into()),
            // WASM::MULHU => Ok(MULHUInstruction::default().into()),

            // WASM::VIRTUAL_MOVSIGN => Ok(MOVSIGNInstruction::default().into()),

            // WASM::VIRTUAL_ASSERT_HALFWORD_ALIGNMENT => Ok(AssertHalfwordAlignmentInstruction::<32>::default().into()),
            // WASM::VIRTUAL_POW2 => Ok(POW2Instruction::<32>::default().into()),
            // WASM::VIRTUAL_POW2I => Ok(POW2Instruction::<32>::default().into()),
            // WASM::VIRTUAL_SRA_PAD => Ok(RightShiftPaddingInstruction::<32>::default().into()),
            // WASM::VIRTUAL_SRA_PADI => Ok(RightShiftPaddingInstruction::<32>::default().into()),
                        // WASM::VIRTUAL_ASSERT_VALID_SIGNED_REMAINDER => Ok(AssertValidSignedRemainderInstruction::default().into()),
            _ => Err("No corresponding RV32I instruction")
        }
    }
}

impl TryFrom<&RVTraceRow> for RV32I {
    type Error = &'static str;

    #[rustfmt::skip] // keep matches pretty
    fn try_from(row: &RVTraceRow) -> Result<Self, Self::Error> {
        match row.instruction.opcode {
            WASM::I32ADD => Ok(ADDInstruction::<WORD_SIZE>(row.register_state.rs1_val.unwrap(), row.register_state.rs2_val.unwrap()).into()),
            WASM::I32SUB => Ok(SUBInstruction::<WORD_SIZE>(row.register_state.rs1_val.unwrap(), row.register_state.rs2_val.unwrap()).into()),
            WASM::I32MUL => Ok(MULInstruction::<WORD_SIZE>(row.register_state.rs1_val.unwrap(), row.register_state.rs2_val.unwrap()).into()),
            WASM::I32XOR => Ok(XORInstruction::<WORD_SIZE>(row.register_state.rs1_val.unwrap(), row.register_state.rs2_val.unwrap()).into()),
            WASM::I32OR  => Ok(ORInstruction::<WORD_SIZE>(row.register_state.rs1_val.unwrap(), row.register_state.rs2_val.unwrap()).into()),
            WASM::I32AND => Ok(ANDInstruction::<WORD_SIZE>(row.register_state.rs1_val.unwrap(), row.register_state.rs2_val.unwrap()).into()),
            WASM::I32SHL => Ok(SLLInstruction::<WORD_SIZE>(row.register_state.rs1_val.unwrap(), row.register_state.rs2_val.unwrap()).into()),
            WASM::I32SHRU => Ok(SRLInstruction::<WORD_SIZE>(row.register_state.rs1_val.unwrap(), row.register_state.rs2_val.unwrap()).into()),
            WASM::I32SHRS => Ok(SRAInstruction::<WORD_SIZE>(row.register_state.rs1_val.unwrap(), row.register_state.rs2_val.unwrap()).into()),

            // immediates
            WASM::I32MULI => Ok(MULInstruction::<WORD_SIZE>(row.register_state.rs1_val.unwrap(), row.imm_u32() as u64).into()),
            WASM::I32ADDI  => Ok(ADDInstruction::<WORD_SIZE>(row.register_state.rs1_val.unwrap(), row.imm_u32() as u64).into()),
            WASM::I32XORI  => Ok(XORInstruction::<WORD_SIZE>(row.register_state.rs1_val.unwrap(), row.imm_u32() as u64).into()),
            WASM::I32ORI   => Ok(ORInstruction::<WORD_SIZE>(row.register_state.rs1_val.unwrap(), row.imm_u32() as u64).into()),
            WASM::I32ANDI  => Ok(ANDInstruction::<WORD_SIZE>(row.register_state.rs1_val.unwrap(), row.imm_u32() as u64).into()),

            WASM::I64ADD => Ok(ADDInstruction::<WORD_SIZE_1>(row.register_state.rs1_val.unwrap(), row.register_state.rs2_val.unwrap()).into()),
            WASM::I64SUB => Ok(SUBInstruction::<WORD_SIZE_1>(row.register_state.rs1_val.unwrap(), row.register_state.rs2_val.unwrap()).into()),
            WASM::I64MUL => Ok(MULInstruction::<WORD_SIZE_1>(row.register_state.rs1_val.unwrap(), row.register_state.rs2_val.unwrap()).into()),
            WASM::I64XOR => Ok(XORInstruction::<WORD_SIZE_1>(row.register_state.rs1_val.unwrap(), row.register_state.rs2_val.unwrap()).into()),
            WASM::I64OR  => Ok(ORInstruction::<WORD_SIZE_1>(row.register_state.rs1_val.unwrap(), row.register_state.rs2_val.unwrap()).into()),
            WASM::I64AND => Ok(ANDInstruction::<WORD_SIZE_1>(row.register_state.rs1_val.unwrap(), row.register_state.rs2_val.unwrap()).into()),
            WASM::I64SHL => Ok(SLLInstruction::<WORD_SIZE_1>(row.register_state.rs1_val.unwrap(), row.register_state.rs2_val.unwrap()).into()),
            WASM::I64SHRU => Ok(SRLInstruction::<WORD_SIZE_1>(row.register_state.rs1_val.unwrap(), row.register_state.rs2_val.unwrap()).into()),
            WASM::I64SHRS => Ok(SRAInstruction::<WORD_SIZE_1>(row.register_state.rs1_val.unwrap(), row.register_state.rs2_val.unwrap()).into()),
            
            // immediates
            WASM::I64MULI => Ok(MULInstruction::<WORD_SIZE_1>(row.register_state.rs1_val.unwrap(), row.imm_u64()).into()),
            WASM::I64ADDI  => Ok(ADDInstruction::<WORD_SIZE_1>(row.register_state.rs1_val.unwrap(), row.imm_u64()).into()),
            WASM::I64XORI  => Ok(XORInstruction::<WORD_SIZE_1>(row.register_state.rs1_val.unwrap(), row.imm_u64()).into()),
            WASM::I64ORI   => Ok(ORInstruction::<WORD_SIZE_1>(row.register_state.rs1_val.unwrap(), row.imm_u64()).into()),
            WASM::I64ANDI  => Ok(ANDInstruction::<WORD_SIZE_1>(row.register_state.rs1_val.unwrap(), row.imm_u64()).into()),

            // WASM::SLL => Ok(SLLInstruction(row.register_state.rs1_val.unwrap(), row.register_state.rs2_val.unwrap()).into()),
            // WASM::SRL => Ok(SRLInstruction(row.register_state.rs1_val.unwrap(), row.register_state.rs2_val.unwrap()).into()),
            // WASM::SRA => Ok(SRAInstruction(row.register_state.rs1_val.unwrap(), row.register_state.rs2_val.unwrap()).into()),
            // WASM::SLT  => Ok(SLTInstruction(row.register_state.rs1_val.unwrap(), row.register_state.rs2_val.unwrap()).into()),
            // WASM::SLTU => Ok(SLTUInstruction(row.register_state.rs1_val.unwrap(), row.register_state.rs2_val.unwrap()).into()),

            // WASM::SLLI  => Ok(SLLInstruction(row.register_state.rs1_val.unwrap(), row.imm_u32() as u64).into()),
            // WASM::SRLI  => Ok(SRLInstruction(row.register_state.rs1_val.unwrap(), row.imm_u32() as u64).into()),
            // WASM::SRAI  => Ok(SRAInstruction(row.register_state.rs1_val.unwrap(), row.imm_u32() as u64).into()),
            // WASM::SLTI  => Ok(SLTInstruction(row.register_state.rs1_val.unwrap(), row.imm_u32() as u64).into()),
            // WASM::SLTIU => Ok(SLTUInstruction(row.register_state.rs1_val.unwrap(), row.imm_u32() as u64).into()),

            // WASM::BEQ  => Ok(BEQInstruction(row.register_state.rs1_val.unwrap(), row.register_state.rs2_val.unwrap()).into()),
            // WASM::BNE  => Ok(BNEInstruction(row.register_state.rs1_val.unwrap(), row.register_state.rs2_val.unwrap()).into()),
            // WASM::BLT  => Ok(SLTInstruction(row.register_state.rs1_val.unwrap(), row.register_state.rs2_val.unwrap()).into()),
            // WASM::BLTU => Ok(SLTUInstruction(row.register_state.rs1_val.unwrap(), row.register_state.rs2_val.unwrap()).into()),
            // WASM::BGE  => Ok(BGEInstruction(row.register_state.rs1_val.unwrap(), row.register_state.rs2_val.unwrap()).into()),
            // WASM::BGEU => Ok(BGEUInstruction(row.register_state.rs1_val.unwrap(), row.register_state.rs2_val.unwrap()).into()),

            WASM::JAL  => Ok(ADDInstruction::<WORD_SIZE>(row.instruction.address, row.imm_u32() as u64).into()),
            WASM::JALR => Ok(ADDInstruction::<WORD_SIZE>(row.register_state.rs1_val.unwrap(), row.imm_u32() as u64).into()),
            WASM::AUIPC => Ok(ADDInstruction::<WORD_SIZE>(row.instruction.address, row.imm_u32() as u64).into()),
            // WASM::LUI => Ok(ADVICEInstruction(row.imm_u32() as u64).into()),

            // WASM::MULHU => Ok(MULHUInstruction(row.register_state.rs1_val.unwrap(), row.register_state.rs2_val.unwrap()).into()),
            
            WASM::MULU => Ok(MULUInstruction::<WORD_SIZE>(row.register_state.rs1_val.unwrap(), row.register_state.rs2_val.unwrap()).into()),
            WASM::VIRTUAL_ADVICE => Ok(ADVICEInstruction::<WORD_SIZE>(row.advice_value.unwrap()).into()),
            WASM::VIRTUAL_ASSERT_EQ => Ok(BEQInstruction::<WORD_SIZE>(row.register_state.rs1_val.unwrap(), row.register_state.rs2_val.unwrap()).into()),
            WASM::VIRTUAL_ASSERT_LTE => Ok(ASSERTLTEInstruction::<WORD_SIZE>(row.register_state.rs1_val.unwrap(), row.register_state.rs2_val.unwrap()).into()),
            WASM::VIRTUAL_ASSERT_VALID_UNSIGNED_REMAINDER => Ok(AssertValidUnsignedRemainderInstruction::<WORD_SIZE>(row.register_state.rs1_val.unwrap(), row.register_state.rs2_val.unwrap()).into()),
            WASM::VIRTUAL_ASSERT_VALID_DIV0 => Ok(AssertValidDiv0Instruction::<WORD_SIZE>(row.register_state.rs1_val.unwrap(), row.register_state.rs2_val.unwrap()).into()),

            // WASM::VIRTUAL_MOVE => Ok(MOVEInstruction(row.register_state.rs1_val.unwrap()).into()),
            // WASM::VIRTUAL_MOVSIGN => Ok(MOVSIGNInstruction(row.register_state.rs1_val.unwrap()).into()),

            WASM::VIRTUAL_ASSERT_VALID_SIGNED_REMAINDER => Ok(AssertValidSignedRemainderInstruction::<WORD_SIZE>(row.register_state.rs1_val.unwrap(), row.register_state.rs2_val.unwrap()).into()),
            // WASM::VIRTUAL_ASSERT_HALFWORD_ALIGNMENT => Ok(AssertHalfwordAlignmentInstruction::<32>(row.register_state.rs1_val.unwrap(), row.imm_u32() as u64).into()),
            // WASM::VIRTUAL_POW2 => Ok(POW2Instruction::<32>(row.register_state.rs1_val.unwrap()).into()),
            // WASM::VIRTUAL_POW2I => Ok(POW2Instruction::<32>(row.imm_u64()).into()),
            // WASM::VIRTUAL_SRA_PAD => Ok(RightShiftPaddingInstruction::<32>(row.register_state.rs1_val.unwrap()).into()),
            // WASM::VIRTUAL_SRA_PADI => Ok(RightShiftPaddingInstruction::<32>(row.imm_u64()).into()),

            _ => Err("No corresponding RV32I instruction")
        }
    }
}
