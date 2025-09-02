#!/bin/bash

# Test script to verify prompt file and URL loading functionality
cd /www/MCP/mycommandmcp

echo "Testing MyCommandMCP with prompt file/URL loading feature..."
echo ""

# Create a test prompt file
mkdir -p /tmp/test_prompts
cat > /tmp/test_prompts/test_prompt.md << 'EOF'
# Test Prompt

This is a test prompt loaded from a file.

It contains multiple lines and markdown formatting.

- List item 1
- List item 2

**Bold text** and *italic text*.
EOF

# Start server in background with extended config
# ./target/release/mycommandmcp --config mycommand-tools-extended.yaml &
# SERVER_PID=$!

# Give server time to start
# sleep 3

echo "1. Testing prompts/list method:"
echo '{"jsonrpc": "2.0", "id": 1, "method": "prompts/list", "params": {}}' | timeout 5s ./target/release/mycommandmcp --config mycommand-tools-extended.yaml

echo ""
echo "2. Testing inline content prompt (summarize):"
echo '{"jsonrpc": "2.0", "id": 2, "method": "prompts/get", "params": {"name": "summarize"}}' | timeout 5s ./target/release/mycommandmcp --config mycommand-tools-extended.yaml

echo ""
echo "3. Testing file-based prompt (code_review):"
echo '{"jsonrpc": "2.0", "id": 3, "method": "prompts/get", "params": {"name": "code_review"}}' | timeout 5s ./target/release/mycommandmcp --config mycommand-tools-extended.yaml

echo ""
echo "4. Testing URL-based prompt (documentation):"
echo '{"jsonrpc": "2.0", "id": 4, "method": "prompts/get", "params": {"name": "documentation"}}' | timeout 5s ./target/release/mycommandmcp --config mycommand-tools-extended.yaml

echo ""
echo "5. Testing non-existent prompt:"
echo '{"jsonrpc": "2.0", "id": 5, "method": "prompts/get", "params": {"name": "nonexistent"}}' | timeout 5s ./target/release/mycommandmcp --config mycommand-tools-extended.yaml

# Clean up
# kill $SERVER_PID 2>/dev/null
# rm -rf /tmp/test_prompts
echo ""
echo "Test completed."
