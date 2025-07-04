#!/bin/bash

echo "=== Manual MCP Test ==="

# Create test requests
cat > /tmp/init_request.json << 'EOF'
{"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {}}
EOF

cat > /tmp/list_request.json << 'EOF'
{"jsonrpc": "2.0", "id": 2, "method": "tools/list", "params": {}}
EOF

echo "Testing initialize..."
timeout 5 ./target/release/mycommandmcp < /tmp/init_request.json

echo -e "\nTesting tools/list..."
timeout 5 ./target/release/mycommandmcp < /tmp/list_request.json

# Cleanup
rm -f /tmp/init_request.json /tmp/list_request.json

echo "Done."
