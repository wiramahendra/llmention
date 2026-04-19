use anyhow::Result;
use std::sync::Arc;

use crate::{geo::prompts, providers::LlmProvider};

pub struct GenerateOptions {
    pub prompt: String,
    pub about: String,
    pub niche: String,
    pub verbose: bool,
}

pub struct GenerateResult {
    pub model: String,
    pub content: String,
}

/// Query all providers concurrently with a GEO system prompt and collect results.
pub async fn generate(
    opts: &GenerateOptions,
    providers: &[Arc<dyn LlmProvider>],
) -> Result<Vec<GenerateResult>> {
    let system = prompts::build_generate_system_prompt(&opts.about, &opts.niche);

    let handles: Vec<_> = providers
        .iter()
        .map(|p| {
            let p = Arc::clone(p);
            let sys = system.clone();
            let user_prompt = opts.prompt.clone();
            tokio::spawn(async move {
                let result = p.query_with_system(Some(&sys), &user_prompt).await;
                (p.name().to_string(), result)
            })
        })
        .collect();

    let mut results = Vec::new();
    for handle in handles {
        match handle.await {
            Ok((model, Ok(content))) => {
                if opts.verbose {
                    let first = content.lines().next().unwrap_or("").trim();
                    eprintln!("  [{}] {}", model, first);
                }
                results.push(GenerateResult { model, content: content.trim().to_string() });
            }
            Ok((model, Err(e))) => {
                eprintln!("  [{}] error: {}", model, e);
            }
            Err(e) => eprintln!("  task panicked: {}", e),
        }
    }

    Ok(results)
}
