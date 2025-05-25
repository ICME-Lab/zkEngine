use crate::{
    field::JoltField,
    jolt::vm::rv32i_vm::{RV32ISubtables, RV32I},
    poly::commitment::commitment_scheme::CommitmentScheme,
    r1cs::constraints::JoltRV32IMConstraints,
    utils::transcript::Transcript,
};

use super::JoltWASM;

pub enum WASMJoltVM {}

pub const C: usize = 8;
pub const M: usize = 1 << 16;

impl<F, PCS, ProofTranscript> JoltWASM<F, PCS, C, M, ProofTranscript> for WASMJoltVM
where
    F: JoltField,
    PCS: CommitmentScheme<ProofTranscript, Field = F>,
    ProofTranscript: Transcript,
{
    type InstructionSet = RV32I;
    type Subtables = RV32ISubtables<F>;
    type Constraints = JoltRV32IMConstraints;
}

#[cfg(test)]
mod tests {
    use super::{WASMJoltVM, C};
    use crate::{
        poly::commitment::hyperkzg::HyperKZG,
        utils::transcript::KeccakTranscript,
        zkE::{
            tests::{
                add_sub_mul_32_wasm_program, add_sub_mul_wasm_program,
                bitwise_arith_32_wasm_program, bitwise_arith_wasm_program, br_2_wasm_program,
                branch_eqz_nez_wasm_program, divrem_arith_32_wasm_program,
                divrem_arith_wasm_program, eq_32_wasm_program, eq_wasm_program,
                eqz_32_wasm_program, eqz_wasm_program, ge_32_wasm_program, ge_wasm_program,
                gt_32_wasm_program, gt_wasm_program, le_32_wasm_program, le_wasm_program,
                lt_32_wasm_program, lt_wasm_program, ne_32_wasm_program, ne_wasm_program,
                poly_bitshift_wasm_program, poly_divrem32_wasm_program, poly_divrem_wasm_program,
                poly_mixed_wasm_program, poly_rotate_wasm_program, poly_simple_wasm_program,
                shifts_arith_32_wasm_program, shifts_arith_wasm_program,
            },
            vm::{JoltProverPreprocessing, JoltWASM},
            wasm_host::WASMProgram,
        },
    };
    use ark_bn254::{Bn254, Fr};

    fn test_wasm_e2e_with(wasm_program: WASMProgram) {
        let (wasm_bytecode, _init_memory) = wasm_program.decode();

        // Preprocessing
        let preprocessing: JoltProverPreprocessing<C, Fr, HyperKZG<Bn254, _>, KeccakTranscript> =
            WASMJoltVM::prover_preprocess(wasm_bytecode.clone(), 1 << 20, 1 << 20);

        // Prove
        let (execution_trace, program_io) = wasm_program.trace();
        let (snark, commitments, _, _debug_info) =
            WASMJoltVM::prove(program_io.clone(), execution_trace, preprocessing.clone());

        // Verify
        WASMJoltVM::verify(preprocessing.shared, snark, commitments, program_io, None).unwrap();
    }

    #[test]
    fn test_add_sub_mul() {
        test_wasm_e2e_with(add_sub_mul_wasm_program());
    }

    #[test]
    fn test_add_sub_mul_32() {
        test_wasm_e2e_with(add_sub_mul_32_wasm_program());
    }

    #[test]
    fn test_bitwise_arith() {
        test_wasm_e2e_with(bitwise_arith_wasm_program());
    }

    #[test]
    fn test_bitwise_arith_32() {
        test_wasm_e2e_with(bitwise_arith_32_wasm_program());
    }

    #[test]
    fn test_poly_simple() {
        test_wasm_e2e_with(poly_simple_wasm_program());
    }

    #[test]
    fn test_poly_mixed() {
        test_wasm_e2e_with(poly_mixed_wasm_program());
    }

    #[test]
    fn test_shifts_arith() {
        test_wasm_e2e_with(shifts_arith_wasm_program());
    }

    #[test]
    fn test_shifts_arith_32() {
        test_wasm_e2e_with(shifts_arith_32_wasm_program());
    }

    #[test]
    fn test_poly_bitshift() {
        test_wasm_e2e_with(poly_bitshift_wasm_program());
    }

    #[test]
    #[ignore]
    fn test_poly_rotate() {
        poly_rotate_wasm_program().print_instructions();
        test_wasm_e2e_with(poly_rotate_wasm_program());
    }

    #[test]
    fn test_divrem_arith() {
        test_wasm_e2e_with(divrem_arith_wasm_program());
    }

    #[test]
    fn test_divrem_arith_32() {
        test_wasm_e2e_with(divrem_arith_32_wasm_program());
    }

    #[test]
    fn test_poly_divrem() {
        test_wasm_e2e_with(poly_divrem_wasm_program());
    }

    #[test]
    fn test_poly_divrem32() {
        test_wasm_e2e_with(poly_divrem32_wasm_program());
    }

    #[test]
    fn test_eq() {
        test_wasm_e2e_with(eq_wasm_program());
    }

    #[test]
    fn test_eq_32() {
        test_wasm_e2e_with(eq_32_wasm_program());
    }

    #[test]
    fn test_eqz() {
        test_wasm_e2e_with(eqz_wasm_program());
    }

    #[test]
    fn test_eqz_32() {
        test_wasm_e2e_with(eqz_32_wasm_program());
    }

    #[test]
    fn test_ne() {
        test_wasm_e2e_with(ne_wasm_program());
    }

    #[test]
    fn test_ne_32() {
        test_wasm_e2e_with(ne_32_wasm_program());
    }

    #[test]
    fn test_lt() {
        test_wasm_e2e_with(lt_wasm_program());
    }

    #[test]
    fn test_lt_32() {
        test_wasm_e2e_with(lt_32_wasm_program());
    }

    #[test]
    fn test_gt() {
        test_wasm_e2e_with(gt_wasm_program());
    }

    #[test]
    fn test_gt_32() {
        test_wasm_e2e_with(gt_32_wasm_program());
    }

    #[test]
    fn test_ge() {
        test_wasm_e2e_with(ge_wasm_program());
    }

    #[test]
    fn test_ge_32() {
        test_wasm_e2e_with(ge_32_wasm_program());
    }

    #[test]
    fn test_le() {
        test_wasm_e2e_with(le_wasm_program());
    }

    #[test]
    fn test_le_32() {
        test_wasm_e2e_with(le_32_wasm_program());
    }

    #[test]
    fn test_branch_eqz_nez() {
        branch_eqz_nez_wasm_program().print_instructions();
        // test_wasm_e2e_with(le_32_wasm_program());
    }

    #[test]
    fn test_br_2() {
        br_2_wasm_program().print_instructions();
        // test_wasm_e2e_with(le_32_wasm_program());
    }
}
