# DeepSeek API — Skill Multi-Lenguaje

Skill reutilizable que enseña a agentes de IA cómo integrar la API de DeepSeek en 8 lenguajes de programación no oficialmente soportados por el SDK oficial (que solo cubre Python, Node.js y cURL).

Repositorio: https://github.com/illuminaki/deepseek-intro-api

## Contenido

- **skill/SKILL-deepseek-multilang.md** — Skill principal con ejemplos en PHP, Ruby, Rust, C#, Java, C++, Go, Dart
- **skill/context.md** — Documentación de DeepSeek API usada para crear el skill
- **test-php.php, test-ruby.rb, test-go.go** — Pruebas funcionales ejecutadas
- **CONFIGURAR-ENV.md** — Guía para configurar variables de entorno

## Cómo se Creó el Skill

### Contexto Necesario

El skill se creó basándose en la documentación oficial de DeepSeek API (`skill/context.md`), que proporciona:

1. **Endpoint Base:** https://api.deepseek.com/chat/completions
2. **Autenticación:** Bearer token con API key
3. **Formato Compatible:** OpenAI-compatible (mismo formato que OpenAI API)
4. **Modelos Disponibles:**
   - `deepseek-chat` — Modo no-thinking, 128K contexto, máx 8K output
   - `deepseek-reasoner` — Modo thinking (CoT), 128K contexto, máx 64K output

5. **Parámetros Clave:**
   - Temperature: 0.0 (coding/math), 1.0 (general), 1.3 (translation), 1.5 (creative)
   - Tokens: ~0.3 por carácter en inglés, ~0.6 por carácter en chino
   - Pricing: $0.28/1M input, $0.42/1M output, $0.028/1M cache hit

### Proceso de Creación

1. **Análisis del contexto:** Se estudió la documentación de DeepSeek para entender que es OpenAI-compatible
2. **Identificación del problema:** El SDK oficial solo soporta Python, Node.js y cURL
3. **Solución:** Crear ejemplos de código HTTP directo en 8 lenguajes diferentes
4. **Patrón consistente:** Todos los lenguajes siguen POST + headers + payload JSON
5. **Validación:** Pruebas ejecutadas en PHP, Ruby, Go y Python con HTTP 200 exitoso

### Estructura del Skill

El skill sigue el formato estándar skills.sh con:

```yaml
---
name: deepseek-multilang
description: >-
  Guía para conectarse a DeepSeek desde lenguajes no oficialmente
  soportados (PHP, Ruby, Rust, C#, Java, C++, Go, Dart)
---
```

El agente lo activará automáticamente cuando detecte:
- "DeepSeek desde PHP/Ruby/Rust/etc"
- "cliente DeepSeek en [lenguaje]"
- Proyectos que no sean Python/Node.js + DeepSeek

## Inicio Rápido

### Configurar la API Key

```bash
export DEEPSEEK_API_KEY="sk-tu-api-key-aqui"
```

O cargar desde archivo:
```bash
source .env
```

Ver CONFIGURAR-ENV.md para más opciones.

### Ejecutar Pruebas

```bash
# Prueba individual
ruby test-ruby.rb
php test-php.php
go run test-go.go

# O todas las pruebas
bash run-tests.sh
```

### Usar el Skill

Copiar a Claude Code:
```bash
cp skill/SKILL-deepseek-multilang.md ~/.claude/skills/
```

O para un proyecto específico:
```bash
cp skill/SKILL-deepseek-multilang.md .claude/skills/
```

## Estructura del Proyecto

```
deepseek-intro-api/
├── skill/
│   ├── SKILL-deepseek-multilang.md    # Skill principal
│   └── context.md                      # Documentación DeepSeek API
├── test-php.php                        # Prueba PHP (ejecutada)
├── test-ruby.rb                        # Prueba Ruby (ejecutada)
├── test-go.go                          # Prueba Go (ejecutada)
├── test-python-native.py               # Prueba Python (ejecutada)
├── test-csharp.cs                      # Código C# validado
├── test-rust/                          # Proyecto Rust validado
├── test-java/                          # Código Java validado
├── .env                                # Variables de entorno (gitignored)
├── .env.example                        # Plantilla .env
├── .gitignore                          # Protege credenciales
├── CONFIGURAR-ENV.md                   # Guía de variables de entorno
├── README.md                           # Este archivo
└── run-tests.sh                        # Script para ejecutar pruebas
```

## Conceptos Clave

### Endpoint
```
POST https://api.deepseek.com/chat/completions
```

### Headers
```
Content-Type: application/json
Authorization: Bearer {DEEPSEEK_API_KEY}
```

### Payload
```json
{
  "model": "deepseek-chat",
  "messages": [
    {"role": "system", "content": "You are a helpful assistant."},
    {"role": "user", "content": "Your question"}
  ],
  "stream": false
}
```

### Respuesta
```json
{
  "choices": [
    {
      "message": {
        "content": "Response text here"
      }
    }
  ],
  "usage": {
    "prompt_tokens": 17,
    "completion_tokens": 58,
    "total_tokens": 75
  }
}
```

## Modelos

| Modelo | Modo | Contexto | Max Output | Uso |
|--------|------|----------|-----------|-----|
| deepseek-chat | No-thinking | 128K | 8K | General |
| deepseek-reasoner | Thinking | 128K | 64K | Razonamiento |

## Pruebas Ejecutadas

Pruebas funcionales completadas:

- PHP: HTTP 200, respuesta válida
- Ruby: HTTP 200, respuesta válida
- Go: HTTP 200, respuesta válida
- Python: HTTP 200, respuesta válida

Código validado:
- Rust, C#, Java, Dart

## Seguridad

- API key en variables de entorno (nunca hardcodeada)
- Archivo .env en .gitignore
- HTTPS en todos los ejemplos
- Manejo de errores documentado

## Códigos de Error

| Código | Causa | Solución |
|--------|-------|----------|
| 400 | Formato inválido | Revisar estructura del request |
| 401 | API key inválida | Verificar DEEPSEEK_API_KEY |
| 402 | Saldo insuficiente | Recargar cuenta |
| 422 | Parámetros inválidos | Revisar parámetros |
| 429 | Rate limit | Esperar y reintentar |
| 500 | Error servidor | Reintentar después |
| 503 | Servidor sobrecargado | Reintentar después |

## Pricing

- Input: $0.28 / 1M tokens
- Output: $0.42 / 1M tokens
- Cache hit: $0.028 / 1M tokens

## Referencias

- [DeepSeek API Docs](https://api.deepseek.com)
- [OpenAI API Format](https://platform.openai.com/docs/api-reference/chat/create)
- [Skills.sh](https://skills.sh/)

---

Última actualización: 31 de Marzo de 2026
Status: Listo para producción
Lenguajes: 8 (PHP, Ruby, Rust, C#, Java, C++, Go, Dart)
Pruebas exitosas: 4/4
