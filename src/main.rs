mod cli_parser;
mod logging;
mod mcp_server;

use anyhow::{Context, Result};
use clap::Parser;
use serde_json::json;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

use cli_parser::{find_config_file, load_config, Args};
use mcp_server::MyCommandMCPServer;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let config_path = find_config_file(args.config)?;
    let config = load_config(&config_path)?;

    // Initialize logger
    let logger = logging::DualLogger::new(args.log_file.as_deref())
        .context("Failed to initialize logging")?;

    let server = MyCommandMCPServer::new(config, logger);

    server.log("MyCommandMCP Server starting...")?;
    server.log(&format!("Config file: {config_path}"))?;
    server.log(&format!(
        "Loaded {} tools, {} prompts, and {} resources:",
        server.tools.len(),
        server.prompts.len(),
        server.resources.len()
    ))?;

    // Print available tools for debugging
    server.log("Tools:")?;
    for tool in server.tools.values() {
        server.log(&format!(
            "  - {}: {} (path: {}, accepts_args: {}, accept_input: {}, default_args: {:?})",
            tool.name,
            tool.description,
            tool.path,
            tool.accepts_args,
            tool.accept_input,
            tool.default_args
        ))?;
    }

    // Print available prompts for debugging
    server.log("Prompts:")?;
    for prompt in server.prompts.values() {
        server.log(&format!("  - {}: {}", prompt.name, prompt.description))?;
    }

    // Print available resources for debugging
    server.log("Resources:")?;
    for resource in server.resources.values() {
        server.log(&format!(
            "  - {}: {} (path: {})",
            resource.name, resource.description, resource.path
        ))?;
    }

    let stdin = tokio::io::stdin();
    let mut stdout = tokio::io::stdout();
    let mut reader = BufReader::new(stdin);
    let mut line = String::new();

    server.log("Server ready, waiting for MCP requests...")?;

    loop {
        line.clear();
        match reader.read_line(&mut line).await {
            Ok(0) => {
                server.log("EOF received, shutting down server")?;
                break; // EOF
            }
            Ok(_) => {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }

                server.log(&format!("Received input: {line}"))?;

                match server.handle_request(line).await {
                    Ok(response) => {
                        if !response.is_empty() {
                            server.log(&format!("Sending response: {response}"))?;
                            stdout.write_all(response.as_bytes()).await?;
                            stdout.write_all(b"\n").await?;
                            stdout.flush().await?;
                        } else {
                            server.log("No response needed (notification handled)")?;
                        }
                    }
                    Err(e) => {
                        server.log(&format!("Failed to handle request: {e}"))?;
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
                server.log(&format!("Error reading from stdin: {e}"))?;
                break;
            }
        }
    }

    Ok(())
}
