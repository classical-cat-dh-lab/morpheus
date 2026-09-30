 (func $28 (param $0 i32) (param $1 i32) (result i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  (local $6 i32)
  (local $7 i32)
  (local $8 i32)
  (global.set $global$0
   (local.tee $2
    (i32.sub
     (global.get $global$0)
     (i32.const 128)
    )
   )
  )
  (call $155
   (local.get $2)
   (local.get $0)
  )
  (call $150
   (local.get $2)
  )
  (local.set $0
   (block $block (result i32)
    (if
     (i32.eqz
      (local.tee $0
       (i32.load
        (i32.const 35200)
       )
      )
     )
     (then
      (i32.store
       (i32.const 35200)
       (local.tee $0
        (call $237
         (i32.const 1)
         (i32.const 12)
        )
       )
      )
      (if
       (i32.eqz
        (local.get $0)
       )
       (then
        (drop
         (call $184
          (i32.const 2650)
          (i32.const 28)
          (i32.const 1)
          (i32.load
           (i32.const 5564)
          )
         )
        )
        (br $block
         (i32.const 0)
        )
       )
      )
      (call $29
       (i32.const 1798)
       (local.get $0)
      )
      (local.set $0
       (i32.load
        (i32.const 35200)
       )
      )
     )
    )
    (call $155
     (i32.sub
      (local.get $2)
      (i32.const -64)
     )
     (local.get $2)
    )
    (call $156
     (i32.sub
      (local.get $2)
      (i32.const -64)
     )
     (i32.const 2420)
     (i32.const 60)
    )
    (local.set $4
     (call $207
      (i32.sub
       (local.get $2)
       (i32.const -64)
      )
     )
    )
    (block $block2
     (if
      (i32.gt_s
       (local.tee $3
        (i32.load offset=8
         (local.get $0)
        )
       )
       (i32.const 0)
      )
      (then
       (local.set $6
        (i32.load offset=4
         (local.get $0)
        )
       )
       (local.set $3
        (i32.sub
         (local.get $3)
         (i32.const 1)
        )
       )
       (local.set $0
        (i32.const 0)
       )
       (loop $label
        (block $block1
         (if
          (i32.lt_s
           (local.tee $8
            (call $91
             (i32.sub
              (local.get $2)
              (i32.const -64)
             )
             (i32.load
              (local.tee $7
               (i32.add
                (local.get $6)
                (i32.shl
                 (local.tee $5
                  (i32.shr_u
                   (i32.add
                    (local.get $0)
                    (local.get $3)
                   )
                   (i32.const 1)
                  )
                 )
                 (i32.const 2)
                )
               )
              )
             )
             (local.get $4)
            )
           )
           (i32.const 0)
          )
          (then
           (local.set $3
            (i32.sub
             (local.get $5)
             (i32.const 1)
            )
           )
           (br $block1)
          )
         )
         (br_if $block2
          (i32.eqz
           (local.get $8)
          )
         )
         (local.set $0
          (i32.add
           (local.get $5)
           (i32.const 1)
          )
         )
        )
        (br_if $label
         (i32.le_s
          (local.get $0)
          (local.get $3)
         )
        )
       )
      )
     )
     (i32.store8
      (local.get $1)
      (i32.const 0)
     )
     (br $block
      (i32.const 0)
     )
    )
    (call $155
     (local.get $1)
     (i32.add
      (i32.load
       (local.get $7)
      )
      (local.get $4)
     )
    )
    (i32.const 1)
   )
  )
  (global.set $global$0
   (i32.add
    (local.get $2)
    (i32.const 128)
   )
  )
  (local.get $0)
 )
