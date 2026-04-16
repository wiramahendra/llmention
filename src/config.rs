use anyhow::{Context, Result};
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Config {
    #[serde(default)]
    pub providers: ProvidersConfig,
    #[serde(default)]
    pub defaults: DefaultsConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ProvidersConfig {
    pub openai: Option<ProviderConfig>,
    pub anthropic: Option<ProviderConfig>,
    pub xai: Option<ProviderConfig>,
    pub ollama: Option<OllamaConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProviderConfig {
    pub api_key: String,
    pub model: String,
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default)]
    pub temperature: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OllamaConfig {
    #[serde(default = "default_ollama_url")]
    pub base_url: String,
    pub model: String,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub temperature: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DefaultsConfig {
    #[serde(default = "default_days")]
    pub days: u32,
    #[serde(default = "default_concurrency")]
    pub concurrency: usize,
}

impl Default for DefaultsConfig {
    fn default() -> Self {
        Self {
            days: default_days(),
            concurrency: default_concurrency(),
        }
    }
}

fn default_true() -> bool {
    true
}
fn default_ollama_url() -> String {
    "http://localhost:11434".to_string()
}
fn default_days() -> u32 {
    7
}
fn default_concurrency() -> usize {
    4
}

impl Config {
    pub fn load() -> Result<Self> {
        let path = config_path();
        if !path.exists() {
            return Ok(Self::default());
        }
        let contents = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config at {}", path.display()))?;
        toml::from_str(&contents).with_context(|| "Failed to parse config.toml")
    }

    pub fn config_dir() -> PathBuf {
        home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".llmention")
    }
}

pub fn config_path() -> PathBuf {
    Config::config_dir().join("config.toml")
}
