use anyhow::{bail, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::{json, Value};

use super::LlmProvider;
use crate::config::ProviderConfig;

pub struct OpenAiProvider {
    client: Client,
    config: ProviderConfig,
}

impl OpenAiProvider {
    pub fn new(config: ProviderConfig) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }
}

#[async_trait]
impl LlmProvider for OpenAiProvider {
    fn name(&self) -> &str {
        "openai"
    }

    async fn query(&self, prompt: &str) -> Result<String> {
        let body = json!({
            "model": self.config.model,
            "temperature": self.config.temperature,
            "messages": [{"role": "user", "content": prompt}]
        });

        let resp = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&self.config.api_key)
            .json(&body)
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await?;
            bail!("OpenAI error {}: {}", status, text);
        }

        let json: Value = resp.json().await?;
        Ok(json["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string())
    }
}
