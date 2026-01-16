use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::fs;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await?;
    println!("HTTP 伺服器已啟動於 http://{}", addr);

    loop {
        let (stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream).await {
                eprintln!("處理連線時出錯: {}", e);
            }
        });
    }
}

async fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await?;
    if n == 0 { return Ok(()); }

    let request = String::from_utf8_lossy(&buffer[..n]);
    let first_line = request.lines().next().unwrap_or("");
    let parts: Vec<&str> = first_line.split_whitespace().collect();
    
    if parts.len() < 2 { return Ok(()); }
    let method = parts[0];
    let mut path = parts[1];

    if method != "GET" { return Ok(()); }
    if path == "/" { path = "/index.html"; }

    let safe_path = path.trim_start_matches('/');
    let file_path = Path::new("public").join(safe_path);

    // 1. 根據副檔名判斷 MIME Type
    let content_type = match file_path.extension().and_then(|s| s.to_str()) {
        Some("html") => "text/html",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("css") => "text/css",
        Some("js") => "application/javascript",
        _ => "application/octet-stream", // 預設下載流
    };

    // 2. 使用 fs::read 讀取原始位元組 (bytes)，而非字串
    match fs::read(&file_path).await {
        Ok(contents) => {
            send_response(&mut stream, "200 OK", content_type, &contents).await?;
        }
        Err(_) => {
            let not_found = fs::read("public/404.html").await
                .unwrap_or_else(|_| b"404 Not Found".to_vec());
            send_response(&mut stream, "404 NOT FOUND", "text/html", &not_found).await?;
        }
    }

    Ok(())
}

// 3. 修改參數，接收 &[u8] 類型的內容
async fn send_response(
    stream: &mut TcpStream, 
    status: &str, 
    content_type: &str, 
    content: &[u8]
) -> std::io::Result<()> {
    let header = format!(
        "HTTP/1.1 {}\r\n\
        Content-Length: {}\r\n\
        Content-Type: {}\r\n\
        Connection: close\r\n\
        \r\n",
        status,
        content.len(),
        content_type
    );
    
    // 先傳送 Header
    stream.write_all(header.as_bytes()).await?;
    // 再傳送二進位內容
    stream.write_all(content).await?;
    stream.flush().await?;
    Ok(())
}