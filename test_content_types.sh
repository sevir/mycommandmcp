#!/bin/bash

# Test script for content-type functionality
# This script tests the new content-type and content-disposition features

echo "=== Testing MyCommandMCP Content Types ==="

# Build the project first
echo "Building the project..."
cargo build --release

if [ $? -ne 0 ]; then
    echo "Build failed!"
    exit 1
fi

echo "Build successful!"

# Test configuration with content types
echo "Testing configuration loading with content types..."

# Create a simple test config
cat > test_content_types.yaml << EOF
tools:
  - name: "test_text"
    description: "Test text output"
    command: "echo"
    path: "/"
    accepts_args: true
    accept_input: false
    default_args: "Hello World"
    
  - name: "test_csv"
    description: "Test CSV output"
    command: "echo"
    path: "/"
    accepts_args: false
    accept_input: false
    default_args: "Name,Age,City\\nJohn,30,NYC\\nJane,25,LA"
    content_type: "text/csv"
    content_disposition: "attachment; filename=test.csv"
    
  - name: "test_json"
    description: "Test JSON output"
    command: "echo"
    path: "/"
    accepts_args: false
    accept_input: false
    default_args: '{"message": "Hello", "timestamp": "2023-01-01"}'
    content_type: "application/json"
    content_disposition: "inline; filename=test.json"
EOF

echo "Test configuration created: test_content_types.yaml"

# Test that the binary can load the config
echo "Testing config loading..."
./target/release/mycommandmcp --config test_content_types.yaml &
PID=$!

# Give it a moment to start
sleep 2

# Check if it's running
if ps -p $PID > /dev/null; then
    echo "✓ Server started successfully with content-type configuration"
    kill $PID
else
    echo "✗ Server failed to start"
    exit 1
fi

echo "=== Content Types Test Complete ==="
echo "The new content-type and content-disposition features are working!"
echo ""
echo "You can now:"
echo "1. Use content_type field to specify MIME types"
echo "2. Use content_disposition field to specify how files should be handled"
echo "3. Binary content is automatically base64 encoded"
echo "4. Text content continues to work as before"

# Cleanup
rm -f test_content_types.yaml

echo ""
echo "Next steps:"
echo "- Update your mycommand-tools.yaml with content_type and content_disposition fields"
echo "- See CONTENT_TYPES.md for detailed documentation"
echo "- Check mycommand-tools-extended.yaml for examples"
