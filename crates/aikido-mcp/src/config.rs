use aikido::config::{Config, ConfigOverrides, Credentials, Region};
use anyhow::Result;

pub struct McpConfig {
    pub region: Region,
    pub credentials: Credentials,
    pub max_results: u32,
}

impl McpConfig {
    pub fn from_env() -> Result<Self> {
        let config = Config::load_and_validate(ConfigOverrides::default())
            .map_err(|e| anyhow::anyhow!("Config error: {e}"))?;

        let max_results = std::env::var("MCP_MAX_RESULTS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(50);

        let region = config.region();
        let credentials = config
            .credentials()
            .map_err(|e| anyhow::anyhow!("Credentials error: {e}"))?;

        Ok(Self {
            region,
            credentials,
            max_results,
        })
    }
}
