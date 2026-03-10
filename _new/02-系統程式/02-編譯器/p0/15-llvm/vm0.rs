use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fs;
use std::io::{self, Write};
use std::process;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

// =========================================================
// 1. 核心資料結構與動態型別 (Value)
// =========================================================

#[derive(Clone)]
pub enum Value {
    Null,
    Int(i64),
    Float(f64),
    String(String),
    Array(Rc<RefCell<Vec<Value>>>),
    Dict(Rc<RefCell<HashMap<String, Value>>>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Int(n) => write!(f, "{}", n),
            Value::Float(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Array(arr) => {
                let vec = arr.borrow();
                let strs: Vec<String> = vec.iter().map(|v| v.to_string()).collect();
                write!(f, "[{}]", strs.join(", "))
            }
            Value::Dict(dict) => {
                let map = dict.borrow();
                let strs: Vec<String> = map.iter().map(|(k, v)| format!("'{}': {}", k, v)).collect();
                write!(f, "{{{}}}", strs.join(", "))
            }
        }
    }
}

impl Value {
    fn is_truthy(&self) -> bool {
        match self {
            Value::Null => false,
            Value::Int(n) => *n != 0,
            Value::Float(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Array(arr) => !arr.borrow().is_empty(),
            Value::Dict(dict) => !dict.borrow().is_empty(),
        }
    }

    fn to_int(&self) -> i64 {
        match self {
            Value::Int(n) => *n,
            Value::Float(f) => *f as i64,
            Value::String(s) => s.parse().unwrap_or(0),
            _ => 0,
        }
    }
}

// =========================================================
// 2. 虛擬機核心 (Virtual Machine)
// =========================================================

#[derive(Clone)]
pub struct Quad {
    pub op: String,
    pub arg1: String,
    pub arg2: String,
    pub result: String,
}

struct Frame {
    vars: HashMap<String, Value>,
    ret_pc: usize,
    ret_var: String,
    incoming_args: Vec<Value>,
    formal_idx: usize,
}

pub struct VM {
    quads: Vec<Quad>,
    string_pool: Vec<String>,
    stack: Vec<Frame>,
    label_map: HashMap<String, usize>, // 用於儲存標籤與 PC 行號的對應
    func_map: HashMap<String, usize>,  // 用於儲存函數與 PC 行號的對應
}

impl VM {
    pub fn new(quads: Vec<Quad>, string_pool: Vec<String>) -> Self {
        let mut vm = VM {
            quads,
            string_pool,
            stack: vec![Frame {
                vars: HashMap::new(),
                ret_pc: 0,
                ret_var: String::new(),
                incoming_args: vec![],
                formal_idx: 0,
            }],
            label_map: HashMap::new(),
            func_map: HashMap::new(),
        };
        vm.build_maps();
        vm
    }

    // 掃描一次 Quad，將所有的函數與標籤建立索引對應表
    fn build_maps(&mut self) {
        for (i, q) in self.quads.iter().enumerate() {
            if q.op == "FUNC_BEG" {
                // 函數進入點是 FUNC_BEG 的下一行
                self.func_map.insert(q.arg1.clone(), i + 1);
            } else if q.op == "LABEL" {
                // 標籤指向目前行
                self.label_map.insert(q.arg1.clone(), i);
            }
        }
    }

    fn get_var(&self, name: &str) -> Value {
        if name == "-" { return Value::Int(0); }
        if let Ok(n) = name.parse::<i64>() { return Value::Int(n); }
        self.stack.last().unwrap().vars.get(name).cloned().unwrap_or(Value::Null)
    }

    fn set_var(&mut self, name: &str, val: Value) {
        if name == "-" || name == "?" { return; }
        self.stack.last_mut().unwrap().vars.insert(name.to_string(), val);
    }

