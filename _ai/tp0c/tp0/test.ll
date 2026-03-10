@.str.1 = private unnamed_addr constant [8 x i8] c"\48\65\6C\6C\6F\2C\20\00"
@.str.2 = private unnamed_addr constant [8 x i8] c"\72\65\73\75\6C\74\3D\00"
@.str.3 = private unnamed_addr constant [8 x i8] c"\4D\61\73\74\65\72\21\00"

declare ptr @concat(ptr, ptr)
declare i64 @print(ptr)

declare ptr @to_str(i64)

define i64 @factorial(i64 %arg_n) {
entry:
  %t1 = alloca i64
  store i64 %arg_n, ptr %t1
  %t2 = load i64, ptr %t1
  %t3 = icmp ne i64 %t2, 0
  br i1 %t3, label %L1, label %L2
L1:
  %t4 = load i64, ptr %t1
  %t5 = load i64, ptr %t1
  %t6 = sub i64 %t5, 1
  %t7 = call i64 @factorial(i64 %t6)
  %t8 = mul i64 %t4, %t7
  ret i64 %t8
L2:
  ret i64 1
}

define ptr @greet(ptr %arg_name) {
entry:
  %t1 = alloca ptr
  store ptr %arg_name, ptr %t1
  %t2 = alloca ptr
  store ptr @.str.1, ptr %t2
  %t3 = load ptr, ptr %t2
  %t4 = load ptr, ptr %t1
  %t5 = call ptr @concat(ptr %t3, ptr %t4)
  %t6 = alloca ptr
  store ptr %t5, ptr %t6
  %t7 = load ptr, ptr %t6
  %t8 = call i64 @print(ptr %t7)
  %t9 = load ptr, ptr %t6
  ret ptr %t9
}

define i64 @main() {
entry:
  %t1 = call i64 @factorial(i64 5)
  %t2 = alloca i64
  store i64 %t1, ptr %t2
  %t3 = load i64, ptr %t2
  %t4 = call ptr @to_str(i64 %t3)
  %t5 = call ptr @concat(ptr @.str.2, ptr %t4)
  %t6 = alloca ptr
  store ptr %t5, ptr %t6
  %t7 = load ptr, ptr %t6
  %t8 = call i64 @print(ptr %t7)
  %t9 = load i64, ptr %t2
  %t10 = icmp ne i64 %t9, 0
  br i1 %t10, label %L1, label %L2
L1:
  %t11 = call ptr @greet(ptr @.str.3)
  br label %L3
L2:
  br label %L3
L3:
  %t12 = load i64, ptr %t2
  ret i64 %t12
}

