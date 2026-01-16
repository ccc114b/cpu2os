use ollama_rs::Ollama;
use ollama_rs::generation::chat::request::ChatMessageRequest;
use ollama_rs::generation::chat::ChatMessage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ollama = Ollama::default();
    let model = "qwen3:0.6b-q4_K_M".to_string();
    let prompt = "你好，請自我介紹。".to_string();

    // 建立訊息
    let messages = vec![ChatMessage::user(prompt)];
    
    // 建立請求物件
    let request = ChatMessageRequest::new(model, messages);

    // 發送請求 (非串流模式)
    let res = ollama.send_chat_messages(request).await?;

    println!("Ollama: {}", res.message.content);

    Ok(())
}