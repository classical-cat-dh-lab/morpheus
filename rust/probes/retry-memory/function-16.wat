 (func $16 (param $0 i32) (result i32)
  (local $1 i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  (local $6 i32)
  (if (result i32)
   (i32.and
    (i32.load8_u offset=58
     (local.get $0)
    )
    (i32.const 2)
   )
   (then
    (i32.const 0)
   )
   (else
    (global.set $global$0
     (local.tee $1
      (i32.sub
       (global.get $global$0)
       (i32.const 128)
      )
     )
    )
    (call $155
     (i32.sub
      (local.get $1)
      (i32.const -64)
     )
     (i32.add
      (local.get $0)
      (i32.const 824)
     )
    )
    (call $155
     (local.get $1)
     (i32.sub
      (local.get $1)
      (i32.const -64)
     )
    )
    (i32.store16 offset=641 align=1
     (local.get $0)
     (i32.const 42)
    )
    (call $155
     (local.tee $4
      (i32.add
       (local.get $0)
       (i32.const 409)
      )
     )
     (i32.sub
      (local.get $1)
      (i32.const -64)
     )
    )
    (local.set $3
     (call $17
      (local.get $0)
     )
    )
    (if
     (i32.load8_u offset=64
      (local.get $1)
     )
     (then
      (local.set $5
       (i32.add
        (local.get $0)
        (i32.const 641)
       )
      )
      (local.set $2
       (i32.sub
        (local.get $1)
        (i32.const -64)
       )
      )
      (loop $label
       (call $155
        (local.get $5)
        (local.get $2)
       )
       (call $155
        (local.get $1)
        (i32.sub
         (local.get $1)
         (i32.const -64)
        )
       )
       (i32.store8
        (i32.add
         (local.get $1)
         (i32.sub
          (local.get $2)
          (i32.sub
           (local.get $1)
           (i32.const -64)
          )
         )
        )
        (i32.const 0)
       )
       (call $155
        (local.get $4)
        (local.get $1)
       )
       (local.set $3
        (i32.add
         (call $17
          (local.get $0)
         )
         (local.get $3)
        )
       )
       (local.set $6
        (i32.load8_u offset=1
         (local.get $2)
        )
       )
       (local.set $2
        (i32.add
         (local.get $2)
         (i32.const 1)
        )
       )
       (br_if $label
        (local.get $6)
       )
      )
     )
    )
    (global.set $global$0
     (i32.add
      (local.get $1)
      (i32.const 128)
     )
    )
    (local.get $3)
   )
  )
 )
