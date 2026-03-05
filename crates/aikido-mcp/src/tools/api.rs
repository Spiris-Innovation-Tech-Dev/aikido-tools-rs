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
    /// Must be true for mutating calls. Adds explicit user intent for write operations.
    #[serde(default)]
    pub confirm_mutation: bool,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AikidoApiPutInput {
    /// Endpoint path, e.g. /issues/123/ignore
    pub endpoint: String,
    /// JSON request body
    #[serde(default)]
    pub body: Option<serde_json::Value>,
    /// Must be true for mutating calls. Adds explicit user intent for write operations.
    #[serde(default)]
    pub confirm_mutation: bool,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AikidoApiDeleteInput {
    /// Endpoint path, e.g. /domains/42
    pub endpoint: String,
    /// Must be true for mutating calls. Adds explicit user intent for write operations.
    #[serde(default)]
    pub confirm_mutation: bool,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AikidoApiOperationsListInput {}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AikidoApiExecInput {
    /// OpenAPI operationId, e.g. listCodeRepos
    pub operation_id: String,
    /// Path params as key/value map
    #[serde(default)]
    pub path_params: Option<std::collections::BTreeMap<String, String>>,
    /// Query params as key/value map
    #[serde(default)]
    pub query_params: Option<std::collections::BTreeMap<String, String>>,
    /// JSON request body for POST/PUT
    #[serde(default)]
    pub body: Option<serde_json::Value>,
    /// Must be true for mutating calls. Adds explicit user intent for write operations.
    #[serde(default)]
    pub confirm_mutation: bool,
}
