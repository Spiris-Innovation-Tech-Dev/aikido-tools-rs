use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AikidoReposListInput {
    /// Page number (starts at 0)
    #[serde(default)]
    pub page: Option<u32>,
    /// Number of items to return
    #[serde(default)]
    pub per_page: Option<u32>,
    /// Include inactive repositories
    #[serde(default)]
    pub include_inactive: Option<bool>,
    /// Filter repositories by name
    #[serde(default)]
    pub filter_name: Option<String>,
    /// Filter repositories by branch
    #[serde(default)]
    pub filter_branch: Option<String>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AikidoRepoGetInput {
    /// The code repository ID
    pub repo_id: i64,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AikidoRepoScanInput {
    /// The code repository ID to scan
    pub repo_id: i64,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AikidoSastRulesListInput {}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AikidoIacRulesListInput {}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AikidoCustomRulesListInput {}
