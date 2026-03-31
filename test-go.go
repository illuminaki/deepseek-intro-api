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
	if apiKey == "" {
		fmt.Fprintln(os.Stderr, "Error: DEEPSEEK_API_KEY no está configurada")
		os.Exit(1)
	}

	reqBody, _ := json.Marshal(ChatRequest{
		Model: "deepseek-chat",
		Messages: []Message{
			{Role: "system", Content: "You are a helpful assistant."},
			{Role: "user", Content: "What is 2+2?"},
		},
		Stream: false,
	})

	fmt.Println("[Go] Enviando petición a DeepSeek API...")
	fmt.Printf("Payload: %s\n\n", string(reqBody))

	req, _ := http.NewRequest("POST", "https://api.deepseek.com/chat/completions",
		bytes.NewBuffer(reqBody))
	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("Authorization", "Bearer "+apiKey)

	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error: %v\n", err)
		os.Exit(1)
	}
	defer resp.Body.Close()

	fmt.Printf("HTTP Status: %d\n", resp.StatusCode)

	body, _ := io.ReadAll(resp.Body)

	if resp.StatusCode == 200 {
		var result ChatResponse
		json.Unmarshal(body, &result)
		if len(result.Choices) > 0 {
			fmt.Println("✓ Respuesta exitosa:")
			fmt.Println(result.Choices[0].Message.Content)
		} else {
			fmt.Println("✗ Respuesta inválida:")
			fmt.Println(string(body))
		}
	} else {
		fmt.Println("✗ Error HTTP:")
		fmt.Println(string(body))
	}
}
