# 10. 程式碼導讀（逐檔）

本章提供維護時最常用的閱讀順序。

## `p0c.rs`

建議閱讀順序：

1. `TokenType` / `Token` / `Lexer`
2. `Parser::statement()`（語句分派）
3. `Parser::expression()`（運算與比較）
4. `Parser::emit()`（IR 輸出）
5. `main()`（CLI 與檔案輸出）

重點：

- 目前採「語法動作直接產生 IR」
- 迴圈控制 (`break/continue`) 透過 `loop_stack` 實現

## `vm0.rs`

建議閱讀順序：

1. `Value` 與 frame 結構
2. `system_call()`
3. `run()` 主迴圈

重點：

- 這是語言語意的第一個可執行參考
- 若語意爭議，通常先以 VM 行為做基準

## `ir0c.rs`

建議閱讀順序：

1. IR 讀取與 `Quad`
2. `LLVMGenerator::generate()` 大流程
3. `emit_block()`（指令 lowering 核心）
4. `main()`（`--target` 解析）

重點：

- `__init__` / `main` 入口策略
- `print` 與 syscall 的特殊轉譯

## `lib0.rs`

建議閱讀順序：

1. `Value` 與轉型輔助
2. `rt_*` 基礎 API
3. `p0_*` syscall
4. print 管線（begin/arg/end）

重點：

- 對應 `ir0c` 的函式宣告名稱必須一致
- 若發生 linker error，先檢查符號名與參數型別

## 腳本與樣例

- `build.sh`：重建所有工具
- `run.sh`：跑完整鏈（VM + LLVM）
- `p0/*.p0`：回歸測試與語法樣例
