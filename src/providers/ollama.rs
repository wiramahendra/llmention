use anyhow::{bail, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::{json, Value};

use super::LlmProvider;
use crate::config::OllamaConfig;

pub struct OllamaProvider {
    client: Client,
    config: OllamaConfig,
}

impl OllamaProvider {
    pub fn new(config: OllamaConfig) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }
}

#[async_trait]
impl LlmProvider for OllamaProvider {
    fn name(&self) -> &str {
        "ollama"
    }

    async fn query(&self, prompt: &str) -> Result<String> {
        let url = format!("{}/api/chat", self.config.base_url);
        let body = json!({
            "model": self.config.model,
            "stream": false,
            "options": { "temperature": self.config.temperature },
            "messages": [{"role": "user", "content": prompt}]
        });

        let resp = self.client.post(&url).json(&body).send().await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await?;
            bail!("Ollama error {}: {}", status, text);
        }

        let json: Value = resp.json().await?;
        Ok(json["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string())
    }
}
