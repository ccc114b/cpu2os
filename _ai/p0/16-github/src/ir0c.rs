use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::process;

/// 四元式 (Quadruple) 結構，用來表示一行中間碼 (IR)
/// 包含操作碼 (op)、兩個運算元 (arg1, arg2) 與結果 (result)
#[derive(Clone, Debug)]
pub struct Quad {
    pub op: String,
    pub arg1: String,
    pub arg2: String,
    pub result: String,
}

/// 目標編譯類型，決定最後要輸出為主程式 (Bin) 或是函式庫 (Lib)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TargetKind {
    Bin, // 產生 main 函式與執行檔入口
    Lib, // 僅產生函式，無 main 進入點
}

/// 將字串格式化為 LLVM 接受的 C-String 格式
/// 例如：將換行轉為 `\0A`，並在結尾加上 `\00` 代表 C 字串結束符
fn escape_llvm_str(s: &str) -> String {
    let mut res = String::new();
    for b in s.bytes() {
        if b >= 32 && b <= 126 && b != b'"' && b != b'\\' {
            res.push(b as char);
        } else {
            res.push_str(&format!("\\{:02X}", b));
        }
    }
    res.push_str("\\00");
    res
}

/// 解析從 .ir0 檔案讀入的 `{:?}` (Debug) 格式的字串常數
/// 將其中的跳脫字元 (如 `\n`, `\t`) 還原回實際的 ASCII 字元
fn parse_debug_str(s: &str) -> String {
    if s.len() < 2 { return String::new(); }
    let mut res = String::new();
    let inner = &s[1..s.len() - 1];
    let mut chars = inner.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\\' {
            if let Some(nc) = chars.next() {
                match nc {
                    'n' => res.push('\n'),
                    't' => res.push('\t'),
                    'r' => res.push('\r'),
                    '\\' => res.push('\\'),
                    '"' => res.push('"'),
                    '\'' => res.push('\''),
                    _ => res.push(nc),
                }
            }
        } else { res.push(c); }
    }
    res
}

/// LLVM IR 產生器，負責將中介四元式轉換為 LLVM 組語格式 (.ll)
struct LLVMGenerator {
    quads: Vec<Quad>,         // 輸入的中介碼序列
    string_pool: Vec<String>, // 字串常數池
    target: TargetKind,       // 編譯目標 (Bin 或 Lib)
    out: String,              // 最終生成的 LLVM IR 字串
    tmp_counter: usize,       // 暫存器計數器，用於產生 %tmp.1, %tmp.2 等
    lbl_counter: usize,       // 標籤計數器，用於產生不重複的 fallthrough.1 標籤
}

impl LLVMGenerator {
    /// 初始化產生器
    fn new(quads: Vec<Quad>, string_pool: Vec<String>, target: TargetKind) -> Self {
        LLVMGenerator { quads, string_pool, target, out: String::new(), tmp_counter: 0, lbl_counter: 0 }
    }

    /// 產生下一個區域暫存器名稱 (例如 `%tmp.1`)
    fn next_tmp(&mut self) -> String {
        self.tmp_counter += 1;
        format!("%tmp.{}", self.tmp_counter)
    }

    /// 產生下一個控制流的落點標籤 (例如 `fallthrough.1`)
    fn next_lbl(&mut self) -> String {
        self.lbl_counter += 1;
        format!("fallthrough.{}", self.lbl_counter)
    }

    /// 將變數從堆疊中載入 (load) 出來，並回傳配置的 LLVM 暫存器名稱 (如 `%tmp.1`)
    fn load_var(&mut self, var: &str) -> String {
        let tmp = self.next_tmp();
        self.out.push_str(&format!("  {} = load ptr, ptr %ptr_{}\n", tmp, var));
        tmp
    }

