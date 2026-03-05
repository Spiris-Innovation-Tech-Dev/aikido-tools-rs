use aikido::config::{Config, ConfigOverrides, Credentials, Region};
use anyhow::Result;

pub struct McpConfig {
    pub region: Region,
    pub credentials: Credentials,
    pub max_results: u32,
    pub allow_raw_api_mutations: bool,
}

impl McpConfig {
    pub fn from_env() -> Result<Self> {
        let config = Config::load_and_validate(ConfigOverrides::default())
            .map_err(|e| anyhow::anyhow!("Config error: {e}"))?;

        let max_results = std::env::var("MCP_MAX_RESULTS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(50);
        let allow_raw_api_mutations = std::env::var("MCP_ALLOW_RAW_API_MUTATIONS")
            .ok()
            .map(|value| {
                matches!(
                    value.to_ascii_lowercase().as_str(),
                    "1" | "true" | "yes" | "on"
                )
            })
            .unwrap_or(false);

        let region = config.region();
        let credentials = config
            .credentials()
            .map_err(|e| anyhow::anyhow!("Credentials error: {e}"))?;

        Ok(Self {
            region,
            credentials,
            max_results,
            allow_raw_api_mutations,
        })
    }
}
