#!/bin/bash

echo "Testing MCP Server - Interactive Mode"
echo "Starting server..."

# Start the server in background and get its PID
./target/release/mycommandmcp > server_output.log 2>&1 &
SERVER_PID=$!

# Give server time to start
sleep 1

echo "Server started with PID: $SERVER_PID"

# Function to send JSON and wait for response
send_json() {
    echo "$1" | nc -q 1 localhost 8080 2>/dev/null || echo "$1" >&${SERVER_FD}
}

# Test 1: Initialize
echo "=== Test 1: Initialize ==="
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{"roots":{"listChanged":true},"sampling":{}},"clientInfo":{"name":"test-client","version":"1.0.0"}}}' > /tmp/test_input.json

# Execute test with direct pipe
cat /tmp/test_input.json | timeout 10 ./target/release/mycommandmcp > /tmp/test_output.json 2>&1

echo "Output:"
cat /tmp/test_output.json

echo -e "\n=== Test 2: Tools List ==="
echo '{"jsonrpc":"2.0","id":2,"method":"tools/list"}' | timeout 10 ./target/release/mycommandmcp > /tmp/test_output2.json 2>&1

echo "Output:"
cat /tmp/test_output2.json

# Clean up
kill $SERVER_PID 2>/dev/null
rm -f /tmp/test_input.json /tmp/test_output.json /tmp/test_output2.json

echo -e "\nTest completed."
