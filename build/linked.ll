; ModuleID = '<string>'
source_filename = "builtin.c"
target datalayout = "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-w64-windows-gnu"

; Function Attrs: mustprogress nofree norecurse nosync nounwind uwtable willreturn
define dso_local void @swap(i8* nocapture noundef %0, i8* nocapture noundef %1) local_unnamed_addr #0 {
  %3 = load i8, i8* %0, align 1, !tbaa !4
  %4 = load i8, i8* %1, align 1, !tbaa !4
  store i8 %4, i8* %0, align 1, !tbaa !4
  store i8 %3, i8* %1, align 1, !tbaa !4
  ret void
}

; Function Attrs: nofree norecurse nosync nounwind uwtable
define dso_local i8* @reverse(i8* noundef returned %0, i32 noundef %1, i32 noundef %2) local_unnamed_addr #1 {
  %4 = icmp slt i32 %1, %2
  br i1 %4, label %5, label %.loopexit

5:                                                ; preds = %3
  %6 = sext i32 %2 to i64
  %7 = sext i32 %1 to i64
  br label %8

8:                                                ; preds = %8, %5
  %9 = phi i64 [ %7, %5 ], [ %11, %8 ]
  %10 = phi i64 [ %6, %5 ], [ %13, %8 ]
  %11 = add nsw i64 %9, 1
  %12 = getelementptr inbounds i8, i8* %0, i64 %9
  %13 = add nsw i64 %10, -1
  %14 = getelementptr inbounds i8, i8* %0, i64 %10
  %15 = load i8, i8* %12, align 1, !tbaa !4
  %16 = load i8, i8* %14, align 1, !tbaa !4
  store i8 %16, i8* %12, align 1, !tbaa !4
  store i8 %15, i8* %14, align 1, !tbaa !4
  %17 = icmp slt i64 %11, %13
  br i1 %17, label %8, label %.loopexit, !llvm.loop !7

.loopexit:                                        ; preds = %8, %3
  ret i8* %0
}

; Function Attrs: nofree nosync nounwind uwtable
define dso_local i8* @itostr(i32 noundef %0, i8* noundef returned %1, i32 noundef %2) local_unnamed_addr #2 {
  %4 = add i32 %2, -33
  %5 = icmp ult i32 %4, -31
  br i1 %5, label %.loopexit, label %6

6:                                                ; preds = %3
  %7 = icmp eq i32 %0, 0
  br i1 %7, label %.thread, label %8

8:                                                ; preds = %6
  %9 = tail call i32 @llvm.abs.i32(i32 %0, i1 true)
  br label %10

10:                                               ; preds = %10, %8
  %11 = phi i64 [ 0, %8 ], [ %20, %10 ]
  %12 = phi i32 [ %9, %8 ], [ %13, %10 ]
  %.frozen = freeze i32 %12
  %13 = sdiv i32 %.frozen, %2
  %14 = mul i32 %13, %2
  %.decomposed = sub i32 %.frozen, %14
  %15 = icmp sgt i32 %.decomposed, 9
  %16 = trunc i32 %.decomposed to i8
  %17 = select i1 %15, i8 55, i8 48
  %18 = add i8 %17, %16
  %19 = getelementptr inbounds i8, i8* %1, i64 %11
  store i8 %18, i8* %19, align 1, !tbaa !4
  %20 = add nuw i64 %11, 1
  %21 = icmp eq i32 %13, 0
  br i1 %21, label %22, label %10, !llvm.loop !10

22:                                               ; preds = %10
  %23 = trunc i64 %20 to i32
  %24 = icmp eq i32 %23, 0
  br i1 %24, label %.thread, label %25

.thread:                                          ; preds = %6, %22
  store i8 48, i8* %1, align 1, !tbaa !4
  br label %25

25:                                               ; preds = %.thread, %22
  %26 = phi i32 [ 1, %.thread ], [ %23, %22 ]
  %27 = icmp slt i32 %0, 0
  %28 = icmp eq i32 %2, 10
  %29 = and i1 %27, %28
  br i1 %29, label %30, label %34

30:                                               ; preds = %25
  %31 = add nuw nsw i32 %26, 1
  %32 = zext i32 %26 to i64
  %33 = getelementptr inbounds i8, i8* %1, i64 %32
  store i8 45, i8* %33, align 1, !tbaa !4
  br label %34