    /// 掃描指令區塊，收集所有被當作區域變數或參數使用的變數名稱
    fn collect_local_vars(&self, quads: &[Quad], include_ret_val_arg: bool) -> HashSet<String> {
        let mut local_vars: HashSet<String> = HashSet::new();
        for sq in quads {
            let op = sq.op.as_str();
            if ["ADD", "SUB", "MUL", "DIV", "CMP_EQ", "CMP_LT", "CMP_GT", "GET_ITEM"].contains(&op) {
                local_vars.insert(sq.arg1.clone());
                local_vars.insert(sq.arg2.clone());
                local_vars.insert(sq.result.clone());
            } else if ["STORE", "APPEND_ITEM"].contains(&op) {
                local_vars.insert(sq.arg1.clone());
                local_vars.insert(sq.result.clone());
            } else if ["SET_ITEM", "INIT_ARR"].contains(&op) {
                local_vars.insert(sq.arg1.clone());
                local_vars.insert(sq.arg2.clone());
                local_vars.insert(sq.result.clone());
            } else if ["IMM", "LOAD_STR", "NEW_ARR", "NEW_DICT", "CALL"].contains(&op) {
                local_vars.insert(sq.result.clone());
            } else if op == "PARAM" || op == "JMP_F" {
                local_vars.insert(sq.arg1.clone());
            } else if include_ret_val_arg && op == "RET_VAL" {
                local_vars.insert(sq.arg1.clone());
            }
        }
        local_vars.remove("-");
        local_vars.remove("?");
        local_vars
    }

