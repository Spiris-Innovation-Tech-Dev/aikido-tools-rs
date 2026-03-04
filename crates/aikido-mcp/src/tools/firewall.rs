use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AikidoFirewallAppsListInput {
    /// Page number (starts at 0)
    #[serde(default)]
    pub page: Option<u32>,
    /// Number of items to return
    #[serde(default)]
    pub per_page: Option<u32>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AikidoFirewallAppGetInput {
    /// The app ID
    pub app_id: i64,
}
