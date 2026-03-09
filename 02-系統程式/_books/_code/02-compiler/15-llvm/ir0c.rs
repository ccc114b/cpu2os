use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::process;

#[derive(Clone, Debug)]
pub struct Quad {
    pub op: String,
    pub arg1: String,
    pub arg2: String,
    pub result: String,
}

// 將字串格式化為 LLVM 接受的 C-String 格式 (例如將換行轉為 \0A，並加上 \00 結尾)
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

// 解析從 ir0 讀入的 {:?} 格式字串
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

struct LLVMGenerator {
    quads: Vec<Quad>,
    string_pool: Vec<String>,
    out: String,
    tmp_counter: usize,
    lbl_counter: usize,
}

impl LLVMGenerator {
    fn new(quads: Vec<Quad>, string_pool: Vec<String>) -> Self {
        LLVMGenerator { quads, string_pool, out: String::new(), tmp_counter: 0, lbl_counter: 0 }
    }

    fn next_tmp(&mut self) -> String {
        self.tmp_counter += 1;
        format!("%tmp.{}", self.tmp_counter)
    }

    fn next_lbl(&mut self) -> String {
        self.lbl_counter += 1;
        format!("fallthrough.{}", self.lbl_counter)
    }

    // 將變數從堆疊中 load 出來，回傳 LLVM 的暫存器名稱 (如 %tmp.1)
    fn load_var(&mut self, var: &str) -> String {
        let tmp = self.next_tmp();
        self.out.push_str(&format!("  {} = load ptr, ptr %ptr_{}\n", tmp, var));
        tmp
    }

    pub fn generate(&mut self) {
        // 1. 輸出 Runtime 函數宣告
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
        ];
        for f in rt_funcs { self.out.push_str(&format!("{}\n", f)); }

        // 2. 尋找所有自訂函數的簽章與被呼叫的外部函數 (如 print, len)
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

        // 宣告尚未定義的外部函數 (System calls)
        self.out.push_str("\n; === External System Calls ===\n");
        for ext in &extern_funcs {
            if !defined_funcs.contains_key(ext) {
                // 使用 (...) 支援任意數量參數的 C 函數
                self.out.push_str(&format!("declare ptr @{}(...)\n", ext));
            }
        }

        // 3. 輸出字串常數池
        self.out.push_str("\n; === String Pool ===\n");
        for (idx, s) in self.string_pool.iter().enumerate() {
            let escaped = escape_llvm_str(s);
            let byte_len = s.len() + 1; // 包含 \00
            self.out.push_str(&format!("@str.{} = private unnamed_addr constant [{} x i8] c\"{}\"\n", idx, byte_len, escaped));
        }

        // 4. 開始產生各個函數的 LLVM IR
        self.out.push_str("\n; === Functions ===\n");
        
