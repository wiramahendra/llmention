use anyhow::{bail, Result};
use clap::{Parser, Subcommand, ValueEnum};
use colored::Colorize;
use std::path::PathBuf;

use llmention::{
    cache::Cache,
    config::{Config, EXAMPLE_CONFIG},
    report,
    storage::Storage,
    tracker::{self, TrackOptions},
};

#[derive(Parser)]
#[command(
    name = "llmention",
    about = "Track how often LLMs mention your brand — local, private, no SaaS",
    long_about = "LLMention queries LLM providers with prompts about your brand and\n\
                  records whether and how they mention it. All data stays on your disk.\n\n\
                  Quick start:\n  \
                  llmention config                         # create config file\n  \
                  llmention audit myproject.com            # run a quick scan\n  \
                  llmention report myproject.com --days 7  # view history",
    version,
    arg_required_else_help = true
)]
struct Cli {
    /// Comma-separated models to query (e.g. openai,anthropic,ollama)
    #[arg(long, short, global = true)]
    models: Option<String>,

    /// Show first line of each raw LLM response
    #[arg(long, short, global = true, default_value = "false")]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Clone, ValueEnum)]
enum ExportFormat {
    Csv,
    Markdown,
}

#[derive(Subcommand)]
enum Commands {
    /// Run prompts against configured models and record brand mentions
    ///
    /// Examples:
    ///   llmention track myproject.com
    ///   llmention track myproject.com --prompts prompts.txt
    ///   llmention track myproject.com --models openai,ollama --judge
    Track {
        /// Domain or brand to track (e.g. myproject.com)
        domain: String,
        /// Path to prompts file (.txt one-per-line or .json array)
        #[arg(long, short)]
        prompts: Option<PathBuf>,
        /// Re-evaluate responses with a local LLM judge for higher accuracy
        #[arg(long)]
        judge: bool,
    },
    /// Quick audit with 12 smart default prompts (no file needed)
    ///
    /// Examples:
    ///   llmention audit myproject.com
    ///   llmention audit myproject.com --niche "Rust CLI tool" --competitor ripgrep
    Audit {
        /// Domain or brand to audit
        domain: String,
        /// Product niche for smarter prompt generation (e.g. "Rust CLI tool")
        #[arg(long)]
        niche: Option<String>,
        /// Main competitor for comparison prompts
        #[arg(long)]
        competitor: Option<String>,
        /// Re-evaluate responses with a local LLM judge for higher accuracy
        #[arg(long)]
        judge: bool,
    },
    /// Show mention history and trends from local SQLite database
    ///
    /// Examples:
    ///   llmention report myproject.com
    ///   llmention report myproject.com --days 30
    ///   llmention report myproject.com --days 30 --export csv > results.csv
    Report {
        /// Domain or brand
        domain: String,
        /// Number of past days to include [default: 7]
        #[arg(long, short, default_value = "7")]
        days: u32,
        /// Export format instead of terminal display
        #[arg(long, value_enum)]
        export: Option<ExportFormat>,
    },
    /// Show config path, create example config, and display setup instructions
    Config,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = Config::load()?;
    let (base_dir, is_first_run) = Config::ensure_dir()?;
    let storage = Storage::open(&base_dir)?;
    let cache = Cache::new(&base_dir)?;

    if is_first_run {
        print_welcome();
    }

    match cli.command {
        Commands::Track { domain, prompts, judge } => {
            let providers = tracker::build_providers_filtered(&config, cli.models.as_deref());
            if providers.is_empty() {
                bail!(
                    "No providers enabled.\n  Run {} to set up your API keys.",
                    "llmention config".cyan()
                );
            }
            let prompts = load_prompts(prompts, &domain)?;
            let judge_provider = build_judge_provider(judge, &config);

            println!(
                "\n  {} {} — {} prompt(s) × {} model(s){}\n",
                "Tracking".bold(),
                domain.cyan().bold(),
                prompts.len(),
                providers.len(),
                if judge { "  [+judge]".dimmed().to_string() } else { String::new() }
            );

            let prev_rate = fetch_prev_rate(&storage, &domain);
            let summary = tracker::run_track(
                &domain, prompts, providers, &storage, &cache,
                TrackOptions {
                    verbose: cli.verbose,
                    concurrency: config.defaults.concurrency,
                    judge: judge_provider,
                },
            ).await?;
            report::print_summary(&summary, prev_rate);
        }

        Commands::Audit { domain, niche, competitor, judge } => {
            let providers = tracker::build_providers_filtered(&config, cli.models.as_deref());
            if providers.is_empty() {
                bail!(
                    "No providers enabled.\n  Run {} to set up your API keys.",
                    "llmention config".cyan()
                );
            }
            let prompts = audit_prompts(&domain, niche.as_deref(), competitor.as_deref());
            let judge_provider = build_judge_provider(judge, &config);

            println!(
                "\n  {} {} — {} prompts × {} model(s){}\n",
                "Auditing".bold(),
                domain.cyan().bold(),
                prompts.len(),
                providers.len(),
                if judge { "  [+judge]".dimmed().to_string() } else { String::new() }
            );

            let prev_rate = fetch_prev_rate(&storage, &domain);
            let summary = tracker::run_track(
                &domain, prompts, providers, &storage, &cache,
                TrackOptions {
                    verbose: cli.verbose,
                    concurrency: config.defaults.concurrency,
                    judge: judge_provider,
                },
            ).await?;
            report::print_summary(&summary, prev_rate);
        }

        Commands::Report { domain, days, export } => {
            let results = storage.query_domain(&domain, days)?;
            match export {
                Some(ExportFormat::Csv) => print!("{}", report::export_csv(&results)),
                Some(ExportFormat::Markdown) => print!("{}", report::export_markdown(&results, &domain)),
                None => report::print_trend_report(&domain, &results, days),
            }
        }

        Commands::Config => run_config_command()?,
    }

