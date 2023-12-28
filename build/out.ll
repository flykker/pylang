; ModuleID = '<string>'
source_filename = "<string>"
target triple = "x86_64-unknown-linux-gnu"

declare void @write(i64) local_unnamed_addr

define i64 @print(i64 %.1) local_unnamed_addr {
entry:
  tail call void @write(i64 %.1)
  ret i64 0
}

; Function Attrs: nofree nosync nounwind readnone
define i64 @fib(i64 %.1) local_unnamed_addr #0 {
entry:
  %lt1 = icmp slt i64 %.1, 2
  br i1 %lt1, label %_ret, label %entry.else

entry.else:                                       ; preds = %entry, %entry.else
  %.1.tr3 = phi i64 [ %sub_tmp.1, %entry.else ], [ %.1, %entry ]
  %accumulator.tr2 = phi i64 [ %add_tmp, %entry.else ], [ 0, %entry ]
  %sub_tmp = add nsw i64 %.1.tr3, -1
  %fib = tail call i64 @fib(i64 %sub_tmp)
  %sub_tmp.1 = add nsw i64 %.1.tr3, -2
  %add_tmp = add i64 %fib, %accumulator.tr2
  %lt = icmp ult i64 %.1.tr3, 4
  br i1 %lt, label %_ret, label %entry.else

_ret:                                             ; preds = %entry.else, %entry
  %accumulator.tr.lcssa = phi i64 [ 0, %entry ], [ %add_tmp, %entry.else ]
  %.1.tr.lcssa = phi i64 [ %.1, %entry ], [ %sub_tmp.1, %entry.else ]
  %accumulator.ret.tr = add i64 %.1.tr.lcssa, %accumulator.tr.lcssa
  ret i64 %accumulator.ret.tr
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define i64 @main() local_unnamed_addr #1 {
entry:
  ret i64 0
}

attributes #0 = { nofree nosync nounwind readnone }
attributes #1 = { mustprogress nofree norecurse nosync nounwind readnone willreturn }
