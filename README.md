# MyCommandMCP

An MCP (Model Context Protocol) server written in Rust that allows executing system commands as MCP tools.

## Features

- Reads tool configuration from a customizable YAML file
- Support for specifying configuration file via `--config` parameter
- Executes system commands safely
- Returns results in JSON format with status code, output, and errors
- Compatible with MCP protocol 2024-11-05
- Supports configurable prompt templates

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

The configuration file supports three main sections: `prompts`, `tools`, and `resources`.

```yaml
prompts:
  - name: "prompt_name"
    description: "Description of what the prompt does"
    content: |
      Multi-line prompt template content
      Can contain instructions and formatting
      Supports multiple lines with proper indentation

  # Alternative: Load content from a local file
  - name: "file_prompt"
    description: "Prompt loaded from a markdown file"
    path: "/path/to/prompt.md"

  # Alternative: Load content from a URL
  - name: "url_prompt"
    description: "Prompt loaded from a remote URL"
    url: "https://example.com/prompt.md"

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
  - **resources**: List of MCP resources to serve files directly by name

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

### Resources section

The `resources` section allows you to define named MCP resources that serve files directly. Each resource must specify:

- **name**: Unique identifier for the resource
- **description**: Human-readable description
- **path**: Path to the file to be served

If the file is binary, the server will automatically detect the correct content-type and return the file as base64-encoded data.

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

### Prompts Configuration

The `prompts` section allows you to define reusable prompt templates. Each prompt must specify:

- **name**: Unique identifier for the prompt
- **description**: Human-readable description shown in MCP

And exactly one of the following content sources:

- **content**: Inline prompt content as a string (supports multi-line with `|` YAML syntax)
- **path**: Path to a local file containing the prompt content (supports markdown files)
- **url**: URL to fetch the prompt content from (supports remote markdown files)

#### Example prompts section

```yaml
prompts:
  - name: "summarize"
    description: "Summarize a given text"
    content: |
      Please summarize the following text in 3 sentences or less.
      Consider the main points and key details.
      Maintain a clear and concise style.

  - name: "code_review"
    description: "Review code for best practices"
    path: "/home/user/prompts/code_review.md"

  - name: "translate"
    description: "Translate text to Spanish"
    url: "https://raw.githubusercontent.com/example/repo/main/prompts/translate.md"
```

#### Content Loading Behavior

- **Inline content**: Loaded directly from the YAML configuration
- **File path**: Content is read from the local file system when the prompt is requested
- **URL**: Content is fetched from the remote URL when the prompt is requested

If loading from a file or URL fails, the server will return an error for that specific prompt request.

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

## MCP Resource API

- `resources/list`: Lists all available resources with their names and descriptions
- `resources/get`: Retrieves the content of a specific resource by name

Example MCP resource call:

```json
{"jsonrpc": "2.0", "id": 10, "method": "resources/get", "params": {"name": "sample_pdf"}}
```

The server will return the file content with the correct MIME type and encoding.

## Included configuration files

- `mycommand-tools.yaml`: Basic configuration with example content-type tools
- `mycommand-tools-extended.yaml`: Extended configuration with content-type examples

## MCP Protocol

The server implements the following MCP methods:

- `initialize`: Initializes the server and returns capabilities
- `tools/list`: Lists all available tools
- `tools/call`: Executes a specific tool
- `prompts/list`: Lists all available prompts with their names and descriptions
- `prompts/get`: Retrieves the full content of a specific prompt by name

### Prompts API

#### List Prompts
```json
{"jsonrpc": "2.0", "id": 1, "method": "prompts/list"}
```
Response:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "prompts": [
      {
        "name": "summarize",
        "description": "Summarize a given text"
      },
      {
        "name": "translate",
        "description": "Translate text to Spanish"
      }
    ]
  }
}
```

#### Get Prompt
```json
{"jsonrpc": "2.0", "id": 2, "method": "prompts/get", "params": {"name": "summarize"}}
```
Response:
```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "result": {
    "name": "summarize",
    "description": "Summarize a given text",
    "content": "Please summarize the following text in 3 sentences or less.\nConsider the main points and key details.\nMaintain a clear and concise style."
  }
}
```

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

interactive: true

```
# goreleaser release --snapshot
cargo build --release
bash build-windows.sh
```

### check

Check code with clippy and fmt

interactive: true

```
cargo check
cargo clippy
cargo fmt
```

### build

Build the project

interactive: true

```
cargo build --release
```