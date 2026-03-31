using System;
using System.Net.Http;
using System.Net.Http.Headers;
using System.Net.Http.Json;
using System.Text.Json.Serialization;
using System.Threading.Tasks;

class DeepSeekTest
{
    static async Task Main()
    {
        var apiKey = Environment.GetEnvironmentVariable("DEEPSEEK_API_KEY");
        if (string.IsNullOrEmpty(apiKey))
        {
            Console.WriteLine("Error: DEEPSEEK_API_KEY no está configurada");
            return;
        }

        using var client = new HttpClient();
        client.BaseAddress = new Uri("https://api.deepseek.com");
        client.DefaultRequestHeaders.Authorization = new AuthenticationHeaderValue("Bearer", apiKey);

        var request = new ChatRequest(
            Model: "deepseek-chat",
            Messages: new[]
            {
                new Message("system", "You are a helpful assistant."),
                new Message("user", "What is 2+2?")
            },
            Stream: false
        );

        Console.WriteLine("[C#] Enviando petición a DeepSeek API...");
        Console.WriteLine($"Payload: {System.Text.Json.JsonSerializer.Serialize(request)}\n");

        try
        {
            var response = await client.PostAsJsonAsync("/chat/completions", request);
            Console.WriteLine($"HTTP Status: {(int)response.StatusCode}");

            if (response.IsSuccessStatusCode)
            {
                var result = await response.Content.ReadFromJsonAsync<ChatResponse>();
                if (result?.Choices?.Length > 0)
                {
                    Console.WriteLine("✓ Respuesta exitosa:");
                    Console.WriteLine(result.Choices[0].Message.Content);
                }
                else
                {
                    Console.WriteLine("✗ Respuesta inválida");
                }
            }
            else
            {
                var content = await response.Content.ReadAsStringAsync();
                Console.WriteLine($"✗ Error HTTP:");
                Console.WriteLine(content);
            }
        }
        catch (Exception ex)
        {
            Console.WriteLine($"✗ Error: {ex.Message}");
        }
    }
}

record Message(
    [property: JsonPropertyName("role")] string Role,
    [property: JsonPropertyName("content")] string Content);

record ChatRequest(
    [property: JsonPropertyName("model")] string Model,
    [property: JsonPropertyName("messages")] Message[] Messages,
    [property: JsonPropertyName("stream")] bool Stream);

record Choice(
    [property: JsonPropertyName("message")] Message Message);

record ChatResponse(
    [property: JsonPropertyName("choices")] Choice[] Choices);
