use common::constants::virtual_register_index;
use tracer::{ELFInstruction, RVTraceRow, RegisterState, WASM};

use super::VirtualInstructionSequence;
use crate::jolt::instruction::{
    add::ADDInstruction, and::ANDInstruction, beq::BEQInstruction, mulu::MULUInstruction,
    or::ORInstruction, sll::SLLInstruction, srl::SRLInstruction, sub::SUBInstruction,
    virtual_advice::ADVICEInstruction, virtual_assert_lte::ASSERTLTEInstruction,
    virtual_assert_valid_div0::AssertValidDiv0Instruction,
    virtual_assert_valid_unsigned_remainder::AssertValidUnsignedRemainderInstruction,
    JoltInstruction,
};

/// Perform rotr operation.
pub struct ROTRInstruction<const WORD_SIZE: usize>;

impl<const WORD_SIZE: usize> VirtualInstructionSequence for ROTRInstruction<WORD_SIZE> {
    const SEQUENCE_LENGTH: usize = 5;

    fn virtual_trace(trace_row: RVTraceRow) -> Vec<RVTraceRow> {
        assert!(
            trace_row.instruction.opcode == WASM::I32ROTR
                || trace_row.instruction.opcode == WASM::I64ROTR,
        );
        // ROTR source registers
        let r_x = trace_row.instruction.rs1;
        let r_y = trace_row.instruction.rs2;
        // Virtual registers used in sequence
        let v_0 = Some(virtual_register_index(0));
        let v_1 = Some(virtual_register_index(1));
        let v_2 = Some(virtual_register_index(2));
        let v_3 = Some(virtual_register_index(3));
        // ROTR operands
        let x = trace_row.register_state.rs1_val.unwrap();
        let y = trace_row.register_state.rs2_val.unwrap();

        let mut virtual_trace = vec![];
        let x_shru = SRLInstruction::<WORD_SIZE>(x, y).lookup_entry();
        let opcode = match WORD_SIZE {
            32 => WASM::I32SHRU,
            64 => WASM::I64SHRU,
            _ => panic!("Unsupported WORD_SIZE: {WORD_SIZE}"),
        };
        virtual_trace.push(RVTraceRow {
            instruction: ELFInstruction {
                address: trace_row.instruction.address,
                opcode,
                rs1: r_x,
                rs2: r_y,
                rd: v_0,
                imm: None,
                virtual_sequence_remaining: Some(Self::SEQUENCE_LENGTH - virtual_trace.len() - 1),
            },
            register_state: RegisterState {
                rs1_val: Some(x),
                rs2_val: Some(y),
                rd_post_val: Some(x_shru),
            },
            memory_state: None,
            advice_value: None,
            precompile_input: None,
            precompile_output_address: None,
        });

        let y_mod = ANDInstruction::<WORD_SIZE>(y, WORD_SIZE as u64 - 1).lookup_entry();
        let opcode = match WORD_SIZE {
            32 => WASM::I32ANDI,
            64 => WASM::I64ANDI,
            _ => panic!("Unsupported WORD_SIZE: {WORD_SIZE}"),
        };
        virtual_trace.push(RVTraceRow {
            instruction: ELFInstruction {
                address: trace_row.instruction.address,
                opcode,
                rs1: r_y,
                rs2: None,
                rd: v_1,
                imm: Some(WORD_SIZE as i64 - 1),
                virtual_sequence_remaining: Some(Self::SEQUENCE_LENGTH - virtual_trace.len() - 1),
            },
            register_state: RegisterState {
                rs1_val: Some(y),
                rs2_val: None,
                rd_post_val: Some(y_mod),
            },
            memory_state: None,
            advice_value: None,
            precompile_input: None,
            precompile_output_address: None,
        });

        let y_sub = SUBInstruction::<WORD_SIZE>(WORD_SIZE as u64, y_mod).lookup_entry();
        let opcode = match WORD_SIZE {
            32 => WASM::I32SUBILHS,
            64 => WASM::I64SUBILHS,
            _ => panic!("Unsupported WORD_SIZE: {WORD_SIZE}"),
        };
        virtual_trace.push(RVTraceRow {
            instruction: ELFInstruction {
                address: trace_row.instruction.address,
                opcode,
                rs1: v_1,
                rs2: None,
                rd: v_2,
                imm: Some(WORD_SIZE as i64),
                virtual_sequence_remaining: Some(Self::SEQUENCE_LENGTH - virtual_trace.len() - 1),
            },
            register_state: RegisterState {
                rs1_val: Some(y_mod),
                rs2_val: None,
                rd_post_val: Some(y_sub),
            },
            memory_state: None,
            advice_value: None,
            precompile_input: None,
            precompile_output_address: None,
        });

        let x_shl = SLLInstruction::<WORD_SIZE>(x, y_sub).lookup_entry();
        let opcode = match WORD_SIZE {
            32 => WASM::I32SHL,
            64 => WASM::I64SHL,
            _ => panic!("Unsupported WORD_SIZE: {WORD_SIZE}"),
        };
        virtual_trace.push(RVTraceRow {
            instruction: ELFInstruction {
                address: trace_row.instruction.address,
                opcode,
                rs1: r_x,
                rs2: v_2,
                rd: v_3,
                imm: None,
                virtual_sequence_remaining: Some(Self::SEQUENCE_LENGTH - virtual_trace.len() - 1),
            },
            register_state: RegisterState {
                rs1_val: Some(x),
                rs2_val: Some(y_sub),
                rd_post_val: Some(x_shl),
            },
            memory_state: None,
            advice_value: None,
            precompile_input: None,
            precompile_output_address: None,
        });

        let res = ORInstruction::<WORD_SIZE>(x_shru, x_shl).lookup_entry();
        let opcode = match WORD_SIZE {
            32 => WASM::I32OR,
            64 => WASM::I64OR,
            _ => panic!("Unsupported WORD_SIZE: {WORD_SIZE}"),
        };
        virtual_trace.push(RVTraceRow {
            instruction: ELFInstruction {
                address: trace_row.instruction.address,
                opcode,
                rs1: v_0,
                rs2: v_3,
                rd: trace_row.instruction.rd,
                imm: None,
                virtual_sequence_remaining: Some(Self::SEQUENCE_LENGTH - virtual_trace.len() - 1),
            },
            register_state: RegisterState {
                rs1_val: Some(x_shru),
                rs2_val: Some(x_shl),
                rd_post_val: Some(res),
            },
            memory_state: None,
            advice_value: None,
            precompile_input: None,
            precompile_output_address: None,
        });

        virtual_trace
    }

