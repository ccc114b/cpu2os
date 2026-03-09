use std::collections::HashMap;
use std::ffi::CStr;
use std::os::raw::c_char;

// 為了對接 C ABI (LLVM)，我們將 Array 和 Dict 內部也改為儲存指標 (*mut Value)
#[derive(Clone)]
pub enum Value {
    Null,
    Int(i64),
    Float(f64),
    String(String),
    Array(Vec<*mut Value>),
    Dict(HashMap<String, *mut Value>),
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

// 將 Value 配置到 Heap 上，並回傳 C 指標供 LLVM 使用
fn alloc_value(v: Value) -> *mut Value {
    Box::into_raw(Box::new(v))
}

// 將 C 指標轉回 Rust 的可變參考 (略過 null 防呆)
unsafe fn deref_val<'a>(ptr: *mut Value) -> &'a mut Value {
    if ptr.is_null() {
        Box::leak(Box::new(Value::Null))
    } else {
        &mut *ptr
    }
}

// ==========================================
// Runtime API (供 LLVM IR 呼叫，必須使用 #[no_mangle] 和 extern "C")
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

// 系統呼叫：Print (為了簡化，先實作單參數 print，C ABI 允許忽略多餘參數)
#[no_mangle]
pub unsafe extern "C" fn print(v: *mut Value) -> *mut Value {
    if !v.is_null() {
        println!("[程式輸出] >> {}", deref_val(v).to_string());
    } else {
        println!("[程式輸出] >> null");
    }
    alloc_value(Value::Int(0))
}