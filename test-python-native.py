#!/usr/bin/env python3
# test-python-native.py
# Prueba del skill DeepSeek en Python (sin SDK, solo requests)

import os
import json
import requests

api_key = os.getenv('DEEPSEEK_API_KEY')

if not api_key:
    print("Error: DEEPSEEK_API_KEY no está configurada")
    exit(1)

payload = {
    'model': 'deepseek-chat',
    'messages': [
        {'role': 'system', 'content': 'You are a helpful assistant.'},
        {'role': 'user', 'content': 'What is 2+2?'},
    ],
    'stream': False,
}

print("[Python Native] Enviando petición a DeepSeek API...")
print(f"Payload: {json.dumps(payload)}\n")

headers = {
    'Content-Type': 'application/json',
    'Authorization': f'Bearer {api_key}',
}

try:
    response = requests.post(
        'https://api.deepseek.com/chat/completions',
        json=payload,
        headers=headers,
        timeout=30
    )

    print(f"HTTP Status: {response.status_code}")

    if response.status_code == 200:
        data = response.json()
        if 'choices' in data and len(data['choices']) > 0:
            print("✓ Respuesta exitosa:")
            print(data['choices'][0]['message']['content'])
        else:
            print("✗ Respuesta inválida:")
            print(response.text)
    else:
        print(f"✗ Error HTTP {response.status_code}:")
        print(response.text)

except Exception as e:
    print(f"✗ Error: {e}")
