(module
  (type (;0;) (func (param i64 i64 i64 i64) (result i64)))
  (func (;0;) (type 0) (param i64 i64 i64 i64) (result i64)
    (local i64)
    local.get 0
    i64.eqz
    i64.extend_i32_s
    local.get 0
    i64.add
    local.get 1
    local.get 3
    i64.mul
    i64.sub
    i64.eqz
    i64.extend_i32_s
    local.get 2
    i64.mul
    i64.eqz
    i64.extend_i32_s

  )
  (table (;0;) 1 1 funcref)
  (memory (;0;) 16)
  (global (;0;) (mut i64) (i64.const 1048576))
  (global (;1;) i64 (i64.const 1048576))
  (global (;2;) i64 (i64.const 1048576))
  (export "memory" (memory 0))
  (export "main" (func 0))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2)))