34:                                               ; preds = %30, %25
  %35 = phi i32 [ %31, %30 ], [ %26, %25 ]
  %36 = sext i32 %35 to i64
  %37 = getelementptr inbounds i8, i8* %1, i64 %36
  store i8 0, i8* %37, align 1, !tbaa !4
  %38 = icmp sgt i32 %35, 1
  br i1 %38, label %39, label %.loopexit

39:                                               ; preds = %34
  %40 = add nsw i32 %35, -1
  %41 = zext i32 %40 to i64
  br label %42

42:                                               ; preds = %42, %39
  %43 = phi i64 [ 0, %39 ], [ %45, %42 ]
  %44 = phi i64 [ %41, %39 ], [ %47, %42 ]
  %45 = add nuw nsw i64 %43, 1
  %46 = getelementptr inbounds i8, i8* %1, i64 %43
  %47 = add nsw i64 %44, -1
  %48 = getelementptr inbounds i8, i8* %1, i64 %44
  %49 = load i8, i8* %46, align 1, !tbaa !4
  %50 = load i8, i8* %48, align 1, !tbaa !4
  store i8 %50, i8* %46, align 1, !tbaa !4
  store i8 %49, i8* %48, align 1, !tbaa !4
  %51 = icmp slt i64 %45, %47
  br i1 %51, label %42, label %.loopexit, !llvm.loop !7

.loopexit:                                        ; preds = %42, %34, %3
  ret i8* %1
}

; Function Attrs: mustprogress nofree nosync nounwind readnone speculatable willreturn
declare i32 @llvm.abs.i32(i32, i1 immarg) #3

; Function Attrs: nofree nounwind
define i64 @str(i64 %.1) local_unnamed_addr #4 {
entry:
  %.7 = alloca [6 x i8], align 1
  %.7.repack = getelementptr inbounds [6 x i8], [6 x i8]* %.7, i64 0, i64 0
  store i8 39, i8* %.7.repack, align 1
  %.7.repack2 = getelementptr inbounds [6 x i8], [6 x i8]* %.7, i64 0, i64 1
  store i8 82, i8* %.7.repack2, align 1
  %.7.repack3 = getelementptr inbounds [6 x i8], [6 x i8]* %.7, i64 0, i64 2
  store i8 117, i8* %.7.repack3, align 1
  %.7.repack4 = getelementptr inbounds [6 x i8], [6 x i8]* %.7, i64 0, i64 3
  store i8 110, i8* %.7.repack4, align 1
  %.7.repack5 = getelementptr inbounds [6 x i8], [6 x i8]* %.7, i64 0, i64 4
  store i8 39, i8* %.7.repack5, align 1
  %.7.repack6 = getelementptr inbounds [6 x i8], [6 x i8]* %.7, i64 0, i64 5
  store i8 0, i8* %.7.repack6, align 1
  %.10 = call i32 @printf(i8* nonnull %.7.repack)
  %gt = icmp sgt i64 %.1, 0
  br i1 %gt, label %while_loop_entry.preheader, label %_ret

while_loop_entry.preheader:                       ; preds = %entry
  %extract.t = trunc i64 %.1 to i32
  br label %while_loop_entry

while_loop_entry:                                 ; preds = %while_loop_entry, %while_loop_entry.preheader
  %int_n.0.off0 = phi i32 [ 1, %while_loop_entry ], [ %extract.t, %while_loop_entry.preheader ]
  %.258 = alloca [33 x i8], align 1
  %.258.sub = getelementptr inbounds [33 x i8], [33 x i8]* %.258, i64 0, i64 0
  %.27 = call i8* @itostr(i32 %int_n.0.off0, i8* nonnull %.258.sub, i32 10)
  %.28 = call i32 @printf(i8* nonnull dereferenceable(1) %.258.sub)
  br label %while_loop_entry

_ret:                                             ; preds = %entry
  ret i64 0
}

; Function Attrs: nofree nounwind
declare noundef i32 @printf(i8* nocapture noundef readonly) local_unnamed_addr #4

; Function Attrs: nofree nosync nounwind readnone
define i64 @fib(i64 %.1) local_unnamed_addr #5 {
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

