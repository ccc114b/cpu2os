; === Runtime API Declarations ===
declare ptr @rt_imm(i64)
declare ptr @rt_load_str(ptr)
declare ptr @rt_add(ptr, ptr)
declare ptr @rt_sub(ptr, ptr)
declare ptr @rt_mul(ptr, ptr)
declare ptr @rt_div(ptr, ptr)
declare ptr @rt_cmp_eq(ptr, ptr)
declare ptr @rt_cmp_lt(ptr, ptr)
declare ptr @rt_cmp_gt(ptr, ptr)
declare ptr @rt_new_arr()
declare ptr @rt_new_dict()
declare ptr @rt_init_arr(ptr, ptr)
declare void @rt_append_item(ptr, ptr)
declare void @rt_set_item(ptr, ptr, ptr)
declare ptr @rt_get_item(ptr, ptr)
declare i1 @rt_is_truthy(ptr)
declare void @rt_print_begin()
declare void @rt_print_arg(ptr)
declare ptr @rt_print_end()
declare ptr @p0_len(ptr)
declare ptr @p0_time()
declare ptr @p0_array(ptr, ptr)
declare ptr @p0_push(ptr, ptr)
declare ptr @p0_pop(ptr)
declare ptr @p0_keys(ptr)
declare ptr @p0_has_key(ptr, ptr)
declare ptr @p0_remove(ptr, ptr)
declare ptr @p0_typeof(ptr)
declare ptr @p0_int(ptr)
declare ptr @p0_str(ptr)
declare ptr @p0_ord(ptr)
declare ptr @p0_chr(ptr)
declare ptr @p0_random()
declare ptr @p0_input(ptr)
declare ptr @p0_exit(ptr)

; === External System Calls ===

; === String Pool ===
@str.0 = private unnamed_addr constant [41 x i8] c"=== 1. \E6\B8\AC\E8\A9\A6 For \E8\BF\B4\E5\9C\88\E8\88\87 Continue ===\00"
@str.1 = private unnamed_addr constant [20 x i8] c"\E9\81\8E\E6\BF\BE\E5\BE\8C\E7\9A\84\E9\99\A3\E5\88\97:\00"
@str.2 = private unnamed_addr constant [40 x i8] c"=== 2. \E6\B8\AC\E8\A9\A6 While \E8\BF\B4\E5\9C\88\E8\88\87 Break ===\00"
@str.3 = private unnamed_addr constant [16 x i8] c"While \E5\9F\B7\E8\A1\8C\E7\AC\AC\00"
@str.4 = private unnamed_addr constant [4 x i8] c"\E6\AC\A1\00"
@str.5 = private unnamed_addr constant [19 x i8] c"\E8\B7\B3\E5\87\BA\E8\BF\B4\E5\9C\88\E4\BA\86\EF\BC\81\00"

; === Functions ===

; === Program Init ===

