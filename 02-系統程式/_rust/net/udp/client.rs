// client.rs
use std::net::UdpSocket;
use std::io;

fn main() -> std::io::Result<()> {
    // 1. 綁定本地位址（使用連接埠 0 代表由系統隨機分配可用連接埠）
    let socket = UdpSocket::bind("127.0.0.1:0")?;
    let server_addr = "127.0.0.1:8080";

    println!("請輸入要發送的訊息：");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    // 2. 發送資料到伺服器
    socket.send_to(input.as_bytes(), server_addr)?;
    println!("訊息已發送至 {}", server_addr);

    // 3. 接收伺服器回傳的資料
    let mut buf = [0u8; 1024];
    let (amt, _src) = socket.recv_from(&mut buf)?;

    println!("來自伺服器的回傳: {}", String::from_utf8_lossy(&buf[..amt]));

    Ok(())
}