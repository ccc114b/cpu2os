use tokio::net::UdpSocket;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    // 1. 綁定位址並建立非同步 UDP Socket
    let addr = "127.0.0.1:8080";
    let socket = UdpSocket::bind(addr).await?;
    println!("Tokio UDP 伺服器正在監聽 {}", addr);

    let mut buf = [0u8; 1024];

    loop {
        // 2. 非同步接收資料
        // recv_from 會讓出執行緒直到資料到達，不會阻塞整個程式
        let (len, peer) = socket.recv_from(&mut buf).await?;
        
        let msg = String::from_utf8_lossy(&buf[..len]);
        println!("收到來自 {} 的訊息: {}", peer, msg);

        // 3. 非同步回傳資料 (Echo)
        let sent_len = socket.send_to(&buf[..len], &peer).await?;
        println!("已將 {} bytes 回傳給 {}", sent_len, peer);
    }
}