use std::thread;
use std::time::Duration;

fn main() {
    println!("--- Thread 示範 ---");
    let handle = thread::spawn(|| {
        for i in 1..4 {
            println!("執行緒任務: 正在運算第 {} 步...", i);
            thread::sleep(Duration::from_millis(500));
        }
        "執行緒完成！"
    });

    // 主執行緒可以做其他事
    println!("主執行緒繼續運行...");
    
    // 等待執行緒結束並獲取結果
    let result = handle.join().unwrap();
    println!("收到結果: {}", result);
}