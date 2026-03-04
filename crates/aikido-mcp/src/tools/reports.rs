use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AikidoComplianceIsoInput {}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AikidoComplianceSoc2Input {}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AikidoComplianceNis2Input {}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AikidoActivityLogInput {
    /// Page number (starts at 0)
    #[serde(default)]
    pub page: Option<u32>,
    /// Number of items to return
    #[serde(default)]
    pub per_page: Option<u32>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AikidoCiScansInput {
    /// Page number (starts at 0)
    #[serde(default)]
    pub page: Option<u32>,
    /// Number of items to return
    #[serde(default)]
    pub per_page: Option<u32>,
}
