use anyhow::{Context, Result};
use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

#[derive(Parser)]
#[command(name = "mycommandmcp")]
#[command(about = "A MCP server that executes system commands from YAML configuration")]
#[command(version = "0.1.0")]
struct Args {
    /// Path to the YAML configuration file
    #[arg(short, long)]
    config: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
struct ToolConfig {
    name: String,
    description: String,
    command: String,
    path: String,
    accepts_args: bool,
}

#[derive(Debug, Deserialize)]
struct ToolsConfig {
    tools: Vec<ToolConfig>,
}

#[derive(Debug, Serialize)]
struct CommandResult {
    status_code: i32,
    output: String,
    error: String,
}

#[derive(Clone)]
struct MyCommandMCPServer {
    tools: HashMap<String, ToolConfig>,
}

impl MyCommandMCPServer {
    fn new(config_path: &str) -> Result<Self> {
        let config_content = fs::read_to_string(config_path)
            .context(format!("Failed to read config file: {}", config_path))?;

        let config: ToolsConfig =
            serde_yaml::from_str(&config_content).context("Failed to parse YAML configuration")?;

        let mut tools = HashMap::new();
        for tool in config.tools {
            tools.insert(tool.name.clone(), tool);
        }

        Ok(MyCommandMCPServer { tools })
    }

    async fn execute_command(&self, tool_name: &str, args: Option<&str>) -> Result<CommandResult> {
        let tool = self
            .tools
            .get(tool_name)
            .context(format!("Tool '{}' not found", tool_name))?;

        let mut cmd = Command::new(&tool.command);
        cmd.current_dir(&tool.path);

        if tool.accepts_args {
            if let Some(args_str) = args {
                // Split arguments by whitespace (simplified)
                let args_vec: Vec<&str> = args_str.split_whitespace().collect();
                cmd.args(args_vec);
            }
        }

        // Use tokio::process::Command for async execution
        let output = tokio::process::Command::from(cmd)
            .output()
            .await
            .context(format!("Failed to execute command: {}", tool.command))?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let status_code = output.status.code().unwrap_or(-1);

        Ok(CommandResult {
            status_code,
            output: stdout,
            error: stderr,
        })
    }

    async fn handle_request(&self, message: &str) -> Result<String> {
        let request: serde_json::Value = serde_json::from_str(message)?;

        let method = request["method"].as_str().unwrap_or("");
        let id = request.get("id").cloned();

        let result = match method {
            "initialize" => {
                json!({
                    "protocolVersion": "2024-11-05",
                    "capabilities": {
                        "tools": {}
                    },
                    "serverInfo": {
                        "name": "mycommandmcp",
                        "version": "0.1.0"
                    }
                })
            }
            "initialized" => json!({}),
            "tools/list" => {
                let mut tools = Vec::new();

                // Add a general execute_tool
                tools.push(json!({
                    "name": "execute_tool",
                    "description": format!(
                        "Execute any configured system command with optional arguments. Available tools: {}",
                        self.tools.keys().cloned().collect::<Vec<_>>().join(", ")
                    ),
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "tool_name": {
                                "type": "string",
                                "description": format!("Name of the tool to execute. Available tools: {}",
                                    self.tools.keys().cloned().collect::<Vec<_>>().join(", "))
                            },
                            "args": {
                                "type": "string",
                                "description": "Arguments to pass to the command (optional)"
                            }
                        },
                        "required": ["tool_name"]
                    }
                }));

                // Add individual tools for backward compatibility
                for tool_config in self.tools.values() {
                    tools.push(json!({
                        "name": tool_config.name,
                        "description": tool_config.description,
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "args": {
                                    "type": "string",
                                    "description": "Arguments for the command (optional)"
                                }
                            }
                        }
                    }));
                }

