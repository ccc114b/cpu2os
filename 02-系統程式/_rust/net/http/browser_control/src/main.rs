use chromiumoxide::browser::{Browser, BrowserConfig};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 【关键修改】在 browser 前面加上 mut
    let (mut browser, mut handler) = Browser::launch(
        BrowserConfig::builder()
            .with_head() // 如果不需要界面，可以删掉这行
            .build()?
    ).await?;

    // 驱动浏览器后台任务
    let handle = tokio::spawn(async move {
        while let Some(h) = handler.next().await {
            if let Err(e) = h {
                eprintln!("Browser error: {}", e);
                break;
            }
        }
    });

    let page = browser.new_page("https://www.rust-lang.org").await?;
    
    // 获取标题
    let title = page.get_title().await?;
    println!("页面标题: {:?}", title);

    // 获取内容（解决 unused variable 警告，加个下划线或直接打印）
    let _content = page.content().await?; 

    // 【此处现在可以正常运行了】
    println!("正在关闭浏览器...");
    browser.close().await?;

    // 等待后台任务结束
    let _ = handle.await;

    Ok(())
}