use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    stream: bool,
}

#[derive(Deserialize, Debug)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
struct Choice {
    message: MessageContent,
}

#[derive(Deserialize, Debug)]
struct MessageContent {
    content: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("DEEPSEEK_API_KEY")
        .expect("DEEPSEEK_API_KEY no está configurada");

    let body = ChatRequest {
        model: "deepseek-chat".into(),
        messages: vec![
            Message {
                role: "system".into(),
                content: "You are a helpful assistant.".into(),
            },
            Message {
                role: "user".into(),
                content: "What is 2+2?".into(),
            },
        ],
        stream: false,
    };

    println!("[Rust] Enviando petición a DeepSeek API...");
    println!("Payload: {}\n", serde_json::to_string(&body)?);

    let client = Client::new();
    let response = client
        .post("https://api.deepseek.com/chat/completions")
        .bearer_auth(&api_key)
        .json(&body)
        .send()
        .await?;

    let status = response.status();
    println!("HTTP Status: {}", status);

    if status.is_success() {
        let data = response.json::<ChatResponse>().await?;
        if !data.choices.is_empty() {
            println!("✓ Respuesta exitosa:");
            println!("{}", data.choices[0].message.content);
        } else {
            println!("✗ Respuesta inválida: choices vacío");
        }
    } else {
        let text = response.text().await?;
        println!("✗ Error HTTP:");
        println!("{}", text);
    }

    Ok(())
}
