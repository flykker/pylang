; ModuleID = 'builtin.c'
source_filename = "builtin.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

; Function Attrs: mustprogress nofree norecurse nosync nounwind uwtable willreturn
define dso_local void @swap(i8* nocapture noundef %0, i8* nocapture noundef %1) local_unnamed_addr #0 {
  %3 = load i8, i8* %0, align 1, !tbaa !5
  %4 = load i8, i8* %1, align 1, !tbaa !5
  store i8 %4, i8* %0, align 1, !tbaa !5
  store i8 %3, i8* %1, align 1, !tbaa !5
  ret void
}

; Function Attrs: nofree norecurse nosync nounwind uwtable
define dso_local i8* @reverse(i8* noundef returned %0, i32 noundef %1, i32 noundef %2) local_unnamed_addr #1 {
  %4 = icmp slt i32 %1, %2
  br i1 %4, label %5, label %18

5:                                                ; preds = %3
  %6 = sext i32 %2 to i64
  %7 = sext i32 %1 to i64
  br label %8

8:                                                ; preds = %5, %8
  %9 = phi i64 [ %7, %5 ], [ %11, %8 ]
  %10 = phi i64 [ %6, %5 ], [ %13, %8 ]
  %11 = add nsw i64 %9, 1
  %12 = getelementptr inbounds i8, i8* %0, i64 %9
  %13 = add nsw i64 %10, -1
  %14 = getelementptr inbounds i8, i8* %0, i64 %10
  %15 = load i8, i8* %12, align 1, !tbaa !5
  %16 = load i8, i8* %14, align 1, !tbaa !5
  store i8 %16, i8* %12, align 1, !tbaa !5
  store i8 %15, i8* %14, align 1, !tbaa !5
  %17 = icmp slt i64 %11, %13
  br i1 %17, label %8, label %18, !llvm.loop !8

18:                                               ; preds = %8, %3
  ret i8* %0
}

; Function Attrs: nofree nosync nounwind uwtable
define dso_local i8* @itostr(i32 noundef %0, i8* noundef %1, i32 noundef %2) local_unnamed_addr #2 {
  %4 = add i32 %2, -33
  %5 = icmp ult i32 %4, -31
  br i1 %5, label %55, label %6

6:                                                ; preds = %3
  %7 = icmp eq i32 %0, 0
  br i1 %7, label %24, label %8

8:                                                ; preds = %6
  %9 = call i32 @llvm.abs.i32(i32 %0, i1 true)
  br label %10

10:                                               ; preds = %8, %10
  %11 = phi i64 [ 0, %8 ], [ %19, %10 ]
  %12 = phi i32 [ %9, %8 ], [ %20, %10 ]
  %13 = srem i32 %12, %2
  %14 = icmp sgt i32 %13, 9
  %15 = trunc i32 %13 to i8
  %16 = select i1 %14, i8 55, i8 48
  %17 = add i8 %16, %15
  %18 = getelementptr inbounds i8, i8* %1, i64 %11
  store i8 %17, i8* %18, align 1, !tbaa !5
  %19 = add nuw i64 %11, 1
  %20 = sdiv i32 %12, %2
  %21 = icmp eq i32 %20, 0
  br i1 %21, label %22, label %10, !llvm.loop !11

22:                                               ; preds = %10
  %23 = trunc i64 %19 to i32
  br label %24

24:                                               ; preds = %22, %6
  %25 = phi i32 [ 0, %6 ], [ %23, %22 ]
  %26 = icmp eq i32 %25, 0
  br i1 %26, label %27, label %28

27:                                               ; preds = %24
  store i8 48, i8* %1, align 1, !tbaa !5
  br label %28

28:                                               ; preds = %27, %24
  %29 = phi i32 [ 1, %27 ], [ %25, %24 ]
  %30 = icmp slt i32 %0, 0
  %31 = icmp eq i32 %2, 10
  %32 = and i1 %30, %31
  br i1 %32, label %33, label %37

