; ModuleID = '<string>'
source_filename = "builtin.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

%class.Socket.9 = type { i32 }
%class.sockaddr_in.8 = type { i16, i16, i32, i64 }
%class.sockaddr.7 = type { i16, [14 x i8] }

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
  %15 = load i8, i8* %12, align 1, !tbaa !5
  %16 = load i8, i8* %14, align 1, !tbaa !5
  store i8 %16, i8* %12, align 1, !tbaa !5
  store i8 %15, i8* %14, align 1, !tbaa !5
  %17 = icmp slt i64 %11, %13
  br i1 %17, label %8, label %.loopexit, !llvm.loop !8

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
  store i8 %18, i8* %19, align 1, !tbaa !5
  %20 = add nuw i64 %11, 1
  %21 = icmp eq i32 %13, 0
  br i1 %21, label %22, label %10, !llvm.loop !11

22:                                               ; preds = %10
  %23 = trunc i64 %20 to i32
  %24 = icmp eq i32 %23, 0
  br i1 %24, label %.thread, label %25

.thread:                                          ; preds = %6, %22
  store i8 48, i8* %1, align 1, !tbaa !5
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
  store i8 45, i8* %33, align 1, !tbaa !5
  br label %34

34:                                               ; preds = %30, %25
  %35 = phi i32 [ %31, %30 ], [ %26, %25 ]
  %36 = sext i32 %35 to i64
  %37 = getelementptr inbounds i8, i8* %1, i64 %36
  store i8 0, i8* %37, align 1, !tbaa !5
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
  %49 = load i8, i8* %46, align 1, !tbaa !5
  %50 = load i8, i8* %48, align 1, !tbaa !5
  store i8 %50, i8* %46, align 1, !tbaa !5
  store i8 %49, i8* %48, align 1, !tbaa !5
  %51 = icmp slt i64 %45, %47
  br i1 %51, label %42, label %.loopexit, !llvm.loop !8

.loopexit:                                        ; preds = %42, %34, %3
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
  %3 = tail call noalias i8* @malloc(i64 noundef %2) #10
  ret i8* %3
}

; Function Attrs: inaccessiblememonly mustprogress nofree nounwind willreturn
declare noalias noundef i8* @malloc(i64 noundef) local_unnamed_addr #5

; Function Attrs: mustprogress nofree nosync nounwind readnone speculatable willreturn
declare i32 @llvm.abs.i32(i32, i1 immarg) #6

; Function Attrs: nofree nosync nounwind readnone
define i32 @fib(i32 %.1) local_unnamed_addr #7 {
entry:
  %lt1 = icmp slt i32 %.1, 2
  br i1 %lt1, label %_ret, label %entry.else

entry.else:                                       ; preds = %entry, %entry.else
  %.1.tr3 = phi i32 [ %sub_tmp.1, %entry.else ], [ %.1, %entry ]
  %accumulator.tr2 = phi i32 [ %add_tmp, %entry.else ], [ 0, %entry ]
  %sub_tmp = add nsw i32 %.1.tr3, -1
  %fib = tail call i32 @fib(i32 %sub_tmp)
  %sub_tmp.1 = add nsw i32 %.1.tr3, -2
  %add_tmp = add i32 %fib, %accumulator.tr2
  %lt = icmp ult i32 %.1.tr3, 4
  br i1 %lt, label %_ret, label %entry.else

_ret:                                             ; preds = %entry.else, %entry
  %accumulator.tr.lcssa = phi i32 [ 0, %entry ], [ %add_tmp, %entry.else ]
  %.1.tr.lcssa = phi i32 [ %.1, %entry ], [ %sub_tmp.1, %entry.else ]
  %accumulator.ret.tr = add i32 %.1.tr.lcssa, %accumulator.tr.lcssa
  ret i32 %accumulator.ret.tr
}

