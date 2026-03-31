---
name: deepseek-multilang
description: >-
  Guía completa para conectarse a la API de DeepSeek desde lenguajes no oficialmente
  soportados por su SDK: PHP, Ruby, Rust, C#, Java, C++, Go y Dart.
  Úsala cuando el agente deba integrar DeepSeek en cualquiera de estos lenguajes,
  hacer una petición al chat API, manejar la autenticación con API key, enviar
  mensajes y procesar la respuesta JSON. También actívate con frases como
  "cómo llamo a DeepSeek desde PHP", "cliente DeepSeek en Java", "integrar
  DeepSeek en Rust", o cuando el usuario trabaje en un proyecto que no sea
  Python ni Node.js y quiera usar DeepSeek.
---

# DeepSeek API — Integraciones Multi-Lenguaje

La API de DeepSeek es **compatible con el formato OpenAI**, por lo que cualquier
cliente HTTP puede consumirla con un simple POST.

## Parámetros base

| Parámetro   | Valor                            |
|-------------|----------------------------------|
| `base_url`  | `https://api.deepseek.com`       |
| `api_key`   | Tu API key de DeepSeek           |
| Endpoint    | `POST /chat/completions`         |
| Modelos     | `deepseek-chat` · `deepseek-reasoner` |

> **Nunca** escribas la API key en el código. Usa variables de entorno:
> `DEEPSEEK_API_KEY`.

---

## Payload JSON estándar

```json
{
  "model": "deepseek-chat",
  "messages": [
    { "role": "system", "content": "You are a helpful assistant." },
    { "role": "user",   "content": "Hello!" }
  ],
  "stream": false
}
```

La respuesta llega en `choices[0].message.content`.

---

## 1. PHP

**Dependencia:** ninguna (usa `curl` nativo de PHP).

```php
<?php
// deepseek_client.php

$apiKey  = getenv('DEEPSEEK_API_KEY');
$payload = json_encode([
    'model'    => 'deepseek-chat',
    'messages' => [
        ['role' => 'system', 'content' => 'You are a helpful assistant.'],
        ['role' => 'user',   'content' => 'Hello!'],
    ],
    'stream' => false,
]);

$ch = curl_init('https://api.deepseek.com/chat/completions');
curl_setopt_array($ch, [
    CURLOPT_RETURNTRANSFER => true,
    CURLOPT_POST           => true,
    CURLOPT_POSTFIELDS     => $payload,
    CURLOPT_HTTPHEADER     => [
        'Content-Type: application/json',
        "Authorization: Bearer {$apiKey}",
    ],
]);

$response = curl_exec($ch);
curl_close($ch);

$data = json_decode($response, true);
echo $data['choices'][0]['message']['content'] . PHP_EOL;
```

---

## 2. Ruby

**Dependencia:** `gem install faraday` (o usa `net/http` estándar).

```ruby
# deepseek_client.rb
require 'net/http'
require 'json'
require 'uri'

uri     = URI('https://api.deepseek.com/chat/completions')
api_key = ENV.fetch('DEEPSEEK_API_KEY')

payload = {
  model: 'deepseek-chat',
  messages: [
    { role: 'system', content: 'You are a helpful assistant.' },
    { role: 'user',   content: 'Hello!' }
  ],
  stream: false
}.to_json

http           = Net::HTTP.new(uri.host, uri.port)
http.use_ssl   = true
request        = Net::HTTP::Post.new(uri)
request['Content-Type']  = 'application/json'
request['Authorization'] = "Bearer #{api_key}"
request.body   = payload

response = http.request(request)
data     = JSON.parse(response.body)
puts data['choices'][0]['message']['content']
```

---

## 3. Rust

**Dependencias en `Cargo.toml`:**

```toml
[dependencies]
reqwest  = { version = "0.12", features = ["json", "blocking"] }
serde    = { version = "1",    features = ["derive"] }
serde_json = "1"
tokio    = { version = "1",    features = ["rt-multi-thread", "macros"] }
```

