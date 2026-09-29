; Extracted with Emscripten 6.0.6, -O2 from the frozen portable C source.
define hidden i32 @low_bit_of(i32 noundef %0) local_unnamed_addr #9 {
  br label %2

2:                                                ; preds = %2, %1
  %3 = phi i32 [ 0, %1 ], [ %5, %2 ]
  %4 = phi i32 [ 0, %1 ], [ %8, %2 ]
  %5 = add nuw nsw i32 %3, %4
  %6 = and i32 %5, %0
  %7 = icmp ne i32 %6, 0
  %8 = add nuw nsw i32 %4, 1
  %9 = icmp eq i32 %8, 32
  %10 = select i1 %7, i1 true, i1 %9
  br i1 %10, label %11, label %2

11:                                               ; preds = %2
  ret i32 %6
}
