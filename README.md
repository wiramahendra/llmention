# LLMention

**Track how often LLMs mention your brand — local-first, private, no SaaS.**

Run `llmention audit myproject.com` weekly and instantly see:  
*"Your brand appeared in 7/12 responses this week (↑42pp vs last run)"*  
with concrete GEO hints like *"Add a comparison table"* or *"Move entity definition to the first paragraph"*.

Built for indie hackers, solo founders, and open-source maintainers who want visibility insights without paying for a heavy dashboard or leaking prompts to a third-party service.

---

## Quick Start

```bash
# 1. Install
cargo install --git https://github.com/schlep-engine/llmention

# 2. Create config
llmention config

# 3. Edit ~/.llmention/config.toml — add at least one API key
#    (or enable Ollama for fully local, free usage)

# 4. Run your first audit
llmention audit myproject.com --niche "Rust CLI tool"
```

---

## Commands

### `audit` — Quick scan (recommended starting point)

Runs 12 smart default prompts across all configured models. No file needed.

```bash
llmention audit myproject.com
llmention audit myproject.com --niche "observability tool" --competitor datadog
llmention audit myproject.com --models openai,ollama
llmention audit myproject.com --judge   # use local LLM for higher-accuracy parsing
```

### `track` — Custom prompts

Load your own prompt list from a file (.txt one-per-line or .json array).

```bash
llmention track myproject.com
llmention track myproject.com --prompts prompts.txt
llmention track myproject.com --prompts prompts.json --models anthropic
```

**Example `prompts.txt`:**
```
what is myproject
best lightweight monitoring tool 2026
myproject vs prometheus
is myproject production ready
```

### `report` — History & trends

Query the local SQLite database. Supports CSV and Markdown export.

```bash
llmention report myproject.com
llmention report myproject.com --days 30
llmention report myproject.com --days 30 --export csv > results.csv
llmention report myproject.com --days 30 --export markdown > report.md
```

### `config` — Setup

Creates `~/.llmention/config.toml` with a commented example on first run.

```bash
llmention config
```

---

## Example Output

```
  Tracking myproject — 12 prompt(s) × 3 model(s)

  ✓  [ 1/36] [openai] what is myproject
  –  [ 2/36] [openai] best developer tool 2026
  ✓  [ 3/36] [openai] myproject review
  ...

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  LLMention Report:  myproject.com
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  Mention rate   67%  (8/12 queries)  (↑ 24pp vs last run)
  Citations      2
  Models         2/3  (openai, anthropic)

  ┌──────────┬──────────────────────────────┬───────────┬───────┬──────────┬───────────┐
  │ Model    │ Prompt                       │ Mentioned │ Cited │ Position │ Sentiment │
  ├──────────┼──────────────────────────────┼───────────┼───────┼──────────┼───────────┤
  │ openai   │ what is myproject            │ Yes       │ Yes   │ Top      │ Positive  │
  │ openai   │ best developer tool 2026     │ No        │ —     │ —        │ —         │
  ...

  Actionable GEO Tips:
  →  Moderate visibility. Lead every major doc section with the direct
     answer (inverted-pyramid). Publish explicit comparison pages
     (e.g. 'MyTool vs Competitor') — they rank well in LLM citations.
```

---

## Configuration

Config file lives at `~/.llmention/config.toml`. Run `llmention config` to create it.

```toml
[providers.openai]
api_key     = "sk-..."
model       = "gpt-4o-mini"
enabled     = true
temperature = 0          # deterministic results (recommended)

[providers.anthropic]
api_key     = "sk-ant-..."
model       = "claude-3-5-haiku-20241022"
enabled     = true
temperature = 0

[providers.xai]
api_key     = "xai-..."
model       = "grok-2-latest"
enabled     = false

[providers.ollama]
base_url  = "http://localhost:11434"
model     = "llama3.2"
enabled   = false        # free, fully local — install: https://ollama.com

# Optional: LLM-as-judge for higher-accuracy mention detection
[judge]
enabled   = false
base_url  = "http://localhost:11434"
model     = "llama3.2"

[defaults]
days        = 7
concurrency = 5
```

### Ollama (local, free, unlimited)

```bash
# Install Ollama: https://ollama.com
ollama pull llama3.2
# Set enabled = true in [providers.ollama]
llmention audit myproject.com --models ollama
```

---

## Why LLMention?

| | LLMention | Enterprise GEO tools |
|---|---|---|
| Price | Free / open-source | $200–$2000/mo |
| Privacy | 100% local | Prompts sent to their servers |
| Rate limits | Your API keys | Platform quotas |
| Custom prompts | ✓ Any file | Limited templates |
| Ollama support | ✓ Fully local | ✗ |
| Data ownership | SQLite on your disk | Their cloud DB |

---

## How It Works

1. **Prompts** — LLMention sends each prompt to every configured LLM provider in parallel (up to 5 concurrent, respecting rate limits).
2. **Parsing** — Rule-based detection checks for domain mentions, links, position in the response, and sentiment. Optionally upgrade to LLM-as-judge with `--judge`.
3. **Cache** — Responses are cached for 24h by `SHA-256(domain|model|prompt|date)` so re-runs are instant.
4. **Storage** — Every result is saved to `~/.llmention/mentions.db` (SQLite). Records older than 90 days are pruned automatically.
5. **Report** — Terminal table + actionable GEO tips based on your results.

---

## Installation

**From source (stable Rust required):**
```bash
git clone https://github.com/schlep-engine/llmention
cd llmention
cargo build --release
cp target/release/llmention ~/.local/bin/
```

**Via cargo install:**
```bash
cargo install --git https://github.com/schlep-engine/llmention
```

---

## License

MIT — see [LICENSE](LICENSE).
