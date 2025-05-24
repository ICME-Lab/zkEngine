(module
  (type (;0;) (func (param i64 i64 i64 i64) (result i64)))
  (func (;0;) (type 0) (param i64 i64 i64 i64) (result i64)
    (local i64)
    local.get 0
    local.get 1
    local.get 2
    local.get 3
    i64.mul
    local.get 3
    i64.add
    i64.sub
    local.get 2
    i64.mul
    i64.add
    local.get 0
    local.get 2
    i64.mul
    local.get 0
    i64.sub
    i64.add
    local.get 0
    i64.mul
    local.get 3
    i64.xor
    drop
    local.get 1
    i64.const 100711 ;; random number larger than 2^16
    i64.and
    local.get 0
    i64.or
    local.get 1
    i64.xor
    local.get 3
    i64.const 230521 ;; random number larger than 2^16
    i64.and
    i64.or
    local.get 0
    i64.shl
    local.get 1
    i64.shr_s
    local.get 2
    i64.shl
    local.get 3
    i64.shr_u
    i64.const 100711 ;; random number larger than 2^16
    i64.and
    local.get 1
    i64.shr_s
    local.get 2
    i64.shl
  )
  (table (;0;) 1 1 funcref)
  (memory (;0;) 16)
  (global (;0;) (mut i32) (i32.const 1048576))
  (global (;1;) i32 (i32.const 1048576))
  (global (;2;) i32 (i32.const 1048576))
  (export "memory" (memory 0))
  (export "main" (func 0))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2)))