    /// 將一段四元式指令區塊 (Block) 翻譯並發行為 LLVM IR
    fn emit_block(
        &mut self,
        quads: &[Quad],                           // 要處理的指令範圍
        param_stack: &mut Vec<String>,            // 當前準備用來傳遞給函式的參數堆疊
        last_was_term: &mut bool,                 // 紀錄上一道指令是否為終止指令 (ret, br)
        ret_ptr: bool,                            // 函式是否需要回傳指標 (ptr)
        func_name_map: &HashMap<String, String>,  // 函式名稱的映射表 (處理 main 重新命名為 __p0_main)
    ) {
        for inner_q in quads {
            // 在 LLVM 中，基本區塊 (Basic Block) 必須要有標籤。
            // 若前一個區塊已用 br/ret 結尾，且接下來的指令不是標籤，則需要插入一個 dummy 標籤
            if *last_was_term && inner_q.op != "LABEL" {
                let dummy = self.next_lbl();
                self.out.push_str(&format!("{}:\n", dummy));
                *last_was_term = false;
            }

            match inner_q.op.as_str() {
                "LABEL" => {
                    // 基本區塊標籤，如果前面沒有斷開控制流，需先無條件跳轉進去
                    if !*last_was_term {
                        self.out.push_str(&format!("  br label %{}\n", inner_q.arg1));
                    }
                    self.out.push_str(&format!("{}:\n", inner_q.arg1));
                    *last_was_term = false;
                }
                "IMM" => {
                    // 產生數字常數物件 (呼叫 C Runtime rt_imm)，並存入指標
                    let tmp = self.next_tmp();
                    self.out.push_str(&format!("  {} = call ptr @rt_imm(i64 {})\n", tmp, inner_q.arg1));
                    self.out.push_str(&format!("  store ptr {}, ptr %ptr_{}\n", tmp, inner_q.result));
                }
                "LOAD_STR" => {
                    // 從字串常數池載入字串
                    let tmp = self.next_tmp();
                    self.out.push_str(&format!("  {} = call ptr @rt_load_str(ptr @str.{})\n", tmp, inner_q.arg1));
                    self.out.push_str(&format!("  store ptr {}, ptr %ptr_{}\n", tmp, inner_q.result));
                }
                op if ["ADD", "SUB", "MUL", "DIV", "CMP_EQ", "CMP_LT", "CMP_GT"].contains(&op) => {
                    // 執行算術或比較操作，委派給 C Runtime
                    let v1 = self.load_var(&inner_q.arg1);
                    let v2 = self.load_var(&inner_q.arg2);
                    let tmp = self.next_tmp();
                    let rt_func = format!("@rt_{}", op.to_lowercase());
                    self.out.push_str(&format!("  {} = call ptr {}(ptr {}, ptr {})\n", tmp, rt_func, v1, v2));
                    self.out.push_str(&format!("  store ptr {}, ptr %ptr_{}\n", tmp, inner_q.result));
                }
                "JMP" => {
                    self.out.push_str(&format!("  br label %{}\n", inner_q.result));
                    *last_was_term = true;
                }
                "JMP_F" => {
                    // 條件跳躍 (If / While)：判斷條件是否為 truthy
                    // 如果為真跳到 next_lbl 繼續執行，為假則跳離 (inner_q.result)
                    let cond = self.load_var(&inner_q.arg1);
                    let is_true = self.next_tmp();
                    self.out.push_str(&format!("  {} = call i1 @rt_is_truthy(ptr {})\n", is_true, cond));
                    let next_lbl = self.next_lbl();
                    self.out.push_str(&format!("  br i1 {}, label %{}, label %{}\n", is_true, next_lbl, inner_q.result));
                    self.out.push_str(&format!("{}:\n", next_lbl));
                    *last_was_term = false;
                }
                "STORE" => {
                    let val = self.load_var(&inner_q.arg1);
                    self.out.push_str(&format!("  store ptr {}, ptr %ptr_{}\n", val, inner_q.result));
                }
                "NEW_ARR" | "NEW_DICT" => {
                    let rt_func = if inner_q.op == "NEW_ARR" { "@rt_new_arr" } else { "@rt_new_dict" };
                    let tmp = self.next_tmp();
                    self.out.push_str(&format!("  {} = call ptr {}()\n", tmp, rt_func));
                    self.out.push_str(&format!("  store ptr {}, ptr %ptr_{}\n", tmp, inner_q.result));
                }
                "INIT_ARR" => {
                    let val = self.load_var(&inner_q.arg1);
                    let size = self.load_var(&inner_q.arg2);
                    let tmp = self.next_tmp();
                    self.out.push_str(&format!("  {} = call ptr @rt_init_arr(ptr {}, ptr {})\n", tmp, val, size));
                    self.out.push_str(&format!("  store ptr {}, ptr %ptr_{}\n", tmp, inner_q.result));
                }
                "APPEND_ITEM" => {
                    let arr = self.load_var(&inner_q.arg1);
                    let val = self.load_var(&inner_q.result);
                    self.out.push_str(&format!("  call void @rt_append_item(ptr {}, ptr {})\n", arr, val));
                }
                "SET_ITEM" => {
                    let obj = self.load_var(&inner_q.arg1);
                    let key = self.load_var(&inner_q.arg2);
                    let val = self.load_var(&inner_q.result);
                    self.out.push_str(&format!("  call void @rt_set_item(ptr {}, ptr {}, ptr {})\n", obj, key, val));
                }
                "GET_ITEM" => {
                    let obj = self.load_var(&inner_q.arg1);
                    let key = self.load_var(&inner_q.arg2);
                    let tmp = self.next_tmp();
                    self.out.push_str(&format!("  {} = call ptr @rt_get_item(ptr {}, ptr {})\n", tmp, obj, key));
                    self.out.push_str(&format!("  store ptr {}, ptr %ptr_{}\n", tmp, inner_q.result));
                }
                "PARAM" => {
                    let val = self.load_var(&inner_q.arg1);
                    param_stack.push(val);
                }
                "CALL" => {
                    let p_count: usize = inner_q.arg2.parse().unwrap();
                    let mut args = Vec::new();
                    for _ in 0..p_count {
                        args.push(param_stack.pop().unwrap());
                    }
                    args.reverse();

                    if inner_q.arg1 == "print" {
                        self.out.push_str("  call void @rt_print_begin()\n");
                        for arg in &args {
                            self.out.push_str(&format!("  call void @rt_print_arg(ptr {})\n", arg));
                        }
                        let tmp = self.next_tmp();
                        self.out.push_str(&format!("  {} = call ptr @rt_print_end()\n", tmp));
                        self.out.push_str(&format!("  store ptr {}, ptr %ptr_{}\n", tmp, inner_q.result));
                    } else {
                        let builtin_callee = match inner_q.arg1.as_str() {
                            "len" => "p0_len",
                            "time" => "p0_time",
                            "array" => "p0_array",
                            "push" => "p0_push",
                            "pop" => "p0_pop",
                            "keys" => "p0_keys",
                            "has_key" => "p0_has_key",
                            "remove" => "p0_remove",
                            "typeof" => "p0_typeof",
                            "int" => "p0_int",
                            "str" => "p0_str",
                            "ord" => "p0_ord",
                            "chr" => "p0_chr",
                            "random" => "p0_random",
                            "input" => "p0_input",
                            "exit" => "p0_exit",
                            _ => "",
                        };
                        let callee = if !builtin_callee.is_empty() {
                            builtin_callee.to_string()
                        } else {
                            func_name_map.get(&inner_q.arg1).cloned().unwrap_or_else(|| inner_q.arg1.clone())
                        };
                        let args_str = args.iter().map(|a| format!("ptr {}", a)).collect::<Vec<_>>().join(", ");
                        let tmp = self.next_tmp();
                        self.out.push_str(&format!("  {} = call ptr @{}({})\n", tmp, callee, args_str));
                        self.out.push_str(&format!("  store ptr {}, ptr %ptr_{}\n", tmp, inner_q.result));
                    }
                }
                "RET_VAL" => {
                    if ret_ptr {
                        let val = self.load_var(&inner_q.arg1);
                        self.out.push_str(&format!("  ret ptr {}\n", val));
                    } else {
                        self.out.push_str("  ret i32 0\n");
                    }
                    *last_was_term = true;
                }
                _ => {}
            }
        }
    }

