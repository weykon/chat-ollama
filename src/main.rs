use reqwest::Error;
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:11434/api/chat")
        .json(&json!({
            "model": "mistral",
            "messages": [
                { "role": "user", "content": "why is the sky blue?" }
            ]
        }))
        .send()
        .await?;
    let text = res.text().await?;
    let mut messages = String::new();
 
    for line in text.lines() {
        let v: Value = serde_json::from_str(line).and_then(|v| Ok(v)).unwrap();
        if let Some(message) = v.get("message") {
            if let Some(content) = message.get("content") {
                messages.push_str(content.as_str().unwrap());
                messages.push(' ');
            }
        }
    }

    println!("Messages: {}", messages);

    Ok(())
}
