; ModuleID = '<string>'
source_filename = "builtin.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

%class.sockaddr_in.3 = type { i16, i16, i32, i64 }
%class.sockaddr.2 = type { i16, [14 x i8] }

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
  %3 = tail call noalias i8* @malloc(i64 noundef %2) #9
  ret i8* %3
}

; Function Attrs: inaccessiblememonly mustprogress nofree nounwind willreturn
declare noalias noundef i8* @malloc(i64 noundef) local_unnamed_addr #5

; Function Attrs: mustprogress nofree nosync nounwind readnone speculatable willreturn
declare i32 @llvm.abs.i32(i32, i1 immarg) #6

define i32 @main() local_unnamed_addr {
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
  %sock_addr = alloca %class.sockaddr_in.3, align 8
  %sock_addr.repack = getelementptr inbounds %class.sockaddr_in.3, %class.sockaddr_in.3* %sock_addr, i64 0, i32 0
  %sock_addr.repack10 = getelementptr inbounds %class.sockaddr_in.3, %class.sockaddr_in.3* %sock_addr, i64 0, i32 1
  %sock_addr.repack12 = getelementptr inbounds %class.sockaddr_in.3, %class.sockaddr_in.3* %sock_addr, i64 0, i32 2
  %sock_addr.repack14 = getelementptr inbounds %class.sockaddr_in.3, %class.sockaddr_in.3* %sock_addr, i64 0, i32 3
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
  %.27 = bitcast %class.sockaddr_in.3* %sock_addr to %class.sockaddr.2*
  %bind = call i32 @bind(i32 %socket, %class.sockaddr.2* nonnull %.27, i32 16)
  %listen = call i32 @listen(i32 %socket, i32 10)
  %bytearray = call i8* @bytearray(i32 1024)
  %output = alloca i8*, align 8
  store i8* %bytearray, i8** %output, align 8
  %server = alloca [34 x i8], align 1
  %server.repack = getelementptr inbounds [34 x i8], [34 x i8]* %server, i64 0, i64 0
  store i8 72, i8* %server.repack, align 1
  %server.repack25 = getelementptr inbounds [34 x i8], [34 x i8]* %server, i64 0, i64 1
  store i8 84, i8* %server.repack25, align 1
  %server.repack26 = getelementptr inbounds [34 x i8], [34 x i8]* %server, i64 0, i64 2
  store i8 84, i8* %server.repack26, align 1
  %server.repack27 = getelementptr inbounds [34 x i8], [34 x i8]* %server, i64 0, i64 3
  store i8 80, i8* %server.repack27, align 1
  %server.repack28 = getelementptr inbounds [34 x i8], [34 x i8]* %server, i64 0, i64 4
  store i8 47, i8* %server.repack28, align 1
  %server.repack29 = getelementptr inbounds [34 x i8], [34 x i8]* %server, i64 0, i64 5
  store i8 49, i8* %server.repack29, align 1
  %server.repack30 = getelementptr inbounds [34 x i8], [34 x i8]* %server, i64 0, i64 6
  store i8 46, i8* %server.repack30, align 1
  %server.repack31 = getelementptr inbounds [34 x i8], [34 x i8]* %server, i64 0, i64 7
  store i8 49, i8* %server.repack31, align 1
  %server.repack32 = getelementptr inbounds [34 x i8], [34 x i8]* %server, i64 0, i64 8
  store i8 32, i8* %server.repack32, align 1
  %server.repack33 = getelementptr inbounds [34 x i8], [34 x i8]* %server, i64 0, i64 9
  store i8 50, i8* %server.repack33, align 1
  %server.repack34 = getelementptr inbounds [34 x i8], [34 x i8]* %server, i64 0, i64 10
  store i8 48, i8* %server.repack34, align 1
  %server.repack35 = getelementptr inbounds [34 x i8], [34 x i8]* %server, i64 0, i64 11
  store i8 48, i8* %server.repack35, align 1
  %server.repack36 = getelementptr inbounds [34 x i8], [34 x i8]* %server, i64 0, i64 12
  store i8 32, i8* %server.repack36, align 1
  %server.repack37 = getelementptr inbounds [34 x i8], [34 x i8]* %server, i64 0, i64 13
  store i8 79, i8* %server.repack37, align 1
  %server.repack38 = getelementptr inbounds [34 x i8], [34 x i8]* %server, i64 0, i64 14
  store i8 75, i8* %server.repack38, align 1
  %server.repack39 = getelementptr inbounds [34 x i8], [34 x i8]* %server, i64 0, i64 15
  store i8 10, i8* %server.repack39, align 1
  %server.repack40 = getelementptr inbounds [34 x i8], [34 x i8]* %server, i64 0, i64 16
  store i8 83, i8* %server.repack40, align 1
  %server.repack41 = getelementptr inbounds [34 x i8], [34 x i8]* %server, i64 0, i64 17
  store i8 101, i8* %server.repack41, align 1
  %server.repack42 = getelementptr inbounds [34 x i8], [34 x i8]* %server, i64 0, i64 18
  store i8 114, i8* %server.repack42, align 1
  %server.repack43 = getelementptr inbounds [34 x i8], [34 x i8]* %server, i64 0, i64 19
  store i8 118, i8* %server.repack43, align 1
  %server.repack44 = getelementptr inbounds [34 x i8], [34 x i8]* %server, i64 0, i64 20
  store i8 101, i8* %server.repack44, align 1
  %server.repack45 = getelementptr inbounds [34 x i8], [34 x i8]* %server, i64 0, i64 21
  store i8 114, i8* %server.repack45, align 1
  %server.repack46 = getelementptr inbounds [34 x i8], [34 x i8]* %server, i64 0, i64 22
  store i8 58, i8* %server.repack46, align 1
  %server.repack47 = getelementptr inbounds [34 x i8], [34 x i8]* %server, i64 0, i64 23
  store i8 32, i8* %server.repack47, align 1
  %server.repack48 = getelementptr inbounds [34 x i8], [34 x i8]* %server, i64 0, i64 24
  store i8 90, i8* %server.repack48, align 1
  %server.repack49 = getelementptr inbounds [34 x i8], [34 x i8]* %server, i64 0, i64 25
  store i8 45, i8* %server.repack49, align 1
  %server.repack50 = getelementptr inbounds [34 x i8], [34 x i8]* %server, i64 0, i64 26
  store i8 83, i8* %server.repack50, align 1
  %server.repack51 = getelementptr inbounds [34 x i8], [34 x i8]* %server, i64 0, i64 27
  store i8 101, i8* %server.repack51, align 1
  %server.repack52 = getelementptr inbounds [34 x i8], [34 x i8]* %server, i64 0, i64 28
  store i8 114, i8* %server.repack52, align 1
  %server.repack53 = getelementptr inbounds [34 x i8], [34 x i8]* %server, i64 0, i64 29
  store i8 118, i8* %server.repack53, align 1
  %server.repack54 = getelementptr inbounds [34 x i8], [34 x i8]* %server, i64 0, i64 30
  store i8 101, i8* %server.repack54, align 1
  %server.repack55 = getelementptr inbounds [34 x i8], [34 x i8]* %server, i64 0, i64 31
  store i8 114, i8* %server.repack55, align 1
  %server.repack56 = getelementptr inbounds [34 x i8], [34 x i8]* %server, i64 0, i64 32
  store i8 10, i8* %server.repack56, align 1
  %server.repack57 = getelementptr inbounds [34 x i8], [34 x i8]* %server, i64 0, i64 33
  store i8 0, i8* %server.repack57, align 1
  %.35 = alloca i32, align 4
  store i32 16, i32* %.35, align 4
  %accept = call i32 @accept(i32 %socket, %class.sockaddr.2* nonnull %.27, i32* nonnull %.35)
  %.38 = alloca [19 x i8], align 1
  %.38.repack = getelementptr inbounds [19 x i8], [19 x i8]* %.38, i64 0, i64 0
  store i8 39, i8* %.38.repack, align 1
  %.38.repack58 = getelementptr inbounds [19 x i8], [19 x i8]* %.38, i64 0, i64 1
  store i8 65, i8* %.38.repack58, align 1
  %.38.repack59 = getelementptr inbounds [19 x i8], [19 x i8]* %.38, i64 0, i64 2
  store i8 99, i8* %.38.repack59, align 1
  %.38.repack60 = getelementptr inbounds [19 x i8], [19 x i8]* %.38, i64 0, i64 3
  store i8 99, i8* %.38.repack60, align 1
  %.38.repack61 = getelementptr inbounds [19 x i8], [19 x i8]* %.38, i64 0, i64 4
  store i8 101, i8* %.38.repack61, align 1
  %.38.repack62 = getelementptr inbounds [19 x i8], [19 x i8]* %.38, i64 0, i64 5
  store i8 112, i8* %.38.repack62, align 1
  %.38.repack63 = getelementptr inbounds [19 x i8], [19 x i8]* %.38, i64 0, i64 6
  store i8 116, i8* %.38.repack63, align 1
  %.38.repack64 = getelementptr inbounds [19 x i8], [19 x i8]* %.38, i64 0, i64 7
  store i8 32, i8* %.38.repack64, align 1
  %.38.repack65 = getelementptr inbounds [19 x i8], [19 x i8]* %.38, i64 0, i64 8
  store i8 99, i8* %.38.repack65, align 1
  %.38.repack66 = getelementptr inbounds [19 x i8], [19 x i8]* %.38, i64 0, i64 9
  store i8 111, i8* %.38.repack66, align 1
  %.38.repack67 = getelementptr inbounds [19 x i8], [19 x i8]* %.38, i64 0, i64 10
  store i8 110, i8* %.38.repack67, align 1
  %.38.repack68 = getelementptr inbounds [19 x i8], [19 x i8]* %.38, i64 0, i64 11
  store i8 110, i8* %.38.repack68, align 1
  %.38.repack69 = getelementptr inbounds [19 x i8], [19 x i8]* %.38, i64 0, i64 12
  store i8 101, i8* %.38.repack69, align 1
  %.38.repack70 = getelementptr inbounds [19 x i8], [19 x i8]* %.38, i64 0, i64 13
  store i8 99, i8* %.38.repack70, align 1
  %.38.repack71 = getelementptr inbounds [19 x i8], [19 x i8]* %.38, i64 0, i64 14
  store i8 116, i8* %.38.repack71, align 1
  %.38.repack72 = getelementptr inbounds [19 x i8], [19 x i8]* %.38, i64 0, i64 15
  store i8 92, i8* %.38.repack72, align 1
  %.38.repack73 = getelementptr inbounds [19 x i8], [19 x i8]* %.38, i64 0, i64 16
  store i8 110, i8* %.38.repack73, align 1
  %.38.repack74 = getelementptr inbounds [19 x i8], [19 x i8]* %.38, i64 0, i64 17
  store i8 39, i8* %.38.repack74, align 1
  %.38.repack75 = getelementptr inbounds [19 x i8], [19 x i8]* %.38, i64 0, i64 18
  store i8 0, i8* %.38.repack75, align 1
  %.41 = call i32 @printf(i8* nonnull %.38.repack)
  %i64.1 = call i64 @i64(i32 1024)
  %recv = call i32 @recv(i32 %accept, i8* %bytearray, i64 %i64.1, i32 0)
  %.44 = bitcast i8** %output to i8*
  %.45 = call i32 @printf(i8* nonnull %.44)
  %i64.2 = call i64 @i64(i32 33)
  %send = call i32 @send(i32 %accept, i8* nonnull %server.repack, i64 %i64.2, i32 0)
  %close = call i32 @close(i32 %accept)
  %close.1 = call i32 @close(i32 %socket)
  ret i32 0
}

