import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;

public class DeepSeekClient {

    public static void main(String[] args) throws Exception {
        String apiKey = System.getenv("DEEPSEEK_API_KEY");
        
        if (apiKey == null || apiKey.isEmpty()) {
            System.err.println("Error: DEEPSEEK_API_KEY no está configurada");
            System.exit(1);
        }

        String body = """
            {
              "model": "deepseek-chat",
              "messages": [
                {"role": "system", "content": "You are a helpful assistant."},
                {"role": "user", "content": "What is 2+2?"}
              ],
              "stream": false
            }
            """;

        System.out.println("[Java] Enviando petición a DeepSeek API...");
        System.out.println("Payload: " + body.replace("\n", "").replace(" ", "") + "\n");

        HttpRequest request = HttpRequest.newBuilder()
            .uri(URI.create("https://api.deepseek.com/chat/completions"))
            .header("Content-Type", "application/json")
            .header("Authorization", "Bearer " + apiKey)
            .POST(HttpRequest.BodyPublishers.ofString(body))
            .build();

        HttpResponse<String> response = HttpClient.newHttpClient()
            .send(request, HttpResponse.BodyHandlers.ofString());

        System.out.println("HTTP Status: " + response.statusCode());

        if (response.statusCode() == 200) {
            String content = response.body();
            // Extraer el contenido de la respuesta JSON
            int start = content.indexOf("\"content\":\"") + 11;
            int end = content.indexOf("\"", start);
            if (start > 10 && end > start) {
                String message = content.substring(start, end)
                    .replace("\\n", "\n")
                    .replace("\\\"", "\"");
                System.out.println("✓ Respuesta exitosa:");
                System.out.println(message);
            } else {
                System.out.println("✗ Respuesta inválida:");
                System.out.println(content);
            }
        } else {
            System.out.println("✗ Error HTTP " + response.statusCode() + ":");
            System.out.println(response.body());
        }
    }
}
