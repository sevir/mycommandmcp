#!/bin/bash

# Test script to verify unified external configuration loading functionality
cd /www/MCP/mycommandmcp

echo "Testing MyCommandMCP unified external configuration loading feature..."
echo ""

# Create test directory
mkdir -p /tmp/test_external_configs

# Create external tools config
cat > /tmp/test_external_configs/external_tools.yaml << 'EOF'
tools:
  - name: "external_list_files"
    description: "External tool: Lists files in a directory"
    command: "ls"
    path: "/"
    accepts_args: true
    accept_input: false
    default_args: "-la"

  - name: "external_disk_usage"
    description: "External tool: Shows disk usage"
    command: "df"
    path: "/"
    accepts_args: true
    accept_input: false
    default_args: "-h"

prompts:
  - name: "external_summarize"
    description: "External prompt: Summarize text"
    content: |
      Please summarize the following text concisely.
      Focus on the main points and key details.

resources:
  - name: "external_sample_text"
    description: "External resource: Sample text file"
    path: "/tmp/sample.txt"
EOF

# Create external prompts config
cat > /tmp/test_external_configs/external_prompts.yaml << 'EOF'
prompts:
  - name: "external_translate"
    description: "External prompt: Translate to French"
    content: |
      Translate the following text to French.
      Maintain the original meaning and tone.

  - name: "external_code_format"
    description: "External prompt: Format code"
    content: |
      Please format the following code according to best practices.
      Ensure proper indentation and naming conventions.

tools:
  - name: "external_get_time"
    description: "External tool: Get current time"
    command: "date"
    path: "/"
    accepts_args: false
    accept_input: false
EOF

# Create external resources config
cat > /tmp/test_external_configs/external_resources.yaml << 'EOF'
resources:
  - name: "external_sample_pdf"
    description: "External resource: Sample PDF"
    path: "/tmp/sample.pdf"

  - name: "external_config_backup"
    description: "External resource: Config backup"
    path: "/tmp/config.bak"
EOF

# Create main config that references external files
cat > /tmp/test_main_config.yaml << 'EOF'
tools:
  - name: "main_list_dir"
    description: "Main config tool: List directory"
    command: "ls"
    path: "/"
    accepts_args: true
    accept_input: false

prompts:
  - name: "main_help"
    description: "Main config prompt: General help"
    content: "How can I assist you today?"

resources:
  - name: "main_readme"
    description: "Main config resource: README"
    path: "/tmp/README.md"

# External configuration lists
external_configs:
  - "/tmp/test_external_configs/external_tools.yaml"
  - "/tmp/test_external_configs/external_prompts.yaml"
  - "/tmp/test_external_configs/external_resources.yaml"
EOF

# Create sample files for resources
echo "This is a sample text file for testing." > /tmp/sample.txt
echo "# Sample README" > /tmp/README.md
echo "Sample PDF content" > /tmp/sample.pdf
echo "Config backup content" > /tmp/config.bak

echo "1. Testing tools/list method (should include tools from main and external configs):"
echo '{"jsonrpc": "2.0", "id": 1, "method": "tools/list", "params": {}}' | timeout 5s ./target/release/mycommandmcp --config /tmp/test_main_config.yaml

echo ""
echo "2. Testing prompts/list method (should include prompts from main and external configs):"
echo '{"jsonrpc": "2.0", "id": 2, "method": "prompts/list", "params": {}}' | timeout 5s ./target/release/mycommandmcp --config /tmp/test_main_config.yaml

echo ""
echo "3. Testing resources/list method (should include resources from main and external configs):"
echo '{"jsonrpc": "2.0", "id": 3, "method": "resources/list", "params": {}}' | timeout 5s ./target/release/mycommandmcp --config /tmp/test_main_config.yaml

echo ""
echo "4. Testing main config tool (main_list_dir):"
echo '{"jsonrpc": "2.0", "id": 4, "method": "tools/call", "params": {"name": "main_list_dir", "arguments": {"args": "/tmp"}}}' | timeout 5s ./target/release/mycommandmcp --config /tmp/test_main_config.yaml

echo ""
echo "5. Testing external config tool (external_list_files):"
echo '{"jsonrpc": "2.0", "id": 5, "method": "tools/call", "params": {"name": "external_list_files", "arguments": {"args": "/tmp"}}}' | timeout 5s ./target/release/mycommandmcp --config /tmp/test_main_config.yaml

echo ""
echo "6. Testing main config prompt (main_help):"
echo '{"jsonrpc": "2.0", "id": 6, "method": "prompts/get", "params": {"name": "main_help"}}' | timeout 5s ./target/release/mycommandmcp --config /tmp/test_main_config.yaml

echo ""
echo "7. Testing external config prompt (external_summarize):"
echo '{"jsonrpc": "2.0", "id": 7, "method": "prompts/get", "params": {"name": "external_summarize"}}' | timeout 5s ./target/release/mycommandmcp --config /tmp/test_main_config.yaml

echo ""
echo "8. Testing main config resource (main_readme):"
echo '{"jsonrpc": "2.0", "id": 8, "method": "resources/read", "params": {"uri": "file://main_readme"}}' | timeout 5s ./target/release/mycommandmcp --config /tmp/test_main_config.yaml

echo ""
echo "9. Testing external config resource (external_sample_text):"
echo '{"jsonrpc": "2.0", "id": 9, "method": "resources/read", "params": {"uri": "file://external_sample_text"}}' | timeout 5s ./target/release/mycommandmcp --config /tmp/test_main_config.yaml

# Clean up
echo ""
echo "Cleaning up test files..."
rm -rf /tmp/test_external_configs
rm -f /tmp/test_main_config.yaml /tmp/sample.txt /tmp/README.md /tmp/sample.pdf /tmp/config.bak

echo "Unified external configuration loading test completed."