33:                                               ; preds = %28
  %34 = add nuw nsw i32 %29, 1
  %35 = zext i32 %29 to i64
  %36 = getelementptr inbounds i8, i8* %1, i64 %35
  store i8 45, i8* %36, align 1, !tbaa !5
  br label %37

37:                                               ; preds = %33, %28
  %38 = phi i32 [ %34, %33 ], [ %29, %28 ]
  %39 = sext i32 %38 to i64
  %40 = getelementptr inbounds i8, i8* %1, i64 %39
  store i8 0, i8* %40, align 1, !tbaa !5
  %41 = icmp sgt i32 %38, 1
  br i1 %41, label %42, label %55

42:                                               ; preds = %37
  %43 = add nsw i32 %38, -1
  %44 = sext i32 %43 to i64
  br label %45

45:                                               ; preds = %45, %42
  %46 = phi i64 [ 0, %42 ], [ %48, %45 ]
  %47 = phi i64 [ %44, %42 ], [ %50, %45 ]
  %48 = add nuw nsw i64 %46, 1
  %49 = getelementptr inbounds i8, i8* %1, i64 %46
  %50 = add nsw i64 %47, -1
  %51 = getelementptr inbounds i8, i8* %1, i64 %47
  %52 = load i8, i8* %49, align 1, !tbaa !5
  %53 = load i8, i8* %51, align 1, !tbaa !5
  store i8 %53, i8* %49, align 1, !tbaa !5
  store i8 %52, i8* %51, align 1, !tbaa !5
  %54 = icmp slt i64 %48, %50
  br i1 %54, label %45, label %55, !llvm.loop !8

55:                                               ; preds = %45, %37, %3
  ret i8* %1
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone uwtable willreturn
define dso_local i64 @i64(i32 noundef %0) local_unnamed_addr #3 {
  %2 = sext i32 %0 to i64
  ret i64 %2
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone uwtable willreturn
define dso_local signext i16 @i16(i32 noundef %0) local_unnamed_addr #3 {
  %2 = trunc i32 %0 to i16
  ret i16 %2
}

; Function Attrs: mustprogress nofree nounwind uwtable willreturn
define dso_local noalias i8* @bytearray(i32 noundef %0) local_unnamed_addr #4 {
  %2 = sext i32 %0 to i64
  %3 = call noalias i8* @malloc(i64 noundef %2) #7
  ret i8* %3
}

; Function Attrs: inaccessiblememonly mustprogress nofree nounwind willreturn
declare noalias noundef i8* @malloc(i64 noundef) local_unnamed_addr #5

; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
declare i32 @llvm.abs.i32(i32, i1 immarg) #6

attributes #0 = { mustprogress nofree norecurse nosync nounwind uwtable willreturn "frame-pointer"="none" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #1 = { nofree norecurse nosync nounwind uwtable "frame-pointer"="none" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #2 = { nofree nosync nounwind uwtable "frame-pointer"="none" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #3 = { mustprogress nofree norecurse nosync nounwind readnone uwtable willreturn "frame-pointer"="none" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #4 = { mustprogress nofree nounwind uwtable willreturn "frame-pointer"="none" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #5 = { inaccessiblememonly mustprogress nofree nounwind willreturn "frame-pointer"="none" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #6 = { nofree nosync nounwind readnone speculatable willreturn }
attributes #7 = { nounwind }

!llvm.module.flags = !{!0, !1, !2, !3}
!llvm.ident = !{!4}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 7, !"PIC Level", i32 2}
!2 = !{i32 7, !"PIE Level", i32 2}
!3 = !{i32 7, !"uwtable", i32 1}
!4 = !{!"Ubuntu clang version 14.0.0-1ubuntu1.1"}
!5 = !{!6, !6, i64 0}
!6 = !{!"omnipotent char", !7, i64 0}
!7 = !{!"Simple C/C++ TBAA"}
!8 = distinct !{!8, !9, !10}
!9 = !{!"llvm.loop.mustprogress"}
!10 = !{!"llvm.loop.unroll.disable"}
!11 = distinct !{!11, !9, !10}
