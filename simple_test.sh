#!/bin/bash

# Test simple del servidor MyCommandMCP

CONFIG_FILE=${1:-"mycommand-tools.yaml"}

echo "=== Prueba básica del servidor MyCommandMCP ==="
echo "Usando archivo de configuración: $CONFIG_FILE"
echo ""

# Test 1: Inicialización
echo "1. Test de inicialización:"
echo '{"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {}}' | ./target/release/mycommandmcp --config "$CONFIG_FILE"
echo ""

# Test 2: Listar herramientas
echo "2. Test de listado de herramientas:"
echo '{"jsonrpc": "2.0", "id": 2, "method": "tools/list", "params": {}}' | ./target/release/mycommandmcp --config "$CONFIG_FILE"
echo ""

# Test 3: Ejecutar comando simple (date)
echo "3. Test de ejecución - comando date:"
echo '{"jsonrpc": "2.0", "id": 3, "method": "tools/call", "params": {"name": "get_date", "arguments": {}}}' | ./target/release/mycommandmcp --config "$CONFIG_FILE"
echo ""

# Test 4: Ejecutar comando con argumentos (ls)
echo "4. Test de ejecución - comando ls con argumentos:"
echo '{"jsonrpc": "2.0", "id": 4, "method": "tools/call", "params": {"name": "list_files", "arguments": {"args": "-la"}}}' | ./target/release/mycommandmcp --config "$CONFIG_FILE"
echo ""

echo "=== Pruebas completadas ==="
