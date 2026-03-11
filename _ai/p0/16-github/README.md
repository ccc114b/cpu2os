# p0compiler

這個專案是一個以 Rust 實作的動態語言 **p0** 編譯器實驗

目前流程包含：

1. `p0c`：將 `.p0` 原始碼編譯成 `.ir0`（四元組 IR）
2. `vm0`：直接解譯執行 `.ir0`
3. `ir0c`：把 `.ir0` 轉成 LLVM IR (`.ll`)
4. `lib0.a`：提供 LLVM 執行時所需 runtime 函式
5. `clang`：把 `.ll + lib0.a` 連結成可執行檔

如果要更詳細了解，請參考 [p0 編譯工具的設計原理](book/README.md) 。

---

## 目錄重點

- `p0c.rs`：p0 前端（Lexer/Parser/IR 產生）
- `vm0.rs`：IR 虛擬機
- `ir0c.rs`：IR -> LLVM IR 轉換器
- `lib0.rs`：runtime（數值/容器/syscall/print 等）
- `p0/`：範例程式（`fact.p0`, `loop.p0`, `syscall.p0`...）
- `build.sh`：編譯工具鏈
- `run.sh`：一鍵跑完整流程

---

## 環境需求

- Rust 工具鏈（`rustc`）
- LLVM/Clang（`clang`）
- macOS/Linux shell 環境

---

## 快速開始

### 1) 建置工具

```bash
./build.sh
```

### 2) 一鍵執行範例

> 注意：`run.sh` 參數是「不含副檔名」的路徑

```bash
./run.sh p0/fact
```

`run.sh` 內容等同：

1. `./p0c p0/fact.p0`
2. `./vm0 p0/fact.ir0`
3. `./ir0c p0/fact.ir0`
4. `clang -Wno-override-module p0/fact.ll lib0.a -o p0/fact`
5. `./p0/fact`

---

## 單步命令

### p0c（來源碼 -> IR）

```bash
./p0c [-d] <source_file.p0> [output_file.ir0]
```

- 預設：只輸出 `.ir0` 到檔案，不把四元組列到終端
- `-d`：額外把 IR 四元組逐行印到畫面（除錯用）

範例：

```bash
./p0c p0/fact.p0
./p0c -d p0/fact.p0
```

### vm0（執行 IR）

```bash
./vm0 <file.ir0>
```

### ir0c（IR -> LLVM IR）

```bash
./ir0c [--target bin|lib] <file.ir0> [file.ll]
```

- `--target` 預設為 `bin`
- `bin`：產生 `@__init__` + `@main`
- `lib`：只產生 `@__init__`（不產生 `@main`）

---

## 語言與執行時說明（目前）

- 已支援：
  - `if / else`
  - `while`
  - `for (init; cond; update)`
  - `break / continue`
  - 函式呼叫與回傳
  - 陣列、字典、索引存取
- 內建 runtime 函式（LLVM 路徑由 `lib0` 提供）：
  - `print`, `len`, `array`, `push`, `pop`, `keys`, `has_key`, `remove`
  - `typeof`, `int`, `str`, `ord`, `chr`
  - `time`, `random`, `input`, `exit`

---

## 常見問題

### 1) 為什麼 `run.sh` 看起來程式跑了兩次？

因為 `run.sh` 會先跑 `vm0`，再編譯並執行 LLVM 產生的最終執行檔。

### 2) `input` 在 `run.sh` 下第二次讀不到輸入？

若你用 pipe 只餵一次輸入，通常會被前面的 `vm0` 先消耗；後面的 LLVM 執行檔就會讀到 EOF。

---

## 開發建議

- 修改 `p0c.rs` 後先跑：

```bash
./build.sh
./p0c -d p0/loop.p0
./vm0 p0/loop.ir0
```

- 修改 `ir0c.rs` / `lib0.rs` 後再跑完整鏈：

```bash
./build.sh
./run.sh p0/loop
```