    /// 執行完整轉換，從宣告 Runtime API、分配變數，到轉換所有函式和頂層敘述
    pub fn generate(&mut self) {
        // 1. 輸出 Runtime 函數宣告 (C 實作的系統與輔助函數)
        self.out.push_str("; === Runtime API Declarations ===\n");
        let rt_funcs = vec![
            "declare ptr @rt_imm(i64)",
            "declare ptr @rt_load_str(ptr)",
            "declare ptr @rt_add(ptr, ptr)",
            "declare ptr @rt_sub(ptr, ptr)",
            "declare ptr @rt_mul(ptr, ptr)",
            "declare ptr @rt_div(ptr, ptr)",
            "declare ptr @rt_cmp_eq(ptr, ptr)",
            "declare ptr @rt_cmp_lt(ptr, ptr)",
            "declare ptr @rt_cmp_gt(ptr, ptr)",
            "declare ptr @rt_new_arr()",
            "declare ptr @rt_new_dict()",
            "declare ptr @rt_init_arr(ptr, ptr)",
            "declare void @rt_append_item(ptr, ptr)",
            "declare void @rt_set_item(ptr, ptr, ptr)",
            "declare ptr @rt_get_item(ptr, ptr)",
            "declare i1 @rt_is_truthy(ptr)",
            "declare void @rt_print_begin()",
            "declare void @rt_print_arg(ptr)",
            "declare ptr @rt_print_end()",
            "declare ptr @p0_len(ptr)",
            "declare ptr @p0_time()",
            "declare ptr @p0_array(ptr, ptr)",
            "declare ptr @p0_push(ptr, ptr)",
            "declare ptr @p0_pop(ptr)",
            "declare ptr @p0_keys(ptr)",
            "declare ptr @p0_has_key(ptr, ptr)",
            "declare ptr @p0_remove(ptr, ptr)",
            "declare ptr @p0_typeof(ptr)",
            "declare ptr @p0_int(ptr)",
            "declare ptr @p0_str(ptr)",
            "declare ptr @p0_ord(ptr)",
            "declare ptr @p0_chr(ptr)",
            "declare ptr @p0_random()",
            "declare ptr @p0_input(ptr)",
            "declare ptr @p0_exit(ptr)",
        ];
        for f in rt_funcs { self.out.push_str(&format!("{}\n", f)); }

        // 2. 掃描四元式以尋找所有自訂函數的簽章與被呼叫的外部函數 (如 print, len)
        let mut defined_funcs = HashMap::new();
        let mut extern_funcs = HashSet::new();
        
        let mut i = 0;
        while i < self.quads.len() {
            if self.quads[i].op == "FUNC_BEG" {
                let f_name = self.quads[i].arg1.clone();
                let mut formals = Vec::new();
                let mut j = i + 1;
                while j < self.quads.len() && self.quads[j].op == "FORMAL" {
                    formals.push(self.quads[j].arg1.clone());
                    j += 1;
                }
                defined_funcs.insert(f_name, formals);
                i = j - 1;
            } else if self.quads[i].op == "CALL" {
                let f_name = self.quads[i].arg1.clone();
                extern_funcs.insert(f_name);
            }
            i += 1;
        }

        if defined_funcs.contains_key("__init__") {
            panic!("函式名稱 __init__ 為保留名稱，請改名");
        }
        if defined_funcs.contains_key("main") && defined_funcs.contains_key("__p0_main") {
            panic!("函式名稱 main 與 __p0_main 會衝突，請改名");
        }
        let mut func_name_map: HashMap<String, String> = HashMap::new();
        for f in defined_funcs.keys() {
            if f == "main" {
                func_name_map.insert(f.clone(), "__p0_main".to_string());
            } else {
                func_name_map.insert(f.clone(), f.clone());
            }
        }

        // 宣告尚未定義的外部函數 (用 C 實作的外部 System calls)
        self.out.push_str("\n; === External System Calls ===\n");
        let builtin_funcs: HashSet<&str> = [
            "print", "len", "time", "array", "push", "pop", "keys", "has_key", "remove",
            "typeof", "int", "str", "ord", "chr", "random", "input", "exit",
        ].iter().copied().collect();
        for ext in &extern_funcs {
            if !builtin_funcs.contains(ext.as_str()) && !defined_funcs.contains_key(ext) {
                // 使用 (...) 支援任意數量參數的 C 函數
                self.out.push_str(&format!("declare ptr @{}(...)\n", ext));
            }
        }

        // 3. 輸出字串常數池 (將原始碼中的所有字串打包為全域常數陣列)
        self.out.push_str("\n; === String Pool ===\n");
        for (idx, s) in self.string_pool.iter().enumerate() {
            let escaped = escape_llvm_str(s);
            let byte_len = s.len() + 1; // 包含 \00
            self.out.push_str(&format!("@str.{} = private unnamed_addr constant [{} x i8] c\"{}\"\n", idx, byte_len, escaped));
        }

        // 4. 開始產生各個函數的 LLVM IR，並同時收集不屬於任何函數的「頂層指令」
        self.out.push_str("\n; === Functions ===\n");
        let mut top_level_quads: Vec<Quad> = Vec::new();
        let mut pc = 0;
        while pc < self.quads.len() {
            let q = &self.quads[pc].clone();
            
            if q.op == "FUNC_BEG" {
                let f_name = &q.arg1;
                let llvm_f_name = func_name_map.get(f_name).unwrap();
                let formals = defined_funcs.get(f_name).unwrap();
                let body_start = pc + formals.len() + 1;
                let mut body_end = body_start;
                while body_end < self.quads.len() && self.quads[body_end].op != "FUNC_END" {
                    body_end += 1;
                }

                // 產生 LLVM 函數標頭 (Signature)
                let args_str = formals.iter().map(|f| format!("ptr %arg_{}", f)).collect::<Vec<_>>().join(", ");
                self.out.push_str(&format!("\ndefine ptr @{}({}) {{\nentry:\n", llvm_f_name, args_str));

                let body_quads: Vec<Quad> = self.quads[body_start..body_end].to_vec();
                let mut local_vars = self.collect_local_vars(&body_quads, true);
                for f in formals {
                    local_vars.insert(f.clone());
                }
                for var in local_vars {
                    self.out.push_str(&format!("  %ptr_{} = alloca ptr\n", var));
                }
                for f in formals {
                    self.out.push_str(&format!("  store ptr %arg_{}, ptr %ptr_{}\n", f, f));
                }

                self.tmp_counter = 0;
                let mut param_stack = Vec::new();
                let mut last_was_term = false;
                self.emit_block(&body_quads, &mut param_stack, &mut last_was_term, true, &func_name_map);
                
                // 如果函數結尾沒有明確的 return，補上預設的 ret null
                if !last_was_term {
                    self.out.push_str("  ret ptr null\n");
                }
                self.out.push_str("}\n");
                pc = body_end.saturating_add(1);
                continue;
            }
            top_level_quads.push(q.clone());
            pc += 1;
        }

        // 5. 產生初始化函式 `__init__`，用來執行全域的頂層敘述
        self.out.push_str("\n; === Program Init ===\n");
        self.out.push_str("\ndefine ptr @__init__() {\nentry:\n");

        let local_vars = self.collect_local_vars(&top_level_quads, false);
        for var in local_vars {
            self.out.push_str(&format!("  %ptr_{} = alloca ptr\n", var));
        }

        self.tmp_counter = 0;
        let mut param_stack = Vec::new();
        let mut last_was_term = false;
        self.emit_block(&top_level_quads, &mut param_stack, &mut last_was_term, true, &func_name_map);

        if !last_was_term {
            self.out.push_str("  ret ptr null\n");
        }
        self.out.push_str("}\n");

        // 6. 若為 Bin (執行檔) 模式，產生程式真正的進入點 `main`
        if self.target == TargetKind::Bin {
            self.out.push_str("\n; === Program Entry ===\n");
            self.out.push_str("\ndefine i32 @main() {\nentry:\n");
            self.out.push_str("  %tmp.entry = call ptr @__init__()\n");
            self.out.push_str("  ret i32 0\n");
            self.out.push_str("}\n");
        }
    }
}

