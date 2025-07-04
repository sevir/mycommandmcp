# MyCommandMCP

An MCP (Model Context Protocol) server written in Rust that allows executing system commands as MCP tools.

## Features

- Reads tool configuration from a customizable YAML file
- Support for specifying configuration file via `--config` parameter
- Executes system commands safely
- Returns results in JSON format with status code, output, and errors
- Compatible with MCP protocol 2024-11-05

## Installation and Usage

1. Make sure you have Rust installed
2. Clone or copy this project
3. Configure your YAML tools file
4. Build and run:

```bash
cargo build --release
```

### Running the server

#### With default configuration file
```bash
./target/release/mycommandmcp
```

#### With custom configuration file
```bash
./target/release/mycommandmcp --config my-configuration.yaml
```

#### View help
```bash
./target/release/mycommandmcp --help
```

## Configuration

The server reads configuration from a YAML file. By default it looks for `mycommand-tools.yaml` in the current directory, but you can specify another file with the `--config` parameter.

### Configuration file structure

```yaml
tools:
  - name: "tool_name"
    description: "Tool description for MCP"
    command: "system_command"
    path: "/path/where/to/execute"
    accepts_args: true/false
    accept_input: true/false
    default_args: "default arguments string"
    content_type: "mime/type"              # Optional: MIME type for file responses
    content_disposition: "attachment; filename=file.ext"  # Optional: Content disposition
```

#### Configuration attributes explained:

- **name**: Unique identifier for the tool
- **description**: Human-readable description shown in MCP
- **command**: The system command to execute
- **path**: Working directory where the command will be executed
- **accepts_args**: Whether the tool accepts additional arguments (true/false)
- **accept_input**: Whether the tool accepts input via stdin (true/false)
- **default_args**: (Optional) Default arguments always applied to the command, concatenated before any additional arguments
- **content_type**: (Optional) MIME type of the command output (e.g., "application/pdf", "image/png", "text/csv")
- **content_disposition**: (Optional) How the content should be handled (e.g., "attachment; filename=report.pdf", "inline")

### Configuration example

```yaml
tools:
  - name: "list_files"
    description: "Lists files in a specific directory"
    command: "ls"
    path: "/"
    accepts_args: true
    accept_input: false
    default_args: "-l"
    
  - name: "get_date"
    description: "Gets the current system date and time"
    command: "date"
    path: "/"
    accepts_args: false
    accept_input: false
    
  - name: "grep_text"
    description: "Search for text patterns using grep from standard input"
    command: "grep"
    path: "/"
    accepts_args: true
    accept_input: true
    default_args: "--color=never"
```

#### How default_args works:
- Default arguments are always applied first when the command is executed
- If additional arguments are provided via the MCP call, they are concatenated after the default arguments
- For example, if `default_args: "-l"` and you provide `args: "-a"`, the final command will be: `ls -l -a`
- If no additional arguments are provided, only the default arguments will be used
- The `default_args` field is optional - if not specified, only user-provided arguments will be used

#### How accept_input works:
- When `accept_input: true`, the tool can receive text input via stdin
- This is useful for commands like `grep`, `wc`, `sort`, etc. that process text streams
- Input is provided through the MCP `input` parameter when calling the tool
- Tools with `accept_input: false` will not accept any stdin input

#### Example MCP tool calls:

1. **Tool with default args only:**
   ```json
   {"jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": {"name": "disk_usage", "arguments": {}}}
   ```
   This executes: `df -h` (using only default_args)

2. **Tool with default args + additional args:**
   ```json
   {"jsonrpc": "2.0", "id": 2, "method": "tools/call", "params": {"name": "list_files", "arguments": {"args": "-a /home"}}}
   ```
   This executes: `ls -l -a /home` (default_args + user args)

3. **Tool with input:**
   ```json
   {"jsonrpc": "2.0", "id": 3, "method": "tools/call", "params": {"name": "grep_text", "arguments": {"args": "pattern", "input": "line1\npattern here\nline3"}}}
   ```
   This executes: `grep --color=never pattern` with input piped to stdin

## Content Types and File Downloads

MyCommandMCP now supports returning command outputs as different content types with proper file handling. This enables tools to return PDFs, images, archives, and other binary content.

### Content Type Configuration

Add optional `content_type` and `content_disposition` fields to enable file downloads:

```yaml
tools:
  - name: "generate_report"
    description: "Generate a PDF report"
    command: "wkhtmltopdf"
    path: "/"
    accepts_args: true
    accept_input: false
    default_args: "- /dev/stdout"
    content_type: "application/pdf"
    content_disposition: "attachment; filename=report.pdf"

  - name: "create_backup"
    description: "Create a system backup archive"
    command: "tar"
    path: "/"
    accepts_args: true
    accept_input: false
    default_args: "-czf - /home"
    content_type: "application/gzip"
    content_disposition: "attachment; filename=backup.tar.gz"

  - name: "export_data_csv"
    description: "Export data as CSV file"
    command: "ps"
    path: "/"
    accepts_args: false
    accept_input: false
    default_args: "aux --no-headers | awk 'BEGIN{print \"USER,PID,CPU,MEM\"} {print $1\",\"$2\",\"$3\",\"$4}'"
    content_type: "text/csv"
    content_disposition: "attachment; filename=processes.csv"
```

### How Content Types Work

- **Text Content**: When no `content_type` is specified or it starts with `text/`, output is returned as plain text
- **Binary Content**: When `content_type` indicates binary content, output is automatically base64 encoded
- **File Response**: Content includes proper MIME type and disposition headers for file handling

For detailed documentation, see [CONTENT_TYPES.md](CONTENT_TYPES.md).

## Included configuration files

- `mycommand-tools.yaml`: Basic configuration with example content-type tools
- `mycommand-tools-extended.yaml`: Extended configuration with content-type examples

## MCP Protocol

The server implements the following MCP methods:

- `initialize`: Initializes the server and returns capabilities
- `tools/list`: Lists all available tools
- `tools/call`: Executes a specific tool

### Response format

When a tool is executed, the server returns a JSON with:

```json
{
  "status_code": 0,
  "output": "command output",
  "error": "errors if any"
}
```

## Security

**IMPORTANT**: This server executes system commands directly. Make sure to:

- Use only in controlled environments
- Configure tools with safe commands
- Do not expose the server to untrusted networks
- Carefully review the YAML configuration

## License

MIT License

## Tasks

This task are automatic scripts to help manage the project

### release

Generate release with goreleaser

```
goreleaser release --snapshot
```