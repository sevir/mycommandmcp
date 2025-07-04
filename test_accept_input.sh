#!/bin/bash

# Test script for the accept_input feature

echo "Testing the new accept_input feature..."
echo

# Build the project
echo "Building project..."
cargo build --release

echo "Testing with the example configuration..."
echo

# Create a simple test input
echo -e "line 1\nline 3\nline 2" > test_input.txt

echo "Test input file contents:"
cat test_input.txt
echo

echo "Now you can test the tools with input by using the test_input_example.yaml configuration"
echo "Example MCP requests you can try:"
echo
echo "1. Count lines with input:"
echo '{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "count_lines",
    "arguments": {
      "args": "-l",
      "input": "line 1\nline 2\nline 3"
    }
  }
}'
echo
echo "2. Sort lines with input:"
echo '{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "tools/call",
  "params": {
    "name": "sort_lines",
    "arguments": {
      "input": "line 3\nline 1\nline 2"
    }
  }
}'
echo
echo "3. Convert to uppercase:"
echo '{
  "jsonrpc": "2.0",
  "id": 3,
  "method": "tools/call",
  "params": {
    "name": "uppercase_text",
    "arguments": {
      "args": "a-z A-Z",
      "input": "hello world"
    }
  }
}'

# Clean up
rm -f test_input.txt
