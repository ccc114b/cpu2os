# 09. 建置與執行流程

## 建置

在 `` 下執行：

```bash
./build.sh
```

內容：

- `rustc p0c.rs`
- `rustc vm0.rs`
- `rustc ir0c.rs`
- `rustc --crate-type=staticlib lib0.rs -o lib0.a`

## 一鍵流程

```bash
./run.sh p0/fact
```

`run.sh` 會依序執行：

1. `./p0c p0/fact.p0`
2. `./vm0 p0/fact.ir0`
3. `./ir0c p0/fact.ir0`
4. `clang -Wno-override-module p0/fact.ll lib0.a -o p0/fact`
5. `./p0/fact`

## 常用手動命令

```bash
./p0c -d p0/loop.p0
./vm0 p0/loop.ir0
./ir0c --target bin p0/loop.ir0
clang -Wno-override-module p0/loop.ll lib0.a -o p0/loop
./p0/loop
```

## 除錯建議

- 前端問題：先看 `p0c -d` 生成的 quads
- VM 與 LLVM 結果不同：
  - 先確認 `vm0` 正確
  - 再檢查 `ir0c` lowering 與 `lib0` runtime 對應
