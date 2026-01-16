use reqwest;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get("https://example.com")?;
    let body = response.text()?;

    println!("{}", body);
    Ok(())
}

/*
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 定義目標 URL
    let url = "https://example.com";

    // 2. 發送 GET 請求
    let response = reqwest::get(url).await?;

    // 3. 檢查狀態碼是否成功 (200 OK)
    if response.status().is_success() {
        // 4. 將響應體轉換為字串 (HTML 原始碼)
        let body = response.text().await?;
        println!("成功抓取網頁內容：\n");
        println!("{}", body);
    } else {
        println!("請求失敗，狀態碼：{}", response.status());
    }

    Ok(())
}
*/
