; ModuleID = 'Std-e931527708f450d7.bc'
source_filename = "Std.e1rrk4sh-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%"std::fmt::Arguments" = type { [0 x i64], { [0 x { [0 x i8]*, i64 }]*, i64 }, [0 x i64], { i64*, i64 }, [0 x i64], { [0 x { i8*, i64* }]*, i64 }, [0 x i64] }
%"unwind::libunwind::_Unwind_Exception" = type { [0 x i64], i64, [0 x i64], void (i32, %"unwind::libunwind::_Unwind_Exception"*)*, [0 x i64], [6 x i64], [0 x i64] }
%"unwind::libunwind::_Unwind_Context" = type { [0 x i8] }

@vtable.0 = private unnamed_addr constant { void (i64**)*, i64, i64, i32 (i64**)*, i32 (i64**)*, i32 (i64**)* } { void (i64**)* @_ZN4core3ptr13drop_in_place17hf6ff9c08695adfa8E, i64 8, i64 8, i32 (i64**)* @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hc5630e23fbbb20daE", i32 (i64**)* @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hc5630e23fbbb20daE", i32 (i64**)* @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h435e2dca1ff79ed3E" }, align 8
@alloc1 = private unnamed_addr constant <{ [14 x i8] }> <{ [14 x i8] c"Hello, world!\0A" }>, align 1
@alloc2 = private unnamed_addr constant <{ i8*, [8 x i8] }> <{ i8* getelementptr inbounds (<{ [14 x i8] }>, <{ [14 x i8] }>* @alloc1, i32 0, i32 0, i32 0), [8 x i8] c"\0E\00\00\00\00\00\00\00" }>, align 8
@alloc4 = private unnamed_addr constant <{ [0 x i8] }> zeroinitializer, align 8

; Function Attrs: noinline nonlazybind uwtable
define internal fastcc void @_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17ha69bce592f162909E(void ()* mpk_immut nocapture nonnull %f) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %dummy.i = alloca {}, align 1
  tail call void %f()
  %0 = bitcast {}* %dummy.i to i8*
  call void @llvm.lifetime.start.p0i8(i64 0, i8* nonnull %0)
  call void asm sideeffect "", "r,~{memory},~{dirflag},~{fpsr},~{flags}"({}* nonnull %dummy.i) #6, !srcloc !3
  call void @llvm.lifetime.end.p0i8(i64 0, i8* nonnull %0)
  ret void
}

; Function Attrs: nonlazybind uwtable
define hidden i64 @_ZN3std2rt10lang_start17hba3f343a621390adE(void ()* mpk_immut nonnull %main, i64 %argc, i8** mpk_immut %argv) unnamed_addr #1 {
start:
  %_7 = alloca i64*, align 8
  %0 = bitcast i64** %_7 to i8*
  call void @llvm.lifetime.start.p0i8(i64 8, i8* nonnull %0)
  %1 = bitcast i64** %_7 to void ()**
  store void ()* %main, void ()** %1, align 8
  %_4.0 = bitcast i64** %_7 to {}*
  %2 = call i64 @_ZN3std2rt19lang_start_internal17h2677a686eee71786E({}* mpk_immut nonnull align 1 %_4.0, [3 x i64]* mpk_immut noalias readonly align 8 dereferenceable(24) bitcast ({ void (i64**)*, i64, i64, i32 (i64**)*, i32 (i64**)*, i32 (i64**)* }* @vtable.0 to [3 x i64]*), i64 %argc, i8** mpk_immut %argv)
  call void @llvm.lifetime.end.p0i8(i64 8, i8* nonnull %0)
  ret i64 %2
}

; Function Attrs: nonlazybind uwtable
define internal i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hc5630e23fbbb20daE"(i64** mpk_immut noalias nocapture readonly align 8 dereferenceable(8) %_1) unnamed_addr #1 {
start:
  %0 = bitcast i64** %_1 to void ()**
  %_3 = load void ()*, void ()** %0, align 8, !nonnull !4
  tail call fastcc void @_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17ha69bce592f162909E(void ()* mpk_immut nonnull %_3)
  ret i32 0
}

; Function Attrs: nonlazybind uwtable
define internal i32 @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h435e2dca1ff79ed3E"(i64** nocapture readonly %_1) unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %0 = bitcast i64** %_1 to void ()**
  %1 = load void ()*, void ()** %0, align 8, !nonnull !4
  tail call fastcc void @_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17ha69bce592f162909E(void ()* mpk_immut nonnull %1), !noalias !5
  ret i32 0
}

; Function Attrs: norecurse nounwind nonlazybind readnone uwtable
define internal void @_ZN4core3ptr13drop_in_place17hf6ff9c08695adfa8E(i64** mpk_unsafe nocapture %_1) unnamed_addr #2 {
start:
  ret void
}

