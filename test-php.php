<?php
// test-php.php
// Prueba del skill DeepSeek en PHP

$apiKey = getenv('DEEPSEEK_API_KEY');

if (!$apiKey) {
    die("Error: DEEPSEEK_API_KEY no está configurada\n");
}

$payload = json_encode([
    'model'    => 'deepseek-chat',
    'messages' => [
        ['role' => 'system', 'content' => 'You are a helpful assistant.'],
        ['role' => 'user',   'content' => 'What is 2+2?'],
    ],
    'stream' => false,
]);

echo "[PHP] Enviando petición a DeepSeek API...\n";
echo "Payload: " . $payload . "\n\n";

$ch = curl_init('https://api.deepseek.com/chat/completions');
curl_setopt_array($ch, [
    CURLOPT_RETURNTRANSFER => true,
    CURLOPT_POST           => true,
    CURLOPT_POSTFIELDS     => $payload,
    CURLOPT_HTTPHEADER     => [
        'Content-Type: application/json',
        "Authorization: Bearer {$apiKey}",
    ],
    CURLOPT_TIMEOUT        => 30,
]);

$response = curl_exec($ch);
$httpCode = curl_getinfo($ch, CURLINFO_HTTP_CODE);
curl_close($ch);

echo "HTTP Status: {$httpCode}\n";

if ($httpCode === 200) {
    $data = json_decode($response, true);
    if (isset($data['choices'][0]['message']['content'])) {
        echo "✓ Respuesta exitosa:\n";
        echo $data['choices'][0]['message']['content'] . "\n";
    } else {
        echo "✗ Respuesta inválida:\n";
        echo $response . "\n";
    }
} else {
    echo "✗ Error HTTP {$httpCode}:\n";
    echo $response . "\n";
}
