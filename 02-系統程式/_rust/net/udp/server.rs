// server.rs
use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    // 1. 綁定伺服器位址與連接埠
    let socket = UdpSocket::bind("127.0.0.1:8080")?;
    println!("UDP 伺服器正在監聽 127.0.0.1:8080...");

    let mut buf = [0u8; 1024]; // 接收緩衝區

    loop {
        // 2. 接收資料，recv_from 會回傳 (位元組長度, 來源位址)
        let (amt, src) = socket.recv_from(&mut buf)?;

        // 將收到的位元組轉為字串並印出
        let filled_buf = &buf[..amt];
        let msg = String::from_utf8_lossy(filled_buf);
        println!("收到來自 {} 的訊息: {}", src, msg);

        // 3. 將同樣的資料回傳給來源位址 (Echo)
        socket.send_to(filled_buf, &src)?;
    }
}