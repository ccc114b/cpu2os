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
@str.0 = private unnamed_addr constant [54 x i8] c"--- 1. \E6\B8\AC\E8\A9\A6\E9\99\A3\E5\88\97\E7\9B\B8\E9\97\9C (array, len, push, pop) ---\00"
@str.1 = private unnamed_addr constant [23 x i8] c"\E5\88\9D\E5\A7\8B\E5\8C\96 array(3, 0):\00"
@str.2 = private unnamed_addr constant [23 x i8] c"\E9\99\A3\E5\88\97\E9\95\B7\E5\BA\A6 len(arr):\00"
@str.3 = private unnamed_addr constant [16 x i8] c"push \E5\85\A9\E6\AC\A1\E5\BE\8C:\00"
@str.4 = private unnamed_addr constant [17 x i8] c"\E6\96\B0\E9\99\A3\E5\88\97\E9\95\B7\E5\BA\A6:\00"
@str.5 = private unnamed_addr constant [20 x i8] c"pop() \E5\BD\88\E5\87\BA\E7\9A\84\E5\80\BC:\00"
@str.6 = private unnamed_addr constant [18 x i8] c"pop \E5\BE\8C\E7\9A\84\E9\99\A3\E5\88\97:\00"
@str.7 = private unnamed_addr constant [1 x i8] c"\00"
@str.8 = private unnamed_addr constant [54 x i8] c"--- 2. \E6\B8\AC\E8\A9\A6\E5\AD\97\E5\85\B8\E7\9B\B8\E9\97\9C (keys, has_key, remove) ---\00"
@str.9 = private unnamed_addr constant [5 x i8] c"name\00"
@str.10 = private unnamed_addr constant [6 x i8] c"Alice\00"
@str.11 = private unnamed_addr constant [4 x i8] c"age\00"
@str.12 = private unnamed_addr constant [5 x i8] c"city\00"
@str.13 = private unnamed_addr constant [7 x i8] c"Taipei\00"
@str.14 = private unnamed_addr constant [14 x i8] c"\E5\88\9D\E5\A7\8B\E5\AD\97\E5\85\B8:\00"
@str.15 = private unnamed_addr constant [21 x i8] c"\E5\AD\97\E5\85\B8\E9\95\B7\E5\BA\A6 len(d):\00"
@str.16 = private unnamed_addr constant [22 x i8] c"\E6\89\80\E6\9C\89\E7\9A\84\E9\8D\B5 keys(d):\00"
@str.17 = private unnamed_addr constant [5 x i8] c"name\00"
@str.18 = private unnamed_addr constant [4 x i8] c"job\00"
@str.19 = private unnamed_addr constant [21 x i8] c"\E6\98\AF\E5\90\A6\E5\8C\85\E5\90\AB 'name'?\00"
@str.20 = private unnamed_addr constant [20 x i8] c"\E6\98\AF\E5\90\A6\E5\8C\85\E5\90\AB 'job'?\00"
@str.21 = private unnamed_addr constant [4 x i8] c"age\00"
@str.22 = private unnamed_addr constant [34 x i8] c"remove(d, 'age') \E4\B9\8B\E5\BE\8C\E7\9A\84\E5\AD\97\E5\85\B8:\00"
@str.23 = private unnamed_addr constant [1 x i8] c"\00"
@str.24 = private unnamed_addr constant [52 x i8] c"--- 3. \E6\B8\AC\E8\A9\A6\E5\9E\8B\E5\88\A5\E8\88\87\E8\BD\89\E6\8F\9B (typeof, int, str) ---\00"
@str.25 = private unnamed_addr constant [4 x i8] c"100\00"
@str.26 = private unnamed_addr constant [2 x i8] c"k\00"
@str.27 = private unnamed_addr constant [14 x i8] c"typeof(42)  :\00"
@str.28 = private unnamed_addr constant [15 x i8] c"typeof('100'):\00"
@str.29 = private unnamed_addr constant [14 x i8] c"typeof([1]) :\00"
@str.30 = private unnamed_addr constant [14 x i8] c"typeof({k}) :\00"
@str.31 = private unnamed_addr constant [18 x i8] c"int('100') + 50 =\00"
@str.32 = private unnamed_addr constant [17 x i8] c" \E6\98\AF\E4\B8\80\E5\80\8B\E5\AD\97\E4\B8\B2\00"
@str.33 = private unnamed_addr constant [22 x i8] c"str(42) \E4\B8\B2\E6\8E\A5\E7\B5\90\E6\9E\9C:\00"
@str.34 = private unnamed_addr constant [1 x i8] c"\00"
@str.35 = private unnamed_addr constant [51 x i8] c"--- 4. \E6\B8\AC\E8\A9\A6\E5\AD\97\E5\85\83\E8\88\87 ASCII \E8\BD\89\E6\8F\9B (ord, chr) ---\00"
@str.36 = private unnamed_addr constant [2 x i8] c"A\00"
@str.37 = private unnamed_addr constant [25 x i8] c"ord('A') \E7\9A\84 ASCII \E7\A2\BC =\00"
@str.38 = private unnamed_addr constant [26 x i8] c"chr(66) \E9\82\84\E5\8E\9F\E7\9A\84\E5\AD\97\E5\85\83 =\00"
@str.39 = private unnamed_addr constant [1 x i8] c"\00"
@str.40 = private unnamed_addr constant [45 x i8] c"--- 5. \E6\B8\AC\E8\A9\A6\E7\B3\BB\E7\B5\B1\E7\8B\80\E6\85\8B (time, random) ---\00"
@str.41 = private unnamed_addr constant [24 x i8] c"\E7\95\B6\E5\89\8D\E6\99\82\E9\96\93\E6\88\B3 time():\00"
@str.42 = private unnamed_addr constant [26 x i8] c"\E7\94\A2\E7\94\9F\E7\9A\84\E4\BA\82\E6\95\B8 random():\00"
@str.43 = private unnamed_addr constant [1 x i8] c"\00"
@str.44 = private unnamed_addr constant [39 x i8] c">>> \E9\96\8B\E5\A7\8B\E5\9F\B7\E8\A1\8C\E7\B3\BB\E7\B5\B1\E5\87\BD\E6\95\B8\E6\B8\AC\E8\A9\A6 <<<\00"
@str.45 = private unnamed_addr constant [1 x i8] c"\00"
@str.46 = private unnamed_addr constant [52 x i8] c"--- 6. \E6\B8\AC\E8\A9\A6 I/O \E8\88\87\E5\BC\B7\E5\88\B6\E7\B5\82\E6\AD\A2 (input, exit) ---\00"
@str.47 = private unnamed_addr constant [52 x i8] c"\E8\AB\8B\E8\BC\B8\E5\85\A5\E4\BB\BB\E6\84\8F\E6\96\87\E5\AD\97 (\E6\88\96\E7\9B\B4\E6\8E\A5\E6\8C\89 Enter \E7\B9\BC\E7\BA\8C): \00"
@str.48 = private unnamed_addr constant [23 x i8] c"\E4\BD\A0\E5\89\9B\E6\89\8D\E8\BC\B8\E5\85\A5\E7\9A\84\E6\98\AF:\00"
@str.49 = private unnamed_addr constant [40 x i8] c"\E6\BA\96\E5\82\99\E5\91\BC\E5\8F\AB exit(0) \E7\B5\90\E6\9D\9F\E8\99\9B\E6\93\AC\E6\A9\9F...\00"
@str.50 = private unnamed_addr constant [65 x i8] c"\E9\80\99\E8\A1\8C\E6\B0\B8\E9\81\A0\E4\B8\8D\E6\87\89\E8\A9\B2\E8\A2\AB\E5\8D\B0\E5\87\BA\E4\BE\86\EF\BC\81\E5\9B\A0\E7\82\BA VM \E5\B7\B2\E7\B6\93\E7\B5\82\E6\AD\A2\E4\BA\86\EF\BC\81\00"

