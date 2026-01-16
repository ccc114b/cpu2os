use std::env; // 引入標準庫中的環境模組

fn main() {
    // 獲取所有命令列參數
    let args: Vec<String> = env::args().collect();

    // 獲取特定的環境變數，例如 "USER" 或 "PATH"
    let user = env::var("USER").unwrap_or_else(|_| "Unknown".to_string());

    println!("你好，來自系統的使用者: {}", user);
    println!("你輸入了 {} 個參數。", args.len());

    // 打印第一個參數（通常是執行檔路徑）
    if let Some(path) = args.get(0) {
        println!("本程式路徑：{}", path);
    }
}