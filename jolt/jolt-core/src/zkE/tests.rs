use super::wasm_host::WASMProgram;

// default entry point and inputs for all of our WASM tests
const DEFAULT_FUNC: &str = "main";
const DEFAULT_FILE_DIR: &str = "../wasms/";

const DEFAULT_WASM_INPUTS: [&str; 4] = [
    "1500", // amount staked
    "3",    // duration boost (months)
    "2",    // volume boost
    "500",  // penalty
];

fn default_wasm_inputs() -> Vec<String> {
    DEFAULT_WASM_INPUTS.iter().map(|s| s.to_string()).collect()
}

fn make_wasm_program(file_name: &str) -> WASMProgram {
    WASMProgram {
        func: DEFAULT_FUNC.to_string(),
        inputs: default_wasm_inputs(),
        file_path: format!("{DEFAULT_FILE_DIR}{file_name}"),
    }
}

pub fn add_sub_mul_wasm_program() -> WASMProgram {
    make_wasm_program("add_sub_mul.wat")
}

pub fn add_sub_mul_32_wasm_program() -> WASMProgram {
    make_wasm_program("add_sub_mul_32.wat")
}

pub fn bitwise_arith_wasm_program() -> WASMProgram {
    make_wasm_program("bitwise_arith.wat")
}

pub fn bitwise_arith_32_wasm_program() -> WASMProgram {
    make_wasm_program("bitwise_arith_32.wat")
}

pub fn shifts_arith_wasm_program() -> WASMProgram {
    make_wasm_program("shifts_arith.wat")
}

pub fn shifts_arith_32_wasm_program() -> WASMProgram {
    make_wasm_program("shifts_arith_32.wat")
}

pub fn divrem_arith_32_wasm_program() -> WASMProgram {
    make_wasm_program("divrem_arith_32.wat")
}

pub fn lt_wasm_program() -> WASMProgram {
    make_wasm_program("lt.wat")
}

pub fn poly_simple_wasm_program() -> WASMProgram {
    let file_name = "poly-simple.wasm";
    WASMProgram {
        func: DEFAULT_FUNC.to_string(),
        inputs: default_wasm_inputs()[..3].to_vec(),
        file_path: format!("{DEFAULT_FILE_DIR}{file_name}"),
    }
}

pub fn poly_mixed_wasm_program() -> WASMProgram {
    let file_name = "poly_mixed.wasm";
    WASMProgram {
        func: DEFAULT_FUNC.to_string(),
        inputs: vec!["42".to_string(), "17".to_string(), "19".to_string()],
        file_path: format!("{DEFAULT_FILE_DIR}{file_name}"),
    }
}

pub fn poly_bitshift_wasm_program() -> WASMProgram {
    let file_name = "poly_bitshift.wasm";
    WASMProgram {
        func: DEFAULT_FUNC.to_string(),
        inputs: vec!["42".to_string(), "17".to_string(), "19".to_string()],
        file_path: format!("{DEFAULT_FILE_DIR}{file_name}"),
    }
}

pub fn poly_rotate_wasm_program() -> WASMProgram {
    let file_name = "poly_rotate.wasm";
    WASMProgram {
        func: DEFAULT_FUNC.to_string(),
        inputs: vec!["42".to_string(), "17".to_string(), "19".to_string()],
        file_path: format!("{DEFAULT_FILE_DIR}{file_name}"),
    }
}

pub fn poly_divrem_wasm_program() -> WASMProgram {
    let file_name = "poly_divrem.wasm";
    WASMProgram {
        func: DEFAULT_FUNC.to_string(),
        inputs: vec!["42".to_string(), "17".to_string(), "19".to_string()],
        file_path: format!("{DEFAULT_FILE_DIR}{file_name}"),
    }
}

pub fn poly_divrem32_wasm_program() -> WASMProgram {
    let file_name = "poly_divrem32.wasm";
    WASMProgram {
        func: DEFAULT_FUNC.to_string(),
        inputs: vec!["42".to_string(), "17".to_string(), "19".to_string()],
        file_path: format!("{DEFAULT_FILE_DIR}{file_name}"),
    }
}