; === Functions ===

define ptr @test_array() {
entry:
  %ptr_t2 = alloca ptr
  %ptr_t6 = alloca ptr
  %ptr_t7 = alloca ptr
  %ptr_t18 = alloca ptr
  %ptr_t8 = alloca ptr
  %ptr_t12 = alloca ptr
  %ptr_t22 = alloca ptr
  %ptr_t26 = alloca ptr
  %ptr_t3 = alloca ptr
  %ptr_t19 = alloca ptr
  %ptr_t21 = alloca ptr
  %ptr_t25 = alloca ptr
  %ptr_t23 = alloca ptr
  %ptr_t14 = alloca ptr
  %ptr_t16 = alloca ptr
  %ptr_t15 = alloca ptr
  %ptr_t1 = alloca ptr
  %ptr_t4 = alloca ptr
  %ptr_t11 = alloca ptr
  %ptr_t10 = alloca ptr
  %ptr_t13 = alloca ptr
  %ptr_t9 = alloca ptr
  %ptr_t17 = alloca ptr
  %ptr_t5 = alloca ptr
  %ptr_t20 = alloca ptr
  %ptr_arr = alloca ptr
  %ptr_last_val = alloca ptr
  %ptr_t24 = alloca ptr
  %tmp.1 = call ptr @rt_load_str(ptr @str.0)
  store ptr %tmp.1, ptr %ptr_t1
  %tmp.2 = load ptr, ptr %ptr_t1
  call void @rt_print_begin()
  call void @rt_print_arg(ptr %tmp.2)
  %tmp.3 = call ptr @rt_print_end()
  store ptr %tmp.3, ptr %ptr_t2
  %tmp.4 = call ptr @rt_imm(i64 3)
  store ptr %tmp.4, ptr %ptr_t3
  %tmp.5 = load ptr, ptr %ptr_t3
  %tmp.6 = call ptr @rt_imm(i64 0)
  store ptr %tmp.6, ptr %ptr_t4
  %tmp.7 = load ptr, ptr %ptr_t4
  %tmp.8 = call ptr @p0_array(ptr %tmp.5, ptr %tmp.7)
  store ptr %tmp.8, ptr %ptr_t5
  %tmp.9 = load ptr, ptr %ptr_t5
  store ptr %tmp.9, ptr %ptr_arr
  %tmp.10 = call ptr @rt_load_str(ptr @str.1)
  store ptr %tmp.10, ptr %ptr_t6
  %tmp.11 = load ptr, ptr %ptr_t6
  %tmp.12 = load ptr, ptr %ptr_arr
  call void @rt_print_begin()
  call void @rt_print_arg(ptr %tmp.11)
  call void @rt_print_arg(ptr %tmp.12)
  %tmp.13 = call ptr @rt_print_end()
  store ptr %tmp.13, ptr %ptr_t7
  %tmp.14 = call ptr @rt_load_str(ptr @str.2)
  store ptr %tmp.14, ptr %ptr_t8
  %tmp.15 = load ptr, ptr %ptr_t8
  %tmp.16 = load ptr, ptr %ptr_arr
  %tmp.17 = call ptr @p0_len(ptr %tmp.16)
  store ptr %tmp.17, ptr %ptr_t9
  %tmp.18 = load ptr, ptr %ptr_t9
  call void @rt_print_begin()
  call void @rt_print_arg(ptr %tmp.15)
  call void @rt_print_arg(ptr %tmp.18)
  %tmp.19 = call ptr @rt_print_end()
  store ptr %tmp.19, ptr %ptr_t10
  %tmp.20 = load ptr, ptr %ptr_arr
  %tmp.21 = call ptr @rt_imm(i64 99)
  store ptr %tmp.21, ptr %ptr_t11
  %tmp.22 = load ptr, ptr %ptr_t11
  %tmp.23 = call ptr @p0_push(ptr %tmp.20, ptr %tmp.22)
  store ptr %tmp.23, ptr %ptr_t12
  %tmp.24 = load ptr, ptr %ptr_arr
  %tmp.25 = call ptr @rt_imm(i64 100)
  store ptr %tmp.25, ptr %ptr_t13
  %tmp.26 = load ptr, ptr %ptr_t13
  %tmp.27 = call ptr @p0_push(ptr %tmp.24, ptr %tmp.26)
  store ptr %tmp.27, ptr %ptr_t14
  %tmp.28 = call ptr @rt_load_str(ptr @str.3)
  store ptr %tmp.28, ptr %ptr_t15
  %tmp.29 = load ptr, ptr %ptr_t15
  %tmp.30 = load ptr, ptr %ptr_arr
  call void @rt_print_begin()
  call void @rt_print_arg(ptr %tmp.29)
  call void @rt_print_arg(ptr %tmp.30)
  %tmp.31 = call ptr @rt_print_end()
  store ptr %tmp.31, ptr %ptr_t16
  %tmp.32 = call ptr @rt_load_str(ptr @str.4)
  store ptr %tmp.32, ptr %ptr_t17
  %tmp.33 = load ptr, ptr %ptr_t17
  %tmp.34 = load ptr, ptr %ptr_arr
  %tmp.35 = call ptr @p0_len(ptr %tmp.34)
  store ptr %tmp.35, ptr %ptr_t18
  %tmp.36 = load ptr, ptr %ptr_t18
  call void @rt_print_begin()
  call void @rt_print_arg(ptr %tmp.33)
  call void @rt_print_arg(ptr %tmp.36)
  %tmp.37 = call ptr @rt_print_end()
  store ptr %tmp.37, ptr %ptr_t19
  %tmp.38 = load ptr, ptr %ptr_arr
  %tmp.39 = call ptr @p0_pop(ptr %tmp.38)
  store ptr %tmp.39, ptr %ptr_t20
  %tmp.40 = load ptr, ptr %ptr_t20
  store ptr %tmp.40, ptr %ptr_last_val
  %tmp.41 = call ptr @rt_load_str(ptr @str.5)
  store ptr %tmp.41, ptr %ptr_t21
  %tmp.42 = load ptr, ptr %ptr_t21
  %tmp.43 = load ptr, ptr %ptr_last_val
  call void @rt_print_begin()
  call void @rt_print_arg(ptr %tmp.42)
  call void @rt_print_arg(ptr %tmp.43)
  %tmp.44 = call ptr @rt_print_end()
  store ptr %tmp.44, ptr %ptr_t22
  %tmp.45 = call ptr @rt_load_str(ptr @str.6)
  store ptr %tmp.45, ptr %ptr_t23
  %tmp.46 = load ptr, ptr %ptr_t23
  %tmp.47 = load ptr, ptr %ptr_arr
  call void @rt_print_begin()
  call void @rt_print_arg(ptr %tmp.46)
  call void @rt_print_arg(ptr %tmp.47)
  %tmp.48 = call ptr @rt_print_end()
  store ptr %tmp.48, ptr %ptr_t24
  %tmp.49 = call ptr @rt_load_str(ptr @str.7)
  store ptr %tmp.49, ptr %ptr_t25
  %tmp.50 = load ptr, ptr %ptr_t25
  call void @rt_print_begin()
  call void @rt_print_arg(ptr %tmp.50)
  %tmp.51 = call ptr @rt_print_end()
  store ptr %tmp.51, ptr %ptr_t26
  ret ptr null
}

