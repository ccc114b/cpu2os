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
@str.0 = private unnamed_addr constant [7 x i8] c"scores\00"
@str.1 = private unnamed_addr constant [42 x i8] c"=== \E5\BB\BA\E7\AB\8B\E8\A4\87\E9\9B\9C\E7\9A\84 JSON \E8\B3\87\E6\96\99\E7\B5\90\E6\A7\8B ===\00"
@str.2 = private unnamed_addr constant [5 x i8] c"name\00"
@str.3 = private unnamed_addr constant [6 x i8] c"Alice\00"
@str.4 = private unnamed_addr constant [4 x i8] c"age\00"
@str.5 = private unnamed_addr constant [7 x i8] c"scores\00"
@str.6 = private unnamed_addr constant [10 x i8] c"is_active\00"
@str.7 = private unnamed_addr constant [17 x i8] c"\E4\BD\BF\E7\94\A8\E8\80\85\E5\A7\93\E5\90\8D:\00"
@str.8 = private unnamed_addr constant [5 x i8] c"name\00"
@str.9 = private unnamed_addr constant [20 x i8] c"\E7\AC\AC\E4\B8\80\E7\AD\86\E6\88\90\E7\B8\BE\E6\98\AF:\00"
@str.10 = private unnamed_addr constant [27 x i8] c"=== \E9\96\8B\E5\A7\8B\E4\BF\AE\E6\94\B9\E8\B3\87\E6\96\99 ===\00"
@str.11 = private unnamed_addr constant [7 x i8] c"scores\00"
@str.12 = private unnamed_addr constant [6 x i8] c"email\00"
@str.13 = private unnamed_addr constant [18 x i8] c"alice@example.com\00"
@str.14 = private unnamed_addr constant [26 x i8] c"\E4\BF\AE\E6\94\B9\E5\BE\8C\E7\9A\84\E6\88\90\E7\B8\BE\E9\99\A3\E5\88\97:\00"
@str.15 = private unnamed_addr constant [7 x i8] c"scores\00"
@str.16 = private unnamed_addr constant [29 x i8] c"\E4\BF\AE\E6\94\B9\E5\BE\8C\E7\9A\84\E4\BD\BF\E7\94\A8\E8\80\85\E7\89\A9\E4\BB\B6:\00"

; === Functions ===

define ptr @get_first_score(ptr %arg_user_obj) {
entry:
  %ptr_t2 = alloca ptr
  %ptr_t4 = alloca ptr
  %ptr_t1 = alloca ptr
  %ptr_user_obj = alloca ptr
  %ptr_t3 = alloca ptr
  store ptr %arg_user_obj, ptr %ptr_user_obj
  %tmp.1 = call ptr @rt_load_str(ptr @str.0)
  store ptr %tmp.1, ptr %ptr_t1
  %tmp.2 = load ptr, ptr %ptr_user_obj
  %tmp.3 = load ptr, ptr %ptr_t1
  %tmp.4 = call ptr @rt_get_item(ptr %tmp.2, ptr %tmp.3)
  store ptr %tmp.4, ptr %ptr_t2
  %tmp.5 = call ptr @rt_imm(i64 0)
  store ptr %tmp.5, ptr %ptr_t3
  %tmp.6 = load ptr, ptr %ptr_t2
  %tmp.7 = load ptr, ptr %ptr_t3
  %tmp.8 = call ptr @rt_get_item(ptr %tmp.6, ptr %tmp.7)
  store ptr %tmp.8, ptr %ptr_t4
  %tmp.9 = load ptr, ptr %ptr_t4
  ret ptr %tmp.9
}

; === Program Init ===

