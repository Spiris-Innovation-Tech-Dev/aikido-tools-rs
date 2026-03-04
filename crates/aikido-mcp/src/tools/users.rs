use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AikidoUsersListInput {
    /// Page number (starts at 0)
    #[serde(default)]
    pub page: Option<u32>,
    /// Number of items to return
    #[serde(default)]
    pub per_page: Option<u32>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AikidoUserGetInput {
    /// The user ID
    pub user_id: i64,
}
