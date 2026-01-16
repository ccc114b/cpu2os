use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    match TcpStream::connect("127.0.0.1:8080") {
        Ok(mut stream) => {
            println!("已連線至伺服器");

            let msg = b"Hello, Rust TCP!";
            stream.write_all(msg).expect("傳送失敗");

            let mut buffer = [0; 512];
            stream.read(&mut buffer).expect("讀取失敗");
            println!("收到回傳: {}", String::from_utf8_lossy(&buffer));
        }
        Err(e) => { println!("無法連線: {}", e); }
    }
}