; Function Attrs: nonlazybind uwtable
define internal void @_ZN3Std4main17h141ccc6b204faf2eE() unnamed_addr #1 {
start:
  %_2 = alloca %"std::fmt::Arguments", align 8
  %0 = bitcast %"std::fmt::Arguments"* %_2 to i8*
  call void @llvm.lifetime.start.p0i8(i64 48, i8* nonnull %0)
  %1 = bitcast %"std::fmt::Arguments"* %_2 to [0 x { [0 x i8]*, i64 }]**
  store [0 x { [0 x i8]*, i64 }]* bitcast (<{ i8*, [8 x i8] }>* @alloc2 to [0 x { [0 x i8]*, i64 }]*), [0 x { [0 x i8]*, i64 }]** %1, align 8, !alias.scope !8
  %2 = getelementptr inbounds %"std::fmt::Arguments", %"std::fmt::Arguments"* %_2, i64 0, i32 1, i32 1
  store i64 1, i64* %2, align 8, !alias.scope !8
  %3 = getelementptr inbounds %"std::fmt::Arguments", %"std::fmt::Arguments"* %_2, i64 0, i32 3, i32 0
  store i64* null, i64** %3, align 8, !alias.scope !8
  %4 = getelementptr inbounds %"std::fmt::Arguments", %"std::fmt::Arguments"* %_2, i64 0, i32 5, i32 0
  store [0 x { i8*, i64* }]* bitcast (<{ [0 x i8] }>* @alloc4 to [0 x { i8*, i64* }]*), [0 x { i8*, i64* }]** %4, align 8, !alias.scope !8
  %5 = getelementptr inbounds %"std::fmt::Arguments", %"std::fmt::Arguments"* %_2, i64 0, i32 5, i32 1
  store i64 0, i64* %5, align 8, !alias.scope !8
  call void @_ZN3std2io5stdio6_print17h17f61b0c82890e70E(%"std::fmt::Arguments"* noalias nocapture nonnull dereferenceable(48) %_2)
  call void @llvm.lifetime.end.p0i8(i64 48, i8* nonnull %0)
  ret void
}

; Function Attrs: mpk_extern nounwind nonlazybind uwtable
declare i32 @rust_eh_personality(i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"* mpk_unsafe, %"unwind::libunwind::_Unwind_Context"* mpk_unsafe) unnamed_addr #3

; Function Attrs: argmemonly nounwind willreturn
declare void @llvm.lifetime.start.p0i8(i64 immarg, i8* nocapture) #4

; Function Attrs: argmemonly nounwind willreturn
declare void @llvm.lifetime.end.p0i8(i64 immarg, i8* nocapture) #4

; Function Attrs: nonlazybind uwtable
declare i64 @_ZN3std2rt19lang_start_internal17h2677a686eee71786E({}* mpk_immut nonnull align 1, [3 x i64]* mpk_immut noalias readonly align 8 dereferenceable(24), i64, i8** mpk_immut) unnamed_addr #1

; Function Attrs: nonlazybind uwtable
declare void @_ZN3std2io5stdio6_print17h17f61b0c82890e70E(%"std::fmt::Arguments"* noalias nocapture dereferenceable(48)) unnamed_addr #1

; Function Attrs: mpk_extern nonlazybind
define i32 @main(i32 %0, i8** %1) unnamed_addr #5 {
top:
  %_7.i = alloca i64*, align 8
  %2 = sext i32 %0 to i64
  %3 = bitcast i64** %_7.i to i8*
  call void @llvm.lifetime.start.p0i8(i64 8, i8* nonnull %3)
  %4 = bitcast i64** %_7.i to void ()**
  store void ()* @_ZN3Std4main17h141ccc6b204faf2eE, void ()** %4, align 8
  %_4.0.i = bitcast i64** %_7.i to {}*
  %5 = call i64 @_ZN3std2rt19lang_start_internal17h2677a686eee71786E({}* mpk_immut nonnull align 1 %_4.0.i, [3 x i64]* mpk_immut noalias readonly align 8 dereferenceable(24) bitcast ({ void (i64**)*, i64, i64, i32 (i64**)*, i32 (i64**)*, i32 (i64**)* }* @vtable.0 to [3 x i64]*), i64 %2, i8** mpk_immut %1)
  call void @llvm.lifetime.end.p0i8(i64 8, i8* nonnull %3)
  %6 = trunc i64 %5 to i32
  ret i32 %6
}

attributes #0 = { noinline nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #1 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #2 = { norecurse nounwind nonlazybind readnone uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #3 = { mpk_extern nounwind nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #4 = { argmemonly nounwind willreturn }
attributes #5 = { mpk_extern nonlazybind "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #6 = { nounwind }

!llvm.module.flags = !{!0, !1, !2}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i32 7, !"PIE Level", i32 2}
!2 = !{i32 2, !"RtLibUseGOT", i32 1}
!3 = !{i32 2705599}
!4 = !{}
!5 = !{!6}
!6 = distinct !{!6, !7, !"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hc5630e23fbbb20daE: %_1"}
!7 = distinct !{!7, !"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hc5630e23fbbb20daE"}
!8 = !{!9}
!9 = distinct !{!9, !10, !"_ZN4core3fmt9Arguments6new_v117hef4f791f58ac5a37E: argument 0"}
!10 = distinct !{!10, !"_ZN4core3fmt9Arguments6new_v117hef4f791f58ac5a37E"}