define ptr @test_dict() {
entry:
  %ptr_has_job = alloca ptr
  %ptr_t42 = alloca ptr
  %ptr_t38 = alloca ptr
  %ptr_t53 = alloca ptr
  %ptr_t50 = alloca ptr
  %ptr_t36 = alloca ptr
  %ptr_t37 = alloca ptr
  %ptr_d = alloca ptr
  %ptr_t31 = alloca ptr
  %ptr_t32 = alloca ptr
  %ptr_t33 = alloca ptr
  %ptr_t27 = alloca ptr
  %ptr_t39 = alloca ptr
  %ptr_k_list = alloca ptr
  %ptr_t47 = alloca ptr
  %ptr_t34 = alloca ptr
  %ptr_t51 = alloca ptr
  %ptr_t55 = alloca ptr
  %ptr_t45 = alloca ptr
  %ptr_t46 = alloca ptr
  %ptr_t41 = alloca ptr
  %ptr_has_name = alloca ptr
  %ptr_t28 = alloca ptr
  %ptr_t43 = alloca ptr
  %ptr_t48 = alloca ptr
  %ptr_t56 = alloca ptr
  %ptr_t30 = alloca ptr
  %ptr_t29 = alloca ptr
  %ptr_t35 = alloca ptr
  %ptr_t49 = alloca ptr
  %ptr_t54 = alloca ptr
  %ptr_t40 = alloca ptr
  %ptr_t52 = alloca ptr
  %ptr_t57 = alloca ptr
  %ptr_t44 = alloca ptr
  %tmp.1 = call ptr @rt_load_str(ptr @str.8)
  store ptr %tmp.1, ptr %ptr_t27
  %tmp.2 = load ptr, ptr %ptr_t27
  call void @rt_print_begin()
  call void @rt_print_arg(ptr %tmp.2)
  %tmp.3 = call ptr @rt_print_end()
  store ptr %tmp.3, ptr %ptr_t28
  %tmp.4 = call ptr @rt_new_dict()
  store ptr %tmp.4, ptr %ptr_t29
  %tmp.5 = call ptr @rt_load_str(ptr @str.9)
  store ptr %tmp.5, ptr %ptr_t30
  %tmp.6 = call ptr @rt_load_str(ptr @str.10)
  store ptr %tmp.6, ptr %ptr_t31
  %tmp.7 = load ptr, ptr %ptr_t29
  %tmp.8 = load ptr, ptr %ptr_t30
  %tmp.9 = load ptr, ptr %ptr_t31
  call void @rt_set_item(ptr %tmp.7, ptr %tmp.8, ptr %tmp.9)
  %tmp.10 = call ptr @rt_load_str(ptr @str.11)
  store ptr %tmp.10, ptr %ptr_t32
  %tmp.11 = call ptr @rt_imm(i64 25)
  store ptr %tmp.11, ptr %ptr_t33
  %tmp.12 = load ptr, ptr %ptr_t29
  %tmp.13 = load ptr, ptr %ptr_t32
  %tmp.14 = load ptr, ptr %ptr_t33
  call void @rt_set_item(ptr %tmp.12, ptr %tmp.13, ptr %tmp.14)
  %tmp.15 = call ptr @rt_load_str(ptr @str.12)
  store ptr %tmp.15, ptr %ptr_t34
  %tmp.16 = call ptr @rt_load_str(ptr @str.13)
  store ptr %tmp.16, ptr %ptr_t35
  %tmp.17 = load ptr, ptr %ptr_t29
  %tmp.18 = load ptr, ptr %ptr_t34
  %tmp.19 = load ptr, ptr %ptr_t35
  call void @rt_set_item(ptr %tmp.17, ptr %tmp.18, ptr %tmp.19)
  %tmp.20 = load ptr, ptr %ptr_t29
  store ptr %tmp.20, ptr %ptr_d
  %tmp.21 = call ptr @rt_load_str(ptr @str.14)
  store ptr %tmp.21, ptr %ptr_t36
  %tmp.22 = load ptr, ptr %ptr_t36
  %tmp.23 = load ptr, ptr %ptr_d
  call void @rt_print_begin()
  call void @rt_print_arg(ptr %tmp.22)
  call void @rt_print_arg(ptr %tmp.23)
  %tmp.24 = call ptr @rt_print_end()
  store ptr %tmp.24, ptr %ptr_t37
  %tmp.25 = call ptr @rt_load_str(ptr @str.15)
  store ptr %tmp.25, ptr %ptr_t38
  %tmp.26 = load ptr, ptr %ptr_t38
  %tmp.27 = load ptr, ptr %ptr_d
  %tmp.28 = call ptr @p0_len(ptr %tmp.27)
  store ptr %tmp.28, ptr %ptr_t39
  %tmp.29 = load ptr, ptr %ptr_t39
  call void @rt_print_begin()
  call void @rt_print_arg(ptr %tmp.26)
  call void @rt_print_arg(ptr %tmp.29)
  %tmp.30 = call ptr @rt_print_end()
  store ptr %tmp.30, ptr %ptr_t40
  %tmp.31 = load ptr, ptr %ptr_d
  %tmp.32 = call ptr @p0_keys(ptr %tmp.31)
  store ptr %tmp.32, ptr %ptr_t41
  %tmp.33 = load ptr, ptr %ptr_t41
  store ptr %tmp.33, ptr %ptr_k_list
  %tmp.34 = call ptr @rt_load_str(ptr @str.16)
  store ptr %tmp.34, ptr %ptr_t42
  %tmp.35 = load ptr, ptr %ptr_t42
  %tmp.36 = load ptr, ptr %ptr_k_list
  call void @rt_print_begin()
  call void @rt_print_arg(ptr %tmp.35)
  call void @rt_print_arg(ptr %tmp.36)
  %tmp.37 = call ptr @rt_print_end()
  store ptr %tmp.37, ptr %ptr_t43
  %tmp.38 = load ptr, ptr %ptr_d
  %tmp.39 = call ptr @rt_load_str(ptr @str.17)
  store ptr %tmp.39, ptr %ptr_t44
  %tmp.40 = load ptr, ptr %ptr_t44
  %tmp.41 = call ptr @p0_has_key(ptr %tmp.38, ptr %tmp.40)
  store ptr %tmp.41, ptr %ptr_t45
  %tmp.42 = load ptr, ptr %ptr_t45
  store ptr %tmp.42, ptr %ptr_has_name
  %tmp.43 = load ptr, ptr %ptr_d
  %tmp.44 = call ptr @rt_load_str(ptr @str.18)
  store ptr %tmp.44, ptr %ptr_t46
  %tmp.45 = load ptr, ptr %ptr_t46
  %tmp.46 = call ptr @p0_has_key(ptr %tmp.43, ptr %tmp.45)
  store ptr %tmp.46, ptr %ptr_t47
  %tmp.47 = load ptr, ptr %ptr_t47
  store ptr %tmp.47, ptr %ptr_has_job
  %tmp.48 = call ptr @rt_load_str(ptr @str.19)
  store ptr %tmp.48, ptr %ptr_t48
  %tmp.49 = load ptr, ptr %ptr_t48
  %tmp.50 = load ptr, ptr %ptr_has_name
  call void @rt_print_begin()
  call void @rt_print_arg(ptr %tmp.49)
  call void @rt_print_arg(ptr %tmp.50)
  %tmp.51 = call ptr @rt_print_end()
  store ptr %tmp.51, ptr %ptr_t49
  %tmp.52 = call ptr @rt_load_str(ptr @str.20)
  store ptr %tmp.52, ptr %ptr_t50
  %tmp.53 = load ptr, ptr %ptr_t50
  %tmp.54 = load ptr, ptr %ptr_has_job
  call void @rt_print_begin()
  call void @rt_print_arg(ptr %tmp.53)
  call void @rt_print_arg(ptr %tmp.54)
  %tmp.55 = call ptr @rt_print_end()
  store ptr %tmp.55, ptr %ptr_t51
  %tmp.56 = load ptr, ptr %ptr_d
  %tmp.57 = call ptr @rt_load_str(ptr @str.21)
  store ptr %tmp.57, ptr %ptr_t52
  %tmp.58 = load ptr, ptr %ptr_t52
  %tmp.59 = call ptr @p0_remove(ptr %tmp.56, ptr %tmp.58)
  store ptr %tmp.59, ptr %ptr_t53
  %tmp.60 = call ptr @rt_load_str(ptr @str.22)
  store ptr %tmp.60, ptr %ptr_t54
  %tmp.61 = load ptr, ptr %ptr_t54
  %tmp.62 = load ptr, ptr %ptr_d
  call void @rt_print_begin()
  call void @rt_print_arg(ptr %tmp.61)
  call void @rt_print_arg(ptr %tmp.62)
  %tmp.63 = call ptr @rt_print_end()
  store ptr %tmp.63, ptr %ptr_t55
  %tmp.64 = call ptr @rt_load_str(ptr @str.23)
  store ptr %tmp.64, ptr %ptr_t56
  %tmp.65 = load ptr, ptr %ptr_t56
  call void @rt_print_begin()
  call void @rt_print_arg(ptr %tmp.65)
  %tmp.66 = call ptr @rt_print_end()
  store ptr %tmp.66, ptr %ptr_t57
  ret ptr null
}