define ptr @__init__() {
entry:
  %ptr_t4 = alloca ptr
  %ptr_t31 = alloca ptr
  %ptr_count = alloca ptr
  %ptr_t28 = alloca ptr
  %ptr_t29 = alloca ptr
  %ptr_t18 = alloca ptr
  %ptr_i = alloca ptr
  %ptr_t22 = alloca ptr
  %ptr_t10 = alloca ptr
  %ptr_t21 = alloca ptr
  %ptr_t26 = alloca ptr
  %ptr_t17 = alloca ptr
  %ptr_t3 = alloca ptr
  %ptr_t19 = alloca ptr
  %ptr_t24 = alloca ptr
  %ptr_t23 = alloca ptr
  %ptr_t20 = alloca ptr
  %ptr_t2 = alloca ptr
  %ptr_t11 = alloca ptr
  %ptr_arr = alloca ptr
  %ptr_t5 = alloca ptr
  %ptr_t14 = alloca ptr
  %ptr_t7 = alloca ptr
  %ptr_t12 = alloca ptr
  %ptr_t32 = alloca ptr
  %ptr_t6 = alloca ptr
  %ptr_t1 = alloca ptr
  %ptr_t27 = alloca ptr
  %ptr_t8 = alloca ptr
  %ptr_t25 = alloca ptr
  %ptr_t15 = alloca ptr
  %ptr_t30 = alloca ptr
  %ptr_t16 = alloca ptr
  %ptr_t13 = alloca ptr
  %ptr_t9 = alloca ptr
  %tmp.1 = call ptr @rt_load_str(ptr @str.0)
  store ptr %tmp.1, ptr %ptr_t1
  %tmp.2 = load ptr, ptr %ptr_t1
  call void @rt_print_begin()
  call void @rt_print_arg(ptr %tmp.2)
  %tmp.3 = call ptr @rt_print_end()
  store ptr %tmp.3, ptr %ptr_t2
  %tmp.4 = call ptr @rt_imm(i64 0)
  store ptr %tmp.4, ptr %ptr_t4
  %tmp.5 = call ptr @rt_new_arr()
  store ptr %tmp.5, ptr %ptr_t3
  %tmp.6 = load ptr, ptr %ptr_t3
  %tmp.7 = load ptr, ptr %ptr_t4
  call void @rt_append_item(ptr %tmp.6, ptr %tmp.7)
  %tmp.8 = call ptr @rt_imm(i64 0)
  store ptr %tmp.8, ptr %ptr_t5
  %tmp.9 = load ptr, ptr %ptr_t3
  %tmp.10 = load ptr, ptr %ptr_t5
  call void @rt_append_item(ptr %tmp.9, ptr %tmp.10)
  %tmp.11 = call ptr @rt_imm(i64 0)
  store ptr %tmp.11, ptr %ptr_t6
  %tmp.12 = load ptr, ptr %ptr_t3
  %tmp.13 = load ptr, ptr %ptr_t6
  call void @rt_append_item(ptr %tmp.12, ptr %tmp.13)
  %tmp.14 = call ptr @rt_imm(i64 0)
  store ptr %tmp.14, ptr %ptr_t7
  %tmp.15 = load ptr, ptr %ptr_t3
  %tmp.16 = load ptr, ptr %ptr_t7
  call void @rt_append_item(ptr %tmp.15, ptr %tmp.16)
  %tmp.17 = call ptr @rt_imm(i64 0)
  store ptr %tmp.17, ptr %ptr_t8
  %tmp.18 = load ptr, ptr %ptr_t3
  %tmp.19 = load ptr, ptr %ptr_t8
  call void @rt_append_item(ptr %tmp.18, ptr %tmp.19)
  %tmp.20 = load ptr, ptr %ptr_t3
  store ptr %tmp.20, ptr %ptr_arr
  %tmp.21 = call ptr @rt_imm(i64 0)
  store ptr %tmp.21, ptr %ptr_t9
  %tmp.22 = load ptr, ptr %ptr_t9
  store ptr %tmp.22, ptr %ptr_i
  br label %L1
L1:
  %tmp.23 = call ptr @rt_imm(i64 5)
  store ptr %tmp.23, ptr %ptr_t10
  %tmp.24 = load ptr, ptr %ptr_i
  %tmp.25 = load ptr, ptr %ptr_t10
  %tmp.26 = call ptr @rt_cmp_lt(ptr %tmp.24, ptr %tmp.25)
  store ptr %tmp.26, ptr %ptr_t11
  %tmp.27 = load ptr, ptr %ptr_t11
  %tmp.28 = call i1 @rt_is_truthy(ptr %tmp.27)
  br i1 %tmp.28, label %fallthrough.1, label %L4
fallthrough.1:
  br label %L2
L3:
  %tmp.29 = call ptr @rt_imm(i64 1)
  store ptr %tmp.29, ptr %ptr_t12
  %tmp.30 = load ptr, ptr %ptr_i
  %tmp.31 = load ptr, ptr %ptr_t12
  %tmp.32 = call ptr @rt_add(ptr %tmp.30, ptr %tmp.31)
  store ptr %tmp.32, ptr %ptr_t13
  %tmp.33 = load ptr, ptr %ptr_t13
  store ptr %tmp.33, ptr %ptr_i
  br label %L1
L2:
  %tmp.34 = call ptr @rt_imm(i64 2)
  store ptr %tmp.34, ptr %ptr_t14
  %tmp.35 = load ptr, ptr %ptr_i
  %tmp.36 = load ptr, ptr %ptr_t14
  %tmp.37 = call ptr @rt_cmp_eq(ptr %tmp.35, ptr %tmp.36)
  store ptr %tmp.37, ptr %ptr_t15
  %tmp.38 = load ptr, ptr %ptr_t15
  %tmp.39 = call i1 @rt_is_truthy(ptr %tmp.38)
  br i1 %tmp.39, label %fallthrough.2, label %L5
fallthrough.2:
  br label %L3
L5:
  %tmp.40 = call ptr @rt_imm(i64 10)
  store ptr %tmp.40, ptr %ptr_t16
  %tmp.41 = load ptr, ptr %ptr_i
  %tmp.42 = load ptr, ptr %ptr_t16
  %tmp.43 = call ptr @rt_mul(ptr %tmp.41, ptr %tmp.42)
  store ptr %tmp.43, ptr %ptr_t17
  %tmp.44 = load ptr, ptr %ptr_arr
  %tmp.45 = load ptr, ptr %ptr_i
  %tmp.46 = load ptr, ptr %ptr_t17
  call void @rt_set_item(ptr %tmp.44, ptr %tmp.45, ptr %tmp.46)
  br label %L3
L4:
  %tmp.47 = call ptr @rt_load_str(ptr @str.1)
  store ptr %tmp.47, ptr %ptr_t18
  %tmp.48 = load ptr, ptr %ptr_t18
  %tmp.49 = load ptr, ptr %ptr_arr
  call void @rt_print_begin()
  call void @rt_print_arg(ptr %tmp.48)
  call void @rt_print_arg(ptr %tmp.49)
  %tmp.50 = call ptr @rt_print_end()
  store ptr %tmp.50, ptr %ptr_t19
  %tmp.51 = call ptr @rt_load_str(ptr @str.2)
  store ptr %tmp.51, ptr %ptr_t20
  %tmp.52 = load ptr, ptr %ptr_t20
  call void @rt_print_begin()
  call void @rt_print_arg(ptr %tmp.52)
  %tmp.53 = call ptr @rt_print_end()
  store ptr %tmp.53, ptr %ptr_t21
  %tmp.54 = call ptr @rt_imm(i64 0)
  store ptr %tmp.54, ptr %ptr_t22
  %tmp.55 = load ptr, ptr %ptr_t22
  store ptr %tmp.55, ptr %ptr_count
  br label %L6
L6:
  %tmp.56 = call ptr @rt_imm(i64 1)
  store ptr %tmp.56, ptr %ptr_t23
  %tmp.57 = load ptr, ptr %ptr_t23
  %tmp.58 = call i1 @rt_is_truthy(ptr %tmp.57)
  br i1 %tmp.58, label %fallthrough.3, label %L7
fallthrough.3:
  %tmp.59 = call ptr @rt_imm(i64 1)
  store ptr %tmp.59, ptr %ptr_t24
  %tmp.60 = load ptr, ptr %ptr_count
  %tmp.61 = load ptr, ptr %ptr_t24
  %tmp.62 = call ptr @rt_add(ptr %tmp.60, ptr %tmp.61)
  store ptr %tmp.62, ptr %ptr_t25
  %tmp.63 = load ptr, ptr %ptr_t25
  store ptr %tmp.63, ptr %ptr_count
  %tmp.64 = call ptr @rt_imm(i64 3)
  store ptr %tmp.64, ptr %ptr_t26
  %tmp.65 = load ptr, ptr %ptr_count
  %tmp.66 = load ptr, ptr %ptr_t26
  %tmp.67 = call ptr @rt_cmp_gt(ptr %tmp.65, ptr %tmp.66)
  store ptr %tmp.67, ptr %ptr_t27
  %tmp.68 = load ptr, ptr %ptr_t27
  %tmp.69 = call i1 @rt_is_truthy(ptr %tmp.68)
  br i1 %tmp.69, label %fallthrough.4, label %L8
fallthrough.4:
  br label %L7
L8:
  %tmp.70 = call ptr @rt_load_str(ptr @str.3)
  store ptr %tmp.70, ptr %ptr_t28
  %tmp.71 = load ptr, ptr %ptr_t28
  %tmp.72 = load ptr, ptr %ptr_count
  %tmp.73 = call ptr @rt_load_str(ptr @str.4)
  store ptr %tmp.73, ptr %ptr_t29
  %tmp.74 = load ptr, ptr %ptr_t29
  call void @rt_print_begin()
  call void @rt_print_arg(ptr %tmp.71)
  call void @rt_print_arg(ptr %tmp.72)
  call void @rt_print_arg(ptr %tmp.74)
  %tmp.75 = call ptr @rt_print_end()
  store ptr %tmp.75, ptr %ptr_t30
  br label %L6
L7:
  %tmp.76 = call ptr @rt_load_str(ptr @str.5)
  store ptr %tmp.76, ptr %ptr_t31
  %tmp.77 = load ptr, ptr %ptr_t31
  call void @rt_print_begin()
  call void @rt_print_arg(ptr %tmp.77)
  %tmp.78 = call ptr @rt_print_end()
  store ptr %tmp.78, ptr %ptr_t32
  ret ptr null
}

; === Program Entry ===

define i32 @main() {
entry:
  %tmp.entry = call ptr @__init__()
  ret i32 0
}
