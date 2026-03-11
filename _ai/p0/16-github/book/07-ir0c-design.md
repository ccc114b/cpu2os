# 07. `ir0c` 設計原理（IR → LLVM）

對應檔案：`ir0c.rs`

## 職責

- 讀入 `.ir0`
- 產生 LLVM IR (`.ll`)
- 處理函式、頂層語句、字串池、外部函式宣告

## 轉譯策略

- 每個 p0 變數先配置 `alloca ptr`
- 運算以 runtime 函式呼叫完成（例如 `rt_add`, `rt_cmp_lt`）
- `PARAM/CALL` 由編譯期 `param_stack` 重建呼叫參數順序

## 函式與入口

- 使用者函式：`define ptr @<name>(...)`
- 頂層語句：自動包成 `define ptr @__init__()`
- `--target bin`（預設）：額外生成 `define i32 @main()` 呼叫 `@__init__`
- `--target lib`：不生成 `@main`

## 名稱與衝突處理

- 保留名稱：`__init__`
- 若使用者定義 `main`，轉譯時改名為 `__p0_main` 避免系統入口衝突

## syscall 對應

語言內建呼叫會轉到 runtime 的 `p0_*` 符號（例如 `exit -> p0_exit`），避免與 libc 同名函式衝突。

## CLI

```bash
./ir0c [--target bin|lib] <file.ir0> [file.ll]
```
