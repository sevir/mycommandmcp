use anyhow::{Context, Result};
use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;
use std::process::Command;

use crate::cli_parser::ToolConfig;

#[derive(Debug, Serialize)]
pub struct CommandResult {
    pub status_code: i32,
    pub output: String,
    pub error: String,
}

#[derive(Clone)]
pub struct MyCommandMCPServer {
    pub tools: HashMap<String, ToolConfig>,
}

impl MyCommandMCPServer {
    pub fn new(tools: HashMap<String, ToolConfig>) -> Self {
        MyCommandMCPServer { tools }
    }

    pub async fn execute_command(&self, tool_name: &str, args: Option<&str>) -> Result<CommandResult> {
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

    pub async fn handle_request(&self, message: &str) -> Result<String> {
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

                // Add individual tools
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
