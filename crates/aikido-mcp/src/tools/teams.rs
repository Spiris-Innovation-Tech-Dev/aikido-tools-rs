use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AikidoTeamsListInput {
    /// Page number (starts at 0)
    #[serde(default)]
    pub page: Option<u32>,
    /// Number of items to return
    #[serde(default)]
    pub per_page: Option<u32>,
    /// Include inactive teams
    #[serde(default)]
    pub include_inactive: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AikidoTeamCreateInput {
    /// Team name
    pub name: String,
}
