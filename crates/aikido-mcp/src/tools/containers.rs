use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AikidoContainersListInput {
    /// Page number (starts at 0)
    #[serde(default)]
    pub page: Option<u32>,
    /// Number of items to return
    #[serde(default)]
    pub per_page: Option<u32>,
    /// Filter by team ID
    #[serde(default)]
    pub filter_team_id: Option<i64>,
    /// Filter by cloud ID
    #[serde(default)]
    pub filter_cloud_id: Option<i64>,
    /// Include inactive containers
    #[serde(default)]
    pub include_inactive: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AikidoContainerGetInput {
    /// The container repository ID
    pub container_id: i64,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AikidoContainerScanInput {
    /// The container repository ID to scan
    pub container_id: i64,
}
