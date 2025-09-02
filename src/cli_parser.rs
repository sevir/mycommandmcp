use anyhow::{Context, Result};
use clap::Parser;
use reqwest::blocking::get;
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
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
}

impl PromptConfig {
    /// Load the content for this prompt, either from inline content, file, or URL
    pub fn load_content(&self) -> Result<String> {
        if let Some(content) = &self.content {
            return Ok(content.clone());
        }

        if let Some(path) = &self.path {
            return fs::read_to_string(path)
                .context(format!("Failed to read prompt file: {}", path));
        }

        if let Some(url) = &self.url {
            return Self::load_from_url(url);
        }

        Err(anyhow::anyhow!(
            "No content, path, or url specified for prompt: {}",
            self.name
        ))
    }

    /// Load content from a URL
    fn load_from_url(url: &str) -> Result<String> {
        let response = get(url).context(format!("Failed to fetch URL: {}", url))?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "HTTP error {} when fetching URL: {}",
                response.status(),
                url
            ));
        }

        let content = response
            .text()
            .context(format!("Failed to read response body from URL: {}", url))?;

        Ok(content)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct ResourceConfig {
    pub name: String,
    pub description: String,
    pub path: String,
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
    #[serde(default)]
    pub tools: Vec<ToolConfig>,
    #[serde(default)]
    pub prompts: Vec<PromptConfig>,
    #[serde(default)]
    pub resources: Vec<ResourceConfig>,
    #[serde(default)]
    pub external_configs: Vec<String>,
}

pub struct ConfigData {
    pub tools: HashMap<String, ToolConfig>,
    pub prompts: HashMap<String, PromptConfig>,
    pub resources: HashMap<String, ResourceConfig>,
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

/// Load and parse an external configuration file from path or URL
fn load_external_config(source: &str) -> Result<ToolsConfig> {
    let content = if source.starts_with("http://") || source.starts_with("https://") {
        // Load from URL
        let response = get(source).context(format!("Failed to fetch URL: {}", source))?;
        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "HTTP error {} when fetching URL: {}",
                response.status(),
                source
            ));
        }
        response
            .text()
            .context(format!("Failed to read response body from URL: {}", source))?
    } else {
        // Load from file
        fs::read_to_string(source).context(format!("Failed to read config file: {}", source))?
    };

    let config: ToolsConfig =
        serde_yaml::from_str(&content).context(format!("Failed to parse YAML from {}", source))?;

    Ok(config)
}

/// Load and parse the configuration file
pub fn load_config(config_path: &str) -> Result<ConfigData> {
    let config_content = fs::read_to_string(config_path)
        .context(format!("Failed to read config file: {config_path}"))?;

    let mut config: ToolsConfig =
        serde_yaml::from_str(&config_content).context("Failed to parse YAML configuration")?;

    // Keep track of loaded external files to avoid duplicates
    let mut loaded_files = std::collections::HashSet::new();

    // Load external configurations and merge them
    for source in &config.external_configs {
        if !loaded_files.contains(source) {
            let external = load_external_config(source)?;
            config.tools.extend(external.tools);
            config.prompts.extend(external.prompts);
            config.resources.extend(external.resources);
            loaded_files.insert(source.clone());
        }
    }

    let mut tools = HashMap::new();
    for tool in config.tools {
        if tools.contains_key(&tool.name) {
            return Err(anyhow::anyhow!("Duplicate tool name: {}", tool.name));
        }
        tools.insert(tool.name.clone(), tool);
    }

    let mut prompts = HashMap::new();
    for prompt in config.prompts {
        // Validate unique prompt names
        if prompts.contains_key(&prompt.name) {
            return Err(anyhow::anyhow!("Duplicate prompt name: {}", prompt.name));
        }

        // Validate that at least one content source is provided
        let has_content = prompt.content.is_some();
        let has_path = prompt.path.is_some();
        let has_url = prompt.url.is_some();

        if !has_content && !has_path && !has_url {
            return Err(anyhow::anyhow!(
                "Prompt '{}' must have either 'content', 'path', or 'url' specified",
                prompt.name
            ));
        }

        if has_content && (has_path || has_url) {
            return Err(anyhow::anyhow!(
                "Prompt '{}' cannot have both 'content' and 'path'/'url' specified",
                prompt.name
            ));
        }

        if has_path && has_url {
            return Err(anyhow::anyhow!(
                "Prompt '{}' cannot have both 'path' and 'url' specified",
                prompt.name
            ));
        }

        prompts.insert(prompt.name.clone(), prompt);
    }

    let mut resources = HashMap::new();
    for resource in config.resources {
        if resources.contains_key(&resource.name) {
            return Err(anyhow::anyhow!(
                "Duplicate resource name: {}",
                resource.name
            ));
        }
        resources.insert(resource.name.clone(), resource);
    }

    Ok(ConfigData {
        tools,
        prompts,
        resources,
    })
}
