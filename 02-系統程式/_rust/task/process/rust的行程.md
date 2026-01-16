### 提示：關於 `fork`
Rust 的標準庫**不直接提供** Unix 風格的 `fork()`，因為 `fork` 在多執行緒環境下非常危險且難以安全管理。如果你真的需要底層的 `fork` 功能，可以使用 `nix` 等第三方 crate，但通常 `Command` 已經能滿足 99% 的需求。

## rust 的行程創建

在 Rust 中，創建新進程（Process）主要使用標準庫中的 **`std::process::Command`**。這個結構體採用了「生成器模式」（Builder Pattern），讓你能夠靈活地配置命令、參數、環境變數以及輸入輸出流。

以下是幾種常見的使用場景與範例：

### 1. 執行並等待結果 (最簡單的方式)
如果你只想執行一個命令，並獲取它的執行結果（退出碼、標準輸出、標準錯誤），可以使用 `.output()`。這會**阻塞**當前執行緒直到子進程結束。

```rust
use std::process::Command;

fn main() {
    let output = Command::new("ls") // Windows 下請改用 "dir" (搭配 cmd /C)
        .arg("-l")
        .arg("-a")
        .output()
        .expect("執行命令失敗");

    if output.status.success() {
        // 將位元組轉為字串
        let s = String::from_utf8_lossy(&output.stdout);
        println!("命令執行成功，輸出如下：\n{}", s);
    } else {
        let err = String::from_utf8_lossy(&output.stderr);
        println!("命令執行失敗，錯誤訊息：{}", err);
    }
}
```

### 2. 只獲取結束狀態 (Exit Status)
如果你不需要輸出內容，只需要知道命令是否執行成功，可以使用 `.status()`。

```rust
let status = Command::new("echo")
    .arg("hello")
    .status()
    .expect("無法執行 echo");

println!("進程退出碼: {:?}", status.code());
```

### 3. 在背景執行 (非阻塞式)
如果你希望子進程與父進程並行運行，可以使用 `.spawn()`。它會立即返回一個 `Child` 結構體，代表正在運行的子進程。

```rust
use std::process::Command;

fn main() {
    let mut child = Command::new("sleep")
        .arg("5")
        .spawn()
        .expect("無法啟動子進程");

    println!("子進程已啟動，PID: {}", child.id());

    // 你可以在這裡做其他事情...

    // 最後如果需要，可以等待子進程結束
    let status = child.wait().expect("等待子進程出錯");
    println!("子進程結束狀態: {}", status);
}
```

### 4. 處理輸入與輸出重定向 (Piping)
你可以透過 `Stdio` 來自定義 `stdin`, `stdout`, 和 `stderr`。例如，將一個進程的輸出作為另一個進程的輸入。

```rust
use std::process::{Command, Stdio};
use std::io::Write;

fn main() {
    // 範例：呼叫 `rev` 命令（反轉字串）並傳入輸入
    let mut child = Command::new("rev")
        .stdin(Stdio::piped())   // 開啟輸入管道
        .stdout(Stdio::piped())  // 開啟輸出管道
        .spawn()
        .expect("啟動失敗");

    // 向子進程寫入數據
    let mut stdin = child.stdin.take().expect("無法取得 stdin");
    stdin.write_all(b"Hello Rust Process").expect("寫入失敗");
    drop(stdin); // 關閉 stdin，否則子進程可能一直等待

    // 讀取子進程的輸出
    let output = child.wait_with_output().expect("讀取輸出失敗");
    println!("反轉結果: {}", String::from_utf8_lossy(&output.stdout));
}
```

### 5. Windows 與 Unix 的跨平台處理
有些命令是作業系統內置的（例如 Windows 的 `dir` 或 `copy`），它們不是獨立的 `.exe`，而是 `cmd.exe` 的一部分。處理這類命令時需要特別注意：

```rust
let output = if cfg!(target_os = "windows") {
    Command::new("cmd")
        .args(["/C", "dir"])
        .output()
} else {
    Command::new("ls")
        .output()
}.expect("執行失敗");
```

### 常用方法總結
| 方法 | 說明 | 是否阻塞 |
| :--- | :--- | :--- |
| **`.spawn()`** | 啟動子進程並返回 `Child` 控制權。 | 否 |
| **`.output()`** | 啟動子進程並收集所有輸出（stdout/stderr）。 | 是 |
| **`.status()`** | 啟動子進程並返回退出狀態。 | 是 |
| **`.arg()`** / **`.args()`** | 傳遞單個或多個參數。 | - |
| **`.env()`** | 設定環境變數。 | - |
| **`.current_dir()`** | 設定子進程的工作目錄。 | - |

