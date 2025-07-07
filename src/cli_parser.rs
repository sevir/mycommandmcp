use anyhow::{Context, Result};
use clap::Parser;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(name = "mycommandmcp")]
#[command(about = "A MCP server that executes system commands from YAML configuration")]
#[command(version = env!("CARGO_PKG_VERSION"))]
pub struct Args {
    /// Path to the YAML configuration file
    #[arg(short, long)]
    pub config: Option<String>,

    /// Path to the log file (if specified, logs will be written to both file and terminal)
    #[arg(short, long)]
    pub log_file: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PromptConfig {
    pub name: String,
    pub description: String,
    pub content: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ToolConfig {
    pub name: String,
    pub description: String,
    pub command: String,
    pub path: String,
    pub accepts_args: bool,
    pub accept_input: bool,
    #[serde(default)]
    pub default_args: Option<String>,
    #[serde(default)]
    pub content_type: Option<String>,
    #[serde(default)]
    pub content_disposition: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ToolsConfig {
    pub tools: Vec<ToolConfig>,
    #[serde(default)]
    pub prompts: Vec<PromptConfig>,
}

pub struct ConfigData {
    pub tools: HashMap<String, ToolConfig>,
    pub prompts: HashMap<String, PromptConfig>,
}

/// Find the configuration file in the appropriate location based on the OS
pub fn find_config_file(explicit_path: Option<String>) -> Result<String> {
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

/// Load and parse the configuration file
pub fn load_config(config_path: &str) -> Result<ConfigData> {
    let config_content = fs::read_to_string(config_path)
        .context(format!("Failed to read config file: {}", config_path))?;

    let config: ToolsConfig =
        serde_yaml::from_str(&config_content).context("Failed to parse YAML configuration")?;

    let mut tools = HashMap::new();
    for tool in config.tools {
        tools.insert(tool.name.clone(), tool);
    }

    let mut prompts = HashMap::new();
    for prompt in config.prompts {
        // Validate unique prompt names
        if prompts.contains_key(&prompt.name) {
            return Err(anyhow::anyhow!("Duplicate prompt name: {}", prompt.name));
        }
        prompts.insert(prompt.name.clone(), prompt);
    }

    Ok(ConfigData { tools, prompts })
}
