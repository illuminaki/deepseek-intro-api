use serde::{Deserialize, Serialize};
use std::env;
use std::process::Command;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let payload = serde_json::to_string(&body)?;
    println!("[Rust] Enviando petición a DeepSeek API...");
    println!("Payload: {}\n", payload);

    let output = Command::new("curl")
        .arg("-s")
        .arg("-X")
        .arg("POST")
        .arg("https://api.deepseek.com/chat/completions")
        .arg("-H")
        .arg("Content-Type: application/json")
        .arg("-H")
        .arg(format!("Authorization: Bearer {}", api_key))
        .arg("-d")
        .arg(&payload)
        .output()?;

    let response_text = String::from_utf8(output.stdout)?;
    
    if output.status.success() {
        match serde_json::from_str::<ChatResponse>(&response_text) {
            Ok(data) => {
                if !data.choices.is_empty() {
                    println!("HTTP Status: 200");
                    println!("✓ Respuesta exitosa:");
                    println!("{}", data.choices[0].message.content);
                } else {
                    println!("✗ Respuesta inválida: choices vacío");
                    println!("{}", response_text);
                }
            }
            Err(e) => {
                println!("✗ Error al parsear JSON: {}", e);
                println!("{}", response_text);
            }
        }
    } else {
        println!("✗ Error en curl:");
        println!("{}", response_text);
    }

    Ok(())
}
