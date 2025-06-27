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
```

### Configuration example

```yaml
tools:
  - name: "list_files"
    description: "Lists files in a specific directory"
    command: "ls"
    path: "/"
    accepts_args: true
    
  - name: "get_date"
    description: "Gets the current system date and time"
    command: "date"
    path: "/"
    accepts_args: false
```

## Included configuration files

- `mycommand-tools.yaml`: Basic configuration with 5 tools
- `mycommand-tools-extended.yaml`: Extended configuration with 12 tools

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
