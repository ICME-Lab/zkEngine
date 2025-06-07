use super::wasm_host::WASMProgram;

// TODO: use a macro to avoid code duplication in this module

// default entry point and inputs for all of our WASM tests
const DEFAULT_FUNC: &str = "main";
const DEFAULT_FILE_DIR: &str = "../wasms/";

const BASIC_WASM_INPUTS: [&str; 4] = ["1500", "3", "2", "500"];

fn basic_wasm_inputs() -> Vec<String> {
    BASIC_WASM_INPUTS.iter().map(|s| s.to_string()).collect()
}

fn make_basic_wasm_program(file_name: &str) -> WASMProgram {
    WASMProgram {
        func: DEFAULT_FUNC.to_string(),
        inputs: basic_wasm_inputs(),
        file_path: format!("{DEFAULT_FILE_DIR}basic/{file_name}"),
    }
}

fn make_poly_wasm_program(file_name: &str) -> WASMProgram {
    WASMProgram {
        func: DEFAULT_FUNC.to_string(),
        inputs: vec!["42".to_string(), "17".to_string(), "19".to_string()],
        file_path: format!("{DEFAULT_FILE_DIR}poly/{file_name}"),
    }
}

pub fn add_sub_mul_wasm_program() -> WASMProgram {
    make_basic_wasm_program("add_sub_mul.wat")
}

pub fn add_sub_mul_32_wasm_program() -> WASMProgram {
    make_basic_wasm_program("add_sub_mul_32.wat")
}

pub fn bitwise_arith_wasm_program() -> WASMProgram {
    make_basic_wasm_program("bitwise_arith.wat")
}

pub fn bitwise_arith_32_wasm_program() -> WASMProgram {
    make_basic_wasm_program("bitwise_arith_32.wat")
}

pub fn shifts_arith_wasm_program() -> WASMProgram {
    make_basic_wasm_program("shifts_arith.wat")
}

pub fn shifts_arith_32_wasm_program() -> WASMProgram {
    make_basic_wasm_program("shifts_arith_32.wat")
}

pub fn divrem_arith_wasm_program() -> WASMProgram {
    make_basic_wasm_program("divrem_arith.wat")
}

pub fn divrem_arith_32_wasm_program() -> WASMProgram {
    make_basic_wasm_program("divrem_arith_32.wat")
}

pub fn eq_wasm_program() -> WASMProgram {
    make_basic_wasm_program("eq.wat")
}

pub fn eq_32_wasm_program() -> WASMProgram {
    make_basic_wasm_program("eq_32.wat")
}

pub fn eqz_wasm_program() -> WASMProgram {
    make_basic_wasm_program("eqz.wat")
}

pub fn eqz_32_wasm_program() -> WASMProgram {
    make_basic_wasm_program("eqz_32.wat")
}

pub fn ne_wasm_program() -> WASMProgram {
    make_basic_wasm_program("ne.wat")
}

pub fn ne_32_wasm_program() -> WASMProgram {
    make_basic_wasm_program("ne_32.wat")
}

pub fn lt_wasm_program() -> WASMProgram {
    make_basic_wasm_program("lt.wat")
}

pub fn lt_32_wasm_program() -> WASMProgram {
    make_basic_wasm_program("lt_32.wat")
}

pub fn gt_wasm_program() -> WASMProgram {
    make_basic_wasm_program("gt.wat")
}

pub fn gt_32_wasm_program() -> WASMProgram {
    make_basic_wasm_program("gt_32.wat")
}

pub fn ge_wasm_program() -> WASMProgram {
    make_basic_wasm_program("ge.wat")
}

pub fn ge_32_wasm_program() -> WASMProgram {
    make_basic_wasm_program("ge_32.wat")
}

pub fn le_wasm_program() -> WASMProgram {
    make_basic_wasm_program("le.wat")
}

pub fn le_32_wasm_program() -> WASMProgram {
    make_basic_wasm_program("le_32.wat")
}

pub fn poly_simple_wasm_program() -> WASMProgram {
    let file_name = "poly-simple.wasm";
    make_poly_wasm_program(file_name)
}

pub fn poly_mixed_wasm_program() -> WASMProgram {
    let file_name = "poly_mixed.wasm";
    make_poly_wasm_program(file_name)
}

pub fn poly_bitshift_wasm_program() -> WASMProgram {
    let file_name = "poly_bitshift.wasm";
    make_poly_wasm_program(file_name)
}

pub fn poly_rotate_wasm_program() -> WASMProgram {
    let file_name = "poly_rotate.wasm";
    make_poly_wasm_program(file_name)
}

pub fn poly_rot_dynamic_wasm_program() -> WASMProgram {
    let file_name = "poly_rot_dynamic.wasm";
    make_poly_wasm_program(file_name)
}

pub fn poly_divrem_wasm_program() -> WASMProgram {
    let file_name = "poly_divrem.wasm";
    make_poly_wasm_program(file_name)
}

pub fn poly_divrem32_wasm_program() -> WASMProgram {
    let file_name = "poly_divrem32.wasm";
    make_poly_wasm_program(file_name)
}

pub fn branch_eqz_nez_wasm_program() -> WASMProgram {
    let file_name = "branch_eqz_nez.wasm";
    WASMProgram {
        func: DEFAULT_FUNC.to_string(),
        inputs: vec!["42".to_string(), "17".to_string(), "19".to_string()],
        file_path: format!("{DEFAULT_FILE_DIR}br/{file_name}"),
    }
}

pub fn br_2_wasm_program() -> WASMProgram {
    let file_name = "br_2.wasm";
    WASMProgram {
        func: DEFAULT_FUNC.to_string(),
        inputs: vec!["42".to_string(), "17".to_string(), "19".to_string()],
        file_path: format!("{DEFAULT_FILE_DIR}br/{file_name}"),
    }
}
