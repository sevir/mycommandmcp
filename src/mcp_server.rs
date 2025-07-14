use anyhow::{Context, Result};
use base64::Engine;
use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;
use std::process::Command;

use crate::cli_parser::{ConfigData, PromptConfig, ResourceConfig, ToolConfig};
use crate::logging::DualLogger;

#[derive(Debug, Serialize)]
pub struct CommandResult {
    pub status_code: i32,
    pub output: String,
    pub error: String,
    pub content_type: Option<String>,
    pub content_disposition: Option<String>,
    pub is_binary: bool,
}

pub struct MyCommandMCPServer {
    pub tools: HashMap<String, ToolConfig>,
    pub prompts: HashMap<String, PromptConfig>,
    pub resources: HashMap<String, ResourceConfig>,
    logger: DualLogger,
}

impl MyCommandMCPServer {
    pub fn new(config: ConfigData, logger: DualLogger) -> Self {
        MyCommandMCPServer {
            tools: config.tools,
            prompts: config.prompts,
            resources: config.resources,
            logger,
        }
    }

    pub fn log(&self, message: &str) -> Result<()> {
        self.logger.log(message).context("Failed to write log")
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
            .context(format!("Tool '{tool_name}' not found"))?;

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
        let mut child = match tokio_cmd.spawn() {
            Ok(child) => child,
            Err(e) => {
                self.log(&format!("Failed to spawn command {}: {}", tool.command, e))?;
                return Err(e.into());
            }
        };

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
        let output = match child.wait_with_output().await {
            Ok(output) => output,
            Err(e) => {
                self.log(&format!(
                    "Failed to execute command {}: {}",
                    tool.command, e
                ))?;
                return Err(e.into());
            }
        };

        // Log command execution result
        self.log(&format!(
            "Command '{}' completed with status: {}",
            tool.command, output.status
        ))?;

        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let status_code = output.status.code().unwrap_or(-1);

        // Determine if output should be treated as binary based on content type
        let is_binary = tool
            .content_type
            .as_ref()
            .map(|ct| {
                !ct.starts_with("text/") && ct != "application/json" && ct != "application/xml"
            })
            .unwrap_or(false);

        let stdout = if is_binary {
            // For binary content, encode as base64
            base64::engine::general_purpose::STANDARD.encode(&output.stdout)
        } else {
            // For text content, convert to string as usual
            String::from_utf8_lossy(&output.stdout).to_string()
        };

        Ok(CommandResult {
            status_code,
            output: stdout,
            error: stderr,
            content_type: tool.content_type.clone(),
            content_disposition: tool.content_disposition.clone(),
            is_binary,
        })
    }

