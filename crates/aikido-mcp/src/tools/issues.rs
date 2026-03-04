use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AikidoIssueGroupsListInput {
    /// Page number (starts at 0)
    #[serde(default)]
    pub page: Option<u32>,
    /// Number of items to return
    #[serde(default)]
    pub per_page: Option<u32>,
    /// Filter by code repository ID
    #[serde(default)]
    pub filter_code_repo_id: Option<i64>,
    /// Filter by repository name
    #[serde(default)]
    pub filter_code_repo_name: Option<String>,
    /// Filter by team ID
    #[serde(default)]
    pub filter_team_id: Option<i64>,
    /// Filter by issue type
    #[serde(default)]
    pub filter_issue_type: Option<String>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AikidoIssueGroupGetInput {
    /// The issue group ID
    pub group_id: i64,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AikidoIssueGetInput {
    /// The issue ID
    pub issue_id: i64,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AikidoIssueCountsInput {}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AikidoIssueExportInput {}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AikidoIssueSnoozeInput {
    /// The issue ID
    pub issue_id: i64,
    /// Snooze until date (ISO 8601 format)
    pub snooze_until: String,
    /// Optional reason
    #[serde(default)]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AikidoIssueUnsnoozeInput {
    /// The issue ID
    pub issue_id: i64,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AikidoIssueIgnoreInput {
    /// The issue ID
    pub issue_id: i64,
    /// Optional reason for ignoring
    #[serde(default)]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AikidoIssueUnignoreInput {
    /// The issue ID
    pub issue_id: i64,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AikidoIssueGroupSnoozeInput {
    /// The issue group ID
    pub group_id: i64,
    /// Snooze until date (ISO 8601 format)
    pub snooze_until: String,
    /// Optional reason
    #[serde(default)]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AikidoIssueGroupUnsnoozeInput {
    /// The issue group ID
    pub group_id: i64,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AikidoIssueGroupIgnoreInput {
    /// The issue group ID
    pub group_id: i64,
    /// Optional reason for ignoring
    #[serde(default)]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AikidoIssueGroupUnignoreInput {
    /// The issue group ID
    pub group_id: i64,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AikidoIssueGroupNotesAddInput {
    /// The issue group ID
    pub group_id: i64,
    /// The note/comment text
    pub comment: String,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AikidoIssueGroupTasksInput {
    /// The issue group ID
    pub group_id: i64,
}
