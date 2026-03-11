# 01. 專案總覽

`p0compiler` 是一個以 Rust 撰寫的教學型編譯器專案，目標語言是動態語言 `p0`。

核心流程是：

1. `p0c`：`.p0` → `.ir0`（四元組 IR）
2. `vm0`：直接執行 `.ir0`
3. `ir0c`：`.ir0` → LLVM IR (`.ll`)
4. `lib0.a`：LLVM runtime 函式庫
5. `clang`：`.ll + lib0.a` 連結成原生執行檔

這個設計讓你可以同時驗證兩條路徑：

- 解譯路徑：`p0c -> vm0`
- 編譯路徑：`p0c -> ir0c -> clang`

若兩條路徑行為一致，通常代表語言語意與後端轉譯一致。

## 主要檔案

- `p0c.rs`：詞法分析、語法分析、IR 產生
- `vm0.rs`：IR 虛擬機
- `ir0c.rs`：IR 轉 LLVM
- `lib0.rs`：runtime
- `p0/`：範例程式
- `build.sh`、`run.sh`：建置與一鍵執行腳本
