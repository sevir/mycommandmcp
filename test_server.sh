#!/bin/bash

# Script de prueba para el servidor MyCommandMCP

echo "Iniciando prueba del servidor MyCommandMCP..."

# Función para enviar una solicitud JSON al servidor
send_request() {
    echo "$1" | cargo run --quiet
}

echo "1. Inicializando el servidor..."
INIT_REQUEST='{"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {}}'
send_request "$INIT_REQUEST"

echo -e "\n2. Listando herramientas disponibles..."
LIST_REQUEST='{"jsonrpc": "2.0", "id": 2, "method": "tools/list", "params": {}}'
send_request "$LIST_REQUEST"

echo -e "\n3. Ejecutando herramienta get_date..."
DATE_REQUEST='{"jsonrpc": "2.0", "id": 3, "method": "tools/call", "params": {"name": "get_date", "arguments": {}}}'
send_request "$DATE_REQUEST"

echo -e "\n4. Ejecutando herramienta list_files con argumentos..."
LIST_FILES_REQUEST='{"jsonrpc": "2.0", "id": 4, "method": "tools/call", "params": {"name": "list_files", "arguments": {"args": "-la"}}}'
send_request "$LIST_FILES_REQUEST"

echo -e "\nPrueba completada."