define i32 @create(%class.Socket.9* nocapture readnone %.1) local_unnamed_addr {
entry:
  %socket = tail call i32 @socket(i32 2, i32 1, i32 0)
  %.7 = alloca [2 x i8], align 1
  %.7.repack = getelementptr inbounds [2 x i8], [2 x i8]* %.7, i64 0, i64 0
  store i8 10, i8* %.7.repack, align 1
  %.7.repack1 = getelementptr inbounds [2 x i8], [2 x i8]* %.7, i64 0, i64 1
  store i8 0, i8* %.7.repack1, align 1
  %.10 = call i32 @printf(i8* nonnull %.7.repack)
  %.11 = alloca [2 x i8], align 1
  %.11.repack = getelementptr inbounds [2 x i8], [2 x i8]* %.11, i64 0, i64 0
  store i8 10, i8* %.11.repack, align 1
  %.11.repack2 = getelementptr inbounds [2 x i8], [2 x i8]* %.11, i64 0, i64 1
  store i8 0, i8* %.11.repack2, align 1
  %.14 = call i32 @printf(i8* nonnull %.11.repack)
  %eq = icmp eq i32 %socket, 1
  br i1 %eq, label %entry.if, label %entry.endif

entry.if:                                         ; preds = %entry
  %.16 = alloca [22 x i8], align 1
  %.16.repack = getelementptr inbounds [22 x i8], [22 x i8]* %.16, i64 0, i64 0
  store i8 34, i8* %.16.repack, align 1
  %.16.repack86 = getelementptr inbounds [22 x i8], [22 x i8]* %.16, i64 0, i64 1
  store i8 69, i8* %.16.repack86, align 1
  %.16.repack87 = getelementptr inbounds [22 x i8], [22 x i8]* %.16, i64 0, i64 2
  store i8 114, i8* %.16.repack87, align 1
  %.16.repack88 = getelementptr inbounds [22 x i8], [22 x i8]* %.16, i64 0, i64 3
  store i8 114, i8* %.16.repack88, align 1
  %.16.repack89 = getelementptr inbounds [22 x i8], [22 x i8]* %.16, i64 0, i64 4
  store i8 111, i8* %.16.repack89, align 1
  %.16.repack90 = getelementptr inbounds [22 x i8], [22 x i8]* %.16, i64 0, i64 5
  store i8 114, i8* %.16.repack90, align 1
  %.16.repack91 = getelementptr inbounds [22 x i8], [22 x i8]* %.16, i64 0, i64 6
  store i8 32, i8* %.16.repack91, align 1
  %.16.repack92 = getelementptr inbounds [22 x i8], [22 x i8]* %.16, i64 0, i64 7
  store i8 99, i8* %.16.repack92, align 1
  %.16.repack93 = getelementptr inbounds [22 x i8], [22 x i8]* %.16, i64 0, i64 8
  store i8 114, i8* %.16.repack93, align 1
  %.16.repack94 = getelementptr inbounds [22 x i8], [22 x i8]* %.16, i64 0, i64 9
  store i8 101, i8* %.16.repack94, align 1
  %.16.repack95 = getelementptr inbounds [22 x i8], [22 x i8]* %.16, i64 0, i64 10
  store i8 97, i8* %.16.repack95, align 1
  %.16.repack96 = getelementptr inbounds [22 x i8], [22 x i8]* %.16, i64 0, i64 11
  store i8 116, i8* %.16.repack96, align 1
  %.16.repack97 = getelementptr inbounds [22 x i8], [22 x i8]* %.16, i64 0, i64 12
  store i8 101, i8* %.16.repack97, align 1
  %.16.repack98 = getelementptr inbounds [22 x i8], [22 x i8]* %.16, i64 0, i64 13
  store i8 32, i8* %.16.repack98, align 1
  %.16.repack99 = getelementptr inbounds [22 x i8], [22 x i8]* %.16, i64 0, i64 14
  store i8 115, i8* %.16.repack99, align 1
  %.16.repack100 = getelementptr inbounds [22 x i8], [22 x i8]* %.16, i64 0, i64 15
  store i8 111, i8* %.16.repack100, align 1
  %.16.repack101 = getelementptr inbounds [22 x i8], [22 x i8]* %.16, i64 0, i64 16
  store i8 99, i8* %.16.repack101, align 1
  %.16.repack102 = getelementptr inbounds [22 x i8], [22 x i8]* %.16, i64 0, i64 17
  store i8 107, i8* %.16.repack102, align 1
  %.16.repack103 = getelementptr inbounds [22 x i8], [22 x i8]* %.16, i64 0, i64 18
  store i8 101, i8* %.16.repack103, align 1
  %.16.repack104 = getelementptr inbounds [22 x i8], [22 x i8]* %.16, i64 0, i64 19
  store i8 116, i8* %.16.repack104, align 1
  %.16.repack105 = getelementptr inbounds [22 x i8], [22 x i8]* %.16, i64 0, i64 20
  store i8 34, i8* %.16.repack105, align 1
  %.16.repack106 = getelementptr inbounds [22 x i8], [22 x i8]* %.16, i64 0, i64 21
  store i8 0, i8* %.16.repack106, align 1
  %.19 = call i32 @printf(i8* nonnull %.16.repack)
  %.20 = alloca [2 x i8], align 1
  %.20.repack = getelementptr inbounds [2 x i8], [2 x i8]* %.20, i64 0, i64 0
  store i8 10, i8* %.20.repack, align 1
  %.20.repack107 = getelementptr inbounds [2 x i8], [2 x i8]* %.20, i64 0, i64 1
  store i8 0, i8* %.20.repack107, align 1
  %.23 = call i32 @printf(i8* nonnull %.20.repack)
  tail call void @exit(i32 0)
  br label %entry.endif

entry.endif:                                      ; preds = %entry, %entry.if
  %sock_addr = alloca %class.sockaddr_in.8, align 8
  %sock_addr.repack = getelementptr inbounds %class.sockaddr_in.8, %class.sockaddr_in.8* %sock_addr, i64 0, i32 0
  %sock_addr.repack10 = getelementptr inbounds %class.sockaddr_in.8, %class.sockaddr_in.8* %sock_addr, i64 0, i32 1
  %sock_addr.repack12 = getelementptr inbounds %class.sockaddr_in.8, %class.sockaddr_in.8* %sock_addr, i64 0, i32 2
  %sock_addr.repack14 = getelementptr inbounds %class.sockaddr_in.8, %class.sockaddr_in.8* %sock_addr, i64 0, i32 3
  %i16 = tail call i16 @i16(i32 2)
  store i16 %i16, i16* %sock_addr.repack, align 8
  %i16.1 = tail call i16 @i16(i32 8000)
  %htons = tail call i16 @htons(i16 %i16.1)
  store i16 %htons, i16* %sock_addr.repack10, align 2
  %.34 = alloca [10 x i8], align 1
  %.34.repack = getelementptr inbounds [10 x i8], [10 x i8]* %.34, i64 0, i64 0
  store i8 49, i8* %.34.repack, align 1
  %.34.repack16 = getelementptr inbounds [10 x i8], [10 x i8]* %.34, i64 0, i64 1
  store i8 50, i8* %.34.repack16, align 1
  %.34.repack17 = getelementptr inbounds [10 x i8], [10 x i8]* %.34, i64 0, i64 2
  store i8 55, i8* %.34.repack17, align 1
  %.34.repack18 = getelementptr inbounds [10 x i8], [10 x i8]* %.34, i64 0, i64 3
  store i8 46, i8* %.34.repack18, align 1
  %.34.repack19 = getelementptr inbounds [10 x i8], [10 x i8]* %.34, i64 0, i64 4
  store i8 48, i8* %.34.repack19, align 1
  %.34.repack20 = getelementptr inbounds [10 x i8], [10 x i8]* %.34, i64 0, i64 5
  store i8 46, i8* %.34.repack20, align 1
  %.34.repack21 = getelementptr inbounds [10 x i8], [10 x i8]* %.34, i64 0, i64 6
  store i8 48, i8* %.34.repack21, align 1
  %.34.repack22 = getelementptr inbounds [10 x i8], [10 x i8]* %.34, i64 0, i64 7
  store i8 46, i8* %.34.repack22, align 1
  %.34.repack23 = getelementptr inbounds [10 x i8], [10 x i8]* %.34, i64 0, i64 8
  store i8 49, i8* %.34.repack23, align 1
  %.34.repack24 = getelementptr inbounds [10 x i8], [10 x i8]* %.34, i64 0, i64 9
  store i8 0, i8* %.34.repack24, align 1
  %inet_addr = call i32 @inet_addr(i8* nonnull %.34.repack)
  store i32 %inet_addr, i32* %sock_addr.repack12, align 4
  %i64 = call i64 @i64(i32 0)
  store i64 %i64, i64* %sock_addr.repack14, align 8
  %.40 = bitcast %class.sockaddr_in.8* %sock_addr to %class.sockaddr.7*
  %bind = call i32 @bind(i32 %socket, %class.sockaddr.7* nonnull %.40, i32 16)
  %lt = icmp slt i32 %bind, 0
  br i1 %lt, label %entry.endif.if, label %entry.endif.endif

entry.endif.if:                                   ; preds = %entry.endif
  %.43 = alloca [20 x i8], align 1
  %.43.repack = getelementptr inbounds [20 x i8], [20 x i8]* %.43, i64 0, i64 0
  store i8 34, i8* %.43.repack, align 1
  %.43.repack66 = getelementptr inbounds [20 x i8], [20 x i8]* %.43, i64 0, i64 1
  store i8 69, i8* %.43.repack66, align 1
  %.43.repack67 = getelementptr inbounds [20 x i8], [20 x i8]* %.43, i64 0, i64 2
  store i8 114, i8* %.43.repack67, align 1
  %.43.repack68 = getelementptr inbounds [20 x i8], [20 x i8]* %.43, i64 0, i64 3
  store i8 114, i8* %.43.repack68, align 1
  %.43.repack69 = getelementptr inbounds [20 x i8], [20 x i8]* %.43, i64 0, i64 4
  store i8 111, i8* %.43.repack69, align 1
  %.43.repack70 = getelementptr inbounds [20 x i8], [20 x i8]* %.43, i64 0, i64 5
  store i8 114, i8* %.43.repack70, align 1
  %.43.repack71 = getelementptr inbounds [20 x i8], [20 x i8]* %.43, i64 0, i64 6
  store i8 32, i8* %.43.repack71, align 1
  %.43.repack72 = getelementptr inbounds [20 x i8], [20 x i8]* %.43, i64 0, i64 7
  store i8 98, i8* %.43.repack72, align 1
  %.43.repack73 = getelementptr inbounds [20 x i8], [20 x i8]* %.43, i64 0, i64 8
  store i8 105, i8* %.43.repack73, align 1
  %.43.repack74 = getelementptr inbounds [20 x i8], [20 x i8]* %.43, i64 0, i64 9
  store i8 110, i8* %.43.repack74, align 1
  %.43.repack75 = getelementptr inbounds [20 x i8], [20 x i8]* %.43, i64 0, i64 10
  store i8 100, i8* %.43.repack75, align 1
  %.43.repack76 = getelementptr inbounds [20 x i8], [20 x i8]* %.43, i64 0, i64 11
  store i8 32, i8* %.43.repack76, align 1
  %.43.repack77 = getelementptr inbounds [20 x i8], [20 x i8]* %.43, i64 0, i64 12
  store i8 115, i8* %.43.repack77, align 1
  %.43.repack78 = getelementptr inbounds [20 x i8], [20 x i8]* %.43, i64 0, i64 13
  store i8 111, i8* %.43.repack78, align 1
  %.43.repack79 = getelementptr inbounds [20 x i8], [20 x i8]* %.43, i64 0, i64 14
  store i8 99, i8* %.43.repack79, align 1
  %.43.repack80 = getelementptr inbounds [20 x i8], [20 x i8]* %.43, i64 0, i64 15
  store i8 107, i8* %.43.repack80, align 1
  %.43.repack81 = getelementptr inbounds [20 x i8], [20 x i8]* %.43, i64 0, i64 16
  store i8 101, i8* %.43.repack81, align 1
  %.43.repack82 = getelementptr inbounds [20 x i8], [20 x i8]* %.43, i64 0, i64 17
  store i8 116, i8* %.43.repack82, align 1
  %.43.repack83 = getelementptr inbounds [20 x i8], [20 x i8]* %.43, i64 0, i64 18
  store i8 34, i8* %.43.repack83, align 1
  %.43.repack84 = getelementptr inbounds [20 x i8], [20 x i8]* %.43, i64 0, i64 19
  store i8 0, i8* %.43.repack84, align 1
  %.46 = call i32 @printf(i8* nonnull %.43.repack)
  %.47 = alloca [2 x i8], align 1
  %.47.repack = getelementptr inbounds [2 x i8], [2 x i8]* %.47, i64 0, i64 0
  store i8 10, i8* %.47.repack, align 1
  %.47.repack85 = getelementptr inbounds [2 x i8], [2 x i8]* %.47, i64 0, i64 1
  store i8 0, i8* %.47.repack85, align 1
  %.50 = call i32 @printf(i8* nonnull %.47.repack)
  call void @exit(i32 0)
  br label %entry.endif.endif

entry.endif.endif:                                ; preds = %entry.endif, %entry.endif.if
  %.53 = alloca [22 x i8], align 1
  %.53.repack = getelementptr inbounds [22 x i8], [22 x i8]* %.53, i64 0, i64 0
  store i8 39, i8* %.53.repack, align 1
  %.53.repack25 = getelementptr inbounds [22 x i8], [22 x i8]* %.53, i64 0, i64 1
  store i8 66, i8* %.53.repack25, align 1
  %.53.repack26 = getelementptr inbounds [22 x i8], [22 x i8]* %.53, i64 0, i64 2
  store i8 105, i8* %.53.repack26, align 1
  %.53.repack27 = getelementptr inbounds [22 x i8], [22 x i8]* %.53, i64 0, i64 3
  store i8 110, i8* %.53.repack27, align 1
  %.53.repack28 = getelementptr inbounds [22 x i8], [22 x i8]* %.53, i64 0, i64 4
  store i8 100, i8* %.53.repack28, align 1
  %.53.repack29 = getelementptr inbounds [22 x i8], [22 x i8]* %.53, i64 0, i64 5
  store i8 32, i8* %.53.repack29, align 1
  %.53.repack30 = getelementptr inbounds [22 x i8], [22 x i8]* %.53, i64 0, i64 6
  store i8 115, i8* %.53.repack30, align 1
  %.53.repack31 = getelementptr inbounds [22 x i8], [22 x i8]* %.53, i64 0, i64 7
  store i8 111, i8* %.53.repack31, align 1
  %.53.repack32 = getelementptr inbounds [22 x i8], [22 x i8]* %.53, i64 0, i64 8
  store i8 99, i8* %.53.repack32, align 1
  %.53.repack33 = getelementptr inbounds [22 x i8], [22 x i8]* %.53, i64 0, i64 9
  store i8 107, i8* %.53.repack33, align 1
  %.53.repack34 = getelementptr inbounds [22 x i8], [22 x i8]* %.53, i64 0, i64 10
  store i8 101, i8* %.53.repack34, align 1
  %.53.repack35 = getelementptr inbounds [22 x i8], [22 x i8]* %.53, i64 0, i64 11
  store i8 116, i8* %.53.repack35, align 1
  %.53.repack36 = getelementptr inbounds [22 x i8], [22 x i8]* %.53, i64 0, i64 12
  store i8 32, i8* %.53.repack36, align 1
  %.53.repack37 = getelementptr inbounds [22 x i8], [22 x i8]* %.53, i64 0, i64 13
  store i8 115, i8* %.53.repack37, align 1
  %.53.repack38 = getelementptr inbounds [22 x i8], [22 x i8]* %.53, i64 0, i64 14
  store i8 117, i8* %.53.repack38, align 1
  %.53.repack39 = getelementptr inbounds [22 x i8], [22 x i8]* %.53, i64 0, i64 15
  store i8 99, i8* %.53.repack39, align 1
  %.53.repack40 = getelementptr inbounds [22 x i8], [22 x i8]* %.53, i64 0, i64 16
  store i8 99, i8* %.53.repack40, align 1
  %.53.repack41 = getelementptr inbounds [22 x i8], [22 x i8]* %.53, i64 0, i64 17
  store i8 101, i8* %.53.repack41, align 1
  %.53.repack42 = getelementptr inbounds [22 x i8], [22 x i8]* %.53, i64 0, i64 18
  store i8 115, i8* %.53.repack42, align 1
  %.53.repack43 = getelementptr inbounds [22 x i8], [22 x i8]* %.53, i64 0, i64 19
  store i8 115, i8* %.53.repack43, align 1
  %.53.repack44 = getelementptr inbounds [22 x i8], [22 x i8]* %.53, i64 0, i64 20
  store i8 39, i8* %.53.repack44, align 1
  %.53.repack45 = getelementptr inbounds [22 x i8], [22 x i8]* %.53, i64 0, i64 21
  store i8 0, i8* %.53.repack45, align 1
  %.56 = call i32 @printf(i8* nonnull %.53.repack)
  %.57 = alloca [2 x i8], align 1
  %.57.repack = getelementptr inbounds [2 x i8], [2 x i8]* %.57, i64 0, i64 0
  store i8 10, i8* %.57.repack, align 1
  %.57.repack46 = getelementptr inbounds [2 x i8], [2 x i8]* %.57, i64 0, i64 1
  store i8 0, i8* %.57.repack46, align 1
  %.60 = call i32 @printf(i8* nonnull %.57.repack)
  %listen = call i32 @listen(i32 %socket, i32 10)
  %bytearray = call i8* @bytearray(i32 1024)
  %server = alloca [1 x i8], align 1
  %0 = getelementptr inbounds [1 x i8], [1 x i8]* %server, i64 0, i64 0
  store i8 0, i8* %0, align 1
  %.67 = alloca i32, align 4
  store i32 16, i32* %.67, align 4
  %accept = call i32 @accept(i32 %socket, %class.sockaddr.7* nonnull %.40, i32* nonnull %.67)
  %.71 = alloca [19 x i8], align 1
  %.71.repack = getelementptr inbounds [19 x i8], [19 x i8]* %.71, i64 0, i64 0
  store i8 39, i8* %.71.repack, align 1
  %.71.repack47 = getelementptr inbounds [19 x i8], [19 x i8]* %.71, i64 0, i64 1
  store i8 65, i8* %.71.repack47, align 1
  %.71.repack48 = getelementptr inbounds [19 x i8], [19 x i8]* %.71, i64 0, i64 2
  store i8 99, i8* %.71.repack48, align 1
  %.71.repack49 = getelementptr inbounds [19 x i8], [19 x i8]* %.71, i64 0, i64 3
  store i8 99, i8* %.71.repack49, align 1
  %.71.repack50 = getelementptr inbounds [19 x i8], [19 x i8]* %.71, i64 0, i64 4
  store i8 101, i8* %.71.repack50, align 1
  %.71.repack51 = getelementptr inbounds [19 x i8], [19 x i8]* %.71, i64 0, i64 5
  store i8 112, i8* %.71.repack51, align 1
  %.71.repack52 = getelementptr inbounds [19 x i8], [19 x i8]* %.71, i64 0, i64 6
  store i8 116, i8* %.71.repack52, align 1
  %.71.repack53 = getelementptr inbounds [19 x i8], [19 x i8]* %.71, i64 0, i64 7
  store i8 32, i8* %.71.repack53, align 1
  %.71.repack54 = getelementptr inbounds [19 x i8], [19 x i8]* %.71, i64 0, i64 8
  store i8 99, i8* %.71.repack54, align 1
  %.71.repack55 = getelementptr inbounds [19 x i8], [19 x i8]* %.71, i64 0, i64 9
  store i8 111, i8* %.71.repack55, align 1
  %.71.repack56 = getelementptr inbounds [19 x i8], [19 x i8]* %.71, i64 0, i64 10
  store i8 110, i8* %.71.repack56, align 1
  %.71.repack57 = getelementptr inbounds [19 x i8], [19 x i8]* %.71, i64 0, i64 11
  store i8 110, i8* %.71.repack57, align 1
  %.71.repack58 = getelementptr inbounds [19 x i8], [19 x i8]* %.71, i64 0, i64 12
  store i8 101, i8* %.71.repack58, align 1
  %.71.repack59 = getelementptr inbounds [19 x i8], [19 x i8]* %.71, i64 0, i64 13
  store i8 99, i8* %.71.repack59, align 1
  %.71.repack60 = getelementptr inbounds [19 x i8], [19 x i8]* %.71, i64 0, i64 14
  store i8 116, i8* %.71.repack60, align 1
  %.71.repack61 = getelementptr inbounds [19 x i8], [19 x i8]* %.71, i64 0, i64 15
  store i8 92, i8* %.71.repack61, align 1
  %.71.repack62 = getelementptr inbounds [19 x i8], [19 x i8]* %.71, i64 0, i64 16
  store i8 110, i8* %.71.repack62, align 1
  %.71.repack63 = getelementptr inbounds [19 x i8], [19 x i8]* %.71, i64 0, i64 17
  store i8 39, i8* %.71.repack63, align 1
  %.71.repack64 = getelementptr inbounds [19 x i8], [19 x i8]* %.71, i64 0, i64 18
  store i8 0, i8* %.71.repack64, align 1
  %.74 = call i32 @printf(i8* nonnull %.71.repack)
  %i64.1 = call i64 @i64(i32 1024)
  %recv = call i32 @recv(i32 %accept, i8* %bytearray, i64 %i64.1, i32 0)
  %i64.2 = call i64 @i64(i32 33)
  %send = call i32 @send(i32 %accept, i8* nonnull %0, i64 %i64.2, i32 0)
  %close = call i32 @close(i32 %accept)
  %close.1 = call i32 @close(i32 %socket)
  ret i32 0
}

