#!/bin/bash
# run-tests.sh - Ejecuta pruebas en lenguajes específicos

set -a
source .env
set +a

if [ -z "$DEEPSEEK_API_KEY" ]; then
    echo "Error: DEEPSEEK_API_KEY no está configurada"
    exit 1
fi

echo "=========================================="
echo "PRUEBAS DE SKILL DEEPSEEK"
echo "=========================================="
echo ""

# Test 1: PHP
echo "1. Probando PHP..."
echo "---"
php test-php.php
echo ""

# Test 2: Go
echo "2. Probando Go..."
echo "---"
go run test-go.go
echo ""

# Test 3: Ruby
echo "3. Probando Ruby..."
echo "---"
ruby test-ruby.rb
echo ""

# Test 4: Rust
echo "4. Probando Rust..."
echo "---"
cd test-rust && cargo run --quiet 2>&1 && cd ..
echo ""

# Test 5: Python (para comparación)
echo "5. Probando Python (referencia)..."
echo "---"
python3 test-python-native.py
echo ""

echo "=========================================="
echo "PRUEBAS COMPLETADAS"
echo "=========================================="
