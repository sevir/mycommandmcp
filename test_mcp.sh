#!/bin/bash

echo "Testing MCP Server..."

# Test 1: Initialize
echo "=== Test 1: Initialize ==="
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{"roots":{"listChanged":true},"sampling":{}},"clientInfo":{"name":"test-client","version":"1.0.0"}}}' | timeout 5 ./target/release/mycommandmcp

echo -e "\n=== Test 2: Tools List ==="
echo '{"jsonrpc":"2.0","id":2,"method":"tools/list"}' | timeout 5 ./target/release/mycommandmcp

echo -e "\n=== Test 3: Call Tool ==="
echo '{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"get_date","arguments":{}}}' | timeout 5 ./target/release/mycommandmcp

echo -e "\nTests completed."
