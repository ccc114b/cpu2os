use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::CStr;
use std::io::{self, Write};
use std::os::raw::c_char;
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

// 為了對接 C ABI (LLVM)，我們將 Array 和 Dict 內部也改為儲存指標 (*mut Value)
// 這使得我們能在編譯後的 Native 程式碼 (LLVM 生成) 中自由傳遞動態型別變數的指標
#[derive(Clone)]
pub enum Value {
    Null,
    Int(i64),
    Float(f64),
    String(String),
    Array(Vec<*mut Value>),           // 陣列：裝載其他 Value 的 C 指標
    Dict(HashMap<String, *mut Value>),// 字典：字串到 Value C 指標的映射表
}

impl Value {
    fn to_int(&self) -> i64 {
        match self {
            Value::Int(n) => *n,
            Value::Float(f) => *f as i64,
            Value::String(s) => s.parse().unwrap_or(0),
            _ => 0,
        }
    }

    /// 判斷目前 Value 在邏輯上是否為真 (truthy)
    fn is_truthy(&self) -> bool {
        match self {
            Value::Null => false,
            Value::Int(n) => *n != 0,
            Value::Float(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Array(a) => !a.is_empty(),
            Value::Dict(d) => !d.is_empty(),
        }
    }

    fn to_string(&self) -> String {
        match self {
            Value::Null => "null".to_string(),
            Value::Int(n) => n.to_string(),
            Value::Float(n) => n.to_string(),
            Value::String(s) => s.clone(),
            Value::Array(arr) => {
                let strs: Vec<String> = arr.iter().map(|&v| unsafe {
                    if v.is_null() { "null".to_string() } else { (*v).to_string() }
                }).collect();
                format!("[{}]", strs.join(", "))
            }
            Value::Dict(dict) => {
                let strs: Vec<String> = dict.iter().map(|(k, &v)| unsafe {
                    let v_str = if v.is_null() { "null".to_string() } else { (*v).to_string() };
                    format!("'{}': {}", k, v_str)
                }).collect();
                format!("{{{}}}", strs.join(", "))
            }
        }
    }
}

// ==========================================
// 記憶體管理輔助函數
// ==========================================

// 將 Value 配置到 Heap (堆積) 上，並回傳 C 格式的裸指標 (Raw Pointer) 供 LLVM 使用
// 由於 Box::into_raw 會放棄 Rust 的記憶體所有權，該變數的生命週期將由外部（此處即編譯後的執行檔）決定
fn alloc_value(v: Value) -> *mut Value {
    Box::into_raw(Box::new(v))
}

// 將 C 裸指標轉回 Rust 的可變參考 (Mutable Reference)
// 若指標為 null，則安全地回傳一個預設 Null 值的靜態參照以防止 Segmentation Fault
unsafe fn deref_val<'a>(ptr: *mut Value) -> &'a mut Value {
    if ptr.is_null() {
        Box::leak(Box::new(Value::Null))
    } else {
        &mut *ptr
    }
}

thread_local! {
    static PRINT_BUF: RefCell<Vec<String>> = RefCell::new(Vec::new());
}

// ==========================================
// Runtime API (供 LLVM IR 呼叫，必須使用 #[no_mangle] 和 extern "C")
// 這些函數不僅在 p0 中呼叫，最終也會被連結 (link) 到 LLVM 生成的可執行檔中
// ==========================================

#[no_mangle]
pub extern "C" fn rt_imm(val: i64) -> *mut Value {
    alloc_value(Value::Int(val))
}

#[no_mangle]
pub unsafe extern "C" fn rt_load_str(s: *const c_char) -> *mut Value {
    let c_str = CStr::from_ptr(s);
    alloc_value(Value::String(c_str.to_string_lossy().into_owned()))
}

#[no_mangle]
pub unsafe extern "C" fn rt_add(v1: *mut Value, v2: *mut Value) -> *mut Value {
    // 解離指標取得實體，呼叫 to_int 處理整數計算，最後再包裹回 C 指標
    alloc_value(Value::Int(deref_val(v1).to_int() + deref_val(v2).to_int()))
}

#[no_mangle]
pub unsafe extern "C" fn rt_sub(v1: *mut Value, v2: *mut Value) -> *mut Value {
    alloc_value(Value::Int(deref_val(v1).to_int() - deref_val(v2).to_int()))
}

#[no_mangle]
pub unsafe extern "C" fn rt_mul(v1: *mut Value, v2: *mut Value) -> *mut Value {
    alloc_value(Value::Int(deref_val(v1).to_int() * deref_val(v2).to_int()))
}

#[no_mangle]
pub unsafe extern "C" fn rt_div(v1: *mut Value, v2: *mut Value) -> *mut Value {
    let num = deref_val(v1).to_int();
    let den = deref_val(v2).to_int();
    alloc_value(Value::Int(if den != 0 { num / den } else { 0 }))
}

#[no_mangle]
pub unsafe extern "C" fn rt_cmp_eq(v1: *mut Value, v2: *mut Value) -> *mut Value {
    let res = if deref_val(v1).to_int() == deref_val(v2).to_int() { 1 } else { 0 };
    alloc_value(Value::Int(res))
}

#[no_mangle]
pub unsafe extern "C" fn rt_cmp_lt(v1: *mut Value, v2: *mut Value) -> *mut Value {
    let res = if deref_val(v1).to_int() < deref_val(v2).to_int() { 1 } else { 0 };
    alloc_value(Value::Int(res))
}

#[no_mangle]
pub unsafe extern "C" fn rt_cmp_gt(v1: *mut Value, v2: *mut Value) -> *mut Value {
    let res = if deref_val(v1).to_int() > deref_val(v2).to_int() { 1 } else { 0 };
    alloc_value(Value::Int(res))
}

#[no_mangle]
pub extern "C" fn rt_new_arr() -> *mut Value {
    // 創建一個空的內部指標陣列
    alloc_value(Value::Array(Vec::new()))
}

#[no_mangle]
pub extern "C" fn rt_new_dict() -> *mut Value {
    alloc_value(Value::Dict(HashMap::new()))
}

#[no_mangle]
pub unsafe extern "C" fn rt_init_arr(val: *mut Value, size: *mut Value) -> *mut Value {
    let s = deref_val(size).to_int() as usize;
    let mut arr = Vec::with_capacity(s);
    let base_val = deref_val(val).clone();
    for _ in 0..s {
        arr.push(alloc_value(base_val.clone()));
    }
    alloc_value(Value::Array(arr))
}

#[no_mangle]
pub unsafe extern "C" fn rt_append_item(arr: *mut Value, val: *mut Value) {
    // 針對特定的 Array 指標推進新的資料
    if let Value::Array(a) = deref_val(arr) {
        a.push(val);
    }
}

#[no_mangle]
pub unsafe extern "C" fn rt_set_item(obj: *mut Value, key: *mut Value, val: *mut Value) {
    match deref_val(obj) {
        Value::Array(a) => {
            let idx = deref_val(key).to_int() as usize;
            if idx < a.len() { a[idx] = val; }
        }
        Value::Dict(d) => {
            let k_str = deref_val(key).to_string();
            d.insert(k_str, val);
        }
        _ => {}
    }
}

#[no_mangle]
pub unsafe extern "C" fn rt_get_item(obj: *mut Value, key: *mut Value) -> *mut Value {
    match deref_val(obj) {
        Value::Array(a) => {
            let idx = deref_val(key).to_int() as usize;
            if idx < a.len() { return a[idx]; }
        }
        Value::Dict(d) => {
            let k_str = deref_val(key).to_string();
            if let Some(&v) = d.get(&k_str) { return v; }
        }
        _ => {}
    }
    alloc_value(Value::Null)
}

#[no_mangle]
pub unsafe extern "C" fn rt_is_truthy(v: *mut Value) -> bool {
    deref_val(v).is_truthy()
}

#[no_mangle]
pub unsafe extern "C" fn p0_len(v: *mut Value) -> *mut Value {
    let n = match deref_val(v) {
        Value::Array(arr) => arr.len(),
        Value::Dict(dict) => dict.len(),
        Value::String(s) => s.len(),
        _ => 0,
    };
    alloc_value(Value::Int(n as i64))
}

#[no_mangle]
pub unsafe extern "C" fn p0_array(len_v: *mut Value, default_v: *mut Value) -> *mut Value {
    let len = deref_val(len_v).to_int().max(0) as usize;
    let default_val = deref_val(default_v).clone();
    let mut arr = Vec::with_capacity(len);
    for _ in 0..len {
        arr.push(alloc_value(default_val.clone()));
    }
    alloc_value(Value::Array(arr))
}

#[no_mangle]
pub unsafe extern "C" fn p0_push(arr_v: *mut Value, item_v: *mut Value) -> *mut Value {
    if let Value::Array(arr) = deref_val(arr_v) {
        arr.push(item_v);
    }
    alloc_value(Value::Null)
}

#[no_mangle]
pub unsafe extern "C" fn p0_pop(arr_v: *mut Value) -> *mut Value {
    if let Value::Array(arr) = deref_val(arr_v) {
        if let Some(v) = arr.pop() {
            return v;
        }
    }
    alloc_value(Value::Null)
}

#[no_mangle]
pub unsafe extern "C" fn p0_keys(dict_v: *mut Value) -> *mut Value {
    if let Value::Dict(dict) = deref_val(dict_v) {
        let mut out = Vec::with_capacity(dict.len());
        for k in dict.keys() {
            out.push(alloc_value(Value::String(k.clone())));
        }
        return alloc_value(Value::Array(out));
    }
    alloc_value(Value::Null)
}

#[no_mangle]
pub unsafe extern "C" fn p0_has_key(dict_v: *mut Value, key_v: *mut Value) -> *mut Value {
    if let Value::Dict(dict) = deref_val(dict_v) {
        let has = dict.contains_key(&deref_val(key_v).to_string());
        return alloc_value(Value::Int(if has { 1 } else { 0 }));
    }
    alloc_value(Value::Int(0))
}

#[no_mangle]
pub unsafe extern "C" fn p0_remove(dict_v: *mut Value, key_v: *mut Value) -> *mut Value {
    if let Value::Dict(dict) = deref_val(dict_v) {
        dict.remove(&deref_val(key_v).to_string());
    }
    alloc_value(Value::Null)
}

#[no_mangle]
pub unsafe extern "C" fn p0_typeof(v: *mut Value) -> *mut Value {
    let t = match deref_val(v) {
        Value::Null => "null",
        Value::Int(_) => "int",
        Value::Float(_) => "float",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Dict(_) => "dict",
    };
    alloc_value(Value::String(t.to_string()))
}

#[no_mangle]
pub unsafe extern "C" fn p0_int(v: *mut Value) -> *mut Value {
    alloc_value(Value::Int(deref_val(v).to_int()))
}

#[no_mangle]
pub unsafe extern "C" fn p0_str(v: *mut Value) -> *mut Value {
    alloc_value(Value::String(deref_val(v).to_string()))
}

#[no_mangle]
pub unsafe extern "C" fn p0_ord(v: *mut Value) -> *mut Value {
    if let Value::String(s) = deref_val(v) {
        if let Some(c) = s.chars().next() {
            return alloc_value(Value::Int(c as u32 as i64));
        }
    }
    alloc_value(Value::Int(0))
}

#[no_mangle]
pub unsafe extern "C" fn p0_chr(v: *mut Value) -> *mut Value {
    let code = deref_val(v).to_int() as u32;
    if let Some(c) = char::from_u32(code) {
        return alloc_value(Value::String(c.to_string()));
    }
    alloc_value(Value::String(String::new()))
}

#[no_mangle]
pub unsafe extern "C" fn p0_time() -> *mut Value {
    let t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64();
    alloc_value(Value::Float(t))
}

#[no_mangle]
pub unsafe extern "C" fn p0_random() -> *mut Value {
    let t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let r = (t % 1000) as f64 / 1000.0;
    alloc_value(Value::Float(r))
}

#[no_mangle]
pub unsafe extern "C" fn p0_input(prompt: *mut Value) -> *mut Value {
    if !prompt.is_null() {
        print!("{}", deref_val(prompt).to_string());
        io::stdout().flush().ok();
    }
    let mut line = String::new();
    if io::stdin().read_line(&mut line).is_err() {
        return alloc_value(Value::String(String::new()));
    }
    alloc_value(Value::String(line.trim_end().to_string()))
}

#[no_mangle]
pub unsafe extern "C" fn p0_exit(code: *mut Value) -> *mut Value {
    let c = if code.is_null() { 0 } else { deref_val(code).to_int() as i32 };
    process::exit(c);
}

#[no_mangle]
pub extern "C" fn rt_print_begin() {
    PRINT_BUF.with(|buf| buf.borrow_mut().clear());
}

#[no_mangle]
pub unsafe extern "C" fn rt_print_arg(v: *mut Value) {
    let out = if v.is_null() { "null".to_string() } else { deref_val(v).to_string() };
    PRINT_BUF.with(|buf| buf.borrow_mut().push(out));
}

#[no_mangle]
pub extern "C" fn rt_print_end() -> *mut Value {
    // 收尾工作：將累積的所有字串透過空白連接，並一次印出附帶換行
    PRINT_BUF.with(|buf| println!("{}", buf.borrow().join(" ")));
    alloc_value(Value::Int(0))
}

#[no_mangle]
pub unsafe extern "C" fn print(v: *mut Value) -> *mut Value {
    // 單一參數的基礎列印函數
    rt_print_begin();
    rt_print_arg(v);
    rt_print_end()
}
