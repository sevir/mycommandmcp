# Usage Instructions - MyCommandMCP

## ✅ Project Status
The **MyCommandMCP** project has been successfully compiled and is working correctly with support for customizable configuration files.

## 🚀 How to run the server

### Compile the project
```bash
cargo build --release
```

### Run the server

#### With default configuration
```bash
./target/release/mycommandmcp
```

#### With custom configuration file
```bash
./target/release/mycommandmcp --config my-config.yaml
```

#### View available options
```bash
./target/release/mycommandmcp --help
```

The server listens on stdin and responds on stdout following the MCP protocol.

## 🧪 Test the server

### Quick test with included script

#### With default configuration
```bash
./simple_test.sh
```

#### With extended configuration
```bash
./simple_test.sh mycommand-tools-extended.yaml
```

### Manual tests

#### 1. Initialize the server (default configuration)
```bash
echo '{"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {}}' | ./target/release/mycommandmcp
```

#### 2. Initialize with specific configuration
```bash
echo '{"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {}}' | ./target/release/mycommandmcp --config mycommand-tools-extended.yaml
```

#### 3. List available tools
```bash
echo '{"jsonrpc": "2.0", "id": 2, "method": "tools/list", "params": {}}' | ./target/release/mycommandmcp --config my-config.yaml
```

#### 4. Execute a tool without arguments
```bash
echo '{"jsonrpc": "2.0", "id": 3, "method": "tools/call", "params": {"name": "get_date", "arguments": {}}}' | ./target/release/mycommandmcp --config my-config.yaml
```

#### 5. Execute a tool with arguments
```bash
echo '{"jsonrpc": "2.0", "id": 4, "method": "tools/call", "params": {"name": "list_files", "arguments": {"args": "-la"}}}' | ./target/release/mycommandmcp --config my-config.yaml
```

## 📋 Available configuration files

### Basic configuration (`mycommand-tools.yaml`)
- **5 tools**: list_files, get_date, disk_usage, process_list, network_info

### Extended configuration (`mycommand-tools-extended.yaml`) 
- **12 tools**: Includes the basic ones plus:
  - current_directory, memory_info, network_interfaces, ping_host
  - file_content, file_info, find_files, grep_text

## 🔧 Customize tools

Edit any YAML file or create a new one:

```yaml
tools:
  - name: "my_tool"
    description: "Description for MCP"
    command: "system_command"
    path: "/execution/path"
    accepts_args: true
```

Then run:
```bash
./target/release/mycommandmcp --config my-file.yaml
```

## 📊 Response format

Each tool execution returns:

```json
{
  "status_code": 0,
  "output": "command output",
  "error": "errors if any"
}
```

## ⚠️ Security

- Only use in controlled environments
- Carefully review configured tools
- Commands are executed with the permissions of the user running the server

## 📁 Project structure

```
mycommandmcp/
├── src/
│   └── main.rs                    # Main server code
├── Cargo.toml                     # Rust configuration
├── mycommand-tools.yaml           # Basic configuration (5 tools)
├── mycommand-tools-extended.yaml  # Extended configuration (12 tools)
├── simple_test.sh                 # Test script
├── demo.sh                        # Demo script
├── test_server.sh                 # Advanced test script
├── README.md                      # Documentation
└── USAGE.md                       # This file
```

## 🎯 New features

✅ **--config parameter**: Specify any YAML configuration file
✅ **Multiple configurations**: Basic and extended included
✅ **Updated scripts**: Support for custom configuration files

The MCP server is ready to use with flexible configurations! 🎉