    fn system_call(&mut self, f_name: &str, args: &mut Vec<Value>) -> Option<Value> {
        match f_name {
            "print" => {
                let out: Vec<String> = args.iter().map(|v| v.to_string()).collect();
                println!("{}", out.join(" "));
                Some(Value::Int(0))
            }
            "len" => {
                let len = match &args[0] {
                    Value::Array(arr) => arr.borrow().len(),
                    Value::Dict(dict) => dict.borrow().len(),
                    Value::String(s) => s.len(),
                    _ => 0,
                };
                Some(Value::Int(len as i64))
            }
            "time" => {
                let t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64();
                Some(Value::Float(t))
            }
            "array" => {
                let len = args[0].to_int() as usize;
                let default_val = args[1].clone();
                let arr = vec![default_val; len];
                Some(Value::Array(Rc::new(RefCell::new(arr))))
            }
            "push" => {
                if let Value::Array(arr) = &args[0] {
                    arr.borrow_mut().push(args[1].clone());
                }
                Some(Value::Null)
            }
            "pop" => {
                if let Value::Array(arr) = &args[0] {
                    let val = arr.borrow_mut().pop().unwrap_or(Value::Null);
                    return Some(val);
                }
                Some(Value::Null)
            }
            "keys" => {
                if let Value::Dict(dict) = &args[0] {
                    let keys: Vec<Value> = dict.borrow().keys().map(|k| Value::String(k.clone())).collect();
                    return Some(Value::Array(Rc::new(RefCell::new(keys))));
                }
                Some(Value::Null)
            }
            "has_key" => {
                if let Value::Dict(dict) = &args[0] {
                    let has = dict.borrow().contains_key(&args[1].to_string());
                    return Some(Value::Int(if has { 1 } else { 0 }));
                }
                Some(Value::Int(0))
            }
            "remove" => {
                if let Value::Dict(dict) = &args[0] {
                    dict.borrow_mut().remove(&args[1].to_string());
                }
                Some(Value::Null)
            }
            "typeof" => {
                let t_str = match &args[0] {
                    Value::Null => "null",
                    Value::Int(_) => "int",
                    Value::Float(_) => "float",
                    Value::String(_) => "string",
                    Value::Array(_) => "array",
                    Value::Dict(_) => "dict",
                };
                Some(Value::String(t_str.to_string()))
            }
            "int" => Some(Value::Int(args[0].to_int())),
            "str" => Some(Value::String(args[0].to_string())),
            "ord" => {
                if let Value::String(s) = &args[0] {
                    if let Some(c) = s.chars().next() { return Some(Value::Int(c as u32 as i64)); }
                }
                Some(Value::Int(0))
            }
            "chr" => {
                let code = args[0].to_int() as u32;
                if let Some(c) = std::char::from_u32(code) { return Some(Value::String(c.to_string())); }
                Some(Value::String(String::new()))
            }
            "random" => {
                let t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
                let r = (t % 1000) as f64 / 1000.0;
                Some(Value::Float(r))
            }
            "input" => {
                if !args.is_empty() {
                    print!("{}", args[0].to_string());
                    io::stdout().flush().unwrap();
                }
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                Some(Value::String(input.trim_end().to_string()))
            }
            "exit" => {
                let code = if !args.is_empty() { args[0].to_int() as i32 } else { 0 };
                process::exit(code);
            }
            _ => None,
        }
    }

