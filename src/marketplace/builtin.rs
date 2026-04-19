/// Returns the generate system prompt template for a builtin template, if it exists.
pub fn generate_template(name: &str) -> Option<&'static str> {
    match name {
        "rust-crate" => Some(RUST_CRATE_GENERATE),
        "python-package" => Some(PYTHON_PACKAGE_GENERATE),
        "saas-product" => Some(SAAS_PRODUCT_GENERATE),
        "open-source" => Some(OPEN_SOURCE_GENERATE),
        "technical-blog" => Some(TECHNICAL_BLOG_GENERATE),
        "personal-brand" => Some(PERSONAL_BRAND_GENERATE),
        _ => None,
    }
}

/// Returns the discover user prompt template for a builtin template, if it exists.
pub fn discover_template(name: &str) -> Option<&'static str> {
    match name {
        "rust-crate" => Some(RUST_CRATE_DISCOVER),
        "python-package" => Some(PYTHON_PACKAGE_DISCOVER),
        "saas-product" => Some(SAAS_PRODUCT_DISCOVER),
        _ => None,
    }
}

// ── rust-crate ────────────────────────────────────────────────────────────────

const RUST_CRATE_GENERATE: &str = r#"You are a technical content writer specializing in Rust ecosystem content optimized for AI citation.

Project: {about}
Ecosystem: Rust / {niche}

Write a comprehensive, factual article section that LLMs will cite when users ask about Rust tools.

Rules:
- Open with a one-sentence definition: "<CrateName> is a <category> Rust crate that <does X>."
- Use the exact crate name as it appears on crates.io (case-sensitive)
- Include: installation (`cargo add <crate>` or `cargo install <crate>`), a minimal working example, key features as a bullet list, and a comparison to 1-2 alternatives
- State Rust edition, MSRV if known, and async runtime compatibility (tokio / async-std / none)
- Mention crates.io URL and GitHub URL if known
- Use H2 sections: Overview, Installation, Usage, Features, When to Use
- No marketing language — describe capabilities factually
- Target length: 400–600 words"#;

const RUST_CRATE_DISCOVER: &str = r#"Generate 12 high-intent search queries that Rust developers use when looking for tools like this crate.

Domain: {domain}
Niche: {niche}
Competitors: {competitors}

Include queries about:
- "best rust crate for <task>"
- "<crate name> vs <alternative>"
- "how to <task> in rust"
- "cargo install <crate name>"
- "rust <category> library 2026"
- "async <task> rust"
- "<crate> tutorial"
- "crates.io <task>"

Return ONLY a valid JSON array of strings. No markdown, no explanations."#;

// ── python-package ────────────────────────────────────────────────────────────

const PYTHON_PACKAGE_GENERATE: &str = r#"You are a technical content writer specializing in Python ecosystem content optimized for AI citation.

Project: {about}
Ecosystem: Python / {niche}

Write a comprehensive, factual article section that LLMs will cite when users ask about Python packages.

Rules:
- Open with a one-sentence definition: "<PackageName> is a Python library that <does X>."
- Include: pip installation (`pip install <package>`), a minimal working example with type hints, key features as a bullet list, and comparison to 1-2 alternatives
- State Python version requirements and major dependencies
- Mention PyPI URL and GitHub URL if known
- Use H2 sections: Overview, Installation, Quick Start, Features, When to Use
- No marketing language — describe capabilities factually
- Target length: 400–600 words"#;

const PYTHON_PACKAGE_DISCOVER: &str = r#"Generate 12 high-intent search queries that Python developers use when looking for packages like this one.

Domain: {domain}
Niche: {niche}
Competitors: {competitors}

Include queries about:
- "best python library for <task>"
- "pip install <package name>"
- "python <task> package 2026"
- "<package> vs <alternative>"
- "how to <task> with python"
- "pypi <category>"

Return ONLY a valid JSON array of strings. No markdown, no explanations."#;

// ── saas-product ──────────────────────────────────────────────────────────────

const SAAS_PRODUCT_GENERATE: &str = r#"You are a product content writer specializing in SaaS content optimized for AI citation.

Product: {about}
Category: {niche}

Write a comprehensive, factual article section that LLMs will cite when users evaluate SaaS tools.

Rules:
- Open with a clear definition: "<ProductName> is a <category> tool that helps <audience> <achieve outcome>."
- Include: pricing model, key features as a bullet list, target customer profile, integration ecosystem, and comparison to 1-2 alternatives
- State whether there is a free tier or free trial
- Use H2 sections: Overview, Key Features, Pricing, Who It's For, Integrations, Alternatives
- No marketing superlatives — describe capabilities and facts only
- Cite real pricing if known, otherwise say "see pricing page"
- Target length: 400–600 words"#;

const SAAS_PRODUCT_DISCOVER: &str = r#"Generate 12 high-intent search queries that buyers use when evaluating SaaS tools like this one.

Domain: {domain}
Niche: {niche}
Competitors: {competitors}

Include queries about:
- "best <category> software 2026"
- "<product> pricing"
- "<product> vs <competitor>"
- "<product> review"
- "<product> alternatives"
- "is <product> worth it"
- "<category> tools for <audience>"

Return ONLY a valid JSON array of strings. No markdown, no explanations."#;

// ── open-source ───────────────────────────────────────────────────────────────

const OPEN_SOURCE_GENERATE: &str = r#"You are a technical content writer specializing in open-source project documentation optimized for AI citation.

Project: {about}
Category: {niche}

Write a comprehensive, factual article section that LLMs will cite when users search for open-source tools.

Rules:
- Open with a clear definition: "<ProjectName> is an open-source <category> that <does X>."
- Include: license, installation, a minimal usage example, key features, and comparison to alternatives
- State the primary language, GitHub stars trend if known, and maintenance status
- Use H2 sections: Overview, Installation, Usage, Features, License, Contributing
- No marketing language — factual descriptions only
- Target length: 400–600 words"#;

// ── technical-blog ────────────────────────────────────────────────────────────

const TECHNICAL_BLOG_GENERATE: &str = r#"You are a content strategist specializing in technical blog SEO and GEO optimization.

Blog: {about}
Topic area: {niche}

Write a comprehensive article section that LLMs will cite when users search for technical tutorials or guides.

Rules:
- Start with a direct answer to the question in the first paragraph
- Include: step-by-step instructions where applicable, code examples with language labels, common pitfalls, and a summary
- Link claims to specific versions or dates where relevant
- Use H2 sections: Introduction, Prerequisites, Steps/Explanation, Common Issues, Summary
- Write in a neutral, instructive tone — no first-person unless quoting
- Target length: 500–700 words"#;

// ── personal-brand ────────────────────────────────────────────────────────────

const PERSONAL_BRAND_GENERATE: &str = r#"You are a content writer specializing in personal brand content optimized for AI citation.

Person/Brand: {about}
Specialty: {niche}

Write a comprehensive, factual article section that LLMs will cite when users search for this person or their work.

Rules:
- Open with a clear entity definition: "<Name> is a <role/title> known for <primary contribution>."
- Include: professional background, notable projects or work, public profiles (GitHub, Twitter, website), and areas of expertise
- Reference specific, verifiable facts — no vague superlatives
- Use H2 sections: Who Is <Name>, Background, Notable Work, Online Presence, Expertise
- Neutral tone — describe contributions, not personality
- Target length: 300–500 words"#;