declare i32 @socket(i32, i32, i32) local_unnamed_addr

; Function Attrs: nofree nounwind
declare noundef i32 @printf(i8* nocapture noundef readonly) local_unnamed_addr #8

declare void @exit(i32) local_unnamed_addr

; Function Attrs: nofree nosync nounwind readnone
declare i16 @htons(i16) local_unnamed_addr #7

declare i32 @inet_addr(i8*) local_unnamed_addr

declare i32 @bind(i32, %class.sockaddr.7*, i32) local_unnamed_addr

declare i32 @listen(i32, i32) local_unnamed_addr

declare i32 @accept(i32, %class.sockaddr.7*, i32*) local_unnamed_addr

declare i32 @recv(i32, i8*, i64, i32) local_unnamed_addr

declare i32 @send(i32, i8*, i64, i32) local_unnamed_addr

declare i32 @close(i32) local_unnamed_addr

; Function Attrs: noreturn
define i32 @main() local_unnamed_addr #9 {
entry:
  %socket = tail call i32 @socket(i32 2, i32 1, i32 0)
  %.5 = alloca [2 x i8], align 1
  %.5.repack = getelementptr inbounds [2 x i8], [2 x i8]* %.5, i64 0, i64 0
  store i8 10, i8* %.5.repack, align 1
  %.5.repack1 = getelementptr inbounds [2 x i8], [2 x i8]* %.5, i64 0, i64 1
  store i8 0, i8* %.5.repack1, align 1
  %.8 = call i32 @printf(i8* nonnull %.5.repack)
  %.9 = alloca [2 x i8], align 1
  %.9.repack = getelementptr inbounds [2 x i8], [2 x i8]* %.9, i64 0, i64 0
  store i8 10, i8* %.9.repack, align 1
  %.9.repack2 = getelementptr inbounds [2 x i8], [2 x i8]* %.9, i64 0, i64 1
  store i8 0, i8* %.9.repack2, align 1
  %.12 = call i32 @printf(i8* nonnull %.9.repack)
  %sock_addr = alloca %class.sockaddr_in.8, align 8
  %sock_addr.repack = getelementptr inbounds %class.sockaddr_in.8, %class.sockaddr_in.8* %sock_addr, i64 0, i32 0
  %sock_addr.repack10 = getelementptr inbounds %class.sockaddr_in.8, %class.sockaddr_in.8* %sock_addr, i64 0, i32 1
  %sock_addr.repack12 = getelementptr inbounds %class.sockaddr_in.8, %class.sockaddr_in.8* %sock_addr, i64 0, i32 2
  %sock_addr.repack14 = getelementptr inbounds %class.sockaddr_in.8, %class.sockaddr_in.8* %sock_addr, i64 0, i32 3
  %i16 = tail call i16 @i16(i32 2)
  store i16 %i16, i16* %sock_addr.repack, align 8
  %i16.1 = tail call i16 @i16(i32 8000)
  %htons = tail call i16 @htons(i16 %i16.1)
  store i16 %htons, i16* %sock_addr.repack10, align 2
  %.21 = alloca [10 x i8], align 1
  %.21.repack = getelementptr inbounds [10 x i8], [10 x i8]* %.21, i64 0, i64 0
  store i8 49, i8* %.21.repack, align 1
  %.21.repack16 = getelementptr inbounds [10 x i8], [10 x i8]* %.21, i64 0, i64 1
  store i8 50, i8* %.21.repack16, align 1
  %.21.repack17 = getelementptr inbounds [10 x i8], [10 x i8]* %.21, i64 0, i64 2
  store i8 55, i8* %.21.repack17, align 1
  %.21.repack18 = getelementptr inbounds [10 x i8], [10 x i8]* %.21, i64 0, i64 3
  store i8 46, i8* %.21.repack18, align 1
  %.21.repack19 = getelementptr inbounds [10 x i8], [10 x i8]* %.21, i64 0, i64 4
  store i8 48, i8* %.21.repack19, align 1
  %.21.repack20 = getelementptr inbounds [10 x i8], [10 x i8]* %.21, i64 0, i64 5
  store i8 46, i8* %.21.repack20, align 1
  %.21.repack21 = getelementptr inbounds [10 x i8], [10 x i8]* %.21, i64 0, i64 6
  store i8 48, i8* %.21.repack21, align 1
  %.21.repack22 = getelementptr inbounds [10 x i8], [10 x i8]* %.21, i64 0, i64 7
  store i8 46, i8* %.21.repack22, align 1
  %.21.repack23 = getelementptr inbounds [10 x i8], [10 x i8]* %.21, i64 0, i64 8
  store i8 49, i8* %.21.repack23, align 1
  %.21.repack24 = getelementptr inbounds [10 x i8], [10 x i8]* %.21, i64 0, i64 9
  store i8 0, i8* %.21.repack24, align 1
  %inet_addr = call i32 @inet_addr(i8* nonnull %.21.repack)
  store i32 %inet_addr, i32* %sock_addr.repack12, align 4
  %i64 = call i64 @i64(i32 0)
  store i64 %i64, i64* %sock_addr.repack14, align 8
  %.27 = bitcast %class.sockaddr_in.8* %sock_addr to %class.sockaddr.7*
  %bind = call i32 @bind(i32 %socket, %class.sockaddr.7* nonnull %.27, i32 16)
  %.29 = alloca [22 x i8], align 1
  %.29.repack = getelementptr inbounds [22 x i8], [22 x i8]* %.29, i64 0, i64 0
  store i8 39, i8* %.29.repack, align 1
  %.29.repack25 = getelementptr inbounds [22 x i8], [22 x i8]* %.29, i64 0, i64 1
  store i8 66, i8* %.29.repack25, align 1
  %.29.repack26 = getelementptr inbounds [22 x i8], [22 x i8]* %.29, i64 0, i64 2
  store i8 105, i8* %.29.repack26, align 1
  %.29.repack27 = getelementptr inbounds [22 x i8], [22 x i8]* %.29, i64 0, i64 3
  store i8 110, i8* %.29.repack27, align 1
  %.29.repack28 = getelementptr inbounds [22 x i8], [22 x i8]* %.29, i64 0, i64 4
  store i8 100, i8* %.29.repack28, align 1
  %.29.repack29 = getelementptr inbounds [22 x i8], [22 x i8]* %.29, i64 0, i64 5
  store i8 32, i8* %.29.repack29, align 1
  %.29.repack30 = getelementptr inbounds [22 x i8], [22 x i8]* %.29, i64 0, i64 6
  store i8 115, i8* %.29.repack30, align 1
  %.29.repack31 = getelementptr inbounds [22 x i8], [22 x i8]* %.29, i64 0, i64 7
  store i8 111, i8* %.29.repack31, align 1
  %.29.repack32 = getelementptr inbounds [22 x i8], [22 x i8]* %.29, i64 0, i64 8
  store i8 99, i8* %.29.repack32, align 1
  %.29.repack33 = getelementptr inbounds [22 x i8], [22 x i8]* %.29, i64 0, i64 9
  store i8 107, i8* %.29.repack33, align 1
  %.29.repack34 = getelementptr inbounds [22 x i8], [22 x i8]* %.29, i64 0, i64 10
  store i8 101, i8* %.29.repack34, align 1
  %.29.repack35 = getelementptr inbounds [22 x i8], [22 x i8]* %.29, i64 0, i64 11
  store i8 116, i8* %.29.repack35, align 1
  %.29.repack36 = getelementptr inbounds [22 x i8], [22 x i8]* %.29, i64 0, i64 12
  store i8 32, i8* %.29.repack36, align 1
  %.29.repack37 = getelementptr inbounds [22 x i8], [22 x i8]* %.29, i64 0, i64 13
  store i8 115, i8* %.29.repack37, align 1
  %.29.repack38 = getelementptr inbounds [22 x i8], [22 x i8]* %.29, i64 0, i64 14
  store i8 117, i8* %.29.repack38, align 1
  %.29.repack39 = getelementptr inbounds [22 x i8], [22 x i8]* %.29, i64 0, i64 15
  store i8 99, i8* %.29.repack39, align 1
  %.29.repack40 = getelementptr inbounds [22 x i8], [22 x i8]* %.29, i64 0, i64 16
  store i8 99, i8* %.29.repack40, align 1
  %.29.repack41 = getelementptr inbounds [22 x i8], [22 x i8]* %.29, i64 0, i64 17
  store i8 101, i8* %.29.repack41, align 1
  %.29.repack42 = getelementptr inbounds [22 x i8], [22 x i8]* %.29, i64 0, i64 18
  store i8 115, i8* %.29.repack42, align 1
  %.29.repack43 = getelementptr inbounds [22 x i8], [22 x i8]* %.29, i64 0, i64 19
  store i8 115, i8* %.29.repack43, align 1
  %.29.repack44 = getelementptr inbounds [22 x i8], [22 x i8]* %.29, i64 0, i64 20
  store i8 39, i8* %.29.repack44, align 1
  %.29.repack45 = getelementptr inbounds [22 x i8], [22 x i8]* %.29, i64 0, i64 21
  store i8 0, i8* %.29.repack45, align 1
  %.32 = call i32 @printf(i8* nonnull %.29.repack)
  %.33 = alloca [2 x i8], align 1
  %.33.repack = getelementptr inbounds [2 x i8], [2 x i8]* %.33, i64 0, i64 0
  store i8 10, i8* %.33.repack, align 1
  %.33.repack46 = getelementptr inbounds [2 x i8], [2 x i8]* %.33, i64 0, i64 1
  store i8 0, i8* %.33.repack46, align 1
  %.36 = call i32 @printf(i8* nonnull %.33.repack)
  %listen = call i32 @listen(i32 %socket, i32 10)
  %bytearray = call i8* @bytearray(i32 1024)
  %output = alloca i8*, align 8
  store i8* %bytearray, i8** %output, align 8
  %server = alloca [93 x i8], align 1
  %server.repack = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 0
  store i8 72, i8* %server.repack, align 1
  %server.repack47 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 1
  store i8 84, i8* %server.repack47, align 1
  %server.repack48 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 2
  store i8 84, i8* %server.repack48, align 1
  %server.repack49 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 3
  store i8 80, i8* %server.repack49, align 1
  %server.repack50 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 4
  store i8 47, i8* %server.repack50, align 1
  %server.repack51 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 5
  store i8 49, i8* %server.repack51, align 1
  %server.repack52 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 6
  store i8 46, i8* %server.repack52, align 1
  %server.repack53 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 7
  store i8 49, i8* %server.repack53, align 1
  %server.repack54 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 8
  store i8 32, i8* %server.repack54, align 1
  %server.repack55 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 9
  store i8 50, i8* %server.repack55, align 1
  %server.repack56 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 10
  store i8 48, i8* %server.repack56, align 1
  %server.repack57 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 11
  store i8 48, i8* %server.repack57, align 1
  %server.repack58 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 12
  store i8 32, i8* %server.repack58, align 1
  %server.repack59 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 13
  store i8 79, i8* %server.repack59, align 1
  %server.repack60 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 14
  store i8 75, i8* %server.repack60, align 1
  %server.repack61 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 15
  store i8 10, i8* %server.repack61, align 1
  %server.repack62 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 16
  store i8 83, i8* %server.repack62, align 1
  %server.repack63 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 17
  store i8 101, i8* %server.repack63, align 1
  %server.repack64 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 18
  store i8 114, i8* %server.repack64, align 1
  %server.repack65 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 19
  store i8 118, i8* %server.repack65, align 1
  %server.repack66 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 20
  store i8 101, i8* %server.repack66, align 1
  %server.repack67 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 21
  store i8 114, i8* %server.repack67, align 1
  %server.repack68 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 22
  store i8 58, i8* %server.repack68, align 1
  %server.repack69 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 23
  store i8 32, i8* %server.repack69, align 1
  %server.repack70 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 24
  store i8 90, i8* %server.repack70, align 1
  %server.repack71 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 25
  store i8 45, i8* %server.repack71, align 1
  %server.repack72 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 26
  store i8 83, i8* %server.repack72, align 1
  %server.repack73 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 27
  store i8 101, i8* %server.repack73, align 1
  %server.repack74 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 28
  store i8 114, i8* %server.repack74, align 1
  %server.repack75 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 29
  store i8 118, i8* %server.repack75, align 1
  %server.repack76 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 30
  store i8 101, i8* %server.repack76, align 1
  %server.repack77 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 31
  store i8 114, i8* %server.repack77, align 1
  %server.repack78 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 32
  store i8 10, i8* %server.repack78, align 1
  %server.repack79 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 33
  store i8 67, i8* %server.repack79, align 1
  %server.repack80 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 34
  store i8 111, i8* %server.repack80, align 1
  %server.repack81 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 35
  store i8 110, i8* %server.repack81, align 1
  %server.repack82 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 36
  store i8 116, i8* %server.repack82, align 1
  %server.repack83 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 37
  store i8 101, i8* %server.repack83, align 1
  %server.repack84 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 38
  store i8 110, i8* %server.repack84, align 1
  %server.repack85 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 39
  store i8 116, i8* %server.repack85, align 1
  %server.repack86 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 40
  store i8 45, i8* %server.repack86, align 1
  %server.repack87 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 41
  store i8 116, i8* %server.repack87, align 1
  %server.repack88 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 42
  store i8 121, i8* %server.repack88, align 1
  %server.repack89 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 43
  store i8 112, i8* %server.repack89, align 1
  %server.repack90 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 44
  store i8 101, i8* %server.repack90, align 1
  %server.repack91 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 45
  store i8 58, i8* %server.repack91, align 1
  %server.repack92 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 46
  store i8 32, i8* %server.repack92, align 1
  %server.repack93 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 47
  store i8 116, i8* %server.repack93, align 1
  %server.repack94 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 48
  store i8 101, i8* %server.repack94, align 1
  %server.repack95 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 49
  store i8 120, i8* %server.repack95, align 1
  %server.repack96 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 50
  store i8 116, i8* %server.repack96, align 1
  %server.repack97 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 51
  store i8 47, i8* %server.repack97, align 1
  %server.repack98 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 52
  store i8 104, i8* %server.repack98, align 1
  %server.repack99 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 53
  store i8 116, i8* %server.repack99, align 1
  %server.repack100 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 54
  store i8 109, i8* %server.repack100, align 1
  %server.repack101 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 55
  store i8 108, i8* %server.repack101, align 1
  %server.repack102 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 56
  store i8 10, i8* %server.repack102, align 1
  %server.repack103 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 57
  store i8 10, i8* %server.repack103, align 1
  %server.repack104 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 58
  store i8 60, i8* %server.repack104, align 1
  %server.repack105 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 59
  store i8 104, i8* %server.repack105, align 1
  %server.repack106 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 60
  store i8 116, i8* %server.repack106, align 1
  %server.repack107 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 61
  store i8 109, i8* %server.repack107, align 1
  %server.repack108 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 62
  store i8 108, i8* %server.repack108, align 1
  %server.repack109 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 63
  store i8 62, i8* %server.repack109, align 1
  %server.repack110 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 64
  store i8 60, i8* %server.repack110, align 1
  %server.repack111 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 65
  store i8 98, i8* %server.repack111, align 1
  %server.repack112 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 66
  store i8 111, i8* %server.repack112, align 1
  %server.repack113 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 67
  store i8 100, i8* %server.repack113, align 1
  %server.repack114 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 68
  store i8 121, i8* %server.repack114, align 1
  %server.repack115 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 69
  store i8 62, i8* %server.repack115, align 1
  %server.repack116 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 70
  store i8 90, i8* %server.repack116, align 1
  %server.repack117 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 71
  store i8 45, i8* %server.repack117, align 1
  %server.repack118 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 72
  store i8 83, i8* %server.repack118, align 1
  %server.repack119 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 73
  store i8 101, i8* %server.repack119, align 1
  %server.repack120 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 74
  store i8 114, i8* %server.repack120, align 1
  %server.repack121 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 75
  store i8 118, i8* %server.repack121, align 1
  %server.repack122 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 76
  store i8 101, i8* %server.repack122, align 1
  %server.repack123 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 77
  store i8 114, i8* %server.repack123, align 1
  %server.repack124 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 78
  store i8 60, i8* %server.repack124, align 1
  %server.repack125 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 79
  store i8 47, i8* %server.repack125, align 1
  %server.repack126 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 80
  store i8 98, i8* %server.repack126, align 1
  %server.repack127 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 81
  store i8 111, i8* %server.repack127, align 1
  %server.repack128 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 82
  store i8 100, i8* %server.repack128, align 1
  %server.repack129 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 83
  store i8 121, i8* %server.repack129, align 1
  %server.repack130 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 84
  store i8 62, i8* %server.repack130, align 1
  %server.repack131 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 85
  store i8 60, i8* %server.repack131, align 1
  %server.repack132 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 86
  store i8 47, i8* %server.repack132, align 1
  %server.repack133 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 87
  store i8 104, i8* %server.repack133, align 1
  %server.repack134 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 88
  store i8 116, i8* %server.repack134, align 1
  %server.repack135 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 89
  store i8 109, i8* %server.repack135, align 1
  %server.repack136 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 90
  store i8 108, i8* %server.repack136, align 1
  %server.repack137 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 91
  store i8 62, i8* %server.repack137, align 1
  %server.repack138 = getelementptr inbounds [93 x i8], [93 x i8]* %server, i64 0, i64 92
  store i8 0, i8* %server.repack138, align 1
  %output.1 = load i8*, i8** %output, align 8
  %.53 = bitcast i8** %output to i8*
  br label %while_loop_entry

