use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512]; // 建立緩衝區
    while match stream.read(&mut buffer) {
        Ok(size) => {
            if size == 0 { false } else {
                // 將收到的資料回傳給客戶端 (Echo)
                stream.write_all(&buffer[0..size]).unwrap();
                true
            }
        }
        Err(_) => {
            println!("連線錯誤");
            false
        }
    } {}
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("無法綁定埠口");
    println!("伺服器啟動於 127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("新連線: {}", stream.peer_addr().unwrap());
                // 使用執行緒處理多個連線，避免阻塞
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => { println!("連線失敗: {}", e); }
        }
    }
}