/// 程式主要入口：處理參數、讀取 .ir0 檔案、解析並轉換出 .ll LLVM IR 檔案
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut target = TargetKind::Bin;
    let mut positional: Vec<String> = Vec::new();

    let mut i = 1;
    while i < args.len() {
        let arg = &args[i];
        if arg == "--target" {
            if i + 1 >= args.len() {
                eprintln!("缺少 --target 參數值，需為 bin 或 lib");
                process::exit(1);
            }
            target = match args[i + 1].as_str() {
                "bin" => TargetKind::Bin,
                "lib" => TargetKind::Lib,
                _ => {
                    eprintln!("不支援的 --target 值: {} (僅支援 bin 或 lib)", args[i + 1]);
                    process::exit(1);
                }
            };
            i += 2;
            continue;
        }
        if let Some(v) = arg.strip_prefix("--target=") {
            target = match v {
                "bin" => TargetKind::Bin,
                "lib" => TargetKind::Lib,
                _ => {
                    eprintln!("不支援的 --target 值: {} (僅支援 bin 或 lib)", v);
                    process::exit(1);
                }
            };
            i += 1;
            continue;
        }
        positional.push(arg.clone());
        i += 1;
    }

    if positional.is_empty() {
        println!("用法: {} [--target bin|lib] <file.ir0> [file.ll]", args[0]);
        process::exit(1);
    }

    let input_file = &positional[0];
    let output_file = if positional.len() >= 2 {
        positional[1].clone()
    } else {
        std::path::Path::new(input_file).with_extension("ll").to_string_lossy().into_owned()
    };

    let ir_content = fs::read_to_string(input_file).expect("無法開啟 IR 檔案");
    
    let mut string_pool = Vec::new();
    let mut quads = Vec::new();
    let mut state = 0;

    for line in ir_content.lines() {
        let trimmed = line.trim_end();
        if trimmed.is_empty() { continue; }
        if trimmed == "===STRINGS===" { state = 1; continue; }
        else if trimmed == "===QUADS===" { state = 2; continue; }

        if state == 1 { string_pool.push(parse_debug_str(trimmed)); }
        else if state == 2 {
            let parts: Vec<&str> = trimmed.split('\t').collect();
            if parts.len() >= 4 {
                quads.push(Quad { op: parts[0].to_string(), arg1: parts[1].to_string(), arg2: parts[2].to_string(), result: parts[3].to_string() });
            }
        }
    }

    // 3. 啟動 LLVM 生成器
    // println!("=== 開始轉換 LLVM IR ===");
    let mut generator = LLVMGenerator::new(quads, string_pool, target);
    generator.generate();

    let mut out_file = File::create(&output_file).expect("無法建立輸出檔案");
    out_file.write_all(generator.out.as_bytes()).expect("寫入失敗");
    
    // println!("✅ LLVM IR 產生成功！已匯出至: {}", output_file);
    // println!("(提示：接下來你需要一個 C Runtime 函式庫來編譯這個 .ll 檔案)");
}
