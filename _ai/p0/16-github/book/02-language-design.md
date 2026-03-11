# 02. 語言設計

## 設計目標

p0 以「最小可用語言核心」為目標，強調：

- 可快速實作編譯器前端
- 易於映射到簡單 IR
- 可同時支持 VM 與 LLVM 後端

## 值模型

語言層面採動態型別，主要值型別：

- `null`
- `int`
- `float`
- `string`
- `array`
- `dict`

## 主要語法構件

- 敘述句：
  - 賦值
  - `if / else`
  - `while`
  - `for (init; cond; update)`
  - `break / continue`
  - `return`
- 表達式：
  - 算術 `+ - * /`
  - 比較 `== < >`
  - 函式呼叫
  - 索引與屬性存取

## 函式與程式入口

- 使用 `func` 宣告函式
- 允許頂層語句
- LLVM 後端會把頂層語句包裝成 `@__init__`
- `bin` 模式再產生系統入口 `@main()` 呼叫 `@__init__`

## 系統函式

語言層提供一組內建函式（由 VM 或 runtime 實作）：

- I/O: `print`, `input`, `exit`
- 容器：`array`, `len`, `push`, `pop`, `keys`, `has_key`, `remove`
- 型別/轉換：`typeof`, `int`, `str`, `ord`, `chr`
- 系統：`time`, `random`
