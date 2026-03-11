# 04. IR 規格（`.ir0`）

`.ir0` 是文字格式中間碼，分兩段：

1. `===STRINGS===`：字串池（逐行）
2. `===QUADS===`：四元組指令（tab 分隔）

## 四元組格式

每行格式：

```text
OP\tARG1\tARG2\tRESULT
```

範例：

```text
IMM	5	-	t1
PARAM	t1	-	-
CALL	fact	1	t2
```

## 常見指令

- 流程控制：
  - `LABEL`, `JMP`, `JMP_F`
  - `FUNC_BEG`, `FORMAL`, `FUNC_END`, `RET_VAL`
- 常數與載入：
  - `IMM`, `LOAD_STR`
- 算術與比較：
  - `ADD`, `SUB`, `MUL`, `DIV`
  - `CMP_EQ`, `CMP_LT`, `CMP_GT`
- 容器：
  - `NEW_ARR`, `NEW_DICT`, `INIT_ARR`
  - `APPEND_ITEM`, `SET_ITEM`, `GET_ITEM`
- 呼叫：
  - `PARAM`, `CALL`
- 變數：
  - `STORE`

## 參數傳遞慣例

- 先用 `PARAM` 按順序壓入參數
- 再用 `CALL f n t` 呼叫，`n` 是參數數量，回傳寫入 `t`

## 函式與頂層語句

- 函式定義包在 `FUNC_BEG ... FUNC_END`
- 檔案中若有函式外指令，視為頂層語句