    pub fn run(&mut self) {
        let mut pc = 0;
        let mut param_stack: Vec<Value> = Vec::new();

        // println!("\n=== VM 執行開始 ===");
        while pc < self.quads.len() {
            let q = self.quads[pc].clone();

            match q.op.as_str() {
                "FUNC_BEG" => {
                    // 主程式如果碰到函數定義，直接跳過函數本體，不執行
                    while self.quads[pc].op != "FUNC_END" { pc += 1; }
                }
                "LABEL" => {
                    // 標籤僅作為跳轉錨點，執行時不做任何事
                }
                "IMM" => {
                    let val = q.arg1.parse().unwrap();
                    self.set_var(&q.result, Value::Int(val));
                }
                "LOAD_STR" => {
                    let idx: usize = q.arg1.parse().unwrap();
                    let s = self.string_pool[idx].clone();
                    self.set_var(&q.result, Value::String(s));
                }
                "ADD" => {
                    let v = Value::Int(self.get_var(&q.arg1).to_int() + self.get_var(&q.arg2).to_int());
                    self.set_var(&q.result, v);
                }
                "SUB" => {
                    let v = Value::Int(self.get_var(&q.arg1).to_int() - self.get_var(&q.arg2).to_int());
                    self.set_var(&q.result, v);
                }
                "MUL" => {
                    let v = Value::Int(self.get_var(&q.arg1).to_int() * self.get_var(&q.arg2).to_int());
                    self.set_var(&q.result, v);
                }
                "DIV" => {
                    let l = self.get_var(&q.arg1).to_int();
                    let r = self.get_var(&q.arg2).to_int();
                    let res = if r != 0 { l / r } else { 0 };
                    self.set_var(&q.result, Value::Int(res));
                }
                "CMP_EQ" => {
                    let res = if self.get_var(&q.arg1).to_int() == self.get_var(&q.arg2).to_int() { 1 } else { 0 };
                    self.set_var(&q.result, Value::Int(res));
                }
                "CMP_LT" => {
                    let res = if self.get_var(&q.arg1).to_int() < self.get_var(&q.arg2).to_int() { 1 } else { 0 };
                    self.set_var(&q.result, Value::Int(res));
                }
                "CMP_GT" => {
                    let res = if self.get_var(&q.arg1).to_int() > self.get_var(&q.arg2).to_int() { 1 } else { 0 };
                    self.set_var(&q.result, Value::Int(res));
                }
                "JMP" => {
                    // 尋找標籤對應的行號
                    if let Some(&target_pc) = self.label_map.get(&q.result) {
                        pc = target_pc;
                    } else {
                        panic!("找不到標籤: {}", q.result);
                    }
                }
                "JMP_F" => {
                    if !self.get_var(&q.arg1).is_truthy() {
                        if let Some(&target_pc) = self.label_map.get(&q.result) {
                            pc = target_pc;
                        } else {
                            panic!("找不到標籤: {}", q.result);
                        }
                    }
                }
                "NEW_ARR" => self.set_var(&q.result, Value::Array(Rc::new(RefCell::new(Vec::new())))),
                "NEW_DICT" => self.set_var(&q.result, Value::Dict(Rc::new(RefCell::new(HashMap::new())))),
                "INIT_ARR" => {
                    let val = self.get_var(&q.arg1);
                    let size = self.get_var(&q.arg2).to_int() as usize;
                    let arr = vec![val; size];
                    self.set_var(&q.result, Value::Array(Rc::new(RefCell::new(arr))));
                }
                "APPEND_ITEM" => {
                    let arr = self.get_var(&q.arg1);
                    let val = self.get_var(&q.result);
                    if let Value::Array(a) = arr {
                        a.borrow_mut().push(val);
                    }
                }
                "SET_ITEM" => {
                    let obj = self.get_var(&q.arg1);
                    let key = self.get_var(&q.arg2);
                    let val = self.get_var(&q.result);
                    match obj {
                        Value::Array(arr) => arr.borrow_mut()[key.to_int() as usize] = val,
                        Value::Dict(dict) => { dict.borrow_mut().insert(key.to_string(), val); },
                        _ => panic!("無法設定屬性"),
                    }
                }
                "GET_ITEM" => {
                    let obj = self.get_var(&q.arg1);
                    let key = self.get_var(&q.arg2);
                    let res = match obj {
                        Value::Array(arr) => arr.borrow()[key.to_int() as usize].clone(),
                        Value::Dict(dict) => dict.borrow().get(&key.to_string()).cloned().unwrap_or(Value::Null),
                        _ => Value::Null,
                    };
                    self.set_var(&q.result, res);
                }
                "STORE" => {
                    let val = self.get_var(&q.arg1);
                    self.set_var(&q.result, val);
                }
                "PARAM" => param_stack.push(self.get_var(&q.arg1)),
                "CALL" => {
                    let p_count: usize = q.arg2.parse().unwrap();
                    
                    let mut f_name = q.arg1.clone();
                    if let Value::String(s) = self.get_var(&q.arg1) {
                        f_name = s;
                    }

                    let mut args = if p_count > 0 {
                        let split_idx = param_stack.len() - p_count;
                        param_stack.split_off(split_idx)
                    } else { vec![] };

                    if let Some(ret_val) = self.system_call(&f_name, &mut args) {
                        self.set_var(&q.result, ret_val);
                        pc += 1;
                        continue;
                    }

                    let target_pc = *self.func_map.get(&f_name).expect(&format!("找不到自訂函數: {}", f_name));
                    self.stack.push(Frame { vars: HashMap::new(), ret_pc: pc + 1, ret_var: q.result.clone(), incoming_args: args, formal_idx: 0 });
                    pc = target_pc;
                    continue;
                }
                "FORMAL" => {
                    let frame = self.stack.last_mut().unwrap();
                    let arg_val = frame.incoming_args[frame.formal_idx].clone();
                    frame.vars.insert(q.arg1.clone(), arg_val);
                    frame.formal_idx += 1;
                }
                "RET_VAL" => {
                    let ret_val = self.get_var(&q.arg1);
                    let frame = self.stack.pop().unwrap();
                    self.set_var(&frame.ret_var, ret_val);
                    pc = frame.ret_pc;
                    continue;
                }
                "FUNC_END" => {
                    if self.stack.len() > 1 {
                        let frame = self.stack.pop().unwrap();
                        self.set_var(&frame.ret_var, Value::Null);
                        pc = frame.ret_pc;
                        continue;
                    }
                }
                _ => {} // 安全忽略
            }
            pc += 1;
        }
        // println!("=== VM 執行完畢 ===");
    }
}

// =========================================================
// 3. 檔案解析與啟動
// =========================================================

// 將 {:?} 格式的字串還原為正常字串（例如將 \" 轉回 " ， \n 轉回換行）
fn parse_debug_str(s: &str) -> String {
    if s.len() < 2 { return String::new(); }
    let mut res = String::new();
    let inner = &s[1..s.len() - 1]; // 去除外層的雙引號
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
        } else {
            res.push(c);
        }
    }
    res
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("用法: {} <file.ir0>", args[0]);
        process::exit(1);
    }

    let ir_content = fs::read_to_string(&args[1]).expect("無法開啟 IR 檔案");
    
    let mut string_pool = Vec::new();
    let mut quads = Vec::new();
    let mut state = 0; // 0: 等待, 1: 讀取字串池, 2: 讀取 Quad

    for line in ir_content.lines() {
        let trimmed = line.trim_end();
        if trimmed.is_empty() { continue; }

        if trimmed == "===STRINGS===" {
            state = 1;
            continue;
        } else if trimmed == "===QUADS===" {
            state = 2;
            continue;
        }

        if state == 1 {
            string_pool.push(parse_debug_str(trimmed));
        } else if state == 2 {
            let parts: Vec<&str> = trimmed.split('\t').collect();
            if parts.len() >= 4 {
                quads.push(Quad {
                    op: parts[0].to_string(),
                    arg1: parts[1].to_string(),
                    arg2: parts[2].to_string(),
                    result: parts[3].to_string(),
                });
            }
        }
    }

    // 啟動 VM
    let mut vm = VM::new(quads, string_pool);
    vm.run();
}