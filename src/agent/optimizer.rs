use anyhow::{bail, Result};
use colored::Colorize;
use std::{collections::HashMap, sync::Arc};

use crate::{
    agent::{
        plan::{prompt_to_filename, GeneratedSection, OptimizationPlan},
        prompt_discovery,
    },
    cache::Cache,
    geo::{
        evaluator,
        generator::{self, GenerateOptions},
    },
    providers::LlmProvider,
    storage::Storage,
    tracker::{self, TrackOptions},
};

pub struct OptimizeOptions {
    pub domain: String,
    pub niche: String,
    pub competitors: Vec<String>,
    /// How many weak prompts to generate content for (default: 3).
    pub steps: usize,
    pub dry_run: bool,
    pub verbose: bool,
}

pub async fn optimize(
    opts: &OptimizeOptions,
    providers: &[Arc<dyn LlmProvider>],
    storage: &Storage,
    cache: &Cache,
) -> Result<OptimizationPlan> {
    // ── Step 1: Discover high-intent prompts ────────────────────────────────
    print_step(1, 5, "Discovering high-intent prompts…");
    let discovered =
        prompt_discovery::discover_with_providers(&opts.domain, &opts.niche, &opts.competitors, providers)
            .await;
    let prompt_count = discovered.len().min(15);
    let audit_prompts = discovered[..prompt_count].to_vec();
    eprintln!("       → Found {} prompts", prompt_count);

    // ── Step 2: Audit current visibility ────────────────────────────────────
    print_step(2, 5, "Auditing current visibility…");
    let summary = tracker::run_track(
        &opts.domain,
        audit_prompts.clone(),
        providers.to_vec(),
        storage,
        cache,
        TrackOptions { verbose: opts.verbose, concurrency: 5, judge: None },
    )
    .await?;

    let current_rate = summary.mention_rate();
    eprintln!(
        "       → Mention rate: {:.0}%  ({}/{})",
        current_rate, summary.mention_count, summary.total_queries
    );

    if summary.total_queries == 0 {
        bail!("Audit returned no results. Check your provider configuration.");
    }

    // ── Step 3: Identify weak prompts ───────────────────────────────────────
    print_step(3, 5, "Identifying optimization opportunities…");
    let weak = find_weak_prompts(&summary.results, opts.steps);
    eprintln!(
        "       → {} weak topic(s) with low visibility — targeting {}",
        count_zero_mention_prompts(&summary.results),
        weak.len()
    );

    // ── Step 4: Generate optimized content ──────────────────────────────────
    print_step(4, 5, "Generating optimized content…");
    let mut sections: Vec<GeneratedSection> = Vec::new();
    let total = weak.len();
    for (i, prompt) in weak.iter().enumerate() {
        eprint!("       → [{}/{}] {}…  ", i + 1, total, truncate(prompt, 46));
        let gen_opts = GenerateOptions {
            prompt: prompt.clone(),
            about: format!("{} — {}", opts.domain, opts.niche),
            niche: opts.niche.clone(),
            verbose: false,
        };
        match generator::generate(&gen_opts, providers).await {
            Ok(results) if !results.is_empty() => {
                let first = &results[0];
                eprintln!("{} ({})", "✓".green(), first.model.cyan());
                sections.push(GeneratedSection {
                    prompt: prompt.clone(),
                    content: first.content.clone(),
                    model: first.model.clone(),
                    citability_rate: 0.0, // filled in step 5
                    file_name: prompt_to_filename(prompt),
                });
            }
            Ok(_) => eprintln!("{}", "✗ no response".red()),
            Err(e) => eprintln!("{} {}", "✗".red(), e),
        }
    }

    // ── Step 5: Evaluate projected improvement ──────────────────────────────
    print_step(5, 5, "Evaluating citability…");
    for section in &mut sections {
        match evaluator::score_content(&section.prompt, &section.content, providers).await {
            Ok(eval_results) => {
                let rate = cite_rate(&eval_results);
                section.citability_rate = rate;
                eprint!("       → ");
                for r in &eval_results {
                    let icon = if r.would_cite { "✓".green() } else { "✗".red() };
                    eprint!("[{}] {} {:.0}%  ", r.model.cyan(), icon, r.confidence * 100.0);
                }
                eprintln!("— {} {:.0}%", truncate(&section.prompt, 36).dimmed(), rate);
            }
            Err(e) => eprintln!("       → eval error: {}", e),
        }
    }

    Ok(OptimizationPlan {
        domain: opts.domain.clone(),
        niche: opts.niche.clone(),
        current_mention_rate: current_rate,
        total_audit_queries: summary.total_queries,
        discovered_prompts: audit_prompts,
        weak_prompts: weak,
        sections,
    })
}

fn print_step(n: u8, total: u8, msg: &str) {
    eprintln!();
    eprintln!("  [{}/{}]  {}", n, total, msg.bold());
}

/// Return up to `limit` prompts with the lowest per-prompt mention rate.
fn find_weak_prompts(results: &[crate::types::MentionResult], limit: usize) -> Vec<String> {
    let mut scores: HashMap<&str, (usize, usize)> = HashMap::new();
    for r in results {
        let e = scores.entry(r.prompt.as_str()).or_insert((0, 0));
        e.1 += 1;
        if r.mentioned {
            e.0 += 1;
        }
    }
    let mut sorted: Vec<(&str, f64)> = scores
        .iter()
        .map(|(p, (m, t))| (*p, *m as f64 / (*t).max(1) as f64))
        .collect();
    sorted.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
    sorted.into_iter().take(limit).map(|(p, _)| p.to_string()).collect()
}

fn count_zero_mention_prompts(results: &[crate::types::MentionResult]) -> usize {
    let mut scores: HashMap<&str, bool> = HashMap::new();
    for r in results {
        if r.mentioned {
            scores.insert(r.prompt.as_str(), true);
        } else {
            scores.entry(r.prompt.as_str()).or_insert(false);
        }
    }
    scores.values().filter(|&&v| !v).count()
}

fn cite_rate(results: &[crate::geo::evaluator::EvalResult]) -> f64 {
    if results.is_empty() {
        return 0.0;
    }
    let avg: f64 = results.iter().map(|r| r.confidence * if r.would_cite { 1.0 } else { 0.0 }).sum::<f64>()
        / results.len() as f64;
    (avg * 100.0).round()
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}…", &s[..max.saturating_sub(1)])
    }
}