declare i32 @socket(i32, i32, i32) local_unnamed_addr

; Function Attrs: nofree nounwind
declare noundef i32 @printf(i8* nocapture noundef readonly) local_unnamed_addr #7

; Function Attrs: nofree nosync nounwind readnone
declare i16 @htons(i16) local_unnamed_addr #8

declare i32 @inet_addr(i8*) local_unnamed_addr

declare i32 @bind(i32, %class.sockaddr.2*, i32) local_unnamed_addr

declare i32 @listen(i32, i32) local_unnamed_addr

declare i32 @accept(i32, %class.sockaddr.2*, i32*) local_unnamed_addr

declare i32 @recv(i32, i8*, i64, i32) local_unnamed_addr

declare i32 @send(i32, i8*, i64, i32) local_unnamed_addr

declare i32 @close(i32) local_unnamed_addr

attributes #0 = { mustprogress nofree norecurse nosync nounwind uwtable willreturn "frame-pointer"="none" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #1 = { nofree norecurse nosync nounwind uwtable "frame-pointer"="none" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #2 = { nofree nosync nounwind uwtable "frame-pointer"="none" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #3 = { mustprogress nofree norecurse nosync nounwind readnone uwtable willreturn "frame-pointer"="none" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #4 = { mustprogress nofree nounwind uwtable willreturn "frame-pointer"="none" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #5 = { inaccessiblememonly mustprogress nofree nounwind willreturn "frame-pointer"="none" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #6 = { mustprogress nofree nosync nounwind readnone speculatable willreturn }
attributes #7 = { nofree nounwind }
attributes #8 = { nofree nosync nounwind readnone }
attributes #9 = { nounwind }

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