    Ok(())
}

fn print_welcome() {
    println!();
    println!("{}", "━".repeat(60).dimmed());
    println!(
        "  {}  Welcome to LLMention!",
        "★".yellow().bold()
    );
    println!("{}", "━".repeat(60).dimmed());
    println!();
    println!("  Track how often LLMs mention your brand — privately,");
    println!("  locally, and without paying for a SaaS dashboard.");
    println!();
    println!("  {}", "Next steps:".bold());
    println!(
        "    1. Edit {}",
        "~/.llmention/config.toml".cyan()
    );
    println!("    2. Add at least one API key (or enable Ollama)");
    println!(
        "    3. Run {}",
        "llmention audit yourdomain.com".cyan()
    );
    println!();
}

fn run_config_command() -> Result<()> {
    let (dir, _) = Config::ensure_dir()?;
    let path = llmention::config::config_path();

    println!();
    println!("{}", "LLMention — Configuration".bold());
    println!("{}", "━".repeat(54).dimmed());
    println!();
    println!("  Config dir   {}", dir.display().to_string().cyan());
    println!("  Config file  {}", path.display().to_string().cyan());
    println!();

    if path.exists() {
        println!("  {} Config file already exists.", "✓".green());
        println!("  Edit it to add or update API keys.");
    } else {
        std::fs::write(&path, EXAMPLE_CONFIG)?;
        println!(
            "  {} Created {}",
            "✓".green(),
            path.display().to_string().cyan()
        );
        println!("  Add your API keys, then run:");
        println!("    {}", "llmention audit yourdomain.com".cyan());
    }

    println!();
    println!("  {}", "Supported providers:".bold());
    println!("    {}  openai    gpt-4o-mini, gpt-4o …", "·".dimmed());
    println!("    {}  anthropic claude-3-5-haiku, claude-3-5-sonnet …", "·".dimmed());
    println!("    {}  xai       grok-2-latest (x.ai)", "·".dimmed());
    println!("    {}  ollama    llama3.2, mistral, phi4 … (local, free)", "·".dimmed());
    println!();
    println!(
        "  {}  Set {} for deterministic, cacheable results.",
        "Tip".yellow().bold(),
        "temperature = 0".cyan()
    );
    println!();
    Ok(())
}

fn build_judge_provider(
    flag: bool,
    config: &Config,
) -> Option<std::sync::Arc<dyn llmention::providers::LlmProvider>> {
    if flag || config.judge.enabled {
        tracker::build_judge(config)
    } else {
        None
    }
}

fn fetch_prev_rate(storage: &Storage, domain: &str) -> Option<f64> {
    let before = chrono::Utc::now().to_rfc3339();
    match storage.previous_run_stats(domain, &before) {
        Ok(Some((m, t))) if t > 0 => Some(m as f64 / t as f64 * 100.0),
        _ => None,
    }
}

fn load_prompts(path: Option<PathBuf>, domain: &str) -> Result<Vec<String>> {
    match path {
        Some(p) => {
            let contents = std::fs::read_to_string(&p)?;
            if p.extension().map_or(false, |e| e == "json") {
                Ok(serde_json::from_str(&contents)?)
            } else {
                Ok(contents
                    .lines()
                    .filter(|l| !l.trim().is_empty())
                    .map(String::from)
                    .collect())
            }
        }
        None => Ok(audit_prompts(domain, None, None)),
    }
}

fn audit_prompts(domain: &str, niche: Option<&str>, competitor: Option<&str>) -> Vec<String> {
    let brand = domain
        .trim_end_matches(".com").trim_end_matches(".io").trim_end_matches(".dev")
        .trim_end_matches(".app").trim_end_matches(".net").trim_end_matches(".org")
        .trim_end_matches(".ai");

    let niche = niche.unwrap_or("developer tool");
    let comp = competitor.unwrap_or("similar tools");

    let mut prompts = vec![
        format!("what is {}", brand),
        format!("best {} 2026", niche),
        format!("{} review", brand),
        format!("is {} open source", brand),
        format!("how does {} work", brand),
        format!("alternatives to {} for {}", comp, niche),
        format!("who uses {}", brand),
        format!("should I use {} for my project", brand),
        format!("{} vs {}", brand, comp),
        format!("getting started with {}", brand),
        format!("pros and cons of {}", brand),
        format!("is {} production ready", brand),
    ];

    prompts.dedup();
    prompts
}