define ptr @test_type_and_cast() {
entry:
  %ptr_t77 = alloca ptr
  %ptr_t88 = alloca ptr
  %ptr_t59 = alloca ptr
  %ptr_t80 = alloca ptr
  %ptr_t65 = alloca ptr
  %ptr_t67 = alloca ptr
  %ptr_t72 = alloca ptr
  %ptr_t68 = alloca ptr
  %ptr_t85 = alloca ptr
  %ptr_t89 = alloca ptr
  %ptr_t62 = alloca ptr
  %ptr_t91 = alloca ptr
  %ptr_t90 = alloca ptr
  %ptr_t60 = alloca ptr
  %ptr_t70 = alloca ptr
  %ptr_parsed = alloca ptr
  %ptr_t84 = alloca ptr
  %ptr_n = alloca ptr
  %ptr_a = alloca ptr
  %ptr_t63 = alloca ptr
  %ptr_t73 = alloca ptr
  %ptr_t66 = alloca ptr
  %ptr_t87 = alloca ptr
  %ptr_t58 = alloca ptr
  %ptr_s = alloca ptr
  %ptr_str_val = alloca ptr
  %ptr_t82 = alloca ptr
  %ptr_t71 = alloca ptr
  %ptr_t69 = alloca ptr
  %ptr_t74 = alloca ptr
  %ptr_t78 = alloca ptr
  %ptr_t76 = alloca ptr
  %ptr_d = alloca ptr
  %ptr_t75 = alloca ptr
  %ptr_t86 = alloca ptr
  %ptr_t81 = alloca ptr
  %ptr_t61 = alloca ptr
  %ptr_t79 = alloca ptr
  %ptr_t64 = alloca ptr
  %ptr_t83 = alloca ptr
  %tmp.1 = call ptr @rt_load_str(ptr @str.24)
  store ptr %tmp.1, ptr %ptr_t58
  %tmp.2 = load ptr, ptr %ptr_t58
  call void @rt_print_begin()
  call void @rt_print_arg(ptr %tmp.2)
  %tmp.3 = call ptr @rt_print_end()
  store ptr %tmp.3, ptr %ptr_t59
  %tmp.4 = call ptr @rt_imm(i64 42)
  store ptr %tmp.4, ptr %ptr_t60
  %tmp.5 = load ptr, ptr %ptr_t60
  store ptr %tmp.5, ptr %ptr_n
  %tmp.6 = call ptr @rt_load_str(ptr @str.25)
  store ptr %tmp.6, ptr %ptr_t61
  %tmp.7 = load ptr, ptr %ptr_t61
  store ptr %tmp.7, ptr %ptr_s
  %tmp.8 = call ptr @rt_imm(i64 1)
  store ptr %tmp.8, ptr %ptr_t63
  %tmp.9 = call ptr @rt_new_arr()
  store ptr %tmp.9, ptr %ptr_t62
  %tmp.10 = load ptr, ptr %ptr_t62
  %tmp.11 = load ptr, ptr %ptr_t63
  call void @rt_append_item(ptr %tmp.10, ptr %tmp.11)
  %tmp.12 = call ptr @rt_imm(i64 2)
  store ptr %tmp.12, ptr %ptr_t64
  %tmp.13 = load ptr, ptr %ptr_t62
  %tmp.14 = load ptr, ptr %ptr_t64
  call void @rt_append_item(ptr %tmp.13, ptr %tmp.14)
  %tmp.15 = load ptr, ptr %ptr_t62
  store ptr %tmp.15, ptr %ptr_a
  %tmp.16 = call ptr @rt_new_dict()
  store ptr %tmp.16, ptr %ptr_t65
  %tmp.17 = call ptr @rt_load_str(ptr @str.26)
  store ptr %tmp.17, ptr %ptr_t66
  %tmp.18 = call ptr @rt_imm(i64 1)
  store ptr %tmp.18, ptr %ptr_t67
  %tmp.19 = load ptr, ptr %ptr_t65
  %tmp.20 = load ptr, ptr %ptr_t66
  %tmp.21 = load ptr, ptr %ptr_t67
  call void @rt_set_item(ptr %tmp.19, ptr %tmp.20, ptr %tmp.21)
  %tmp.22 = load ptr, ptr %ptr_t65
  store ptr %tmp.22, ptr %ptr_d
  %tmp.23 = call ptr @rt_load_str(ptr @str.27)
  store ptr %tmp.23, ptr %ptr_t68
  %tmp.24 = load ptr, ptr %ptr_t68
  %tmp.25 = load ptr, ptr %ptr_n
  %tmp.26 = call ptr @p0_typeof(ptr %tmp.25)
  store ptr %tmp.26, ptr %ptr_t69
  %tmp.27 = load ptr, ptr %ptr_t69
  call void @rt_print_begin()
  call void @rt_print_arg(ptr %tmp.24)
  call void @rt_print_arg(ptr %tmp.27)
  %tmp.28 = call ptr @rt_print_end()
  store ptr %tmp.28, ptr %ptr_t70
  %tmp.29 = call ptr @rt_load_str(ptr @str.28)
  store ptr %tmp.29, ptr %ptr_t71
  %tmp.30 = load ptr, ptr %ptr_t71
  %tmp.31 = load ptr, ptr %ptr_s
  %tmp.32 = call ptr @p0_typeof(ptr %tmp.31)
  store ptr %tmp.32, ptr %ptr_t72
  %tmp.33 = load ptr, ptr %ptr_t72
  call void @rt_print_begin()
  call void @rt_print_arg(ptr %tmp.30)
  call void @rt_print_arg(ptr %tmp.33)
  %tmp.34 = call ptr @rt_print_end()
  store ptr %tmp.34, ptr %ptr_t73
  %tmp.35 = call ptr @rt_load_str(ptr @str.29)
  store ptr %tmp.35, ptr %ptr_t74
  %tmp.36 = load ptr, ptr %ptr_t74
  %tmp.37 = load ptr, ptr %ptr_a
  %tmp.38 = call ptr @p0_typeof(ptr %tmp.37)
  store ptr %tmp.38, ptr %ptr_t75
  %tmp.39 = load ptr, ptr %ptr_t75
  call void @rt_print_begin()
  call void @rt_print_arg(ptr %tmp.36)
  call void @rt_print_arg(ptr %tmp.39)
  %tmp.40 = call ptr @rt_print_end()
  store ptr %tmp.40, ptr %ptr_t76
  %tmp.41 = call ptr @rt_load_str(ptr @str.30)
  store ptr %tmp.41, ptr %ptr_t77
  %tmp.42 = load ptr, ptr %ptr_t77
  %tmp.43 = load ptr, ptr %ptr_d
  %tmp.44 = call ptr @p0_typeof(ptr %tmp.43)
  store ptr %tmp.44, ptr %ptr_t78
  %tmp.45 = load ptr, ptr %ptr_t78
  call void @rt_print_begin()
  call void @rt_print_arg(ptr %tmp.42)
  call void @rt_print_arg(ptr %tmp.45)
  %tmp.46 = call ptr @rt_print_end()
  store ptr %tmp.46, ptr %ptr_t79
  %tmp.47 = load ptr, ptr %ptr_s
  %tmp.48 = call ptr @p0_int(ptr %tmp.47)
  store ptr %tmp.48, ptr %ptr_t80
  %tmp.49 = call ptr @rt_imm(i64 50)
  store ptr %tmp.49, ptr %ptr_t81
  %tmp.50 = load ptr, ptr %ptr_t80
  %tmp.51 = load ptr, ptr %ptr_t81
  %tmp.52 = call ptr @rt_add(ptr %tmp.50, ptr %tmp.51)
  store ptr %tmp.52, ptr %ptr_t82
  %tmp.53 = load ptr, ptr %ptr_t82
  store ptr %tmp.53, ptr %ptr_parsed
  %tmp.54 = call ptr @rt_load_str(ptr @str.31)
  store ptr %tmp.54, ptr %ptr_t83
  %tmp.55 = load ptr, ptr %ptr_t83
  %tmp.56 = load ptr, ptr %ptr_parsed
  call void @rt_print_begin()
  call void @rt_print_arg(ptr %tmp.55)
  call void @rt_print_arg(ptr %tmp.56)
  %tmp.57 = call ptr @rt_print_end()
  store ptr %tmp.57, ptr %ptr_t84
  %tmp.58 = load ptr, ptr %ptr_n
  %tmp.59 = call ptr @p0_str(ptr %tmp.58)
  store ptr %tmp.59, ptr %ptr_t85
  %tmp.60 = call ptr @rt_load_str(ptr @str.32)
  store ptr %tmp.60, ptr %ptr_t86
  %tmp.61 = load ptr, ptr %ptr_t85
  %tmp.62 = load ptr, ptr %ptr_t86
  %tmp.63 = call ptr @rt_add(ptr %tmp.61, ptr %tmp.62)
  store ptr %tmp.63, ptr %ptr_t87
  %tmp.64 = load ptr, ptr %ptr_t87
  store ptr %tmp.64, ptr %ptr_str_val
  %tmp.65 = call ptr @rt_load_str(ptr @str.33)
  store ptr %tmp.65, ptr %ptr_t88
  %tmp.66 = load ptr, ptr %ptr_t88
  %tmp.67 = load ptr, ptr %ptr_str_val
  call void @rt_print_begin()
  call void @rt_print_arg(ptr %tmp.66)
  call void @rt_print_arg(ptr %tmp.67)
  %tmp.68 = call ptr @rt_print_end()
  store ptr %tmp.68, ptr %ptr_t89
  %tmp.69 = call ptr @rt_load_str(ptr @str.34)
  store ptr %tmp.69, ptr %ptr_t90
  %tmp.70 = load ptr, ptr %ptr_t90
  call void @rt_print_begin()
  call void @rt_print_arg(ptr %tmp.70)
  %tmp.71 = call ptr @rt_print_end()
  store ptr %tmp.71, ptr %ptr_t91
  ret ptr null
}

