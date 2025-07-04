#!/bin/bash

# Test script to verify default_args functionality
cd /www/MyCommandMCP/mycommandmcp

echo "Testing MyCommandMCP with default_args feature..."
echo ""

# Start server in background
./target/release/mycommandmcp &
SERVER_PID=$!

# Give server time to start
sleep 2

echo "1. Testing tools/list method:"
echo '{"jsonrpc": "2.0", "id": 1, "method": "tools/list", "params": {}}' | timeout 3s ./target/release/mycommandmcp

echo ""
echo "2. Testing list_files tool (should use default_args: '-l'):"
echo '{"jsonrpc": "2.0", "id": 2, "method": "tools/call", "params": {"name": "list_files", "arguments": {"args": "/tmp"}}}' | timeout 3s ./target/release/mycommandmcp

echo ""
echo "3. Testing disk_usage tool (should use default_args: '-h'):"
echo '{"jsonrpc": "2.0", "id": 3, "method": "tools/call", "params": {"name": "disk_usage", "arguments": {}}}' | timeout 3s ./target/release/mycommandmcp

# Clean up
kill $SERVER_PID 2>/dev/null
echo ""
echo "Test completed."
