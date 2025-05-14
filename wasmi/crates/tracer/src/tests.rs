use super::*;
use std::borrow::Borrow;
use wasmi::{core::ValType, FuncType};

const DEFAULT_FUNC: &str = "main";
const DEFAULT_FILE_DIR: &str = "../../../jolt/wasms/";

const DEFAULT_WASM_INPUTS: [&str; 4] = [
    "1500", // amount staked
    "3",    // duration boost (months)
    "2",    // volume boost
    "500",  // penalty
];

fn default_wasm_inputs() -> Vec<String> {
    DEFAULT_WASM_INPUTS.iter().map(|s| s.to_string()).collect()
}

fn make_wasm_program(file_name: &str) -> Args {
    Args::new(
        &format!("{DEFAULT_FILE_DIR}{file_name}"),
        "main",
        default_wasm_inputs(),
    )
}

pub fn add_sub_mul_32_wasm_program() -> Args {
    make_wasm_program("add_sub_mul_32.wat")
}

pub fn bitwise_arith_wasm_program() -> Args {
    make_wasm_program("bitwise_arith.wat")
}

pub fn shifts_arith_wasm_program() -> Args {
    make_wasm_program("shifts_arith.wat")
}

pub fn lt_wasm_program() -> Args {
    make_wasm_program("lt.wat")
}

fn assert_display(func_type: impl Borrow<FuncType>, expected: &str) {
    assert_eq!(
        format!("{}", DisplayFuncType::from(func_type.borrow())),
        String::from(expected),
    );
}

macro_rules! func_ty {
    ($params:expr, $results:expr $(,)?) => {{
        FuncType::new($params, $results)
    }};
}

#[test]
fn display_0in_0out() {
    assert_display(func_ty!([], []), "fn()");
}

#[test]
fn display_1in_0out() {
    assert_display(func_ty!([ValType::I32], []), "fn(i32)");
}

#[test]
fn display_0in_1out() {
    assert_display(func_ty!([], [ValType::I32]), "fn() -> i32");
}

#[test]
fn display_1in_1out() {
    assert_display(func_ty!([ValType::I32], [ValType::I32]), "fn(i32) -> i32");
}

#[test]
fn display_4in_0out() {
    assert_display(
        func_ty!([ValType::I32, ValType::I64, ValType::F32, ValType::F64], []),
        "fn(i32, i64, f32, f64)",
    );
}

#[test]
fn display_0in_4out() {
    assert_display(
        func_ty!([], [ValType::I32, ValType::I64, ValType::F32, ValType::F64]),
        "fn() -> (i32, i64, f32, f64)",
    );
}

#[test]
fn display_4in_4out() {
    assert_display(
        func_ty!(
            [ValType::I32, ValType::I64, ValType::F32, ValType::F64],
            [ValType::I32, ValType::I64, ValType::F32, ValType::F64],
        ),
        "fn(i32, i64, f32, f64) -> (i32, i64, f32, f64)",
    );
}
