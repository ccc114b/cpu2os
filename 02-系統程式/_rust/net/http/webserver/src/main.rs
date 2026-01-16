use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // 1. 定義路由：當使用者發送 GET 請求到 "/" 時，執行後面的匿名函式
    let app = Router::new().route("/", get(|| async { "Hello, Rust Server! 🦀" }));

    // 2. 定義伺服器要監聽的地址 (localhost:3000)
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("伺服器已啟動，請訪問 http://{}", addr);

    // 3. 啟動伺服器
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