define ptr @test_char_conversion() {
entry:
  %ptr_t92 = alloca ptr
  %ptr_t103 = alloca ptr
  %ptr_t102 = alloca ptr
  %ptr_char_B = alloca ptr
  %ptr_t97 = alloca ptr
  %ptr_t95 = alloca ptr
  %ptr_t94 = alloca ptr
  %ptr_code = alloca ptr
  %ptr_t96 = alloca ptr
  %ptr_t100 = alloca ptr
  %ptr_char_A = alloca ptr
  %ptr_t104 = alloca ptr
  %ptr_t99 = alloca ptr
  %ptr_t98 = alloca ptr
  %ptr_t101 = alloca ptr
  %ptr_t93 = alloca ptr
  %tmp.1 = call ptr @rt_load_str(ptr @str.35)
  store ptr %tmp.1, ptr %ptr_t92
  %tmp.2 = load ptr, ptr %ptr_t92
  call void @rt_print_begin()
  call void @rt_print_arg(ptr %tmp.2)
  %tmp.3 = call ptr @rt_print_end()
  store ptr %tmp.3, ptr %ptr_t93
  %tmp.4 = call ptr @rt_load_str(ptr @str.36)
  store ptr %tmp.4, ptr %ptr_t94
  %tmp.5 = load ptr, ptr %ptr_t94
  store ptr %tmp.5, ptr %ptr_char_A
  %tmp.6 = load ptr, ptr %ptr_char_A
  %tmp.7 = call ptr @p0_ord(ptr %tmp.6)
  store ptr %tmp.7, ptr %ptr_t95
  %tmp.8 = load ptr, ptr %ptr_t95
  store ptr %tmp.8, ptr %ptr_code
  %tmp.9 = call ptr @rt_load_str(ptr @str.37)
  store ptr %tmp.9, ptr %ptr_t96
  %tmp.10 = load ptr, ptr %ptr_t96
  %tmp.11 = load ptr, ptr %ptr_code
  call void @rt_print_begin()
  call void @rt_print_arg(ptr %tmp.10)
  call void @rt_print_arg(ptr %tmp.11)
  %tmp.12 = call ptr @rt_print_end()
  store ptr %tmp.12, ptr %ptr_t97
  %tmp.13 = call ptr @rt_imm(i64 1)
  store ptr %tmp.13, ptr %ptr_t98
  %tmp.14 = load ptr, ptr %ptr_code
  %tmp.15 = load ptr, ptr %ptr_t98
  %tmp.16 = call ptr @rt_add(ptr %tmp.14, ptr %tmp.15)
  store ptr %tmp.16, ptr %ptr_t99
  %tmp.17 = load ptr, ptr %ptr_t99
  %tmp.18 = call ptr @p0_chr(ptr %tmp.17)
  store ptr %tmp.18, ptr %ptr_t100
  %tmp.19 = load ptr, ptr %ptr_t100
  store ptr %tmp.19, ptr %ptr_char_B
  %tmp.20 = call ptr @rt_load_str(ptr @str.38)
  store ptr %tmp.20, ptr %ptr_t101
  %tmp.21 = load ptr, ptr %ptr_t101
  %tmp.22 = load ptr, ptr %ptr_char_B
  call void @rt_print_begin()
  call void @rt_print_arg(ptr %tmp.21)
  call void @rt_print_arg(ptr %tmp.22)
  %tmp.23 = call ptr @rt_print_end()
  store ptr %tmp.23, ptr %ptr_t102
  %tmp.24 = call ptr @rt_load_str(ptr @str.39)
  store ptr %tmp.24, ptr %ptr_t103
  %tmp.25 = load ptr, ptr %ptr_t103
  call void @rt_print_begin()
  call void @rt_print_arg(ptr %tmp.25)
  %tmp.26 = call ptr @rt_print_end()
  store ptr %tmp.26, ptr %ptr_t104
  ret ptr null
}

