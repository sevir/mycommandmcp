#!/bin/bash

# Script de demostración de MyCommandMCP con diferentes configuraciones

echo "🚀 Demo de MyCommandMCP - Configuraciones Múltiples"
echo "=================================================="
echo ""

echo "📋 1. Mostrando ayuda del comando:"
./target/release/mycommandmcp --help
echo ""

echo "📋 2. Configuración básica (5 herramientas):"
echo '{"jsonrpc": "2.0", "id": 1, "method": "tools/list", "params": {}}' | timeout 5s ./target/release/mycommandmcp | jq '.result.tools | length'
echo ""

echo "📋 3. Configuración extendida (12 herramientas):"
echo '{"jsonrpc": "2.0", "id": 1, "method": "tools/list", "params": {}}' | timeout 5s ./target/release/mycommandmcp --config mycommand-tools-extended.yaml | jq '.result.tools | length'
echo ""

echo "📋 4. Probando herramienta específica de configuración extendida (current_directory):"
echo '{"jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": {"name": "current_directory", "arguments": {}}}' | timeout 5s ./target/release/mycommandmcp --config mycommand-tools-extended.yaml | jq '.result.content[0].text' | jq '.output'
echo ""

echo "✅ Demo completado!"
