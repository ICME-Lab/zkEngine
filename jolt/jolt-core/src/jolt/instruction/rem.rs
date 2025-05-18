use common::constants::virtual_register_index;
use tracer::{ELFInstruction, RVTraceRow, RegisterState, WASM};

use super::VirtualInstructionSequence;
use crate::jolt::instruction::{
    add::ADDInstruction, beq::BEQInstruction, mul::MULInstruction,
    virtual_advice::ADVICEInstruction,
    virtual_assert_valid_signed_remainder::AssertValidSignedRemainderInstruction, JoltInstruction,
};

/// Perform signed division and return the remainder
pub struct REMInstruction<const WORD_SIZE: usize>;

impl<const WORD_SIZE: usize> VirtualInstructionSequence for REMInstruction<WORD_SIZE> {
    const SEQUENCE_LENGTH: usize = 7;

    fn virtual_trace(trace_row: RVTraceRow) -> Vec<RVTraceRow> {
        assert!(
            trace_row.instruction.opcode == WASM::I32REMS
                || trace_row.instruction.opcode == WASM::I64REMS
        );
        // REM source registers
        let r_x = trace_row.instruction.rs1;
        let r_y = trace_row.instruction.rs2;
        // Virtual registers used in sequence
        let v_0 = Some(virtual_register_index(0));
        let v_q = Some(virtual_register_index(1));
        let v_r = Some(virtual_register_index(2));
        let v_qy = Some(virtual_register_index(3));
        // REM operands
        let x = trace_row.register_state.rs1_val.unwrap();
        let y = trace_row.register_state.rs2_val.unwrap();

        let mut virtual_trace = vec![];

        let (quotient, remainder) = match WORD_SIZE {
            32 => {
                if y == 0 {
                    (u32::MAX as u64, x)
                } else {
                    let mut quotient = x as i32 / y as i32;
                    let mut remainder = x as i32 % y as i32;
                    if (remainder < 0 && (y as i32) > 0) || (remainder > 0 && (y as i32) < 0) {
                        remainder += y as i32;
                        quotient -= 1;
                    }
                    (quotient as u32 as u64, remainder as u32 as u64)
                }
            }
            64 => {
                if y == 0 {
                    (u64::MAX, x)
                } else {
                    let mut quotient = x as i64 / y as i64;
                    let mut remainder = x as i64 % y as i64;
                    if (remainder < 0 && (y as i64) > 0) || (remainder > 0 && (y as i64) < 0) {
                        remainder += y as i64;
                        quotient -= 1;
                    }
                    (quotient as u64, remainder as u64)
                }
            }
            _ => panic!("Unsupported WORD_SIZE: {WORD_SIZE}"),
        };

        let q = ADVICEInstruction::<WORD_SIZE>(quotient).lookup_entry();
        let opcode = match WORD_SIZE {
            32 => WASM::VIRTUAL_ADVICE,
            64 => WASM::I64VIRTUAL_ADVICE,
            _ => panic!("Unsupported WORD_SIZE: {WORD_SIZE}"),
        };
        virtual_trace.push(RVTraceRow {
            instruction: ELFInstruction {
                address: trace_row.instruction.address,
                opcode,
                rs1: None,
                rs2: None,
                rd: v_q,
                imm: None,
                virtual_sequence_remaining: Some(Self::SEQUENCE_LENGTH - virtual_trace.len() - 1),
            },
            register_state: RegisterState {
                rs1_val: None,
                rs2_val: None,
                rd_post_val: Some(q),
            },
            memory_state: None,
            advice_value: Some(quotient),
            precompile_input: None,
            precompile_output_address: None,
        });

        let r = ADVICEInstruction::<WORD_SIZE>(remainder).lookup_entry();
        let opcode = match WORD_SIZE {
            32 => WASM::VIRTUAL_ADVICE,
            64 => WASM::I64VIRTUAL_ADVICE,
            _ => panic!("Unsupported WORD_SIZE: {WORD_SIZE}"),
        };
        virtual_trace.push(RVTraceRow {
            instruction: ELFInstruction {
                address: trace_row.instruction.address,
                opcode,
                rs1: None,
                rs2: None,
                rd: v_r,
                imm: None,
                virtual_sequence_remaining: Some(Self::SEQUENCE_LENGTH - virtual_trace.len() - 1),
            },
            register_state: RegisterState {
                rs1_val: None,
                rs2_val: None,
                rd_post_val: Some(r),
            },
            memory_state: None,
            advice_value: Some(remainder),
            precompile_input: None,
            precompile_output_address: None,
        });

        let is_valid: u64 = AssertValidSignedRemainderInstruction::<WORD_SIZE>(r, y).lookup_entry();
        assert_eq!(is_valid, 1);
        let opcode = match WORD_SIZE {
            32 => WASM::VIRTUAL_ASSERT_VALID_SIGNED_REMAINDER,
            64 => WASM::I64VIRTUAL_ASSERT_VALID_SIGNED_REMAINDER,
            _ => panic!("Unsupported WORD_SIZE: {WORD_SIZE}"),
        };
        virtual_trace.push(RVTraceRow {
            instruction: ELFInstruction {
                address: trace_row.instruction.address,
                opcode,
                rs1: v_r,
                rs2: r_y,
                rd: None,
                imm: None,
                virtual_sequence_remaining: Some(Self::SEQUENCE_LENGTH - virtual_trace.len() - 1),
            },
            register_state: RegisterState {
                rs1_val: Some(r),
                rs2_val: Some(y),
                rd_post_val: None,
            },
            memory_state: None,
            advice_value: None,
            precompile_input: None,
            precompile_output_address: None,
        });

        let q_y = MULInstruction::<WORD_SIZE>(q, y).lookup_entry();
        let opcode = match WORD_SIZE {
            32 => WASM::I32MUL,
            64 => WASM::I64MUL,
            _ => panic!("Unsupported WORD_SIZE: {WORD_SIZE}"),
        };
        virtual_trace.push(RVTraceRow {
            instruction: ELFInstruction {
                address: trace_row.instruction.address,
                opcode,
                rs1: v_q,
                rs2: r_y,
                rd: v_qy,
                imm: None,
                virtual_sequence_remaining: Some(Self::SEQUENCE_LENGTH - virtual_trace.len() - 1),
            },
            register_state: RegisterState {
                rs1_val: Some(q),
                rs2_val: Some(y),
                rd_post_val: Some(q_y),
            },
            memory_state: None,
            advice_value: None,
            precompile_input: None,
            precompile_output_address: None,
        });

        let add_0: u64 = ADDInstruction::<WORD_SIZE>(q_y, r).lookup_entry();
        let opcode = match WORD_SIZE {
            32 => WASM::I32ADD,
            64 => WASM::I64ADD,
            _ => panic!("Unsupported WORD_SIZE: {WORD_SIZE}"),
        };
        virtual_trace.push(RVTraceRow {
            instruction: ELFInstruction {
                address: trace_row.instruction.address,
                opcode,
                rs1: v_qy,
                rs2: v_r,
                rd: v_0,
                imm: None,
                virtual_sequence_remaining: Some(Self::SEQUENCE_LENGTH - virtual_trace.len() - 1),
            },
            register_state: RegisterState {
                rs1_val: Some(q_y),
                rs2_val: Some(r),
                rd_post_val: Some(add_0),
            },
            memory_state: None,
            advice_value: None,
            precompile_input: None,
            precompile_output_address: None,
        });

        let _assert_eq = BEQInstruction::<WORD_SIZE>(add_0, x).lookup_entry();
        let opcode = match WORD_SIZE {
            32 => WASM::VIRTUAL_ASSERT_EQ,
            64 => WASM::I64VIRTUAL_ASSERT_EQ,
            _ => panic!("Unsupported WORD_SIZE: {WORD_SIZE}"),
        };
        virtual_trace.push(RVTraceRow {
            instruction: ELFInstruction {
                address: trace_row.instruction.address,
                opcode,
                rs1: v_0,
                rs2: r_x,
                rd: None,
                imm: None,
                virtual_sequence_remaining: Some(Self::SEQUENCE_LENGTH - virtual_trace.len() - 1),
            },
            register_state: RegisterState {
                rs1_val: Some(add_0),
                rs2_val: Some(x),
                rd_post_val: None,
            },
            memory_state: None,
            advice_value: None,
            precompile_input: None,
            precompile_output_address: None,
        });

        let opcode = match WORD_SIZE {
            32 => WASM::VIRTUAL_MOVE,
            64 => WASM::I64VIRTUAL_MOVE,
            _ => panic!("Unsupported WORD_SIZE: {WORD_SIZE}"),
        };
        virtual_trace.push(RVTraceRow {
            instruction: ELFInstruction {
                address: trace_row.instruction.address,
                opcode,
                rs1: v_r,
                rs2: None,
                rd: trace_row.instruction.rd,
                imm: None,
                virtual_sequence_remaining: Some(Self::SEQUENCE_LENGTH - virtual_trace.len() - 1),
            },
            register_state: RegisterState {
                rs1_val: Some(r),
                rs2_val: None,
                rd_post_val: Some(r),
            },
            memory_state: None,
            advice_value: None,
            precompile_input: None,
            precompile_output_address: None,
        });

        virtual_trace
    }

    fn sequence_output(x: u64, y: u64) -> u64 {
        if y == 0 {
            return x;
        }
        match WORD_SIZE {
            32 => {
                let mut remainder = (x as i32) % (y as i32);
                if (remainder < 0 && (y as i32) > 0) || (remainder > 0 && (y as i32) < 0) {
                    remainder += y as i32;
                }
                remainder as u32 as u64
            }
            64 => {
                let mut remainder = (x as i64) % (y as i64);
                if (remainder < 0 && (y as i64) > 0) || (remainder > 0 && (y as i64) < 0) {
                    remainder += y as i64;
                }
                remainder as u64
            }
            _ => panic!("Unsupported WORD_SIZE: {WORD_SIZE}"),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::jolt::instruction::test::jolt_virtual_sequence_test;

    use super::*;

    #[test]
    fn rem_virtual_sequence_32() {
        jolt_virtual_sequence_test::<REMInstruction<32>>(WASM::I32REMS);
    }

    #[test]
    fn rem_virtual_sequence_64() {
        jolt_virtual_sequence_test::<REMInstruction<64>>(WASM::I64REMS);
    }
}
