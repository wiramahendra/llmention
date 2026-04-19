# LLMention

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust 1.75+](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)

**The terminal-native GEO agent for indie hackers and open-source maintainers.**

LLMention tracks, generates, and optimizes your brand's visibility in ChatGPT, Claude, Grok, Perplexity, and any LLM you configure — privately, locally, no SaaS.

```
  Mention rate   67%  (8/12 queries)  (↑ 24pp vs last run)
  Citations      2
  Models         2/3  (openai, anthropic)
```

---

## Why LLMention?

- **Private.** Your prompts never leave your machine. No telemetry, no sign-up, no cloud DB.
- **Unlimited.** Use your own API keys or run 100% locally with [Ollama](https://ollama.com) — no per-query pricing.
- **Agentic.** The `optimize` command discovers weak spots, generates GEO-optimized markdown, and projects your visibility lift — one command, zero manual work.

|                  | LLMention          | Enterprise GEO tools |
|------------------|--------------------|----------------------|
| Price            | Free / open-source | $200–$2 000/mo       |
| Data stays local | ✓                  | ✗ (their servers)    |
| Content generation | ✓ built-in       | ✗                    |
| Agentic optimize | ✓ 5-step agent    | ✗                    |
| Ollama support   | ✓ fully local      | ✗                    |
| Single binary    | ✓ 10 MB           | Web dashboard        |

---

## Quick Start

```bash
# 1. Install
cargo install --git https://github.com/wiramahendra/llMention

# 2. Create config
llmention config

# 3. Edit ~/.llmention/config.toml — add an API key or enable Ollama

# 4. Verify your setup
llmention doctor

# 5. Run your first audit
llmention audit myproject.com --niche "Rust CLI tool"

# 6. Let the agent optimize your visibility
llmention optimize myproject.com --niche "Rust CLI tool" --auto-apply
```

> **Zero-cost option:** Install [Ollama](https://ollama.com), run `ollama pull llama3.2`,
> set `enabled = true` under `[providers.ollama]` in config, and use `--models ollama`.

---

## Commands

### `optimize` — Full GEO agent (new in Phase 2)

The flagship command. Runs a 5-step autonomous workflow:

1. **Discover** — uses LLMs to generate 10–15 high-intent prompts for your niche
2. **Audit** — queries all configured models, records current mention rates
3. **Identify** — finds the weakest topics (zero or low visibility)
4. **Generate** — writes GEO-optimized markdown for each weak topic
5. **Evaluate** — scores each section's citability and shows projected lift

```bash
# Basic — shows plan and suggested next steps
llmention optimize igrisinertial.com --niche "deterministic edge runtime"

# With competitors for richer comparison content
llmention optimize myproject.com --niche "rust cli tool" --competitors "ripgrep,fd" --steps 5

# Preview without writing files
llmention optimize myproject.com --niche "observability" --dry-run

# Write generated sections to ./geo/ automatically
llmention optimize myproject.com --niche "edge AI runtime" --auto-apply

# Use a specific model for generation
llmention optimize myproject.com --niche "..." --models anthropic --steps 3
```

**Example output:**
```
  Optimizing  igrisinertial.com
  Niche:      deterministic edge runtime
  Mode:       3 steps

  [1/5]  Discovering high-intent prompts…
         → Found 12 prompts

  [2/5]  Auditing current visibility…
         – [  1/12] [anthropic] best deterministic runtime for edge AI
         – [  2/12] [anthropic] alternatives to ros2 for robotics
         → Mention rate: 0%  (0/12)

  [3/5]  Identifying optimization opportunities…
         → 12 weak topics — targeting 3

  [4/5]  Generating optimized content…
         → [1/3] "alternatives to ros2 for robotics"…  ✓ (anthropic)
         → [2/3] "best deterministic runtime for edge AI"…  ✓ (anthropic)
         → [3/3] "what is igrisinertial"…  ✓ (anthropic)

  [5/5]  Evaluating citability…
         → [anthropic] ✓ 92%  — alternatives to ros2 for robotics
         → [anthropic] ✓ 87%  — best deterministic runtime for edge AI
         → [anthropic] ✓ 79%  — what is igrisinertial

  ════════════════════════════════════════════════════════════════
  Optimization Plan  igrisinertial.com
  ════════════════════════════════════════════════════════════════

  Current visibility     0%   (0 queries across 12 topics)
  Projected citability  86%   (+86pp on optimized topics)

  ┌─────────────────────────────────────┬────────────┬────────────────────────────────────────┐
  │ Prompt                              │ Citability │ File                                   │
  ├─────────────────────────────────────┼────────────┼────────────────────────────────────────┤
  │ alternatives to ros2 for robotics   │ ✓ 92%      │ geo/alternatives-to-ros2-for-robotics.md│
  │ best deterministic runtime for edg… │ ✓ 87%      │ geo/best-deterministic-runtime-for.md  │
  │ what is igrisinertial               │ ✓ 79%      │ geo/what-is-igrisinertial.md           │
  └─────────────────────────────────────┴────────────┴────────────────────────────────────────┘

  Next steps:
  →  Review content:  cat geo/alternatives-to-ros2-for-robotics.md
  →  Commit:          git add geo/ && git commit -m "docs: add GEO-optimized content"
  →  Re-audit:        llmention audit igrisinertial.com --niche "deterministic edge runtime"
```

### `generate` — Generate content for a single query

Generates one GEO-optimized markdown section for any target prompt.

```bash
# Print to stdout
llmention generate "best deterministic runtime for edge AI agents" \
  --about "igrisinertial.com is a deterministic, failure-resilient runtime"

# With niche context
llmention generate "alternatives to ROS 2" \
  --about "igrisinertial.com is a robotics runtime" \
  --niche "edge robotics"

# Save to file
llmention generate "what is igrisinertial" \
  --about "..." --output geo/what-is-igrisinertial.md

# Generate and run before/after visibility evaluation
llmention generate "best runtime for AI robots" \
  --about "igrisinertial.com is ..." --evaluate
```

The generated content follows GEO best practices:
- Answer-first (inverted pyramid) — direct answer in first paragraph
- H2 headings that match real user search queries
- Self-contained sections for independent LLM citation
- Tables for comparisons, bullets for features
- Factual, authoritative tone — zero marketing fluff

### `audit` — Quick visibility scan

Generates 12 smart prompts and queries all enabled models. No file needed.

```bash
llmention audit myproject.com
llmention audit myproject.com --niche "observability tool" --competitor datadog
llmention audit myproject.com --models openai,ollama
llmention audit myproject.com --judge     # local LLM re-evaluates each response
```

### `track` — Custom prompts

Load your own prompt list from a `.txt` (one per line) or `.json` array file.

```bash
llmention track myproject.com --prompts prompts.txt
llmention track myproject.com --prompts prompts.json --models anthropic
```

**`prompts.txt` example:**
```
what is myproject
best lightweight monitoring tool 2026
myproject vs prometheus
is myproject production ready
how to install myproject on Ubuntu
```

### `report` — History & trends

Query the local SQLite history. Supports CSV and Markdown export.

```bash
llmention report myproject.com
llmention report myproject.com --days 30
llmention report myproject.com --export csv > results.csv
llmention report myproject.com --export markdown > report.md
```

### `config` — Setup

Creates `~/.llmention/config.toml` with a commented example on first run.

```bash
llmention config
```

### `doctor` — Verify setup

Checks config, database, cache directory, provider status, and Ollama connectivity.

```bash
llmention doctor
```

---

## Configuration

Config file: `~/.llmention/config.toml` — run `llmention config` to create it.

```toml
[providers.openai]
api_key     = "sk-..."
model       = "gpt-4o-mini"
enabled     = true
temperature = 0          # deterministic, cacheable (recommended)

[providers.anthropic]
api_key     = "sk-ant-..."
model       = "claude-3-5-haiku-20241022"
enabled     = true
temperature = 0

[providers.xai]
api_key     = "xai-..."
model       = "grok-2-latest"
enabled     = false

[providers.perplexity]
api_key     = "pplx-..."
model       = "sonar"            # sonar-pro for deeper web-grounded answers
enabled     = false
temperature = 0

# Local, free, unlimited — install: https://ollama.com
[providers.ollama]
base_url  = "http://localhost:11434"
model     = "llama3.2"
enabled   = false

# LLM-as-judge: re-evaluates each response for higher-accuracy parsing.
# Enable per-run with: --judge flag
[judge]
enabled   = false
base_url  = "http://localhost:11434"
model     = "llama3.2"

[defaults]
days        = 7    # default window for `report`
concurrency = 5    # max parallel API calls
```

---

## How It Works

### Tracking & Audit
1. **Prompts** — Each prompt is sent to every enabled provider concurrently (semaphore-limited).
2. **Parsing** — Detects domain mentions, link citations, position (Top/Middle/Bottom), and sentiment via keyword heuristics. Optionally upgrade accuracy with `--judge` (local model returning structured JSON).
3. **Cache** — Responses cached for 24 h by `SHA-256(domain|model|prompt|date)`. Re-runs are instant.
4. **Storage** — Every result saved to `~/.llmention/mentions.db` (SQLite). Records older than 90 days are pruned automatically.

### Generate
Builds a GEO system prompt from embedded templates (`src/geo/templates/`), queries providers concurrently, and returns clean markdown optimized for LLM citation.

### Optimize (Agent)
Orchestrates all 5 steps in `src/agent/optimizer.rs`:
- Prompt discovery uses the `discover_prompts.prompt.md` template
- Weak prompt selection is done by per-prompt mention rate (lowest first)
- Content generation reuses `geo::generator`
- Evaluation reuses `geo::evaluator::score_content()`
- `--auto-apply` writes to `./geo/*.md` with kebab-case filenames

---

## Project Structure

```
src/
  bin/llmention.rs        CLI entrypoint (clap)
  agent/
    optimizer.rs          5-step GEO agent orchestrator
    plan.rs               OptimizationPlan, GeneratedSection, prompt_to_filename()
    prompt_discovery.rs   LLM-based high-intent prompt discovery
  geo/
    generator.rs          GEO content generation via LlmProvider
    evaluator.rs          Citability scoring (before/after)
    prompts.rs            Template loading, default_prompts(), domain extraction
    templates/            Embedded .prompt.md files (compiled into binary)
  providers/              LlmProvider trait + OpenAI, Anthropic, xAI, Perplexity, Ollama
  tracker.rs              Parallel query orchestrator with semaphore
  parser.rs               Rule-based mention/citation/sentiment detection
  cache.rs                24-hour file cache (SHA-256 keyed)
  storage.rs              SQLite persistence
  report.rs               Terminal output + CSV/Markdown export
  types.rs                Shared types (MentionResult, TrackSummary, …)
```

To add a new provider: implement `LlmProvider` in `src/providers/`, add a config field in `src/config.rs`, and wire it in `tracker::build_providers`.

```bash
cargo test        # unit tests
cargo clippy      # lints
cargo build --release
```

---

## Roadmap

| Phase | Feature | Status |
|-------|---------|--------|
| 1 — Tracker | `audit`, `track`, `report`, `config`, `doctor` | ✅ Done |
| 2 — Generate | `llmention generate` — produce GEO-optimized markdown | ✅ Done |
| 3 — Optimize | `llmention optimize` — full 5-step GEO agent | ✅ Done |
| 4 — GUI | Thin Tauri desktop app wrapping the same SQLite store | Planned |
| 5 — Web | Optional self-hosted dashboard | Planned |

---

## License

MIT — see [LICENSE](LICENSE).