define ptr @test_system() {
entry:
  %ptr_t111 = alloca ptr
  %ptr_t110 = alloca ptr
  %ptr_t107 = alloca ptr
  %ptr_t112 = alloca ptr
  %ptr_t105 = alloca ptr
  %ptr_t108 = alloca ptr
  %ptr_t114 = alloca ptr
  %ptr_t113 = alloca ptr
  %ptr_r = alloca ptr
  %ptr_t1 = alloca ptr
  %ptr_t106 = alloca ptr
  %ptr_t109 = alloca ptr
  %tmp.1 = call ptr @rt_load_str(ptr @str.40)
  store ptr %tmp.1, ptr %ptr_t105
  %tmp.2 = load ptr, ptr %ptr_t105
  call void @rt_print_begin()
  call void @rt_print_arg(ptr %tmp.2)
  %tmp.3 = call ptr @rt_print_end()
  store ptr %tmp.3, ptr %ptr_t106
  %tmp.4 = call ptr @p0_time()
  store ptr %tmp.4, ptr %ptr_t107
  %tmp.5 = load ptr, ptr %ptr_t107
  store ptr %tmp.5, ptr %ptr_t1
  %tmp.6 = call ptr @rt_load_str(ptr @str.41)
  store ptr %tmp.6, ptr %ptr_t108
  %tmp.7 = load ptr, ptr %ptr_t108
  %tmp.8 = load ptr, ptr %ptr_t1
  call void @rt_print_begin()
  call void @rt_print_arg(ptr %tmp.7)
  call void @rt_print_arg(ptr %tmp.8)
  %tmp.9 = call ptr @rt_print_end()
  store ptr %tmp.9, ptr %ptr_t109
  %tmp.10 = call ptr @p0_random()
  store ptr %tmp.10, ptr %ptr_t110
  %tmp.11 = load ptr, ptr %ptr_t110
  store ptr %tmp.11, ptr %ptr_r
  %tmp.12 = call ptr @rt_load_str(ptr @str.42)
  store ptr %tmp.12, ptr %ptr_t111
  %tmp.13 = load ptr, ptr %ptr_t111
  %tmp.14 = load ptr, ptr %ptr_r
  call void @rt_print_begin()
  call void @rt_print_arg(ptr %tmp.13)
  call void @rt_print_arg(ptr %tmp.14)
  %tmp.15 = call ptr @rt_print_end()
  store ptr %tmp.15, ptr %ptr_t112
  %tmp.16 = call ptr @rt_load_str(ptr @str.43)
  store ptr %tmp.16, ptr %ptr_t113
  %tmp.17 = load ptr, ptr %ptr_t113
  call void @rt_print_begin()
  call void @rt_print_arg(ptr %tmp.17)
  %tmp.18 = call ptr @rt_print_end()
  store ptr %tmp.18, ptr %ptr_t114
  ret ptr null
}

