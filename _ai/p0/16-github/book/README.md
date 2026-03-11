# p0compiler Book

這份文件把 `p0compiler` 專案整理成一本可循序閱讀的技術手冊。

建議閱讀順序：

1. [01. 專案總覽](./01-overview.md)
2. [02. 語言設計](./02-language-design.md)
3. [03. BNF 語法](./03-bnf.md)
4. [04. IR 規格（.ir0）](./04-ir-spec.md)
5. [05. p0c 設計原理（前端）](./05-p0c-design.md)
6. [06. vm0 設計原理（IR 虛擬機）](./06-vm0-design.md)
7. [07. ir0c 設計原理（IR → LLVM）](./07-ir0c-design.md)
8. [08. lib0 設計原理（LLVM Runtime）](./08-lib0-design.md)
9. [09. 建置與執行流程](./09-build-and-run.md)
10. [10. 程式碼導讀（逐檔）](./10-code-walkthrough.md)
11. [11. 架構圖與資料流](./11-architecture.md)

---

## 這本書回答的問題

- p0 語言目前支援哪些語法？
- BNF 定義與實作是否一致？
- IR 長什麼樣子？
- 為什麼要同時有 VM 與 LLVM 兩條路徑？
- `p0c/vm0/ir0c/lib0` 四個工具分工是什麼？
- 如何快速定位是前端、VM、lowering、還是 runtime 問題？

---

## 文件維護約定

- 語法有增減時：同步更新 `03-bnf.md` 與 `05-p0c-design.md`
- IR 指令有增減時：同步更新 `04-ir-spec.md`、`06-vm0-design.md`、`07-ir0c-design.md`
- runtime syscall 有增減時：同步更新 `08-lib0-design.md`
- 腳本流程變更時：同步更新 `09-build-and-run.md`