while_loop_entry:                                 ; preds = %while_loop_entry, %entry
  %.44 = alloca i32, align 4
  store i32 16, i32* %.44, align 4
  %accept = call i32 @accept(i32 %socket, %class.sockaddr.7* nonnull %.27, i32* nonnull %.44)
  %.47 = alloca [19 x i8], align 1
  %.47.repack = getelementptr inbounds [19 x i8], [19 x i8]* %.47, i64 0, i64 0
  store i8 39, i8* %.47.repack, align 1
  %.47.repack139 = getelementptr inbounds [19 x i8], [19 x i8]* %.47, i64 0, i64 1
  store i8 65, i8* %.47.repack139, align 1
  %.47.repack140 = getelementptr inbounds [19 x i8], [19 x i8]* %.47, i64 0, i64 2
  store i8 99, i8* %.47.repack140, align 1
  %.47.repack141 = getelementptr inbounds [19 x i8], [19 x i8]* %.47, i64 0, i64 3
  store i8 99, i8* %.47.repack141, align 1
  %.47.repack142 = getelementptr inbounds [19 x i8], [19 x i8]* %.47, i64 0, i64 4
  store i8 101, i8* %.47.repack142, align 1
  %.47.repack143 = getelementptr inbounds [19 x i8], [19 x i8]* %.47, i64 0, i64 5
  store i8 112, i8* %.47.repack143, align 1
  %.47.repack144 = getelementptr inbounds [19 x i8], [19 x i8]* %.47, i64 0, i64 6
  store i8 116, i8* %.47.repack144, align 1
  %.47.repack145 = getelementptr inbounds [19 x i8], [19 x i8]* %.47, i64 0, i64 7
  store i8 32, i8* %.47.repack145, align 1
  %.47.repack146 = getelementptr inbounds [19 x i8], [19 x i8]* %.47, i64 0, i64 8
  store i8 99, i8* %.47.repack146, align 1
  %.47.repack147 = getelementptr inbounds [19 x i8], [19 x i8]* %.47, i64 0, i64 9
  store i8 111, i8* %.47.repack147, align 1
  %.47.repack148 = getelementptr inbounds [19 x i8], [19 x i8]* %.47, i64 0, i64 10
  store i8 110, i8* %.47.repack148, align 1
  %.47.repack149 = getelementptr inbounds [19 x i8], [19 x i8]* %.47, i64 0, i64 11
  store i8 110, i8* %.47.repack149, align 1
  %.47.repack150 = getelementptr inbounds [19 x i8], [19 x i8]* %.47, i64 0, i64 12
  store i8 101, i8* %.47.repack150, align 1
  %.47.repack151 = getelementptr inbounds [19 x i8], [19 x i8]* %.47, i64 0, i64 13
  store i8 99, i8* %.47.repack151, align 1
  %.47.repack152 = getelementptr inbounds [19 x i8], [19 x i8]* %.47, i64 0, i64 14
  store i8 116, i8* %.47.repack152, align 1
  %.47.repack153 = getelementptr inbounds [19 x i8], [19 x i8]* %.47, i64 0, i64 15
  store i8 92, i8* %.47.repack153, align 1
  %.47.repack154 = getelementptr inbounds [19 x i8], [19 x i8]* %.47, i64 0, i64 16
  store i8 110, i8* %.47.repack154, align 1
  %.47.repack155 = getelementptr inbounds [19 x i8], [19 x i8]* %.47, i64 0, i64 17
  store i8 39, i8* %.47.repack155, align 1
  %.47.repack156 = getelementptr inbounds [19 x i8], [19 x i8]* %.47, i64 0, i64 18
  store i8 0, i8* %.47.repack156, align 1
  %.50 = call i32 @printf(i8* nonnull %.47.repack)
  %i64.1 = call i64 @i64(i32 1024)
  %recv = call i32 @recv(i32 %accept, i8* %output.1, i64 %i64.1, i32 0)
  %.54 = call i32 @printf(i8* nonnull %.53)
  %i64.2 = call i64 @i64(i32 96)
  %send = call i32 @send(i32 %accept, i8* nonnull %server.repack, i64 %i64.2, i32 0)
  %close = call i32 @close(i32 %accept)
  br label %while_loop_entry
}

attributes #0 = { mustprogress nofree norecurse nosync nounwind uwtable willreturn "frame-pointer"="none" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #1 = { nofree norecurse nosync nounwind uwtable "frame-pointer"="none" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #2 = { nofree nosync nounwind uwtable "frame-pointer"="none" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #3 = { mustprogress nofree norecurse nosync nounwind readnone uwtable willreturn "frame-pointer"="none" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #4 = { mustprogress nofree nounwind uwtable willreturn "frame-pointer"="none" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #5 = { inaccessiblememonly mustprogress nofree nounwind willreturn "frame-pointer"="none" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #6 = { mustprogress nofree nosync nounwind readnone speculatable willreturn }
attributes #7 = { nofree nosync nounwind readnone }
attributes #8 = { nofree nounwind }
attributes #9 = { noreturn }
attributes #10 = { nounwind }

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
