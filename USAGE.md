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

## 🔧 Customize configuration

You can define tools, prompts, and resources in your YAML configuration file.

### Resources section

The `resources` section allows you to expose files as MCP resources. Each resource entry must include:

- **name**: Unique resource identifier
- **description**: Description of the resource
- **path**: Path to the file to be served

If the file is binary, the server will automatically detect the MIME type and encode the content as base64.

#### Example resources section

```yaml
resources:
  - name: "sample_text"
    description: "Returns the content of a sample text file"
    path: "/tmp/sample.txt"

  - name: "sample_pdf"
    description: "Returns a sample PDF file"
    path: "/tmp/sample.pdf"

  - name: "sample_image"
    description: "Returns a sample PNG image"
    path: "/tmp/sample.png"
```

Edit any YAML file or create a new one:

```yaml
tools:
  - name: "my_tool"
    description: "Description for MCP"
    command: "system_command"
    path: "/execution/path"
    accepts_args: true
    accept_input: false
    default_args: "default arguments"
```

### Configuration attributes:

- **name**: Unique tool identifier
- **description**: Tool description for MCP clients
- **command**: System command to execute
- **path**: Working directory for command execution
- **accepts_args**: Whether tool accepts additional arguments (true/false)
- **accept_input**: Whether tool accepts stdin input (true/false)
- **default_args**: (Optional) Default arguments always applied to the command

### Examples with new attributes:

#### Tool with default arguments:
```yaml
- name: "list_files_detailed"
  description: "Lists files with detailed information"
  command: "ls"
  path: "/"
  accepts_args: true
  accept_input: false
  default_args: "-la --color=never"
```

#### Tool that accepts input:
```yaml
- name: "search_text"
  description: "Search for patterns in text input"
  command: "grep"
  path: "/"
  accepts_args: true
  accept_input: true
  default_args: "--color=never -n"
```

#### Using tools with input:
```bash
echo '{"jsonrpc": "2.0", "id": 5, "method": "tools/call", "params": {"name": "search_text", "arguments": {"args": "pattern", "input": "line 1\npattern found\nline 3"}}}' | ./target/release/mycommandmcp
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

## MCP Resource API

- `resources/list`: Lists all available resources
- `resources/get`: Retrieves the content of a resource by name

Example:

```json
{"jsonrpc": "2.0", "id": 10, "method": "resources/get", "params": {"name": "sample_image"}}
```

The server will return the file content with the correct MIME type and encoding.

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
✅ **default_args attribute**: Default arguments automatically applied to commands
✅ **accept_input attribute**: Support for stdin input to commands

### How default_args works:
- Default arguments are applied first, then any additional arguments are concatenated
- Example: `default_args: "-l"` + `args: "-a"` = `ls -l -a`

### How accept_input works:
- Tools with `accept_input: true` can receive text via stdin
- Useful for commands like grep, wc, sort that process text streams
- Provide input using the `input` parameter in MCP tool calls

The MCP server is ready to use with flexible configurations! 🎉