    pub async fn handle_request(&self, message: &str) -> Result<String> {
        let request: serde_json::Value = serde_json::from_str(message)?;

        let method = request["method"].as_str().unwrap_or("");
        let id = request.get("id").cloned();

        // Handle notifications (no response needed)
        if method.starts_with("notifications/") {
            self.log(&format!("Received notification: {}", method))?;
            return Ok(String::new()); // Empty response for notifications
        }

        let result = match method {
            "resources/list" => {
                let mut resources = Vec::new();
                for resource_config in self.resources.values() {
                    resources.push(json!({
                        "uri": format!("file://{}", resource_config.name),
                        "name": resource_config.name,
                        "description": resource_config.description,
                        "mimeType": if resource_config.path.ends_with(".json") {
                            "application/json"
                        } else if resource_config.path.ends_with(".txt") {
                            "text/plain"
                        } else {
                            "application/octet-stream"
                        }
                    }));
                }
                json!({ "resources": resources })
            }
            "resources/read" => {
                let params = request["params"]
                    .as_object()
                    .context("Missing params in resources/read request")?;
                let resource_uri = params["uri"]
                    .as_str()
                    .context("Missing resource uri in request")?;

                // Extract resource name from URI (remove file:// prefix if present)
                let resource_name = if resource_uri.starts_with("file://") {
                    &resource_uri[7..]
                } else {
                    resource_uri
                };

                let resource = self
                    .resources
                    .get(resource_name)
                    .context(format!("Resource '{resource_name}' not found"))?;

                self.log(&format!(
                    "Fetching resource '{}' from path: {}",
                    resource_name, resource.path
                ))?;

                let (content, mime_type) = if resource.path.starts_with("http://")
                    || resource.path.starts_with("https://")
                {
                    // Handle URL resources
                    self.log(&format!("Fetching URL: {}", resource.path))?;
                    let response = reqwest::get(&resource.path).await.map_err(|e| {
                        anyhow::anyhow!("Failed to fetch URL '{}': {}", resource.path, e)
                    })?;

                    let content_type = response
                        .headers()
                        .get("content-type")
                        .and_then(|ct| ct.to_str().ok())
                        .unwrap_or("application/octet-stream");

                    let mime_type = content_type
                        .split(';')
                        .next()
                        .unwrap_or("application/octet-stream")
                        .to_string();
                    let content = response
                        .bytes()
                        .await
                        .map_err(|e| anyhow::anyhow!("Failed to read response body: {}", e))?
                        .to_vec();

                    self.log(&format!(
                        "Successfully fetched {} bytes from URL, content-type: {}",
                        content.len(),
                        mime_type
                    ))?;
                    (content, mime_type)
                } else {
                    // Handle local file resources
                    use std::fs;
                    use std::path::Path;
                    let path = Path::new(&resource.path);
                    self.log(&format!("Reading local file: {}", path.display()))?;
                    let content = fs::read(path).map_err(|e| {
                        anyhow::anyhow!("Failed to read resource file '{}': {}", resource.path, e)
                    })?;
                    let mime_type = mime_guess::from_path(path)
                        .first_or_octet_stream()
                        .essence_str()
                        .to_string();

                    self.log(&format!(
                        "Successfully read {} bytes from file, mime-type: {}",
                        content.len(),
                        mime_type
                    ))?;
                    (content, mime_type)
                };

                let is_binary = !mime_type.starts_with("text/")
                    && mime_type != "application/json"
                    && mime_type != "application/xml";

                self.log(&format!(
                    "Resource '{}' processed successfully as {}",
                    resource_name,
                    if is_binary { "binary" } else { "text" }
                ))?;

                if is_binary {
                    let encoded_content =
                        base64::engine::general_purpose::STANDARD.encode(&content);
                    json!({
                        "contents": [{
                            "uri": resource_uri,
                            "mimeType": mime_type,
                            "blob": encoded_content
                        }]
                    })
                } else {
                    let text_content = String::from_utf8_lossy(&content).to_string();
                    json!({
                        "contents": [{
                            "uri": resource_uri,
                            "mimeType": mime_type,
                            "text": text_content
                        }]
                    })
                }
            }
            "initialize" => {
                json!({
                    "protocolVersion": "2024-11-05",
                    "capabilities": {
                        "tools": {},
                        "prompts": {},
                        "resources": {}
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
                        properties.insert(
                            "input".to_string(),
                            json!({
                                "type": "string",
                                "description": "Text input to send to the command's standard input"
                            }),
                        );
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

                // Create response based on content type
                if result.is_binary || result.content_type.is_some() {
                    // Return as resource/file content
                    let mut content = json!({
                        "type": "resource",
                        "resource": {
                            "uri": format!("data:{}", result.content_type.as_deref().unwrap_or("application/octet-stream")),
                            "mimeType": result.content_type.as_deref().unwrap_or("application/octet-stream")
                        }
                    });

                    // If binary, the output is already base64 encoded
                    if result.is_binary {
                        content["resource"]["uri"] = json!(format!(
                            "data:{};base64,{}",
                            result
                                .content_type
                                .as_deref()
                                .unwrap_or("application/octet-stream"),
                            result.output
                        ));
                    } else {
                        content["resource"]["text"] = json!(result.output);
                    }

                    // Add content disposition if available
                    if let Some(disposition) = &result.content_disposition {
                        content["resource"]["contentDisposition"] = json!(disposition);
                    }

                    json!({
                        "content": [content],
                        "isError": result.status_code != 0
                    })
                } else {
                    // Return as plain text (existing behavior)
                    json!({
                        "content": [{
                            "type": "text",
                            "text": serde_json::to_string_pretty(&result)?
                        }],
                        "isError": result.status_code != 0
                    })
                }
            }
            "prompts/list" => {
                let prompts = self
                    .prompts
                    .values()
                    .map(|p| {
                        json!({
                            "name": p.name,
                            "description": p.description
                        })
                    })
                    .collect::<Vec<_>>();

                json!({ "prompts": prompts })
            }

            "prompts/get" => {
                let params = request["params"]
                    .as_object()
                    .context("Missing params in prompts/get request")?;

                let prompt_name = params["name"]
                    .as_str()
                    .context("Missing prompt name in request")?;

                if let Some(prompt) = self.prompts.get(prompt_name) {
                    json!({
                        "name": prompt.name,
                        "description": prompt.description,
                        "messages": [{
                            "role": "user",
                            "content": {
                                "type": "text",
                                "text": prompt.content
                            }
                        }]
                    })
                } else {
                    return Ok(json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "error": {
                            "code": -32000,
                            "message": format!("Prompt not found: {}", prompt_name)
                        }
                    })
                    .to_string());
                }
            }

            "resources/templates/list" => {
                // Return empty templates list since we don't support templates
                json!({ "resourceTemplates": [] })
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
