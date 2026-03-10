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
@str.0 = private unnamed_addr constant [10 x i8] c"fact(5) =\00"
@str.1 = private unnamed_addr constant [10 x i8] c"10 + 20 =\00"
@str.2 = private unnamed_addr constant [15 x i8] c", \E5\BE\88\E7\B0\A1\E5\96\AE\EF\BC\81\00"

; === Functions ===

define ptr @fact(ptr %arg_n) {
entry:
  %ptr_t6 = alloca ptr
  %ptr_t1 = alloca ptr
  %ptr_t5 = alloca ptr
  %ptr_n = alloca ptr
  %ptr_t2 = alloca ptr
  %ptr_t3 = alloca ptr
  %ptr_t7 = alloca ptr
  %ptr_t4 = alloca ptr
  store ptr %arg_n, ptr %ptr_n
  %tmp.1 = call ptr @rt_imm(i64 2)
  store ptr %tmp.1, ptr %ptr_t1
  %tmp.2 = load ptr, ptr %ptr_n
  %tmp.3 = load ptr, ptr %ptr_t1
  %tmp.4 = call ptr @rt_cmp_lt(ptr %tmp.2, ptr %tmp.3)
  store ptr %tmp.4, ptr %ptr_t2
  %tmp.5 = load ptr, ptr %ptr_t2
  %tmp.6 = call i1 @rt_is_truthy(ptr %tmp.5)
  br i1 %tmp.6, label %fallthrough.1, label %L1
fallthrough.1:
  %tmp.7 = call ptr @rt_imm(i64 1)
  store ptr %tmp.7, ptr %ptr_t3
  %tmp.8 = load ptr, ptr %ptr_t3
  ret ptr %tmp.8
L1:
  %tmp.9 = call ptr @rt_imm(i64 1)
  store ptr %tmp.9, ptr %ptr_t4
  %tmp.10 = load ptr, ptr %ptr_n
  %tmp.11 = load ptr, ptr %ptr_t4
  %tmp.12 = call ptr @rt_sub(ptr %tmp.10, ptr %tmp.11)
  store ptr %tmp.12, ptr %ptr_t5
  %tmp.13 = load ptr, ptr %ptr_t5
  %tmp.14 = call ptr @fact(ptr %tmp.13)
  store ptr %tmp.14, ptr %ptr_t6
  %tmp.15 = load ptr, ptr %ptr_n
  %tmp.16 = load ptr, ptr %ptr_t6
  %tmp.17 = call ptr @rt_mul(ptr %tmp.15, ptr %tmp.16)
  store ptr %tmp.17, ptr %ptr_t7
  %tmp.18 = load ptr, ptr %ptr_t7
  ret ptr %tmp.18
}

; === Program Init ===

define ptr @__init__() {
entry:
  %ptr_t17 = alloca ptr
  %ptr_t8 = alloca ptr
  %ptr_t11 = alloca ptr
  %ptr_t15 = alloca ptr
  %ptr_t9 = alloca ptr
  %ptr_t10 = alloca ptr
  %ptr_t12 = alloca ptr
  %ptr_t14 = alloca ptr
  %ptr_t13 = alloca ptr
  %ptr_t16 = alloca ptr
  %tmp.1 = call ptr @rt_load_str(ptr @str.0)
  store ptr %tmp.1, ptr %ptr_t8
  %tmp.2 = load ptr, ptr %ptr_t8
  %tmp.3 = call ptr @rt_imm(i64 5)
  store ptr %tmp.3, ptr %ptr_t9
  %tmp.4 = load ptr, ptr %ptr_t9
  %tmp.5 = call ptr @fact(ptr %tmp.4)
  store ptr %tmp.5, ptr %ptr_t10
  %tmp.6 = load ptr, ptr %ptr_t10
  call void @rt_print_begin()
  call void @rt_print_arg(ptr %tmp.2)
  call void @rt_print_arg(ptr %tmp.6)
  %tmp.7 = call ptr @rt_print_end()
  store ptr %tmp.7, ptr %ptr_t11
  %tmp.8 = call ptr @rt_load_str(ptr @str.1)
  store ptr %tmp.8, ptr %ptr_t12
  %tmp.9 = load ptr, ptr %ptr_t12
  %tmp.10 = call ptr @rt_imm(i64 10)
  store ptr %tmp.10, ptr %ptr_t13
  %tmp.11 = call ptr @rt_imm(i64 20)
  store ptr %tmp.11, ptr %ptr_t14
  %tmp.12 = load ptr, ptr %ptr_t13
  %tmp.13 = load ptr, ptr %ptr_t14
  %tmp.14 = call ptr @rt_add(ptr %tmp.12, ptr %tmp.13)
  store ptr %tmp.14, ptr %ptr_t15
  %tmp.15 = load ptr, ptr %ptr_t15
  %tmp.16 = call ptr @rt_load_str(ptr @str.2)
  store ptr %tmp.16, ptr %ptr_t16
  %tmp.17 = load ptr, ptr %ptr_t16
  call void @rt_print_begin()
  call void @rt_print_arg(ptr %tmp.9)
  call void @rt_print_arg(ptr %tmp.15)
  call void @rt_print_arg(ptr %tmp.17)
  %tmp.18 = call ptr @rt_print_end()
  store ptr %tmp.18, ptr %ptr_t17
  ret ptr null
}

; === Program Entry ===

define i32 @main() {
entry:
  %tmp.entry = call ptr @__init__()
  ret i32 0
}
