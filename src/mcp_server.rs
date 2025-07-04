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

    pub async fn execute_command(
        &self,
        tool_name: &str,
        args: Option<&str>,
        input: Option<&str>,
    ) -> Result<CommandResult> {
        let tool = self
            .tools
            .get(tool_name)
            .context(format!("Tool '{}' not found", tool_name))?;

        let mut cmd = Command::new(&tool.command);
        cmd.current_dir(&tool.path);

        if tool.accepts_args {
            // First add default args if they exist
            if let Some(default_args_str) = &tool.default_args {
                if !default_args_str.is_empty() {
                    let default_args_vec: Vec<&str> = default_args_str.split_whitespace().collect();
                    cmd.args(default_args_vec);
                }
            }

            // Then add any additional args provided
            if let Some(args_str) = args {
                // Split arguments by whitespace (simplified)
                let args_vec: Vec<&str> = args_str.split_whitespace().collect();
                cmd.args(args_vec);
            }
        }

        // Create tokio process command
        let mut tokio_cmd = tokio::process::Command::from(cmd);

        // Configure stdin if the tool accepts input
        if tool.accept_input {
            tokio_cmd.stdin(std::process::Stdio::piped());
        }

        tokio_cmd.stdout(std::process::Stdio::piped());
        tokio_cmd.stderr(std::process::Stdio::piped());

        // Spawn the process
        let mut child = tokio_cmd
            .spawn()
            .context(format!("Failed to spawn command: {}", tool.command))?;

        // If tool accepts input and input is provided, write to stdin
        if tool.accept_input && input.is_some() {
            if let Some(stdin) = child.stdin.as_mut() {
                if let Some(input_str) = input {
                    use tokio::io::AsyncWriteExt;
                    stdin.write_all(input_str.as_bytes()).await?;
                    stdin.flush().await?;
                }
            }
            // Close stdin to signal end of input
            child.stdin.take();
        }

        // Wait for the process to complete and get output
        let output = child
            .wait_with_output()
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

        let result =
            match method {
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
                        let mut properties = serde_json::Map::new();

                        // Always include args property
                        properties.insert(
                            "args".to_string(),
                            json!({
                                "type": "string",
                                "description": "Arguments for the command (optional)"
                            }),
                        );

                        // Add input property if tool accepts input
                        if tool_config.accept_input {
                            properties.insert("input".to_string(), json!({
                            "type": "string",
                            "description": "Text input to send to the command's standard input"
                        }));
                        }

                        tools.push(json!({
                            "name": tool_config.name,
                            "description": tool_config.description,
                            "inputSchema": {
                                "type": "object",
                                "properties": properties
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

                    let input = arguments
                        .and_then(|args| args.get("input"))
                        .and_then(|v| v.as_str());

                    let result = self.execute_command(tool_name, args, input).await?;

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
