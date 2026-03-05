use crate::config::McpConfig;
use aikido::AikidoClient;
use std::sync::Arc;

#[allow(dead_code)]
pub struct ServerState {
    pub client: Arc<AikidoClient>,
    pub max_results: u32,
    pub allow_raw_api_mutations: bool,
}

impl ServerState {
    pub fn new(config: McpConfig) -> Self {
        let client = AikidoClient::new(config.region, config.credentials);
        Self {
            client: Arc::new(client),
            max_results: config.max_results,
            allow_raw_api_mutations: config.allow_raw_api_mutations,
        }
    }

    pub async fn test_connection(&self) -> anyhow::Result<()> {
        self.client
            .get_workspace_info()
            .await
            .map_err(|e| anyhow::anyhow!("Connection test failed: {e}"))?;
        Ok(())
    }
}
