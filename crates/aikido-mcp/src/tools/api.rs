use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AikidoApiGetInput {
    /// Endpoint path, e.g. /repositories/code?page=0&per_page=20
    pub endpoint: String,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AikidoApiPostInput {
    /// Endpoint path, e.g. /domains
    pub endpoint: String,
    /// JSON request body
    #[serde(default)]
    pub body: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AikidoApiPutInput {
    /// Endpoint path, e.g. /issues/123/ignore
    pub endpoint: String,
    /// JSON request body
    #[serde(default)]
    pub body: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AikidoApiDeleteInput {
    /// Endpoint path, e.g. /domains/42
    pub endpoint: String,
}