        let mut pc = 0;
        while pc < self.quads.len() {
            let q = &self.quads[pc].clone();
            
            if q.op == "FUNC_BEG" {
                let f_name = &q.arg1;
                let formals = defined_funcs.get(f_name).unwrap();
                
                // 產生函數標頭
                let args_str = formals.iter().map(|f| format!("ptr %arg_{}", f)).collect::<Vec<_>>().join(", ");
                self.out.push_str(&format!("\ndefine ptr @{}({}) {{\nentry:\n", f_name, args_str));
                
                // 找出此函數內所有的變數，一次性 alloca
                let mut local_vars: HashSet<String> = HashSet::new();
                let mut scan_pc = pc + 1;
                while scan_pc < self.quads.len() && self.quads[scan_pc].op != "FUNC_END" {
                    let sq = &self.quads[scan_pc];
                    let op = sq.op.as_str();
                    // 將會用到的變數加入集合
                    if["ADD", "SUB", "MUL", "DIV", "CMP_EQ", "CMP_LT", "CMP_GT", "GET_ITEM"].contains(&op) {
                        local_vars.insert(sq.arg1.clone()); local_vars.insert(sq.arg2.clone()); local_vars.insert(sq.result.clone());
                    } else if ["STORE", "APPEND_ITEM"].contains(&op) {
                        local_vars.insert(sq.arg1.clone()); local_vars.insert(sq.result.clone());
                    } else if ["SET_ITEM", "INIT_ARR"].contains(&op) {
                        local_vars.insert(sq.arg1.clone()); local_vars.insert(sq.arg2.clone()); local_vars.insert(sq.result.clone());
                    } else if["IMM", "LOAD_STR", "NEW_ARR", "NEW_DICT", "CALL"].contains(&op) {
                        local_vars.insert(sq.result.clone());
                    } else if op == "PARAM" || op == "RET_VAL" || op == "JMP_F" {
                        local_vars.insert(sq.arg1.clone());
                    }
                    scan_pc += 1;
                }
                for f in formals { local_vars.insert(f.clone()); }
                local_vars.remove("-"); local_vars.remove("?");
                
                // 配置記憶體並初始化參數
                for var in &local_vars {
                    self.out.push_str(&format!("  %ptr_{} = alloca ptr\n", var));
                }
                for f in formals {
                    self.out.push_str(&format!("  store ptr %arg_{}, ptr %ptr_{}\n", f, f));
                }

                // 函數內部的翻譯狀態
                self.tmp_counter = 0;
                let mut param_stack = Vec::new();
                let mut last_was_term = false;

                pc += formals.len() + 1; // 跳過 FUNC_BEG 和 FORMAL 指令
                
                while pc < self.quads.len() {
                    let inner_q = &self.quads[pc].clone();
                    if inner_q.op == "FUNC_END" { break; }

                    // 如果上一行是終結指令 (如 JMP)，但這一行不是標籤，LLVM 會報錯。安插一個 Dummy 標籤。
                    if last_was_term && inner_q.op != "LABEL" {
                        let dummy = self.next_lbl();
                        self.out.push_str(&format!("{}:\n", dummy));
                        last_was_term = false;
                    }

                    match inner_q.op.as_str() {
                        "LABEL" => {
                            if !last_was_term { self.out.push_str(&format!("  br label %{}\n", inner_q.arg1)); }
                            self.out.push_str(&format!("{}:\n", inner_q.arg1));
                            last_was_term = false;
                        }
                        "IMM" => {
                            let tmp = self.next_tmp();
                            self.out.push_str(&format!("  {} = call ptr @rt_imm(i64 {})\n", tmp, inner_q.arg1));
                            self.out.push_str(&format!("  store ptr {}, ptr %ptr_{}\n", tmp, inner_q.result));
                        }
                        "LOAD_STR" => {
                            let tmp = self.next_tmp();
                            self.out.push_str(&format!("  {} = call ptr @rt_load_str(ptr @str.{})\n", tmp, inner_q.arg1));
                            self.out.push_str(&format!("  store ptr {}, ptr %ptr_{}\n", tmp, inner_q.result));
                        }
                        op if["ADD", "SUB", "MUL", "DIV", "CMP_EQ", "CMP_LT", "CMP_GT"].contains(&op) => {
                            let v1 = self.load_var(&inner_q.arg1);
                            let v2 = self.load_var(&inner_q.arg2);
                            let tmp = self.next_tmp();
                            let rt_func = format!("@rt_{}", op.to_lowercase());
                            self.out.push_str(&format!("  {} = call ptr {}(ptr {}, ptr {})\n", tmp, rt_func, v1, v2));
                            self.out.push_str(&format!("  store ptr {}, ptr %ptr_{}\n", tmp, inner_q.result));
                        }
                        "JMP" => {
                            self.out.push_str(&format!("  br label %{}\n", inner_q.result));
                            last_was_term = true;
                        }
                        "JMP_F" => {
                            let cond = self.load_var(&inner_q.arg1);
                            let is_true = self.next_tmp();
                            self.out.push_str(&format!("  {} = call i1 @rt_is_truthy(ptr {})\n", is_true, cond));
                            let next_lbl = self.next_lbl();
                            // 條件為 true 時繼續往下 (fallthrough)，false 時跳到標籤
                            self.out.push_str(&format!("  br i1 {}, label %{}, label %{}\n", is_true, next_lbl, inner_q.result));
                            self.out.push_str(&format!("{}:\n", next_lbl));
                            last_was_term = false;
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
                        "APPEND_ITEM" => {
                            let arr = self.load_var(&inner_q.arg1);
                            let val = self.load_var(&inner_q.result);
                            self.out.push_str(&format!("  call void @rt_append_item(ptr {}, ptr {})\n", arr, val));
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
                            param_stack.push(val); // 在編譯期記住這個參數
                        }
                        "CALL" => {
                            let p_count: usize = inner_q.arg2.parse().unwrap();
                            let mut args = Vec::new();
                            for _ in 0..p_count { args.push(param_stack.pop().unwrap()); }
                            args.reverse(); // 因為是 pop，順序要反轉
                            
                            let args_str = args.iter().map(|a| format!("ptr {}", a)).collect::<Vec<_>>().join(", ");
                            let tmp = self.next_tmp();
                            self.out.push_str(&format!("  {} = call ptr @{}({})\n", tmp, inner_q.arg1, args_str));
                            self.out.push_str(&format!("  store ptr {}, ptr %ptr_{}\n", tmp, inner_q.result));
                        }
                        "RET_VAL" => {
                            let val = self.load_var(&inner_q.arg1);
                            self.out.push_str(&format!("  ret ptr {}\n", val));
                            last_was_term = true;
                        }
                        _ => {}
                    }
                    pc += 1;
                }
                
                // 如果函數結尾沒有明確的 return，補上預設的 ret null
                if !last_was_term {
                    self.out.push_str("  ret ptr null\n");
                }
                self.out.push_str("}\n");
            }
            pc += 1;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("用法: {} <file.ir0> [file.ll]", args[0]);
        process::exit(1);
    }

    let input_file = &args[1];
    let output_file = if args.len() >= 3 {
        args[2].clone()
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

    println!("=== 開始轉換 LLVM IR ===");
    let mut generator = LLVMGenerator::new(quads, string_pool);
    generator.generate();

    let mut out_file = File::create(&output_file).expect("無法建立輸出檔案");
    out_file.write_all(generator.out.as_bytes()).expect("寫入失敗");
    
    println!("✅ LLVM IR 產生成功！已匯出至: {}", output_file);
    println!("(提示：接下來你需要一個 C Runtime 函式庫來編譯這個 .ll 檔案)");
}