                json!({ "tools": tools })
            }
            "tools/call" => {
                let params = request["params"]
                    .as_object()
                    .context("Missing params in tools/call request")?;

                let tool_name = params["name"]
                    .as_str()
                    .context("Missing tool name in request")?;

                if tool_name == "execute_tool" {
                    let arguments = params["arguments"]
                        .as_object()
                        .context("Missing arguments in execute_tool call")?;

                    let target_tool = arguments["tool_name"]
                        .as_str()
                        .context("Missing tool_name in execute_tool arguments")?;

                    let args = arguments.get("args").and_then(|v| v.as_str());

                    let result = self.execute_command(target_tool, args).await?;

                    json!({
                        "content": [{
                            "type": "text",
                            "text": serde_json::to_string_pretty(&result)?
                        }],
                        "isError": result.status_code != 0
                    })
                } else {
                    let arguments = params.get("arguments").and_then(|v| v.as_object());
                    let args = arguments
                        .and_then(|args| args.get("args"))
                        .and_then(|v| v.as_str());

                    let result = self.execute_command(tool_name, args).await?;

                    json!({
                        "content": [{
                            "type": "text",
                            "text": serde_json::to_string_pretty(&result)?
                        }],
                        "isError": result.status_code != 0
                    })
                }
            }
            _ => {
                return Ok(json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "error": {
                        "code": -32601,
                        "message": format!("Method not found: {}", method)
                    }
                })
                .to_string());
            }
        };

        Ok(json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": result
        })
        .to_string())
    }
}

/// Find the configuration file in the appropriate location based on the OS
fn find_config_file(explicit_path: Option<String>) -> Result<String> {
    // If explicit path is provided, use it
    if let Some(path) = explicit_path {
        if Path::new(&path).exists() {
            return Ok(path);
        } else {
            return Err(anyhow::anyhow!("Specified config file not found: {}", path));
        }
    }

    const CONFIG_FILENAME: &str = "mycommand-tools.yaml";

    // First, check current directory
    if Path::new(CONFIG_FILENAME).exists() {
        return Ok(CONFIG_FILENAME.to_string());
    }

    // Check platform-specific config directories
    if cfg!(target_os = "windows") {
        // Windows: AppData\Roaming\
        if let Some(mut config_dir) = dirs::config_dir() {
            config_dir.push(CONFIG_FILENAME);
            if config_dir.exists() {
                return Ok(config_dir.to_string_lossy().to_string());
            }
        }
    } else {
        // Linux/macOS: $HOME/.config/
        if let Some(mut config_dir) = dirs::config_dir() {
            config_dir.push(CONFIG_FILENAME);
            if config_dir.exists() {
                return Ok(config_dir.to_string_lossy().to_string());
            }
        }
    }

    // If not found anywhere, default to current directory
    Ok(CONFIG_FILENAME.to_string())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let config_path = find_config_file(args.config)?;
    let server = MyCommandMCPServer::new(&config_path)?;

    eprintln!("MyCommandMCP Server starting...");
    eprintln!("Config file: {}", config_path);
    eprintln!("Loaded {} tools:", server.tools.len());

    // Print available tools for debugging
    for tool in server.tools.values() {
        eprintln!(
            "  - {}: {} (path: {}, accepts_args: {})",
            tool.name, tool.description, tool.path, tool.accepts_args
        );
    }

    let stdin = tokio::io::stdin();
    let mut stdout = tokio::io::stdout();
    let mut reader = BufReader::new(stdin);
    let mut line = String::new();

    eprintln!("Server ready, waiting for MCP requests...");

    loop {
        line.clear();
        match reader.read_line(&mut line).await {
            Ok(0) => {
                eprintln!("EOF received, shutting down server");
                break; // EOF
            }
            Ok(_) => {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }

                eprintln!("Received input: {}", line);

                match server.handle_request(line).await {
                    Ok(response) => {
                        eprintln!("Sending response: {}", response);
                        stdout.write_all(response.as_bytes()).await?;
                        stdout.write_all(b"\n").await?;
                        stdout.flush().await?;
                    }
                    Err(e) => {
                        eprintln!("Failed to handle request: {}", e);
                        let error_response = json!({
                            "jsonrpc": "2.0",
                            "id": null,
                            "error": {
                                "code": -32700,
                                "message": format!("Parse error: {}", e)
                            }
                        });
                        let response_json = error_response.to_string();
                        stdout.write_all(response_json.as_bytes()).await?;
                        stdout.write_all(b"\n").await?;
                        stdout.flush().await?;
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading from stdin: {}", e);
                break;
            }
        }
    }

    Ok(())
}
