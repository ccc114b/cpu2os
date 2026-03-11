# 11. 架構圖與資料流

## 編譯/執行雙路徑

```text
p0 source (.p0)
      |
      v
    p0c
      |
      v
   IR (.ir0)
    /     \
   v       v
 vm0      ir0c
 (VM)      |
           v
       LLVM IR (.ll)
           |
           v
   clang + lib0.a
           |
           v
      native binary
```

## 角色分工

- `p0c`：語法正確性 + IR 正確性
- `vm0`：語意驗證（快速）
- `ir0c`：後端 lowering
- `lib0`：執行時語意落地

## 一致性原則

同一份 `.ir0` 同時交給 `vm0` 與 `ir0c`，可降低「前端正確、後端錯誤」的排錯成本。
