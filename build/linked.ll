; ModuleID = 'llvm-link'
source_filename = "llvm-link"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@.str = private unnamed_addr constant [5 x i8] c"%ld\0A\00", align 1

define i64 @print(i64 %.1) local_unnamed_addr {
entry:
  tail call void bitcast (i64 (i64)* @write to void (i64)*)(i64 %.1)
  ret i64 0
}

; Function Attrs: nofree nosync nounwind readnone
define i64 @fib(i64 %.1) local_unnamed_addr #0 {
entry:
  %lt1 = icmp slt i64 %.1, 2
  br i1 %lt1, label %_ret, label %entry.else

entry.else:                                       ; preds = %entry.else, %entry
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

; Function Attrs: nofree norecurse nosync nounwind readnone willreturn mustprogress
define i64 @main() local_unnamed_addr #1 {
entry:
  ret i64 0
}

; Function Attrs: nofree nounwind uwtable
define dso_local i64 @write(i64 %0) local_unnamed_addr #2 {
  %2 = call i32 (i8*, ...) @printf(i8* nonnull dereferenceable(1) getelementptr inbounds ([5 x i8], [5 x i8]* @.str, i64 0, i64 0), i64 %0)
  %3 = sext i32 %2 to i64
  ret i64 %3
}

; Function Attrs: nofree nounwind
declare dso_local noundef i32 @printf(i8* nocapture noundef readonly, ...) local_unnamed_addr #3

attributes #0 = { nofree nosync nounwind readnone }
attributes #1 = { nofree norecurse nosync nounwind readnone willreturn mustprogress }
attributes #2 = { nofree nounwind uwtable "disable-tail-calls"="false" "frame-pointer"="none" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #3 = { nofree nounwind "disable-tail-calls"="false" "frame-pointer"="none" "less-precise-fpmad"="false" "no-infs-fp-math"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" "unsafe-fp-math"="false" "use-soft-float"="false" }

!llvm.ident = !{!0}
!llvm.module.flags = !{!1}

!0 = !{!"Ubuntu clang version 12.0.1-19ubuntu3"}
!1 = !{i32 1, !"wchar_size", i32 4}
