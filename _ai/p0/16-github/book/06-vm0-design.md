# 06. `vm0` 設計原理（IR 虛擬機）

對應檔案：`vm0.rs`

## 職責

- 載入 `.ir0`
- 建立標籤表與函式表
- 逐條執行四元組
- 提供語言內建 syscall（`print/len/array/...`）

## 執行模型

- 以 `pc` 走訪 `quads`
- `param_stack` 暫存呼叫參數
- call stack / frame 管理區域變數與回傳位址

## 函式呼叫

- `PARAM`：把參數推入 `param_stack`
- `CALL`：
  - 先嘗試系統函式
  - 否則進入自訂函式 frame
- `RET_VAL`：帶回結果給呼叫端

## 控制流程

- `JMP`: 無條件跳轉到 label
- `JMP_F`: 條件為 false 才跳
- `LABEL`: 作為跳轉目標

## 特色

- 系統函式語意可作為 LLVM runtime 的參考規格
- 可快速驗證 parser/IR 是否正確，不需 LLVM toolchain
