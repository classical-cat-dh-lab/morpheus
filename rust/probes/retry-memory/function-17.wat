 (func $17 (param $0 i32) (result i32)
  (local $1 i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  (local.set $3
   (call $235
    (i32.const 1024)
   )
  )
  (local.set $1
   (call $235
    (i32.const 1024)
   )
  )
  (local.set $4
   (call $50
    (i32.const 1)
   )
  )
  (local.set $5
   (call $235
    (i32.const 60)
   )
  )
  (i32.store8
   (local.get $1)
   (i32.const 0)
  )
  (i32.store8
   (local.get $3)
   (i32.const 0)
  )
  (call $145
   (local.tee $2
    (i32.add
     (local.get $0)
     (i32.const 641)
    )
   )
  )
  (call $155
   (local.get $5)
   (local.get $2)
  )
  (local.set $0
   (block $block (result i32)
    (drop
     (br_if $block
      (i32.const 0)
      (i32.eqz
       (call $28
        (local.get $5)
        (local.get $3)
       )
      )
     )
    )
    (call $155
     (local.tee $2
      (i32.add
       (local.get $4)
       (i32.const 53)
      )
     )
     (i32.add
      (local.get $0)
      (i32.const 409)
     )
    )
    (call $145
     (local.get $2)
    )
    (i32.store8
     (local.get $1)
     (i32.const 0)
    )
    (drop
     (br_if $block
      (i32.const 0)
      (i32.eqz
       (if (result i32)
        (call $37
         (local.get $2)
         (local.get $1)
         (i32.const 1)
        )
        (then
         (call $10
          (local.get $2)
          (local.get $1)
          (local.get $3)
         )
        )
        (else
         (i32.const 0)
        )
       )
      )
     )
    )
    (call $15
     (local.get $0)
     (local.get $1)
     (local.get $4)
    )
   )
  )
  (call $126
   (local.get $3)
   (i32.const 1194)
  )
  (call $126
   (local.get $1)
   (i32.const 1185)
  )
  (call $51
   (local.get $4)
  )
  (call $126
   (local.get $5)
   (i32.const 1866)
  )
  (local.get $0)
 )
