use std::str::FromStr;

use crate::constants::{MEMORY_OPS_PER_INSTRUCTION, RAM_START_ADDRESS, REGISTER_COUNT};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use serde::{Deserialize, Serialize};
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumIter, FromRepr};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RVTraceRow {
    pub instruction: ELFInstruction,
    pub register_state: RegisterState,
    pub memory_state: Option<MemoryState>,
    pub advice_value: Option<u64>,
    pub precompile_input: Option<[u32; 16]>,
    pub precompile_output_address: Option<u64>,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum MemoryOp {
    Read(u64),       // (address)
    Write(u64, u64), // (address, new_value)
}

impl MemoryOp {
    pub fn noop_read() -> Self {
        Self::Read(0)
    }

    pub fn noop_write() -> Self {
        Self::Write(0, 0)
    }
}

fn sum_u64_i32(a: u64, b: i32) -> u64 {
    if b.is_negative() {
        let abs_b = b.unsigned_abs() as u64;
        if a < abs_b {
            panic!("overflow")
        }
        a - abs_b
    } else {
        let b_u64: u64 = b.try_into().expect("failed u64 conversion");
        a + b_u64
    }
}

impl From<&RVTraceRow> for [MemoryOp; MEMORY_OPS_PER_INSTRUCTION] {
    fn from(val: &RVTraceRow) -> Self {
        let rs1_read = || MemoryOp::Read(val.instruction.rs1.unwrap());
        let rs2_read = || MemoryOp::Read(val.instruction.rs2.unwrap());
        let rd_write = || {
            MemoryOp::Write(
                val.instruction.rd.unwrap(),
                val.register_state.rd_post_val.unwrap(),
            )
        };

        let ram_write_value = || match val.memory_state {
            Some(MemoryState::Read {
                address: _,
                value: _,
            }) => panic!("Unexpected MemoryState::Read"),
            Some(MemoryState::Write {
                address: _,
                pre_value: _,
                post_value,
            }) => post_value,
            None => panic!("Memory state not found"),
        };

        let rs1_offset = || -> u64 {
            let rs1_val = val.register_state.rs1_val.unwrap();
            let imm = val.instruction.imm.unwrap();
            sum_u64_i32(rs1_val, imm as i32)
        };

        // Canonical ordering for memory instructions
        // 0: rs1
        // 1: rs2
        // 2: rd
        // 3: byte_0
        // 4: byte_1
        // 5: byte_2
        // 6: byte_3
        // If any are empty a no_op is inserted.

        // TODO: DOUBLE CHECK THAT ALL INSTRUCTIONS ARE HANDLED HERE.
        match val.instruction.opcode {
            // --- Binary operations ---
            // i32
            WASM::I32ADD
            | WASM::I32SUB
            | WASM::I32MUL
            | WASM::I32DIVU
            | WASM::I32DIVS
            | WASM::I32REMU
            | WASM::I32REMS
            | WASM::I32XOR
            | WASM::I32OR
            | WASM::I32AND
            | WASM::I32SHL
            | WASM::I32SHRU
            | WASM::I32SHRS
            // i64
            | WASM::I64ADD
            | WASM::I64SUB
            | WASM::I64MUL
            | WASM::I64DIVU
            | WASM::I64DIVS
            | WASM::I64REMU
            | WASM::I64REMS
            | WASM::I64XOR
            | WASM::I64OR
            | WASM::I64AND
            | WASM::I64SHL
            | WASM::I64SHRU
            | WASM::I64SHRS

            // --- Comparisons ---
            // i32
            | WASM::I32EQ
            | WASM::I32NE
            | WASM::I32LTS
            | WASM::I32LTU
            | WASM::I32GES
            | WASM::I32GEU

            // i64
            | WASM::I64EQ
            | WASM::I64NE
            | WASM::I64LTS
            | WASM::I64LTU
            | WASM::I64GES
            | WASM::I64GEU

            | WASM::MULH
            | WASM::MULHU
            | WASM::MULHSU
            // --- Virtual instructions ---
            | WASM::MULU
            | WASM::I64MULU => [rs1_read(), rs2_read(), rd_write(), MemoryOp::noop_read()],

            WASM::LUI 
            | WASM::AUIPC 
            | WASM::VIRTUAL_ADVICE 
            | WASM::I64VIRTUAL_ADVICE => [
                MemoryOp::noop_read(),
                MemoryOp::noop_read(),
                rd_write(),
                MemoryOp::noop_read(),
            ],

            WASM::VIRTUAL_ASSERT_HALFWORD_ALIGNMENT => [
                rs1_read(),
                MemoryOp::noop_read(),
                MemoryOp::noop_write(),
                MemoryOp::noop_read(),
            ],

            WASM::I32ADDI
            | WASM::I32MULI
            | WASM::I32XORI
            | WASM::I32ANDI
            | WASM::I32ORI
            | WASM::SLLI
            | WASM::SRLI
            | WASM::SRAI
            | WASM::SLTI
            | WASM::SLTIU
            | WASM::JALR
            | WASM::VIRTUAL_MOVE
            | WASM::VIRTUAL_MOVSIGN
            | WASM::I64VIRTUAL_MOVE
            | WASM::I64MULI
            | WASM::I64ADDI
            | WASM::I64XORI
            | WASM::I64ANDI
            | WASM::I64ORI
             => [
                rs1_read(),
                MemoryOp::noop_read(),
                rd_write(),
                MemoryOp::noop_read(),
            ],

            WASM::LW => [
                rs1_read(),
                MemoryOp::noop_read(),
                rd_write(),
                MemoryOp::Read(rs1_offset()),
            ],
            WASM::FENCE => [
                MemoryOp::noop_read(),
                MemoryOp::noop_read(),
                MemoryOp::noop_write(),
                MemoryOp::noop_read(),
            ],

            WASM::SB | WASM::SH | WASM::SW => [
                rs1_read(),
                rs2_read(),
                MemoryOp::noop_write(),
                MemoryOp::Write(rs1_offset(), ram_write_value()),
            ],

            // WASM::LB | WASM::LH | WASM::LBU | WASM::LHU => [
            WASM::JAL => [
                MemoryOp::noop_read(),
                MemoryOp::noop_read(),
                rd_write(),
                MemoryOp::noop_read(),
            ],

            | WASM::BLT // HACK: Keep this in for now, so when i get to branches i remembeer to add them here, also some code in bytecode.rs gets affected by removing this.
            | WASM::BGE
            | WASM::BLTU
            | WASM::BGEU
            | WASM::VIRTUAL_ASSERT_EQ
            | WASM::VIRTUAL_ASSERT_LTE
            | WASM::VIRTUAL_ASSERT_VALID_DIV0
            | WASM::VIRTUAL_ASSERT_VALID_SIGNED_REMAINDER
            | WASM::VIRTUAL_ASSERT_VALID_UNSIGNED_REMAINDER
            | WASM::I64VIRTUAL_ASSERT_EQ
            | WASM::I64VIRTUAL_ASSERT_LTE
            | WASM::I64VIRTUAL_ASSERT_VALID_DIV0
            | WASM::I64VIRTUAL_ASSERT_VALID_SIGNED_REMAINDER
            | WASM::I64VIRTUAL_ASSERT_VALID_UNSIGNED_REMAINDER => [
                rs1_read(),
                rs2_read(),
                MemoryOp::noop_write(),
                MemoryOp::noop_read(),
            ],

            WASM::ECALL => [
                MemoryOp::noop_read(),
                MemoryOp::noop_read(),
                MemoryOp::noop_write(),
                MemoryOp::Write(rs1_offset(), ram_write_value()),
            ],

            // HACK: Used for testing purposes for tracer
            WASM::UNIMPL => [
                MemoryOp::noop_read(),
                MemoryOp::noop_read(),
                MemoryOp::noop_write(),
                MemoryOp::noop_read(),
            ],

            _ => unreachable!("{val:?}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ELFInstruction {
    pub address: u64,
    pub opcode: WASM,
    pub rs1: Option<u64>,
    pub rs2: Option<u64>,
    pub rd: Option<u64>,
    pub imm: Option<i64>,
    /// If this instruction is part of a "virtual sequence" (see Section 6.2 of the
    /// Jolt paper), then this contains the number of virtual instructions after this
    /// one in the sequence. I.e. if this is the last instruction in the sequence,
    /// `virtual_sequence_remaining` will be Some(0); if this is the penultimate instruction
    /// in the sequence, `virtual_sequence_remaining` will be Some(1); etc.
    pub virtual_sequence_remaining: Option<usize>,
}

/// Boolean flags used in Jolt's R1CS constraints (`opflags` in the Jolt paper).
/// Note that the flags below deviate slightly from those described in Appendix A.1
/// of the Jolt paper.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash, Ord, EnumCountMacro, EnumIter, Default,
)]
pub enum CircuitFlags {
    #[default] // Need a default so that we can derive EnumIter on `JoltR1CSInputs`
    /// 1 if the first lookup operand is the program counter; 0 otherwise (first lookup operand is RS1 value).
    LeftOperandIsPC,
    /// 1 if the second lookup operand is `imm`; 0 otherwise (second lookup operand is RS2 value).
    RightOperandIsImm,
    /// 1 if the instruction is a load (i.e. `LW`)
    Load,
    /// 1 if the instruction is a store (i.e. `SW`)
    Store,
    /// 1 if the instruction is a jump (i.e. `JAL`, `JALR`)
    Jump,
    /// 1 if the instruction is a branch (i.e. `BEQ`, `BNE`, etc.)
    Branch,
    /// 1 if the lookup output is to be stored in `rd` at the end of the step.
    WriteLookupOutputToRD,
    /// Indicates whether the instruction performs a concat-type lookup.
    ConcatLookupQueryChunks,
    /// 1 if the instruction is "virtual", as defined in Section 6.1 of the Jolt paper.
    Virtual,
    /// 1 if the instruction is an assert, as defined in Section 6.1.1 of the Jolt paper.
    Assert,
    /// Used in virtual sequences; the program counter should be the same for the full sequence.
    DoNotUpdatePC,
}
pub const NUM_CIRCUIT_FLAGS: usize = CircuitFlags::COUNT;

impl ELFInstruction {
    // TODO: CHECK ALL INSTRUCTIONS ARE HANDLED HERE.
    #[rustfmt::skip]
    pub fn to_circuit_flags(&self) -> [bool; NUM_CIRCUIT_FLAGS] {
        let mut flags = [false; NUM_CIRCUIT_FLAGS];

        flags[CircuitFlags::LeftOperandIsPC as usize] = matches!(
            self.opcode,
            WASM::JAL | WASM::LUI | WASM::AUIPC,
        );

        flags[CircuitFlags::RightOperandIsImm as usize] = matches!(
            self.opcode,
             WASM::I32MULI
            | WASM::I32ADDI
            | WASM::I32XORI
            | WASM::I32ORI
            | WASM::I32ANDI

            | WASM::I64MULI
            | WASM::I64ADDI
            | WASM::I64XORI
            | WASM::I64ORI
            | WASM::I64ANDI

            | WASM::SLLI
            | WASM::SRLI
            | WASM::SRAI
            | WASM::SLTI
            | WASM::SLTIU
            | WASM::AUIPC
            | WASM::JAL
            | WASM::JALR
            | WASM::SW
            | WASM::LW
            | WASM::VIRTUAL_ASSERT_HALFWORD_ALIGNMENT,
        );

        flags[CircuitFlags::Load as usize] = matches!(
            self.opcode,
            WASM::LW,
        );

        flags[CircuitFlags::Store as usize] = matches!(
            self.opcode,
            WASM::SW,
        );

        flags[CircuitFlags::Jump as usize] = matches!(
            self.opcode,
            WASM::JAL | WASM::JALR,
        );

        flags[CircuitFlags::Branch as usize] = matches!(
            self.opcode,
            WASM::BLT | WASM::BGE | WASM::BLTU | WASM::BGEU,
        );

        // Stores, branches, jumps, and asserts do not store the lookup output to rd (they may update rd in other ways)
        flags[CircuitFlags::WriteLookupOutputToRD as usize] = !matches!(
            self.opcode,
            WASM::SW
            | WASM::LW
            | WASM::BLT
            | WASM::BGE
            | WASM::BLTU
            | WASM::BGEU
            | WASM::JAL
            | WASM::JALR
            | WASM::LUI
            | WASM::VIRTUAL_ASSERT_EQ
            | WASM::VIRTUAL_ASSERT_LTE
            | WASM::VIRTUAL_ASSERT_VALID_DIV0
            | WASM::VIRTUAL_ASSERT_VALID_SIGNED_REMAINDER
            | WASM::VIRTUAL_ASSERT_VALID_UNSIGNED_REMAINDER
            | WASM::VIRTUAL_ASSERT_HALFWORD_ALIGNMENT

            | WASM::I64VIRTUAL_ASSERT_EQ
            | WASM::I64VIRTUAL_ASSERT_LTE
            | WASM::I64VIRTUAL_ASSERT_VALID_DIV0
            | WASM::I64VIRTUAL_ASSERT_VALID_SIGNED_REMAINDER
            | WASM::I64VIRTUAL_ASSERT_VALID_UNSIGNED_REMAINDER
        );

        flags[CircuitFlags::ConcatLookupQueryChunks as usize] = matches!(
            self.opcode,
            WASM::I32XOR
            | WASM::I32XORI
            | WASM::I32OR
            | WASM::I32ORI
            | WASM::I32AND
            | WASM::I32ANDI
            | WASM::I32SHL
            | WASM::I32SHRU
            | WASM::I32SHRS

            | WASM::I64XOR
            | WASM::I64XORI
            | WASM::I64OR
            | WASM::I64ORI
            | WASM::I64AND
            | WASM::I64ANDI
            | WASM::I64SHL
            | WASM::I64SHRU
            | WASM::I64SHRS

            | WASM::I32EQ
            | WASM::I32NE
            | WASM::I32LTS
            | WASM::I32LTU
            | WASM::I32GES
            | WASM::I32GEU

            | WASM::I64EQ
            | WASM::I64NE
            | WASM::I64LTS
            | WASM::I64LTU
            | WASM::I64GES
            | WASM::I64GEU
            
            | WASM::SLLI
            | WASM::SRLI
            | WASM::SRAI
            | WASM::SLTI
            | WASM::SLTIU
            | WASM::BLT
            | WASM::BGE
            | WASM::BLTU
            | WASM::BGEU
            | WASM::VIRTUAL_ASSERT_EQ
            | WASM::VIRTUAL_ASSERT_LTE
            | WASM::VIRTUAL_ASSERT_VALID_SIGNED_REMAINDER
            | WASM::VIRTUAL_ASSERT_VALID_UNSIGNED_REMAINDER
            | WASM::VIRTUAL_ASSERT_VALID_DIV0
            | WASM::I64VIRTUAL_ASSERT_EQ
            | WASM::I64VIRTUAL_ASSERT_LTE
            | WASM::I64VIRTUAL_ASSERT_VALID_SIGNED_REMAINDER
            | WASM::I64VIRTUAL_ASSERT_VALID_UNSIGNED_REMAINDER
            | WASM::I64VIRTUAL_ASSERT_VALID_DIV0,
        );

        flags[CircuitFlags::Virtual as usize] = self.virtual_sequence_remaining.is_some();

        flags[CircuitFlags::Assert as usize] = matches!(self.opcode,
            WASM::VIRTUAL_ASSERT_EQ                        |
            WASM::VIRTUAL_ASSERT_LTE                       |
            WASM::VIRTUAL_ASSERT_HALFWORD_ALIGNMENT        |
            WASM::VIRTUAL_ASSERT_VALID_SIGNED_REMAINDER    |
            WASM::VIRTUAL_ASSERT_VALID_UNSIGNED_REMAINDER  |
            WASM::VIRTUAL_ASSERT_VALID_DIV0|
            WASM::I64VIRTUAL_ASSERT_EQ                        |
            WASM::I64VIRTUAL_ASSERT_LTE                       |
            WASM::I64VIRTUAL_ASSERT_VALID_SIGNED_REMAINDER    |
            WASM::I64VIRTUAL_ASSERT_VALID_UNSIGNED_REMAINDER  |
            WASM::I64VIRTUAL_ASSERT_VALID_DIV0
        );

        // All instructions in virtual sequence are mapped from the same
        // ELF address. Thus if an instruction is virtual (and not the last one
        // in its sequence), then we should *not* update the PC.
        flags[CircuitFlags::DoNotUpdatePC as usize] = match self.virtual_sequence_remaining {
            Some(i) => i != 0,
            None => false
        };

        flags
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RegisterState {
    pub rs1_val: Option<u64>,
    pub rs2_val: Option<u64>,
    pub rd_post_val: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MemoryState {
    Read {
        address: u64,
        value: u64,
    },
    Write {
        address: u64,
        pre_value: u64,
        post_value: u64,
    },
}

impl RVTraceRow {
    pub fn imm_u64(&self) -> u64 {
        self.instruction.imm.unwrap() as u64
    }

    pub fn imm_u32(&self) -> u32 {
        self.instruction.imm.unwrap() as u64 as u32
    }
}

// Reference: https://www.cs.sfu.ca/~ashriram/Courses/CS295/assets/notebooks/RISCV/RISCV_CARD.pdf
#[derive(Debug, PartialEq, Eq, Clone, Copy, FromRepr, Serialize, Deserialize, Hash)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum WASM {
    // --- Binary operations ---
    // i32
    I32ADD,
    I32SUB,
    I32MUL,
    I32DIVU,
    I32DIVS,
    I32REMU,
    I32REMS,
    I32AND,
    I32OR,
    I32XOR,
    I32SHL,
    I32SHRU,
    I32SHRS,
    I32ROTR,
    // i32 immediates
    I32MULI,
    I32ADDI,
    I32SUBILHS,
    I32XORI,
    I32ANDI,
    I32ORI,
    // i64
    I64ADD,
    I64SUB,
    I64MUL,
    I64DIVU,
    I64DIVS,
    I64REMU,
    I64REMS,
    I64AND,
    I64OR,
    I64XOR,
    I64SHL,
    I64SHRU,
    I64SHRS,
    I64ROTR,
    // i64 immediates
    I64MULI,
    I64ADDI,
    I64SUBILHS,
    I64XORI,
    I64ANDI,
    I64ORI,

    // --- Comparisons ---
    // i32
    I32EQ,
    I32NE,
    I32LTS,
    I32LTU,
    I32GES,
    I32GEU,
    // i64
    I64EQ,
    I64NE,
    I64LTS,
    I64LTU,
    I64GES,
    I64GEU,

    SLLI,
    SRLI,
    SRAI,
    SLTI,
    SLTIU,
    LB,
    LH,
    LW,
    LBU,
    LHU,
    SB,
    SH,
    SW,
    BLT,
    BGE, // HACK: Keep in for now, as there is a cascading effect when i rm it
    BLTU,
    BGEU,
    JAL,
    JALR,
    LUI,
    AUIPC,
    ECALL,
    EBREAK,
    MULH,
    MULHU,
    MULHSU,

    FENCE,
    UNIMPL,
    
    // --- Virtual instructions ---
    // i32
    MULU,
    VIRTUAL_MOVE,
    VIRTUAL_ADVICE,
    VIRTUAL_ASSERT_LTE,
    VIRTUAL_ASSERT_VALID_UNSIGNED_REMAINDER,
    VIRTUAL_ASSERT_VALID_SIGNED_REMAINDER,
    VIRTUAL_ASSERT_EQ,
    VIRTUAL_ASSERT_VALID_DIV0,
    // i64
    I64MULU,
    I64VIRTUAL_MOVE,
    I64VIRTUAL_ADVICE,
    I64VIRTUAL_ASSERT_LTE,
    I64VIRTUAL_ASSERT_VALID_UNSIGNED_REMAINDER,
    I64VIRTUAL_ASSERT_VALID_SIGNED_REMAINDER,
    I64VIRTUAL_ASSERT_EQ,
    I64VIRTUAL_ASSERT_VALID_DIV0,

    VIRTUAL_MOVSIGN,
    VIRTUAL_ASSERT_HALFWORD_ALIGNMENT,
    VIRTUAL_POW2,
    VIRTUAL_POW2I,
    VIRTUAL_SRA_PAD,
    VIRTUAL_SRA_PADI,

    // Temp instructions
    ReturnImm32,
}

impl FromStr for WASM {
    type Err = String;

    fn from_str(s: &str) -> Result<WASM, String> {
        match s {
            // --- Binary ---
            // i32
            "I32Add" => Ok(Self::I32ADD),
            "I32Sub" => Ok(Self::I32SUB),
            "I32Mul" => Ok(Self::I32MUL),
            "I32DivS" => Ok(Self::I32DIVS),
            "I32DivU" => Ok(Self::I32DIVU),
            "I32RemS" => Ok(Self::I32REMS),
            "I32RemU" => Ok(Self::I32REMU),
            "I32BitXor" => Ok(Self::I32XOR),
            "I32BitOr" => Ok(Self::I32OR),
            "I32BitAnd" => Ok(Self::I32AND),
            "I32Shl" => Ok(Self::I32SHL), 
            "I32ShrU" => Ok(Self::I32SHRU), 
            "I32ShrS" => Ok(Self::I32SHRS), 
            "I32Rotl" => Ok(Self::UNIMPL), // todo
            "I32Rotr" => Ok(Self::I32ROTR), 
            // i32 Immediates
            "I32MulImm" => Ok(Self::I32MULI),
            "I32AddImm" => Ok(Self::I32ADDI),
            "I32BitXorImm" => Ok(Self::I32XORI),
            "I32BitAndImm" => Ok(Self::I32ANDI),
            "I32BitOrImm" => Ok(Self::I32ORI),
            "I32ShlBy" => Ok(Self::UNIMPL), // todo
            "I32ShrUBy" => Ok(Self::UNIMPL), // todo
            "I32ShrSBy" => Ok(Self::UNIMPL), // todo
            "I32RotlBy" => Ok(Self::UNIMPL), // todo
            "I32RotrBy" => Ok(Self::UNIMPL), // todo
            // lhs immediate
            "I32SubImm16Lhs" => Ok(Self::UNIMPL), // todo
            // i64
            "I64Add" => Ok(Self::I64ADD),
            "I64Sub" => Ok(Self::I64SUB),
            "I64Mul" => Ok(Self::I64MUL),
            "I64DivS" => Ok(Self::I64DIVS), // todo
            "I64DivU" => Ok(Self::I64DIVU), 
            "I64RemS" => Ok(Self::I64REMS),
            "I64RemU" => Ok(Self::I64REMU), 
            "I64BitXor" => Ok(Self::I64XOR),
            "I64BitOr" => Ok(Self::I64OR),
            "I64BitAnd" => Ok(Self::I64AND),
            "I64Shl" => Ok(Self::I64SHL), 
            "I64ShrU" => Ok(Self::I64SHRU), 
            "I64ShrS" => Ok(Self::I64SHRS), 
            "I64Rotl" => Ok(Self::UNIMPL), // todo
            "I64Rotr" => Ok(Self::I64ROTR), 
            // i64 Immediates
            "I64MulImm" => Ok(Self::I64MULI),
            "I64AddImm" => Ok(Self::I64ADDI),
            "I64BitXorImm" => Ok(Self::I64XORI),
            "I64BitAndImm" => Ok(Self::I64ANDI),
            "I64BitOrImm" => Ok(Self::I64ORI),
            "I64ShlBy" => Ok(Self::UNIMPL), // todo
            "I64ShrUBy" => Ok(Self::UNIMPL), // todo
            "I64ShrSBy" => Ok(Self::UNIMPL), // todo
            "I64RotlBy" => Ok(Self::UNIMPL), // todo
            "I64RotrBy" => Ok(Self::UNIMPL), // todo
            // lhs immediate
            "I64SubImm16Lhs" => Ok(Self::UNIMPL), // todo

            // --- Comparisons ---
            // i32
            "I32Eq" => Ok(Self::I32EQ), 
            "I32Ne" => Ok(Self::I32NE),
            "I32LtS" => Ok(Self::I32LTS), 
            "I32LtU" => Ok(Self::I32LTU), 
            "I32GeS" => Ok(Self::I32GES), 
            "I32GeU" => Ok(Self::I32GEU),
            // i32 immediates
            "I32EqImm" => Ok(Self::UNIMPL), // todo
            "I32NeImm" => Ok(Self::UNIMPL), // todo
            // i64
            "I64Eq" => Ok(Self::I64EQ), 
            "I64Ne" => Ok(Self::I64NE), 
            "I64LtS" => Ok(Self::I64LTS), 
            "I64LtU" => Ok(Self::I64LTU),
            "I64GeS" => Ok(Self::I64GES),
            "I64GeU" => Ok(Self::I64GEU),
            // i64 immediates
            "I64EqImm" => Ok(Self::UNIMPL), // todo

            // --- Branches ---
            "Branch" => Ok(Self::UNIMPL), // todo
            "BranchI32Ne" => Ok(Self::UNIMPL), // todo
            "BranchI32NeImm" => Ok(Self::UNIMPL), // todo
            "BranchI64Ne" => Ok(Self::UNIMPL), // todo
            "BranchI64NeImm" => Ok(Self::UNIMPL), // todo

            // --- Conversions ---
            "I32WrapI64" => Ok(Self::UNIMPL), // todo
            "I64Extend32S" => Ok(Self::UNIMPL), // todo

            // --- Store ---
            "Store32Offset16" 
            | "I32StoreOffset16Imm" 
            | "I32Store16Offset16Imm" 
            | "I64StoreOffset16Imm" => Ok(Self::UNIMPL), // todo

            // --- Registers ---
            "Register" => Ok(Self::UNIMPL), // todo
            "Register2" => Ok(Self::UNIMPL), // todo
            "Register3" => Ok(Self::UNIMPL), // todo

            // --- Calls ---
            "CallInternal" => Ok(Self::UNIMPL), // todo

            // --- Globals ---
            "GlobalGet" => Ok(Self::UNIMPL), // todo
            "GlobalSet" => Ok(Self::UNIMPL), // todo

            // --- Returns ---
            "Return" |"ReturnImm32" | "ReturnReg" | "ReturnI64Imm32"  => Ok(Self::UNIMPL), // HACK: This should have its own instruction

            // --- Traps ---
            "Trap" => Ok(Self::UNIMPL), // todo

            "SLLI" => Ok(Self::SLLI),
            "SRLI" => Ok(Self::SRLI),
            "SRAI" => Ok(Self::SRAI),
            "SLTI" => Ok(Self::SLTI),
            "SLTIU" => Ok(Self::SLTIU),
            "LB" => Ok(Self::LB),
            "LH" => Ok(Self::LH),
            "LW" => Ok(Self::LW),
            "LBU" => Ok(Self::LBU),
            "LHU" => Ok(Self::LHU),
            "SB" => Ok(Self::SB),
            "SH" => Ok(Self::SH),
            "SW" => Ok(Self::SW),
            "BLT" => Ok(Self::BLT),
            "BGE" => Ok(Self::BGE),
            "BLTU" => Ok(Self::BLTU),
            "BGEU" => Ok(Self::BGEU),
            "JAL" => Ok(Self::JAL),
            "JALR" => Ok(Self::JALR),
            "LUI" => Ok(Self::LUI),
            "AUIPC" => Ok(Self::AUIPC),
            "ECALL" => Ok(Self::ECALL),
            "EBREAK" => Ok(Self::EBREAK),
            "MULH" => Ok(Self::MULH),
            "MULHU" => Ok(Self::MULHU),
            "MULHSU" => Ok(Self::MULHSU),
            "MULU" => Ok(Self::MULU),
            "FENCE" => Ok(Self::FENCE),
            "UNIMPL" => Ok(Self::UNIMPL),
            _ => Err(format!("Could not match {s:?} instruction to RV32IM set.")),
        }
    }
}

#[allow(clippy::too_long_first_doc_paragraph)]
/// Represented as a "peripheral device" in the RISC-V emulator, this captures
/// all reads from the reserved memory address space for program inputs and all writes
/// to the reserved memory address space for program outputs.
/// The inputs and outputs are part of the public inputs to the proof.
#[derive(
    Debug, Clone, PartialEq, Serialize, Deserialize, CanonicalSerialize, CanonicalDeserialize,
)]
pub struct JoltDevice {
    pub inputs: Vec<u8>,
    pub outputs: Vec<u8>,
    pub panic: bool,
    pub memory_layout: MemoryLayout,
}

impl JoltDevice {
    pub fn new(max_input_size: u64, max_output_size: u64) -> Self {
        Self {
            inputs: Vec::new(),
            outputs: Vec::new(),
            panic: false,
            memory_layout: MemoryLayout::new(max_input_size, max_output_size),
        }
    }

    pub fn load(&self, address: u64) -> u8 {
        if self.is_panic(address) {
            self.panic as u8
        } else if self.is_termination(address) {
            0 // Termination bit should never be loaded after it is set
        } else if self.is_input(address) {
            let internal_address = self.convert_read_address(address);
            if self.inputs.len() <= internal_address {
                0
            } else {
                self.inputs[internal_address]
            }
        } else if self.is_output(address) {
            let internal_address = self.convert_write_address(address);
            if self.outputs.len() <= internal_address {
                0
            } else {
                self.outputs[internal_address]
            }
        } else {
            0 // zero-padding
        }
    }

    pub fn store(&mut self, address: u64, value: u8) {
        if address == self.memory_layout.panic {
            println!("GUEST PANIC");
            self.panic = true;
            return;
        }

        if address == self.memory_layout.termination {
            return;
        }

        let internal_address = self.convert_write_address(address);
        if self.outputs.len() <= internal_address {
            self.outputs.resize(internal_address + 1, 0);
        }

        self.outputs[internal_address] = value;
    }

    pub fn size(&self) -> usize {
        self.inputs.len() + self.outputs.len()
    }

    pub fn is_input(&self, address: u64) -> bool {
        address >= self.memory_layout.input_start && address < self.memory_layout.input_end
    }

    pub fn is_output(&self, address: u64) -> bool {
        address >= self.memory_layout.output_start && address < self.memory_layout.termination
    }

    pub fn is_panic(&self, address: u64) -> bool {
        address == self.memory_layout.panic
    }

    pub fn is_termination(&self, address: u64) -> bool {
        address == self.memory_layout.termination
    }

    fn convert_read_address(&self, address: u64) -> usize {
        (address - self.memory_layout.input_start) as usize
    }

    fn convert_write_address(&self, address: u64) -> usize {
        (address - self.memory_layout.output_start) as usize
    }
}

#[derive(
    Debug, Clone, PartialEq, Serialize, Deserialize, CanonicalSerialize, CanonicalDeserialize,
)]
pub struct MemoryLayout {
    pub max_input_size: u64,
    pub max_output_size: u64,
    pub input_start: u64,
    pub input_end: u64,
    pub output_start: u64,
    pub output_end: u64,
    pub panic: u64,
    pub termination: u64,
}

impl MemoryLayout {
    pub fn new(mut max_input_size: u64, mut max_output_size: u64) -> Self {
        // Must be word-aligned
        max_input_size = max_input_size.next_multiple_of(4);
        max_output_size = max_output_size.next_multiple_of(4);

        // Adds 8 to account for panic bit and termination bit
        // (they each occupy one full 4-byte word)
        let io_region_num_bytes = max_input_size + max_output_size + 8;

        // Padded so that the witness index corresponding to `RAM_START_ADDRESS`
        // is a power of 2
        let io_region_num_words =
            (REGISTER_COUNT + io_region_num_bytes / 4).next_power_of_two() - REGISTER_COUNT;
        let input_start = RAM_START_ADDRESS - io_region_num_words * 4;
        let input_end = input_start + max_input_size;
        let output_start = input_end;
        let output_end = output_start + max_output_size;
        let panic = output_end;
        let termination = panic + 4;

        Self {
            max_input_size,
            max_output_size,
            input_start,
            input_end,
            output_start,
            output_end,
            panic,
            termination,
        }
    }
}