```rust
// src/main.rs
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
    let api_key = env::var("DEEPSEEK_API_KEY")?;

    let body = ChatRequest {
        model: "deepseek-chat".into(),
        messages: vec![
            Message { role: "system".into(), content: "You are a helpful assistant.".into() },
            Message { role: "user".into(),   content: "Hello!".into() },
        ],
        stream: false,
    };

    let client   = Client::new();
    let response = client
        .post("https://api.deepseek.com/chat/completions")
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .await?
        .json::<ChatResponse>()
        .await?;

    println!("{}", response.choices[0].message.content);
    Ok(())
}
```

---

## 4. C# (.NET 6+)

**Dependencia:** `System.Net.Http.Json` (incluida en .NET 6+).

```csharp
// DeepSeekClient.cs
using System.Net.Http.Headers;
using System.Net.Http.Json;
using System.Text.Json.Serialization;

var apiKey = Environment.GetEnvironmentVariable("DEEPSEEK_API_KEY")!;

using var client = new HttpClient();
client.BaseAddress = new Uri("https://api.deepseek.com");
client.DefaultRequestHeaders.Authorization =
    new AuthenticationHeaderValue("Bearer", apiKey);

var request = new ChatRequest(
    Model: "deepseek-chat",
    Messages: new[]
    {
        new Message("system", "You are a helpful assistant."),
        new Message("user",   "Hello!")
    },
    Stream: false
);

var response = await client.PostAsJsonAsync("/chat/completions", request);
response.EnsureSuccessStatusCode();

var result = await response.Content.ReadFromJsonAsync<ChatResponse>();
Console.WriteLine(result!.Choices[0].Message.Content);

// --- Records ---
record Message(
    [property: JsonPropertyName("role")]    string Role,
    [property: JsonPropertyName("content")] string Content);

record ChatRequest(
    [property: JsonPropertyName("model")]    string   Model,
    [property: JsonPropertyName("messages")] Message[] Messages,
    [property: JsonPropertyName("stream")]   bool     Stream);

record Choice(
    [property: JsonPropertyName("message")] Message Message);

record ChatResponse(
    [property: JsonPropertyName("choices")] Choice[] Choices);
```

---

## 5. Java

**Dependencia:** Java 11+ (usa `HttpClient` estándar). Sin dependencias externas.

```java
// DeepSeekClient.java
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;

public class DeepSeekClient {

    public static void main(String[] args) throws Exception {
        String apiKey = System.getenv("DEEPSEEK_API_KEY");

        String body = """
            {
              "model": "deepseek-chat",
              "messages": [
                {"role": "system", "content": "You are a helpful assistant."},
                {"role": "user",   "content": "Hello!"}
              ],
              "stream": false
            }
            """;

        HttpRequest request = HttpRequest.newBuilder()
            .uri(URI.create("https://api.deepseek.com/chat/completions"))
            .header("Content-Type", "application/json")
            .header("Authorization", "Bearer " + apiKey)
            .POST(HttpRequest.BodyPublishers.ofString(body))
            .build();

        HttpResponse<String> response = HttpClient.newHttpClient()
            .send(request, HttpResponse.BodyHandlers.ofString());

        System.out.println(response.body());
        // Parsear con Gson/Jackson o manualmente con indexOf/substring
    }
}
```

> Para parsear el JSON en Java sin dependencias extra, extrae el contenido así:
> ```java
> String content = response.body();
> int start = content.indexOf("\"content\":\"") + 11;
> int end   = content.indexOf("\"", start);
> System.out.println(content.substring(start, end));
> ```

---

## 6. C++

**Dependencia:** `libcurl` (disponible en todas las plataformas).

```cpp
// deepseek_client.cpp
// Compilar: g++ deepseek_client.cpp -lcurl -o deepseek_client

#include <curl/curl.h>
#include <cstdlib>
#include <iostream>
#include <string>

static size_t writeCallback(char* ptr, size_t size, size_t nmemb, std::string* data) {
    data->append(ptr, size * nmemb);
    return size * nmemb;
}

int main() {
    const char* apiKey = std::getenv("DEEPSEEK_API_KEY");

    std::string payload = R"({
        "model": "deepseek-chat",
        "messages": [
            {"role": "system", "content": "You are a helpful assistant."},
            {"role": "user",   "content": "Hello!"}
        ],
        "stream": false
    })";

    std::string authHeader = std::string("Authorization: Bearer ") + apiKey;
    std::string response;

    struct curl_slist* headers = nullptr;
    headers = curl_slist_append(headers, "Content-Type: application/json");
    headers = curl_slist_append(headers, authHeader.c_str());

    CURL* curl = curl_easy_init();
    curl_easy_setopt(curl, CURLOPT_URL, "https://api.deepseek.com/chat/completions");
    curl_easy_setopt(curl, CURLOPT_HTTPHEADER,    headers);
    curl_easy_setopt(curl, CURLOPT_POSTFIELDS,    payload.c_str());
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, writeCallback);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA,     &response);

    curl_easy_perform(curl);
    curl_easy_cleanup(curl);
    curl_slist_free_all(headers);

    std::cout << response << std::endl;
    return 0;
}
```

