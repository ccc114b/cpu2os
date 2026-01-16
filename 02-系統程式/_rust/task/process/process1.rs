use std::process::Command;

fn main() {
    println!("--- Process 示範 ---");
    // 啟動一個外部進程（例如：echo）
    let output = Command::new("echo")
        .arg("Hello from a separate process!")
        .output()
        .expect("無法啟動進程");

    println!("進程輸出: {}", String::from_utf8_lossy(&output.stdout));
}