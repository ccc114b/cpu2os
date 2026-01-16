use tokio::time::{sleep, Duration};

async fn async_task(id: u32) {
    println!("Async 任務 {} 開始等待 I/O...", id);
    sleep(Duration::from_secs(1)).await; // 不會阻塞整個執行緒
    println!("Async 任務 {} 收到回應！", id);
}

#[tokio::main]
async fn main() { // <--- 這裡必須改成 main
    println!("--- Async 示範 ---");
    let mut tasks = vec![];
    
    for i in 1..=3 {
        // 同時啟動多個任務，只需少量的執行緒
        tasks.push(tokio::spawn(async_task(i)));
    }

    for task in tasks {
        let _ = task.await;
    }
    println!("所有 Async 任務完成！");
}