---

## 7. Go

**Sin dependencias externas** (usa `net/http` estándar).

```go
// main.go
package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"os"
)

type Message struct {
	Role    string `json:"role"`
	Content string `json:"content"`
}

type ChatRequest struct {
	Model    string    `json:"model"`
	Messages []Message `json:"messages"`
	Stream   bool      `json:"stream"`
}

type ChatResponse struct {
	Choices []struct {
		Message struct {
			Content string `json:"content"`
		} `json:"message"`
	} `json:"choices"`
}

func main() {
	apiKey := os.Getenv("DEEPSEEK_API_KEY")

	reqBody, _ := json.Marshal(ChatRequest{
		Model: "deepseek-chat",
		Messages: []Message{
			{Role: "system", Content: "You are a helpful assistant."},
			{Role: "user",   Content: "Hello!"},
		},
		Stream: false,
	})

	req, _ := http.NewRequest("POST", "https://api.deepseek.com/chat/completions",
		bytes.NewBuffer(reqBody))
	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("Authorization", "Bearer "+apiKey)

	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		fmt.Fprintln(os.Stderr, "Error:", err)
		os.Exit(1)
	}
	defer resp.Body.Close()

	body, _ := io.ReadAll(resp.Body)

	var result ChatResponse
	json.Unmarshal(body, &result)
	fmt.Println(result.Choices[0].Message.Content)
}
```

---

## 8. Dart / Flutter

**Dependencia:** `http: ^1.2.0` en `pubspec.yaml`.

```dart
// deepseek_client.dart
import 'dart:convert';
import 'dart:io';
import 'package:http/http.dart' as http;

Future<void> main() async {
  final apiKey = Platform.environment['DEEPSEEK_API_KEY']!;

  final response = await http.post(
    Uri.parse('https://api.deepseek.com/chat/completions'),
    headers: {
      'Content-Type': 'application/json',
      'Authorization': 'Bearer $apiKey',
    },
    body: jsonEncode({
      'model': 'deepseek-chat',
      'messages': [
        {'role': 'system', 'content': 'You are a helpful assistant.'},
        {'role': 'user',   'content': 'Hello!'},
      ],
      'stream': false,
    }),
  );

  final data = jsonDecode(response.body);
  print(data['choices'][0]['message']['content']);
}
```

---

## Modelos disponibles

| Modelo               | Modo           | Max Output | Temperatura recomendada |
|----------------------|----------------|------------|-------------------------|
| `deepseek-chat`      | No-thinking    | 8K         | 1.0 (general)           |
| `deepseek-reasoner`  | Thinking (CoT) | 64K        | 0.0 (razonamiento)      |

## Manejo de errores — Códigos comunes

| HTTP | Significado                       | Acción                            |
|------|-----------------------------------|-----------------------------------|
| 401  | API key inválida o ausente        | Verificar `DEEPSEEK_API_KEY`      |
| 402  | Saldo insuficiente                | Recargar cuenta DeepSeek          |
| 429  | Rate limit alcanzado              | Esperar y reintentar con backoff  |
| 500  | Error interno del servidor        | Reintentar después de unos segundos |

---

## Checklist antes de integrar

- [ ] `DEEPSEEK_API_KEY` en variables de entorno (nunca hardcodeada)
- [ ] Header `Content-Type: application/json` presente
- [ ] Header `Authorization: Bearer <key>` presente
- [ ] Body como JSON válido con `model`, `messages` y `stream`
- [ ] Manejo de errores HTTP (status != 200)
- [ ] Extraer respuesta de `choices[0].message.content`
