pub static BASE_GENERATE: &str = include_str!("templates/base_generate.prompt.md");
pub static EVALUATE_IMPROVEMENT: &str = include_str!("templates/evaluate_improvement.prompt.md");

pub fn build_generate_system_prompt(about: &str, niche: &str) -> String {
    BASE_GENERATE
        .replace("{about}", about)
        .replace("{niche}", niche)
}

pub fn build_evaluate_user_prompt(prompt: &str, content: &str) -> String {
    EVALUATE_IMPROVEMENT
        .replace("{prompt}", prompt)
        .replace("{content}", content)
}

/// Best-effort extraction of a domain name from an `--about` string.
/// E.g. "igrisinertial.com is a runtime" → "igrisinertial.com"
pub fn extract_domain_hint(about: &str) -> Option<String> {
    let tlds = [".com", ".io", ".dev", ".net", ".org", ".app", ".ai", ".co", ".tech", ".tools"];
    let words: Vec<&str> = about.split_whitespace().collect();
    for word in words {
        let clean = word.trim_matches(|c: char| !c.is_alphanumeric() && c != '.' && c != '-');
        if tlds.iter().any(|t| clean.ends_with(t)) && clean.contains('.') {
            return Some(clean.to_lowercase());
        }
    }
    None
}
