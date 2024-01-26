(module
  (func $sum
        (export "sum")
        (param $a i32)
        (param $b i32)
        (result i32)
    (i32.add (local.get $a) (local.get $b)))

  (func $main
      (export "main")
      (result i32)
      (call $sum (i32.const 1) (i32.const 2)))
)