define ptr @__init__() {
entry:
  %ptr_t19 = alloca ptr
  %ptr_t16 = alloca ptr
  %ptr_t13 = alloca ptr
  %ptr_t23 = alloca ptr
  %ptr_t38 = alloca ptr
  %ptr_t33 = alloca ptr
  %ptr_t8 = alloca ptr
  %ptr_t29 = alloca ptr
  %ptr_user = alloca ptr
  %ptr_t32 = alloca ptr
  %ptr_t7 = alloca ptr
  %ptr_t27 = alloca ptr
  %ptr_t28 = alloca ptr
  %ptr_t30 = alloca ptr
  %ptr_t39 = alloca ptr
  %ptr_t21 = alloca ptr
  %ptr_t31 = alloca ptr
  %ptr_t34 = alloca ptr
  %ptr_t35 = alloca ptr
  %ptr_t9 = alloca ptr
  %ptr_t22 = alloca ptr
  %ptr_t6 = alloca ptr
  %ptr_t14 = alloca ptr
  %ptr_t18 = alloca ptr
  %ptr_t26 = alloca ptr
  %ptr_t11 = alloca ptr
  %ptr_t17 = alloca ptr
  %ptr_t24 = alloca ptr
  %ptr_t36 = alloca ptr
  %ptr_t10 = alloca ptr
  %ptr_t15 = alloca ptr
  %ptr_t37 = alloca ptr
  %ptr_t12 = alloca ptr
  %ptr_t20 = alloca ptr
  %ptr_t25 = alloca ptr
  %ptr_t5 = alloca ptr
  %tmp.1 = call ptr @rt_load_str(ptr @str.1)
  store ptr %tmp.1, ptr %ptr_t5
  %tmp.2 = load ptr, ptr %ptr_t5
  call void @rt_print_begin()
  call void @rt_print_arg(ptr %tmp.2)
  %tmp.3 = call ptr @rt_print_end()
  store ptr %tmp.3, ptr %ptr_t6
  %tmp.4 = call ptr @rt_new_dict()
  store ptr %tmp.4, ptr %ptr_t7
  %tmp.5 = call ptr @rt_load_str(ptr @str.2)
  store ptr %tmp.5, ptr %ptr_t8
  %tmp.6 = call ptr @rt_load_str(ptr @str.3)
  store ptr %tmp.6, ptr %ptr_t9
  %tmp.7 = load ptr, ptr %ptr_t7
  %tmp.8 = load ptr, ptr %ptr_t8
  %tmp.9 = load ptr, ptr %ptr_t9
  call void @rt_set_item(ptr %tmp.7, ptr %tmp.8, ptr %tmp.9)
  %tmp.10 = call ptr @rt_load_str(ptr @str.4)
  store ptr %tmp.10, ptr %ptr_t10
  %tmp.11 = call ptr @rt_imm(i64 25)
  store ptr %tmp.11, ptr %ptr_t11
  %tmp.12 = load ptr, ptr %ptr_t7
  %tmp.13 = load ptr, ptr %ptr_t10
  %tmp.14 = load ptr, ptr %ptr_t11
  call void @rt_set_item(ptr %tmp.12, ptr %tmp.13, ptr %tmp.14)
  %tmp.15 = call ptr @rt_load_str(ptr @str.5)
  store ptr %tmp.15, ptr %ptr_t12
  %tmp.16 = call ptr @rt_imm(i64 100)
  store ptr %tmp.16, ptr %ptr_t14
  %tmp.17 = call ptr @rt_new_arr()
  store ptr %tmp.17, ptr %ptr_t13
  %tmp.18 = load ptr, ptr %ptr_t13
  %tmp.19 = load ptr, ptr %ptr_t14
  call void @rt_append_item(ptr %tmp.18, ptr %tmp.19)
  %tmp.20 = call ptr @rt_imm(i64 95)
  store ptr %tmp.20, ptr %ptr_t15
  %tmp.21 = load ptr, ptr %ptr_t13
  %tmp.22 = load ptr, ptr %ptr_t15
  call void @rt_append_item(ptr %tmp.21, ptr %tmp.22)
  %tmp.23 = call ptr @rt_imm(i64 80)
  store ptr %tmp.23, ptr %ptr_t16
  %tmp.24 = load ptr, ptr %ptr_t13
  %tmp.25 = load ptr, ptr %ptr_t16
  call void @rt_append_item(ptr %tmp.24, ptr %tmp.25)
  %tmp.26 = load ptr, ptr %ptr_t7
  %tmp.27 = load ptr, ptr %ptr_t12
  %tmp.28 = load ptr, ptr %ptr_t13
  call void @rt_set_item(ptr %tmp.26, ptr %tmp.27, ptr %tmp.28)
  %tmp.29 = call ptr @rt_load_str(ptr @str.6)
  store ptr %tmp.29, ptr %ptr_t17
  %tmp.30 = call ptr @rt_imm(i64 1)
  store ptr %tmp.30, ptr %ptr_t18
  %tmp.31 = load ptr, ptr %ptr_t7
  %tmp.32 = load ptr, ptr %ptr_t17
  %tmp.33 = load ptr, ptr %ptr_t18
  call void @rt_set_item(ptr %tmp.31, ptr %tmp.32, ptr %tmp.33)
  %tmp.34 = load ptr, ptr %ptr_t7
  store ptr %tmp.34, ptr %ptr_user
  %tmp.35 = call ptr @rt_load_str(ptr @str.7)
  store ptr %tmp.35, ptr %ptr_t19
  %tmp.36 = load ptr, ptr %ptr_t19
  %tmp.37 = call ptr @rt_load_str(ptr @str.8)
  store ptr %tmp.37, ptr %ptr_t20
  %tmp.38 = load ptr, ptr %ptr_user
  %tmp.39 = load ptr, ptr %ptr_t20
  %tmp.40 = call ptr @rt_get_item(ptr %tmp.38, ptr %tmp.39)
  store ptr %tmp.40, ptr %ptr_t21
  %tmp.41 = load ptr, ptr %ptr_t21
  call void @rt_print_begin()
  call void @rt_print_arg(ptr %tmp.36)
  call void @rt_print_arg(ptr %tmp.41)
  %tmp.42 = call ptr @rt_print_end()
  store ptr %tmp.42, ptr %ptr_t22
  %tmp.43 = call ptr @rt_load_str(ptr @str.9)
  store ptr %tmp.43, ptr %ptr_t23
  %tmp.44 = load ptr, ptr %ptr_t23
  %tmp.45 = load ptr, ptr %ptr_user
  %tmp.46 = call ptr @get_first_score(ptr %tmp.45)
  store ptr %tmp.46, ptr %ptr_t24
  %tmp.47 = load ptr, ptr %ptr_t24
  call void @rt_print_begin()
  call void @rt_print_arg(ptr %tmp.44)
  call void @rt_print_arg(ptr %tmp.47)
  %tmp.48 = call ptr @rt_print_end()
  store ptr %tmp.48, ptr %ptr_t25
  %tmp.49 = call ptr @rt_load_str(ptr @str.10)
  store ptr %tmp.49, ptr %ptr_t26
  %tmp.50 = load ptr, ptr %ptr_t26
  call void @rt_print_begin()
  call void @rt_print_arg(ptr %tmp.50)
  %tmp.51 = call ptr @rt_print_end()
  store ptr %tmp.51, ptr %ptr_t27
  %tmp.52 = call ptr @rt_load_str(ptr @str.11)
  store ptr %tmp.52, ptr %ptr_t28
  %tmp.53 = call ptr @rt_imm(i64 2)
  store ptr %tmp.53, ptr %ptr_t29
  %tmp.54 = call ptr @rt_imm(i64 99)
  store ptr %tmp.54, ptr %ptr_t30
  %tmp.55 = load ptr, ptr %ptr_user
  %tmp.56 = load ptr, ptr %ptr_t28
  %tmp.57 = call ptr @rt_get_item(ptr %tmp.55, ptr %tmp.56)
  store ptr %tmp.57, ptr %ptr_t31
  %tmp.58 = load ptr, ptr %ptr_t31
  %tmp.59 = load ptr, ptr %ptr_t29
  %tmp.60 = load ptr, ptr %ptr_t30
  call void @rt_set_item(ptr %tmp.58, ptr %tmp.59, ptr %tmp.60)
  %tmp.61 = call ptr @rt_load_str(ptr @str.12)
  store ptr %tmp.61, ptr %ptr_t32
  %tmp.62 = call ptr @rt_load_str(ptr @str.13)
  store ptr %tmp.62, ptr %ptr_t33
  %tmp.63 = load ptr, ptr %ptr_user
  %tmp.64 = load ptr, ptr %ptr_t32
  %tmp.65 = load ptr, ptr %ptr_t33
  call void @rt_set_item(ptr %tmp.63, ptr %tmp.64, ptr %tmp.65)
  %tmp.66 = call ptr @rt_load_str(ptr @str.14)
  store ptr %tmp.66, ptr %ptr_t34
  %tmp.67 = load ptr, ptr %ptr_t34
  %tmp.68 = call ptr @rt_load_str(ptr @str.15)
  store ptr %tmp.68, ptr %ptr_t35
  %tmp.69 = load ptr, ptr %ptr_user
  %tmp.70 = load ptr, ptr %ptr_t35
  %tmp.71 = call ptr @rt_get_item(ptr %tmp.69, ptr %tmp.70)
  store ptr %tmp.71, ptr %ptr_t36
  %tmp.72 = load ptr, ptr %ptr_t36
  call void @rt_print_begin()
  call void @rt_print_arg(ptr %tmp.67)
  call void @rt_print_arg(ptr %tmp.72)
  %tmp.73 = call ptr @rt_print_end()
  store ptr %tmp.73, ptr %ptr_t37
  %tmp.74 = call ptr @rt_load_str(ptr @str.16)
  store ptr %tmp.74, ptr %ptr_t38
  %tmp.75 = load ptr, ptr %ptr_t38
  %tmp.76 = load ptr, ptr %ptr_user
  call void @rt_print_begin()
  call void @rt_print_arg(ptr %tmp.75)
  call void @rt_print_arg(ptr %tmp.76)
  %tmp.77 = call ptr @rt_print_end()
  store ptr %tmp.77, ptr %ptr_t39
  ret ptr null
}

; === Program Entry ===

define i32 @main() {
entry:
  %tmp.entry = call ptr @__init__()
  ret i32 0
}