; Function Attrs: nofree nounwind
define i64 @main() local_unnamed_addr #4 {
entry:
  %fib = tail call i64 @fib(i64 40)
  %.4 = alloca [13 x i8], align 1
  %.4.repack = getelementptr inbounds [13 x i8], [13 x i8]* %.4, i64 0, i64 0
  store i8 34, i8* %.4.repack, align 1
  %.4.repack1 = getelementptr inbounds [13 x i8], [13 x i8]* %.4, i64 0, i64 1
  store i8 70, i8* %.4.repack1, align 1
  %.4.repack2 = getelementptr inbounds [13 x i8], [13 x i8]* %.4, i64 0, i64 2
  store i8 105, i8* %.4.repack2, align 1
  %.4.repack3 = getelementptr inbounds [13 x i8], [13 x i8]* %.4, i64 0, i64 3
  store i8 98, i8* %.4.repack3, align 1
  %.4.repack4 = getelementptr inbounds [13 x i8], [13 x i8]* %.4, i64 0, i64 4
  store i8 32, i8* %.4.repack4, align 1
  %.4.repack5 = getelementptr inbounds [13 x i8], [13 x i8]* %.4, i64 0, i64 5
  store i8 116, i8* %.4.repack5, align 1
  %.4.repack6 = getelementptr inbounds [13 x i8], [13 x i8]* %.4, i64 0, i64 6
  store i8 101, i8* %.4.repack6, align 1
  %.4.repack7 = getelementptr inbounds [13 x i8], [13 x i8]* %.4, i64 0, i64 7
  store i8 115, i8* %.4.repack7, align 1
  %.4.repack8 = getelementptr inbounds [13 x i8], [13 x i8]* %.4, i64 0, i64 8
  store i8 116, i8* %.4.repack8, align 1
  %.4.repack9 = getelementptr inbounds [13 x i8], [13 x i8]* %.4, i64 0, i64 9
  store i8 58, i8* %.4.repack9, align 1
  %.4.repack10 = getelementptr inbounds [13 x i8], [13 x i8]* %.4, i64 0, i64 10
  store i8 32, i8* %.4.repack10, align 1
  %.4.repack11 = getelementptr inbounds [13 x i8], [13 x i8]* %.4, i64 0, i64 11
  store i8 34, i8* %.4.repack11, align 1
  %.4.repack12 = getelementptr inbounds [13 x i8], [13 x i8]* %.4, i64 0, i64 12
  store i8 0, i8* %.4.repack12, align 1
  %.7 = call i32 @printf(i8* nonnull %.4.repack)
  %.8 = trunc i64 %fib to i32
  %.913 = alloca [33 x i8], align 1
  %.913.sub = getelementptr inbounds [33 x i8], [33 x i8]* %.913, i64 0, i64 0
  %.11 = call i8* @itostr(i32 %.8, i8* nonnull %.913.sub, i32 10)
  %.12 = call i32 @printf(i8* nonnull dereferenceable(1) %.913.sub)
  %.13 = alloca [2 x i8], align 1
  %.13.repack = getelementptr inbounds [2 x i8], [2 x i8]* %.13, i64 0, i64 0
  store i8 10, i8* %.13.repack, align 1
  %.13.repack14 = getelementptr inbounds [2 x i8], [2 x i8]* %.13, i64 0, i64 1
  store i8 0, i8* %.13.repack14, align 1
  %.16 = call i32 @printf(i8* nonnull %.13.repack)
  %str = call i64 @str(i64 10)
  ret i64 0
}

attributes #0 = { mustprogress nofree norecurse nosync nounwind uwtable willreturn "frame-pointer"="none" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #1 = { nofree norecurse nosync nounwind uwtable "frame-pointer"="none" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #2 = { nofree nosync nounwind uwtable "frame-pointer"="none" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #3 = { mustprogress nofree nosync nounwind readnone speculatable willreturn }
attributes #4 = { nofree nounwind }
attributes #5 = { nofree nosync nounwind readnone }

!llvm.module.flags = !{!0, !1, !2}
!llvm.ident = !{!3}

!0 = !{i32 1, !"wchar_size", i32 2}
!1 = !{i32 7, !"PIC Level", i32 2}
!2 = !{i32 7, !"uwtable", i32 1}
!3 = !{!"clang version 14.0.6"}
!4 = !{!5, !5, i64 0}
!5 = !{!"omnipotent char", !6, i64 0}
!6 = !{!"Simple C/C++ TBAA"}
!7 = distinct !{!7, !8, !9}
!8 = !{!"llvm.loop.mustprogress"}
!9 = !{!"llvm.loop.unroll.disable"}
!10 = distinct !{!10, !8, !9}
