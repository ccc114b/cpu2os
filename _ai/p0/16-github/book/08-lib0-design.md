# 08. `lib0` 設計原理（LLVM Runtime）

對應檔案：`lib0.rs`

## 職責

- 提供 LLVM 產生碼所需 API
- 實作動態值模型與容器行為
- 實作語言內建函式

## 值表示

`Value` enum：

- `Null`
- `Int(i64)`
- `Float(f64)`
- `String(String)`
- `Array(Vec<*mut Value>)`
- `Dict(HashMap<String, *mut Value>)`

透過 heap 配置 `*mut Value`，在 LLVM 端以 `ptr` 傳遞。

## Runtime API 類型

1. `rt_*`：IR 基礎操作
- `rt_imm`, `rt_add`, `rt_cmp_*`, `rt_get_item` ...

2. `p0_*`：語言內建 syscall
- `p0_len`, `p0_array`, `p0_push`, `p0_pop`, `p0_keys`, `p0_has_key`, `p0_remove`
- `p0_typeof`, `p0_int`, `p0_str`, `p0_ord`, `p0_chr`
- `p0_time`, `p0_random`, `p0_input`, `p0_exit`

3. 列印 API
- `rt_print_begin / rt_print_arg / rt_print_end`
- 用於支援 `print` 多參數輸出

## 設計考量

- syscall 採 `p0_*` 命名避免與系統庫撞名
- 以簡化與可讀性優先，非高效能 runtime
- 記憶體生命週期目前偏向教學模式（以可運作為優先）
