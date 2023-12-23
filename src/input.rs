use serde_json::json;

pub fn create_json_message(content: &str) -> serde_json::Value {
    json!({
        "model": "mistral",
        "messages": [
            { "role": "user", "content": content }
        ]
    })
}


