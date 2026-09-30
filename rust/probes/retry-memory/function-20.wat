 (func $20 (param $0 i32) (result i32)
  (local $1 i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  (local $6 i32)
  (local $7 i32)
  (local $8 i32)
  (local $9 i32)
  (local $10 i32)
  (local $11 i32)
  (local $12 i32)
  (global.set $global$0
   (local.tee $6
    (i32.sub
     (global.get $global$0)
     (i32.const 128)
    )
   )
  )
  (local.set $4
   (i32.load
    (i32.const 38364)
   )
  )
  (call $155
   (i32.sub
    (local.get $6)
    (i32.const -64)
   )
   (local.tee $2
    (i32.add
     (local.get $0)
     (i32.const 824)
    )
   )
  )
  (local.set $3
   (call $21
    (local.get $0)
   )
  )
  (block $block4
   (block $block
    (br_if $block
     (i32.and
      (i32.ne
       (local.tee $1
        (i32.load8_s offset=824
         (local.get $0)
        )
       )
       (i32.const 42)
      )
      (i32.gt_u
       (i32.sub
        (local.get $1)
        (i32.const 65)
       )
       (i32.const 25)
      )
     )
    )
    (br_if $block
     (i32.and
      (i32.load8_u offset=57
       (local.get $0)
      )
      (i32.const 2)
     )
    )
    (block $block2
     (block $block1
      (if
       (i32.ne
        (i32.load
         (i32.const 38364)
        )
        (i32.const 32768)
       )
       (then
        (br_if $block1
         (i32.ne
          (i32.load
           (i32.const 38364)
          )
          (i32.const 262144)
         )
        )
       )
      )
      (i32.store8
       (local.get $2)
       (local.tee $1
        (call $211
         (i32.load8_s
          (local.get $2)
         )
        )
       )
      )
      (br_if $block2
       (i32.ne
        (i32.and
         (local.get $1)
         (i32.const 255)
        )
        (i32.const 118)
       )
      )
      (br_if $block2
       (i32.gt_u
        (i32.sub
         (i32.or
          (i32.load8_s offset=825
           (local.get $0)
          )
          (i32.const 32)
         )
         (i32.const 97)
        )
        (i32.const 25)
       )
      )
      (i32.store8
       (local.get $2)
       (i32.const 117)
      )
      (br $block2)
     )
     (if
      (i32.eq
       (i32.load8_u
        (local.get $2)
       )
       (i32.const 42)
      )
      (then
       (local.set $3
        (local.get $2)
       )
       (loop $label
        (block $block3
         (local.set $3
          (i32.add
           (local.tee $5
            (local.get $3)
           )
           (i32.const 1)
          )
         )
         (br_if $block3
          (i32.eqz
           (local.tee $10
            (i32.load8_u
             (local.get $5)
            )
           )
          )
         )
         (br_if $label
          (i32.gt_u
           (i32.sub
            (i32.extend8_s
             (i32.or
              (local.get $10)
              (i32.const 32)
             )
            )
            (i32.const 97)
           )
           (i32.const 25)
          )
         )
        )
       )
       (i32.store8
        (local.get $2)
        (local.get $10)
       )
       (drop
        (call $206
         (local.get $5)
         (local.get $3)
        )
       )
      )
     )
    )
    (br_if $block4
     (local.tee $5
      (call $21
       (local.get $0)
      )
     )
    )
    (if
     (i32.load
      (i32.const 38364)
     )
     (then
      (if
       (local.tee $3
        (i32.load8_u
         (local.get $2)
        )
       )
       (then
        (local.set $1
         (i32.const 0)
        )
        (loop $label1
         (i32.store8
          (local.tee $7
           (i32.add
            (local.get $1)
            (local.get $2)
           )
          )
          (local.tee $3
           (call $211
            (i32.extend8_s
             (local.get $3)
            )
           )
          )
         )
         (block $block5
          (br_if $block5
           (i32.ne
            (i32.and
             (local.get $3)
             (i32.const 255)
            )
            (i32.const 118)
           )
          )
          (br_if $block5
           (call $188
            (i32.const 1112)
            (i32.load8_s offset=1
             (local.get $7)
            )
            (i32.const 6)
           )
          )
          (i32.store8
           (local.get $7)
           (i32.const 117)
          )
         )
         (br_if $label1
          (local.tee $3
           (i32.load8_u
            (i32.add
             (local.get $2)
             (local.tee $1
              (i32.add
               (local.get $1)
               (i32.const 1)
              )
             )
            )
           )
          )
         )
        )
       )
      )
      (br_if $block4
       (local.tee $5
        (call $21
         (local.get $0)
        )
       )
      )
      (i32.store8 offset=824
       (local.get $0)
       (call $212
        (i32.load8_s offset=824
         (local.get $0)
        )
       )
      )
      (br_if $block4
       (local.tee $5
        (call $21
         (local.get $0)
        )
       )
      )
     )
    )
    (call $155
     (local.get $2)
     (i32.sub
      (local.get $6)
      (i32.const -64)
     )
    )
    (local.set $3
     (i32.const 0)
    )
   )
   (if
    (i32.eq
     (i32.load8_u
      (i32.sub
       (i32.add
        (call $207
         (local.get $2)
        )
        (local.get $2)
       )
       (i32.const 1)
      )
     )
     (i32.const 39)
    )
    (then
     (br_if $block4
      (local.tee $5
       (i32.add
        (call $22
         (local.get $0)
        )
        (local.get $3)
       )
      )
     )
     (local.set $3
      (i32.const 0)
     )
    )
   )
   (block $block6
    (br_if $block6
     (i32.load offset=60
      (local.get $0)
     )
    )
    (local.set $8
     (select
      (i32.const 12368)
      (select
       (i32.const 13200)
       (i32.const 12240)
       (i32.eq
        (local.get $4)
        (i32.const 262144)
       )
      )
      (i32.eq
       (local.get $4)
       (i32.const 32768)
      )
     )
    )
    (loop $label3
     (br_if $block6
      (i32.eqz
       (i32.load8_u
        (local.get $8)
       )
      )
     )
     (if
      (call $47
       (local.get $2)
       (local.get $8)
       (local.get $6)
      )
      (then
       (call $155
        (local.get $2)
        (local.get $6)
       )
       (local.set $5
        (i32.add
         (call $20
          (local.get $0)
         )
         (local.get $3)
        )
       )
       (if
        (i32.load offset=60
         (local.get $8)
        )
        (then
         (block $block7
          (if
           (i32.le_s
            (local.tee $4
             (i32.load offset=60
              (local.get $0)
             )
            )
            (i32.const 0)
           )
           (then
            (local.set $10
             (i32.const 0)
            )
            (br $block7)
           )
          )
          (local.set $3
           (i32.const 0)
          )
          (local.set $10
           (i32.const 0)
          )
          (local.set $1
           (local.tee $7
            (i32.load offset=948
             (local.get $0)
            )
           )
          )
          (loop $label2
           (if
            (i32.and
             (i32.load offset=60
              (local.get $8)
             )
             (i32.load offset=4
              (local.get $1)
             )
            )
            (then
             (memory.copy
              (local.get $7)
              (local.get $1)
              (i32.const 1116)
             )
             (local.set $10
              (i32.add
               (local.get $10)
               (i32.const 1)
              )
             )
             (local.set $7
              (i32.add
               (local.get $7)
               (i32.const 1116)
              )
             )
             (local.set $4
              (i32.load offset=60
               (local.get $0)
              )
             )
            )
           )
           (local.set $1
            (i32.add
             (local.get $1)
             (i32.const 1116)
            )
           )
           (br_if $label2
            (i32.lt_s
             (local.tee $3
              (i32.add
               (local.get $3)
               (i32.const 1)
              )
             )
             (local.get $4)
            )
           )
          )
         )
         (i32.store offset=60
          (local.get $0)
          (local.get $10)
         )
        )
       )
       (br_if $block4
        (local.get $5)
       )
       (call $155
        (local.get $2)
        (i32.sub
         (local.get $6)
         (i32.const -64)
        )
       )
       (local.set $3
        (i32.const 0)
       )
      )
     )
     (local.set $8
      (i32.sub
       (local.get $8)
       (i32.const -64)
      )
     )
     (br_if $label3
      (i32.eqz
       (i32.load offset=60
        (local.get $0)
       )
      )
     )
    )
   )
   (block $block8
    (br_if $block8
     (i32.ne
      (i32.load
       (i32.const 38364)
      )
      (i32.const 32768)
     )
    )
    (block $block10
     (block $block9
      (br_if $block9
       (call $47
        (local.get $2)
        (i32.const 1168)
        (local.get $6)
       )
      )
      (br_if $block9
       (call $47
        (local.get $2)
        (i32.const 1164)
        (local.get $6)
       )
      )
      (br_if $block9
       (call $47
        (local.get $2)
        (i32.const 1140)
        (local.get $6)
       )
      )
      (br_if $block9
       (call $47
        (local.get $2)
        (i32.const 1155)
        (local.get $6)
       )
      )
      (br_if $block9
       (call $47
        (local.get $2)
        (i32.const 1150)
        (local.get $6)
       )
      )
      (br_if $block10
       (i32.eqz
        (call $47
         (local.get $2)
         (i32.const 1145)
         (local.get $6)
        )
       )
      )
     )
     (local.set $3
      (i32.const 0)
     )
     (i32.store8
      (i32.sub
       (i32.add
        (call $207
         (local.tee $1
          (call $206
           (local.get $6)
           (local.get $2)
          )
         )
        )
        (local.get $1)
       )
       (i32.const 2)
      )
      (i32.const 0)
     )
     (call $155
      (local.get $2)
      (local.get $1)
     )
     (br_if $block4
      (local.tee $5
       (call $20
        (local.get $0)
       )
      )
     )
    )
    (if
     (call $47
      (local.get $2)
      (i32.const 1132)
      (local.get $6)
     )
     (then
      (local.set $3
       (i32.const 0)
      )
      (i32.store8
       (i32.sub
        (i32.add
         (call $207
          (local.tee $1
           (call $206
            (local.get $6)
            (local.get $2)
           )
          )
         )
         (local.get $1)
        )
        (i32.const 1)
       )
       (i32.const 0)
      )
      (call $155
       (local.get $2)
       (local.get $1)
      )
      (br_if $block4
       (local.tee $5
        (call $20
         (local.get $0)
        )
       )
      )
     )
    )
    (block $block12
     (block $block11
      (br_if $block11
       (call $47
        (local.get $2)
        (i32.const 1160)
        (local.get $6)
       )
      )
      (br_if $block11
       (call $47
        (local.get $2)
        (i32.const 1136)
        (local.get $6)
       )
      )
      (local.set $4
       (local.get $3)
      )
      (br $block12)
     )
     (local.set $4
      (i32.const 0)
     )
     (i32.store8
      (i32.sub
       (i32.add
        (call $207
         (local.tee $1
          (call $206
           (local.get $6)
           (local.get $2)
          )
         )
        )
        (local.get $1)
       )
       (i32.const 1)
      )
      (i32.const 0)
     )
     (call $155
      (local.get $2)
      (local.get $1)
     )
     (local.set $7
      (call $20
       (local.get $0)
      )
     )
     (i32.store8
      (i32.sub
       (i32.add
        (call $207
         (local.get $1)
        )
        (local.get $1)
       )
       (i32.const 1)
      )
      (i32.const 0)
     )
     (call $155
      (local.get $2)
      (local.get $1)
     )
     (br_if $block4
      (local.tee $5
       (i32.add
        (call $20
         (local.get $0)
        )
        (i32.add
         (local.get $3)
         (local.get $7)
        )
       )
      )
     )
    )
    (if
     (i32.load offset=60
      (local.get $0)
     )
     (then
      (local.set $3
       (local.get $4)
      )
      (br $block8)
     )
    )
    (if
     (i32.eqz
      (call $47
       (local.get $2)
       (i32.const 1427)
       (local.get $6)
      )
     )
     (then
      (local.set $3
       (local.get $4)
      )
      (br $block8)
     )
    )
    (i32.store8
     (i32.sub
      (i32.add
       (call $207
        (local.tee $1
         (call $206
          (local.get $6)
          (local.get $2)
         )
        )
       )
       (local.get $1)
      )
      (i32.const 1)
     )
     (i32.const 115)
    )
    (call $155
     (local.get $2)
     (local.get $1)
    )
    (local.set $7
     (call $20
      (local.get $0)
     )
    )
    (local.set $3
     (i32.const 0)
    )
    (i32.store8
     (i32.sub
      (i32.add
       (call $207
        (local.get $1)
       )
       (local.get $1)
      )
      (i32.const 1)
     )
     (i32.const 0)
    )
    (call $155
     (local.get $2)
     (local.get $1)
    )
    (br_if $block4
     (local.tee $5
      (i32.add
       (call $20
        (local.get $0)
       )
       (i32.add
        (local.get $4)
        (local.get $7)
       )
      )
     )
    )
   )
   (block $block22
    (block $block20
     (block $block21
      (if
       (i32.eq
        (i32.load
         (i32.const 38364)
        )
        (i32.const 32768)
       )
       (then
        (local.set $5
         (local.tee $1
          (call $206
           (local.get $6)
           (i32.sub
            (local.get $6)
            (i32.const -64)
           )
          )
         )
        )
        (global.set $global$0
         (local.tee $7
          (i32.sub
           (global.get $global$0)
           (i32.const 1024)
          )
         )
        )
        (local.set $8
         (i32.load8_s offset=1
          (local.get $5)
         )
        )
        (block $block13
         (br_if $block13
          (i32.ne
           (i32.or
            (local.tee $9
             (i32.load8_u
              (local.get $5)
             )
            )
            (i32.const 32)
           )
           (i32.const 117)
          )
         )
         (br_if $block13
          (i32.eqz
           (call $188
            (i32.const 2017)
            (local.get $8)
            (i32.const 11)
           )
          )
         )
         (local.set $11
          (i32.const 1)
         )
         (i32.store8
          (local.get $5)
          (local.tee $9
           (if (result i32)
            (i32.eq
             (local.get $9)
             (i32.const 85)
            )
            (then
             (i32.const 86)
            )
            (else
             (br_if $block13
              (i32.ne
               (local.get $9)
               (i32.const 117)
              )
             )
             (i32.const 118)
            )
           )
          )
         )
        )
        (i32.store8 offset=1
         (local.get $7)
         (i32.const 0)
        )
        (i32.store8
         (local.get $7)
         (local.get $9)
        )
        (if
         (local.get $8)
         (then
          (local.set $12
           (i32.or
            (local.get $7)
            (i32.const 1)
           )
          )
          (local.set $4
           (i32.add
            (local.get $5)
            (i32.const 1)
           )
          )
          (loop $label5
           (local.set $10
            (local.get $5)
           )
           (local.set $5
            (local.get $4)
           )
           (block $block14
            (br_if $block14
             (i32.eqz
              (call $188
               (i32.const 2017)
               (i32.extend8_s
                (local.get $9)
               )
               (i32.const 11)
              )
             )
            )
            (local.set $9
             (call $188
              (i32.const 2017)
              (i32.load8_s offset=2
               (local.get $10)
              )
              (i32.const 11)
             )
            )
            (br_if $block14
             (i32.ne
              (i32.and
               (local.get $8)
               (i32.const 255)
              )
              (i32.const 117)
             )
            )
            (br_if $block14
             (i32.eqz
              (local.get $9)
             )
            )
            (i32.store8 offset=1
             (local.get $10)
             (i32.const 118)
            )
            (local.set $11
             (i32.add
              (local.get $11)
              (i32.const 1)
             )
            )
           )
           (local.set $9
            (block $block16 (result i32)
             (local.set $4
              (i32.const 0)
             )
             (if (result i32)
              (i32.gt_s
               (local.tee $9
                (i32.load
                 (i32.const 38300)
                )
               )
               (i32.const 0)
              )
              (then
               (local.set $8
                (i32.load
                 (i32.const 38296)
                )
               )
               (block $block15
                (loop $label4
                 (br_if $block15
                  (i32.eqz
                   (call $205
                    (local.get $7)
                    (i32.add
                     (i32.add
                      (local.get $8)
                      (i32.mul
                       (local.get $4)
                       (i32.const 116)
                      )
                     )
                     (i32.const 53)
                    )
                   )
                  )
                 )
                 (br_if $label4
                  (i32.ne
                   (local.tee $4
                    (i32.add
                     (local.get $4)
                     (i32.const 1)
                    )
                   )
                   (local.get $9)
                  )
                 )
                )
                (br $block16
                 (i32.const 0)
                )
               )
               (i32.const 1)
              )
              (else
               (i32.const 0)
              )
             )
            )
           )
           (local.set $4
            (i32.load8_u offset=1
             (local.get $10)
            )
           )
           (block $block17
            (block $block19
             (block $block18
              (if
               (local.get $9)
               (then
                (br_if $block17
                 (i32.ne
                  (i32.and
                   (local.get $4)
                   (i32.const 255)
                  )
                  (i32.const 117)
                 )
                )
                (br_if $block18
                 (i32.eqz
                  (call $188
                   (i32.const 2017)
                   (i32.load8_s offset=2
                    (local.get $10)
                   )
                   (i32.const 11)
                  )
                 )
                )
                (br $block19)
               )
              )
              (br_if $block17
               (i32.ne
                (i32.and
                 (local.get $4)
                 (i32.const 255)
                )
                (i32.const 117)
               )
              )
             )
             (global.set $global$0
              (local.tee $4
               (i32.sub
                (global.get $global$0)
                (i32.const 1024)
               )
              )
             )
             (i32.store8
              (local.get $4)
              (i32.const 0)
             )
             (local.set $8
              (i32.const 1)
             )
             (if
              (i32.eqz
               (call $31
                (local.tee $9
                 (i32.add
                  (local.get $10)
                  (i32.const 2)
                 )
                )
                (local.get $4)
               )
              )
              (then
               (local.set $8
                (call $28
                 (local.get $9)
                 (local.get $4)
                )
               )
              )
             )
             (global.set $global$0
              (i32.add
               (local.get $4)
               (i32.const 1024)
              )
             )
             (br_if $block17
              (i32.eqz
               (local.get $8)
              )
             )
             (br_if $block17
              (i32.eqz
               (call $188
                (i32.const 2017)
                (i32.load8_s
                 (local.get $9)
                )
                (i32.const 11)
               )
              )
             )
            )
            (i32.store8 offset=1
             (local.get $10)
             (i32.const 118)
            )
            (local.set $11
             (i32.add
              (local.get $11)
              (i32.const 1)
             )
            )
           )
           (local.set $9
            (i32.load8_u offset=1
             (local.get $10)
            )
           )
           (i32.store8 offset=1
            (local.get $12)
            (i32.const 0)
           )
           (i32.store8
            (local.get $12)
            (local.get $9)
           )
           (local.set $4
            (i32.add
             (local.get $5)
             (i32.const 1)
            )
           )
           (local.set $12
            (i32.add
             (local.get $12)
             (i32.const 1)
            )
           )
           (br_if $label5
            (local.tee $8
             (i32.load8_u offset=1
              (local.get $5)
             )
            )
           )
          )
         )
        )
        (global.set $global$0
         (i32.add
          (local.get $7)
          (i32.const 1024)
         )
        )
        (br_if $block20
         (i32.eqz
          (local.get $11)
         )
        )
        (call $155
         (local.get $2)
         (local.get $1)
        )
        (br_if $block21
         (i32.eqz
          (local.tee $5
           (call $20
            (local.get $0)
           )
          )
         )
        )
        (br $block4)
       )
      )
      (br_if $block22
       (i32.ne
        (i32.load
         (i32.const 38364)
        )
        (i32.const 262144)
       )
      )
      (br_if $block22
       (i32.load offset=60
        (local.get $0)
       )
      )
      (local.set $4
       (i32.const 0)
      )
      (local.set $1
       (local.tee $8
        (call $206
         (local.get $6)
         (i32.sub
          (local.get $6)
          (i32.const -64)
         )
        )
       )
      )
      (loop $label6
       (block $block23
        (i32.store8
         (local.get $1)
         (if (result i32)
          (i32.eq
           (local.tee $7
            (i32.load8_u
             (local.get $1)
            )
           )
           (i32.const 85)
          )
          (then
           (i32.const 86)
          )
          (else
           (if
            (i32.ne
             (local.get $7)
             (i32.const 117)
            )
            (then
             (br_if $block23
              (local.get $7)
             )
             (br_if $block20
              (i32.eqz
               (local.get $4)
              )
             )
             (call $155
              (local.get $2)
              (local.get $8)
             )
             (br_if $block4
              (local.tee $5
               (i32.add
                (call $20
                 (local.get $0)
                )
                (local.get $3)
               )
              )
             )
             (br $block21)
            )
           )
           (i32.const 118)
          )
         )
        )
        (local.set $4
         (i32.add
          (local.get $4)
          (i32.const 1)
         )
        )
       )
       (local.set $1
        (i32.add
         (local.get $1)
         (i32.const 1)
        )
       )
       (br $label6)
      )
      (unreachable)
     )
     (local.set $3
      (i32.const 0)
     )
    )
    (drop
     (call $206
      (local.get $6)
      (i32.sub
       (local.get $6)
       (i32.const -64)
      )
     )
    )
   )
   (block $block24
    (br_if $block24
     (i32.ne
      (i32.load
       (i32.const 38364)
      )
      (i32.const 32768)
     )
    )
    (if
     (i32.eq
      (i32.load8_u
       (local.tee $4
        (call $206
         (local.get $6)
         (i32.sub
          (local.get $6)
          (i32.const -64)
         )
        )
       )
      )
      (i32.const 73)
     )
     (then
      (i32.store8
       (local.get $4)
       (i32.const 74)
      )
      (call $155
       (local.get $2)
       (local.get $4)
      )
      (br_if $block4
       (local.tee $5
        (call $20
         (local.get $0)
        )
       )
      )
      (local.set $3
       (i32.const 0)
      )
     )
    )
    (local.set $1
     (local.get $4)
    )
    (loop $label7
     (block $block25
      (if
       (i32.ne
        (local.tee $7
         (i32.load8_u
          (local.get $1)
         )
        )
        (i32.const 105)
       )
       (then
        (br_if $block25
         (local.get $7)
        )
        (br $block24)
       )
      )
      (br_if $block25
       (i32.eqz
        (i32.load8_u offset=2
         (local.get $1)
        )
       )
      )
      (br_if $block25
       (i32.eqz
        (call $188
         (i32.const 1112)
         (i32.load8_s offset=1
          (local.get $1)
         )
         (i32.const 6)
        )
       )
      )
      (i32.store8
       (local.get $1)
       (i32.const 106)
      )
      (call $155
       (local.get $2)
       (local.get $4)
      )
      (br_if $block4
       (local.tee $5
        (call $20
         (local.get $0)
        )
       )
      )
      (i32.store8
       (local.get $1)
       (i32.const 105)
      )
      (local.set $3
       (i32.const 0)
      )
     )
     (local.set $1
      (i32.add
       (local.get $1)
       (i32.const 1)
      )
     )
     (br $label7)
    )
    (unreachable)
   )
   (if
    (i32.ne
     (i32.load
      (i32.const 38364)
     )
     (i32.const 32768)
    )
    (then
     (local.set $5
      (local.get $3)
     )
     (br $block4)
    )
   )
   (if
    (i32.ne
     (i32.load8_u
      (local.get $2)
     )
     (i32.const 101)
    )
    (then
     (local.set $5
      (local.get $3)
     )
     (br $block4)
    )
   )
   (if
    (i32.ne
     (i32.load8_u offset=825
      (local.get $0)
     )
     (i32.const 120)
    )
    (then
     (local.set $5
      (local.get $3)
     )
     (br $block4)
    )
   )
   (if
    (i32.gt_u
     (local.tee $7
      (i32.sub
       (i32.load8_u offset=2
        (local.tee $1
         (call $206
          (local.get $6)
          (local.get $2)
         )
        )
       )
       (i32.const 99)
      )
     )
     (i32.const 17)
    )
    (then
     (local.set $5
      (local.get $3)
     )
     (br $block4)
    )
   )
   (local.set $5
    (local.get $3)
   )
   (br_if $block4
    (i32.eqz
     (i32.and
      (i32.shl
       (i32.const 1)
       (local.get $7)
      )
      (i32.const 139265)
     )
    )
   )
   (if
    (local.tee $7
     (call $207
      (local.tee $3
       (i32.add
        (local.get $1)
        (i32.const 2)
       )
      )
     )
    )
    (then
     (memory.copy
      (i32.add
       (local.get $1)
       (i32.const 3)
      )
      (local.get $3)
      (local.get $7)
     )
    )
   )
   (i32.store8 offset=2
    (local.get $1)
    (i32.const 115)
   )
   (call $155
    (local.get $2)
    (local.get $1)
   )
   (local.set $5
    (call $20
     (local.get $0)
    )
   )
  )
  (call $155
   (local.get $2)
   (i32.sub
    (local.get $6)
    (i32.const -64)
   )
  )
  (global.set $global$0
   (i32.add
    (local.get $6)
    (i32.const 128)
   )
  )
  (local.get $5)
 )
