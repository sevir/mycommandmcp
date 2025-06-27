use anyhow::{Context, Result};
use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::process::Command;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

#[derive(Parser)]
#[command(name = "mycommandmcp")]
#[command(about = "A MCP server that executes system commands from YAML configuration")]
#[command(version = "0.1.0")]
struct Args {
    /// Path to the YAML configuration file
    #[arg(short, long, default_value = "mycommand-tools.yaml")]
    config: String,
}

#[derive(Debug, Deserialize, Clone)]
struct Tool {
    name: String,
    description: String,
    command: String,
    path: String,
    accepts_args: bool,
}

#[derive(Debug, Deserialize)]
struct ToolsConfig {
    tools: Vec<Tool>,
}

#[derive(Debug, Serialize)]
struct CommandResult {
    status_code: i32,
    output: String,
    error: String,
}

#[derive(Debug, Deserialize)]
struct MCPRequest {
    #[allow(dead_code)]
    jsonrpc: String,
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

#[derive(Debug, Serialize)]
struct MCPResponse {
    jsonrpc: String,
    id: Option<Value>,
    result: Option<Value>,
    error: Option<MCPError>,
}

#[derive(Debug, Serialize)]
struct MCPError {
    code: i32,
    message: String,
}

struct MCPServer {
    tools: HashMap<String, Tool>,
}

impl MCPServer {
    fn new(config_path: &str) -> Result<Self> {
        let config_content = fs::read_to_string(config_path)
            .context(format!("Failed to read config file: {}", config_path))?;
        
        let config: ToolsConfig = serde_yaml::from_str(&config_content)
            .context("Failed to parse YAML configuration")?;
        
        let mut tools = HashMap::new();
        for tool in config.tools {
            tools.insert(tool.name.clone(), tool);
        }
        
        Ok(MCPServer { tools })
    }
    
    fn execute_command(&self, tool_name: &str, args: Option<&str>) -> Result<CommandResult> {
        let tool = self.tools.get(tool_name)
            .context(format!("Tool '{}' not found", tool_name))?;
        
        let mut cmd = Command::new(&tool.command);
        cmd.current_dir(&tool.path);
        
        if tool.accepts_args {
            if let Some(args_str) = args {
                // Dividir los argumentos por espacios (simplificado)
                let args_vec: Vec<&str> = args_str.split_whitespace().collect();
                cmd.args(args_vec);
            }
        }
        
        let output = cmd.output()
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
    
    fn handle_list_tools(&self) -> Value {
        let tools: Vec<Value> = self.tools.values().map(|tool| {
            serde_json::json!({
                "name": tool.name,
                "description": tool.description,
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "args": {
                            "type": "string",
                            "description": "Argumentos para el comando"
                        }
                    }
                }
            })
        }).collect();
        
        serde_json::json!({
            "tools": tools
        })
    }
    
    fn handle_call_tool(&self, params: &Value) -> Result<Value> {
        let tool_name = params["name"].as_str()
            .context("Missing tool name")?;
        
        let args = params["arguments"]
            .get("args")
            .and_then(|v| v.as_str());
        
        let result = self.execute_command(tool_name, args)?;
        
        Ok(serde_json::json!({
            "content": [{
                "type": "text",
                "text": serde_json::to_string_pretty(&result)?
            }]
        }))
    }
    
    fn handle_request(&self, request: MCPRequest) -> MCPResponse {
        let result = match request.method.as_str() {
            "tools/list" => {
                Some(self.handle_list_tools())
            },
            "tools/call" => {
                match request.params.as_ref().and_then(|p| self.handle_call_tool(p).ok()) {
                    Some(result) => Some(result),
                    None => return MCPResponse {
                        jsonrpc: "2.0".to_string(),
                        id: request.id,
                        result: None,
                        error: Some(MCPError {
                            code: -32000,
                            message: "Failed to execute tool".to_string(),
                        }),
                    },
                }
            },
            "initialize" => {
                Some(serde_json::json!({
                    "protocolVersion": "2024-11-05",
                    "capabilities": {
                        "tools": {}
                    },
                    "serverInfo": {
                        "name": "mycommandmcp",
                        "version": "0.1.0"
                    }
                }))
            },
            _ => return MCPResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: None,
                error: Some(MCPError {
                    code: -32601,
                    message: "Method not found".to_string(),
                }),
            },
        };
        
        MCPResponse {
            jsonrpc: "2.0".to_string(),
            id: request.id,
            result,
            error: None,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    let server = MCPServer::new(&args.config)?;
    
    eprintln!("MyCommandMCP Server starting...");
    eprintln!("Config file: {}", args.config);
    eprintln!("Loaded {} tools", server.tools.len());
    
    let stdin = tokio::io::stdin();
    let mut stdout = tokio::io::stdout();
    let mut reader = BufReader::new(stdin);
    let mut line = String::new();
    
    loop {
        line.clear();
        match reader.read_line(&mut line).await {
            Ok(0) => break, // EOF
            Ok(_) => {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }
                
                match serde_json::from_str::<MCPRequest>(line) {
                    Ok(request) => {
                        let response = server.handle_request(request);
                        let response_json = serde_json::to_string(&response)?;
                        stdout.write_all(response_json.as_bytes()).await?;
                        stdout.write_all(b"\n").await?;
                        stdout.flush().await?;
                    },
                    Err(e) => {
                        eprintln!("Failed to parse request: {}", e);
                        let error_response = MCPResponse {
                            jsonrpc: "2.0".to_string(),
                            id: None,
                            result: None,
                            error: Some(MCPError {
                                code: -32700,
                                message: "Parse error".to_string(),
                            }),
                        };
                        let response_json = serde_json::to_string(&error_response)?;
                        stdout.write_all(response_json.as_bytes()).await?;
                        stdout.write_all(b"\n").await?;
                        stdout.flush().await?;
                    }
                }
            },
            Err(e) => {
                eprintln!("Error reading from stdin: {}", e);
                break;
            }
        }
    }
    
    Ok(())
}
