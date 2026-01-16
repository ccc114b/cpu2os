use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("非同步伺服器啟動於 8080");

    loop {
        let (mut socket, _) = listener.accept().await?;

        // 產生一個非同步任務處理連線，不會阻塞主迴圈
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(n) if n == 0 => return, // 客戶端斷開
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("讀取錯誤: {}", e);
                        return;
                    }
                };

                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("寫入錯誤: {}", e);
                    return;
                }
            }
        });
    }
}