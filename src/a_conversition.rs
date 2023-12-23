use super::input;
use reqwest::{Client, Error};
use serde_json::{json, Value};

pub async fn speak(client: &Client) -> Result<(), Error> {
    let content = "why is the sky blue?";

    let res = client
        .post("http://localhost:11434/api/chat")
        .json(&json!(input::create_json_message(content)))
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
