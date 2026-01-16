use sqlx::sqlite::SqlitePool;
use std::env;
use dotenvy::dotenv;

// 定義一個結構來對應資料庫中的資料
// sqlx::FromRow 讓 SQLx 能自動將查詢結果轉換成這個 Struct
#[derive(Debug, sqlx::FromRow)]
struct User {
    id: i64,
    name: String,
    email: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 載入 .env 變數
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;

    // 2. 建立資料庫連線池 (Connection Pool)
    let pool = SqlitePool::connect(&database_url).await?;

    // 3. 建立資料表 (如果是第一次執行)
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            email TEXT NOT NULL UNIQUE
        )
        "#
    )
    .execute(&pool)
    .await?;

    println!("資料表已準備就緒。");

    // 4. 插入資料
    let new_user_name = "Rust 小明";
    let new_user_email = "ming@rust.tw";

    sqlx::query("INSERT INTO users (name, email) VALUES (?, ?)")
    .bind(new_user_name)
    .bind(new_user_email)
    .execute(&pool)
    .await?;

    println!("資料插入成功！");

    // 5. 查詢資料
    let users = sqlx::query_as::<_, User>("SELECT id, name, email FROM users")
    .fetch_all(&pool)
    .await?;

    println!("目前的用戶列表：");
    for user in users {
        println!("{:?}", user);
    }

    Ok(())
}