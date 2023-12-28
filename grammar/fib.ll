declare void @"write"(i64 %".1")

define i64 @"fib"(i64 %".1")
{
entry:
  %"i" = alloca i64, i32 8
  %"_ret.1" = alloca i64, i32 8
  store i64 %".1", i64* %"i"
  %"i.1" = load i64, i64* %"i"
  %"i.2" = load i64, i64* %"i"
  %"le" = icmp sle i64 %"i.2", 1
  br i1 %"le", label %"entry.if", label %"entry.else"
entry.if:
  %"i.3" = load i64, i64* %"i"
  store i64 %"i.3", i64* %"_ret.1"
  br label %"_ret"
entry.else:
  %"f1" = alloca i64, i32 8
  %"i.4" = load i64, i64* %"i"
  %"sub_tmp" = sub i64 %"i.4", 1
  %"fib" = call i64 @"fib"(i64 %"sub_tmp")
  %"f2" = alloca i64, i32 8
  %"i.5" = load i64, i64* %"i"
  %"sub_tmp.1" = sub i64 %"i.5", 2
  %"fib.1" = call i64 @"fib"(i64 %"sub_tmp.1")
  %"f1.1" = load i64, i64* %"f1"
  %"f2.1" = load i64, i64* %"f2"
  %"add_tmp" = add i64 %"f1.1", %"f2.1"
  store i64 %"add_tmp", i64* %"_ret.1"
  br label %"_ret"
entry.endif:
  store i64 0, i64* %"_ret.1"
  br label %"_ret"
_ret:
  %"fib.2" = load i64, i64* %"_ret.1"
  ret i64 %"fib.2"
}

define i64 @"main"()
{
entry:
  %"_ret.1" = alloca i64, i32 8
  %"fib" = call i64 @"fib"(i64 40)
  call void @"write"(i64 %"fib")
  store i64 0, i64* %"_ret.1"
  br label %"_ret"
_ret:
  %"main" = load i64, i64* %"_ret.1"
  ret i64 %"main"
}