    fn sequence_output(x: u64, y: u64) -> u64 {
        match WORD_SIZE {
            32 => {
                let x = x as u32;
                let y = (y % 32) as u32;
                (x.rotate_right(y)) as u64
            }
            64 => {
                let y = (y % 64) as u32;
                x.rotate_right(y)
            }
            _ => panic!("Unsupported WORD_SIZE: {WORD_SIZE}"),
        }
    }
}

#[cfg(test)]
mod test {

    use ark_std::rand::thread_rng;
    use rand_core::RngCore;

    use crate::jolt::instruction::{
        or::ORInstruction, sll::SLLInstruction, sub::SUBInstruction,
        test::jolt_virtual_sequence_test,
    };

    use super::*;

    #[test]
    fn rotr_virtual_sequence_32() {
        jolt_virtual_sequence_test::<ROTRInstruction<32>>(WASM::I32ROTR);
    }

    #[test]
    fn rotr_virtual_sequence_64() {
        jolt_virtual_sequence_test::<ROTRInstruction<64>>(WASM::I64ROTR);
    }

    fn virtual_rotr<const WORD_SIZE: usize>(x: u64, y: u64) -> u64 {
        let x_shru = SRLInstruction::<WORD_SIZE>(x, y).lookup_entry();
        let y_mod = ANDInstruction::<WORD_SIZE>(y, WORD_SIZE as u64 - 1).lookup_entry();
        let y_sub = SUBInstruction::<WORD_SIZE>(WORD_SIZE as u64, y_mod).lookup_entry();
        let x_shl = SLLInstruction::<WORD_SIZE>(x, y_sub).lookup_entry();
        ORInstruction::<WORD_SIZE>(x_shru, x_shl).lookup_entry()
    }

    #[test]
    fn test_rotr() {
        let mut rng = thread_rng();
        const WORD_SIZE: usize = 64;
        for _ in 0..1000 {
            let x = rng.next_u64();
            let y = rng.next_u64() % WORD_SIZE as u64;
            let instruction = ROTRInstruction::<WORD_SIZE>::sequence_output(x, y);
            let expected = virtual_rotr::<WORD_SIZE>(x, y);
            assert_eq!(instruction, expected, "x: {x}, y: {y}");
        }
    }
}