define ptr @__p0_main() {
entry:
  %ptr_t126 = alloca ptr
  %ptr_t133 = alloca ptr
  %ptr_t116 = alloca ptr
  %ptr_t123 = alloca ptr
  %ptr_t134 = alloca ptr
  %ptr_t124 = alloca ptr
  %ptr_user_in = alloca ptr
  %ptr_t129 = alloca ptr
  %ptr_t130 = alloca ptr
  %ptr_t117 = alloca ptr
  %ptr_t125 = alloca ptr
  %ptr_t122 = alloca ptr
  %ptr_t121 = alloca ptr
  %ptr_t118 = alloca ptr
  %ptr_t115 = alloca ptr
  %ptr_t119 = alloca ptr
  %ptr_t120 = alloca ptr
  %ptr_t127 = alloca ptr
  %ptr_t128 = alloca ptr
  %ptr_t132 = alloca ptr
  %ptr_t131 = alloca ptr
  %ptr_t135 = alloca ptr
  %tmp.1 = call ptr @rt_load_str(ptr @str.44)
  store ptr %tmp.1, ptr %ptr_t115
  %tmp.2 = load ptr, ptr %ptr_t115
  call void @rt_print_begin()
  call void @rt_print_arg(ptr %tmp.2)
  %tmp.3 = call ptr @rt_print_end()
  store ptr %tmp.3, ptr %ptr_t116
  %tmp.4 = call ptr @rt_load_str(ptr @str.45)
  store ptr %tmp.4, ptr %ptr_t117
  %tmp.5 = load ptr, ptr %ptr_t117
  call void @rt_print_begin()
  call void @rt_print_arg(ptr %tmp.5)
  %tmp.6 = call ptr @rt_print_end()
  store ptr %tmp.6, ptr %ptr_t118
  %tmp.7 = call ptr @test_array()
  store ptr %tmp.7, ptr %ptr_t119
  %tmp.8 = call ptr @test_dict()
  store ptr %tmp.8, ptr %ptr_t120
  %tmp.9 = call ptr @test_type_and_cast()
  store ptr %tmp.9, ptr %ptr_t121
  %tmp.10 = call ptr @test_char_conversion()
  store ptr %tmp.10, ptr %ptr_t122
  %tmp.11 = call ptr @test_system()
  store ptr %tmp.11, ptr %ptr_t123
  %tmp.12 = call ptr @rt_load_str(ptr @str.46)
  store ptr %tmp.12, ptr %ptr_t124
  %tmp.13 = load ptr, ptr %ptr_t124
  call void @rt_print_begin()
  call void @rt_print_arg(ptr %tmp.13)
  %tmp.14 = call ptr @rt_print_end()
  store ptr %tmp.14, ptr %ptr_t125
  %tmp.15 = call ptr @rt_load_str(ptr @str.47)
  store ptr %tmp.15, ptr %ptr_t126
  %tmp.16 = load ptr, ptr %ptr_t126
  %tmp.17 = call ptr @p0_input(ptr %tmp.16)
  store ptr %tmp.17, ptr %ptr_t127
  %tmp.18 = load ptr, ptr %ptr_t127
  store ptr %tmp.18, ptr %ptr_user_in
  %tmp.19 = call ptr @rt_load_str(ptr @str.48)
  store ptr %tmp.19, ptr %ptr_t128
  %tmp.20 = load ptr, ptr %ptr_t128
  %tmp.21 = load ptr, ptr %ptr_user_in
  call void @rt_print_begin()
  call void @rt_print_arg(ptr %tmp.20)
  call void @rt_print_arg(ptr %tmp.21)
  %tmp.22 = call ptr @rt_print_end()
  store ptr %tmp.22, ptr %ptr_t129
  %tmp.23 = call ptr @rt_load_str(ptr @str.49)
  store ptr %tmp.23, ptr %ptr_t130
  %tmp.24 = load ptr, ptr %ptr_t130
  call void @rt_print_begin()
  call void @rt_print_arg(ptr %tmp.24)
  %tmp.25 = call ptr @rt_print_end()
  store ptr %tmp.25, ptr %ptr_t131
  %tmp.26 = call ptr @rt_imm(i64 0)
  store ptr %tmp.26, ptr %ptr_t132
  %tmp.27 = load ptr, ptr %ptr_t132
  %tmp.28 = call ptr @p0_exit(ptr %tmp.27)
  store ptr %tmp.28, ptr %ptr_t133
  %tmp.29 = call ptr @rt_load_str(ptr @str.50)
  store ptr %tmp.29, ptr %ptr_t134
  %tmp.30 = load ptr, ptr %ptr_t134
  call void @rt_print_begin()
  call void @rt_print_arg(ptr %tmp.30)
  %tmp.31 = call ptr @rt_print_end()
  store ptr %tmp.31, ptr %ptr_t135
  ret ptr null
}

; === Program Init ===

define ptr @__init__() {
entry:
  %ptr_t136 = alloca ptr
  %tmp.1 = call ptr @__p0_main()
  store ptr %tmp.1, ptr %ptr_t136
  ret ptr null
}

; === Program Entry ===

define i32 @main() {
entry:
  %tmp.entry = call ptr @__init__()
  ret i32 0
}
