# 05. `p0c` 設計原理（前端）

對應檔案：`p0c.rs`

## 職責

- 詞法分析：把字串切成 token
- 語法分析：遞迴下降 parser
- 語意動作：直接產生四元組 IR
- 輸出：寫成 `.ir0`

## 主要資料結構

- `TokenType`：關鍵字、符號、字面值
- `Token`：`t_type/text/pos`
- `Parser`：
  - `quads`: 四元組序列
  - `string_pool`: 字串池
  - `loop_stack`: break/continue 目標管理

## 關鍵方法

- `Lexer::next_token()`
  - 跳過空白與註解
  - 辨識字串、數字、識別字、運算子
- `Parser::expression()`
  - 處理算術與比較
- `Parser::statement()`
  - 分派 `if/while/for/break/continue/return/assign/call`
- `Parser::emit()`
  - 追加四元組（`-d` 時印到終端）

## 迴圈 lowering

- `while`：
  - `LABEL start` → `JMP_F cond end` → body → `JMP start` → `LABEL end`
- `for`：
  - init
  - `LABEL cond` + 條件檢查
  - `LABEL update` + 更新
  - `LABEL body` + 內容
  - `continue` 跳 `update`，`break` 跳 `end`

## CLI

```bash
./p0c [-d] <source_file.p0> [output_file.ir0]
```

- 預設不印 IR 到畫面
- `-d` 顯示四元組除錯輸出
