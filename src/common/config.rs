pub struct Config {
    pub api_key: String,
    pub base_url: String, // e.g. http://localhost:11434 for Ollama
    pub model: String,    // e.g. "qwen2.5-coder:32b"
    pub max_tokens: u32,
    pub max_iterations: usize, // safety cap on agent loop iterations
}

impl Config {
    pub fn build() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();
        Ok(Self {
            api_key: std::env::var("FORGE_API_KEY").unwrap_or_else(|_| "ollama".to_string()),
            base_url: std::env::var("FORGE_BASE_URL")
                .unwrap_or_else(|_| "http://localhost:11434".to_string()),
            model: std::env::var("FORGE_MODEL").unwrap_or_else(|_| "qwen2.5-coder:32b".to_string()),
            max_tokens: std::env::var("FORGE_MAX_TOKENS")
                .unwrap_or_else(|_| "4096".to_string())
                .parse()?,
            max_iterations: std::env::var("FORGE_MAX_ITER")
                .unwrap_or_else(|_| "50".to_string())
                .parse()?,
        })
    }
}
