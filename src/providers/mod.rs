use anyhow::Result;
use async_trait::async_trait;

pub mod anthropic;
pub mod ollama;
pub mod openai;
pub mod xai;

#[async_trait]
pub trait LlmProvider: Send + Sync {
    fn name(&self) -> &str;
    async fn query(&self, prompt: &str) -> Result<String>;
}
