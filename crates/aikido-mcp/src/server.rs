use crate::state::ServerState;
#[allow(unused_imports)]
use crate::tools::{
    api::{
        AikidoApiDeleteInput, AikidoApiExecInput, AikidoApiGetInput,
        AikidoApiOperationsListInput, AikidoApiPostInput, AikidoApiPutInput,
    },
    clouds::AikidoCloudsListInput,
    containers::{AikidoContainerGetInput, AikidoContainerScanInput, AikidoContainersListInput},
    domains::{AikidoDomainScanInput, AikidoDomainsListInput},
    firewall::{AikidoFirewallAppGetInput, AikidoFirewallAppsListInput},
    issues::{
        AikidoIssueCountsInput, AikidoIssueExportInput, AikidoIssueGetInput,
        AikidoIssueGroupGetInput, AikidoIssueGroupIgnoreInput, AikidoIssueGroupNotesAddInput,
        AikidoIssueGroupSnoozeInput, AikidoIssueGroupTasksInput, AikidoIssueGroupUnignoreInput,
        AikidoIssueGroupUnsnoozeInput, AikidoIssueGroupsListInput, AikidoIssueIgnoreInput,
        AikidoIssueSnoozeInput, AikidoIssueUnignoreInput, AikidoIssueUnsnoozeInput,
    },
    reports::{
        AikidoActivityLogInput, AikidoCiScansInput, AikidoComplianceIsoInput,
        AikidoComplianceNis2Input, AikidoComplianceSoc2Input,
    },
    repositories::{
        AikidoCustomRulesListInput, AikidoIacRulesListInput, AikidoRepoGetInput,
        AikidoRepoScanInput, AikidoReposListInput, AikidoSastRulesListInput,
    },
    teams::{AikidoTeamCreateInput, AikidoTeamsListInput},
    users::{AikidoUserGetInput, AikidoUsersListInput},
    workspace::AikidoWorkspaceInfoInput,
};
use aikido::models::*;
use rmcp::{
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{
        CallToolResult, Content, ErrorData, Implementation, ProtocolVersion, ServerCapabilities,
        ServerInfo,
    },
    tool, tool_handler, tool_router, ServerHandler,
};
use serde_json::Value;
use std::sync::Arc;
use urlencoding::encode;

#[allow(dead_code)]
fn aikido_error_to_mcp(err: aikido::error::AikidoError) -> ErrorData {
    use rmcp::model::ErrorCode;
    ErrorData::new(ErrorCode(-32603), err.to_string(), None)
}

#[derive(Clone)]
#[allow(dead_code)]
pub struct AikidoMcpServer {
    pub state: Arc<ServerState>,
    tool_router: ToolRouter<Self>,
}

fn normalize_endpoint(endpoint: &str) -> String {
    if endpoint.starts_with('/') {
        endpoint.to_string()
    } else {
        format!("/{endpoint}")
    }
}

const RAW_API_ALLOWED_PREFIXES: &[&str] = &[
    "/workspace",
    "/open-issue-groups",
    "/issues",
    "/repositories",
    "/containers",
    "/clouds",
    "/domains",
    "/teams",
    "/users",
    "/firewall",
    "/report",
    "/task_tracking",
    "/pentests",
    "/licenses",
    "/changelog-summary",
    "/localscan",
    "/virtual-machines",
];

fn raw_api_invalid_params(message: impl Into<String>) -> ErrorData {
    use rmcp::model::ErrorCode;
    ErrorData::new(ErrorCode(-32602), message.into(), None)
}

fn raw_api_permission_denied(message: impl Into<String>) -> ErrorData {
    use rmcp::model::ErrorCode;
    ErrorData::new(ErrorCode(-32001), message.into(), None)
}

fn has_allowed_prefix(path: &str) -> bool {
    RAW_API_ALLOWED_PREFIXES.iter().any(|prefix| {
        path == *prefix
            || path
                .strip_prefix(prefix)
                .is_some_and(|remaining| remaining.starts_with('/'))
    })
}

fn validate_raw_api_endpoint(endpoint: &str) -> Result<String, ErrorData> {
    let endpoint = normalize_endpoint(endpoint);
    let path = endpoint.split('?').next().unwrap_or(endpoint.as_str());

    if path.is_empty() || path == "/" {
        return Err(raw_api_invalid_params(
            "Raw API endpoint must contain a non-empty path.",
        ));
    }
    if endpoint.contains("://") {
        return Err(raw_api_invalid_params(
            "Raw API endpoint must be a relative path, not a full URL.",
        ));
    }
    if endpoint.contains('#')
        || endpoint.contains('\\')
        || path.contains("..")
        || endpoint.chars().any(|ch| ch.is_control())
    {
        return Err(raw_api_invalid_params(
            "Raw API endpoint contains invalid characters.",
        ));
    }
    if !has_allowed_prefix(path) {
        return Err(raw_api_invalid_params(format!(
            "Raw API endpoint '{path}' is not allowed."
        )));
    }

    Ok(endpoint)
}

fn append_query_param(endpoint: &mut String, key: &str, value: &str) {
    let sep = if endpoint.contains('?') { "&" } else { "?" };
    endpoint.push_str(sep);
    endpoint.push_str(key);
    endpoint.push('=');
    endpoint.push_str(&encode(value));
}

fn append_query_map(endpoint: &mut String, values: &std::collections::BTreeMap<String, String>) {
    for (k, v) in values {
        append_query_param(endpoint, k, v);
    }
}

fn resolve_per_page(requested: Option<u32>, max_results: u32) -> u32 {
    let fallback = if max_results == 0 { 1 } else { max_results };
    requested.unwrap_or(fallback).min(fallback).max(1)
}

#[tool_router(router = tool_router)]
impl AikidoMcpServer {
    pub fn new(state: ServerState) -> Self {
        Self {
            state: Arc::new(state),
            tool_router: Self::tool_router(),
        }
    }

    fn ensure_raw_api_mutation_allowed(&self, confirm_mutation: bool) -> Result<(), ErrorData> {
        if !self.state.allow_raw_api_mutations {
            return Err(raw_api_permission_denied(
                "Raw API mutations are disabled. Set MCP_ALLOW_RAW_API_MUTATIONS=true to enable them.",
            ));
        }
        if !confirm_mutation {
            return Err(raw_api_invalid_params(
                "Mutating raw API calls require confirm_mutation=true.",
            ));
        }
        Ok(())
    }

    #[tool(
        description = "Get workspace information including name, linked provider, and organization."
    )]
    pub async fn aikido_workspace_info(
        &self,
        Parameters(_input): Parameters<AikidoWorkspaceInfoInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let info = self
            .state
            .client
            .get_workspace_info()
            .await
            .map_err(aikido_error_to_mcp)?;
        let json = serde_json::to_string_pretty(&info).unwrap_or_default();
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "List all open issue groups with severity, type, status, and locations.")]
    pub async fn aikido_issues_list(
        &self,
        Parameters(input): Parameters<AikidoIssueGroupsListInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let mut endpoint = "/open-issue-groups".to_string();
        if let Some(page) = input.page {
            append_query_param(&mut endpoint, "page", &page.to_string());
        }
        append_query_param(
            &mut endpoint,
            "per_page",
            &resolve_per_page(input.per_page, self.state.max_results).to_string(),
        );
        if let Some(repo_id) = input.filter_code_repo_id {
            append_query_param(&mut endpoint, "filter_code_repo_id", &repo_id.to_string());
        }
        if let Some(repo_name) = input.filter_code_repo_name {
            append_query_param(&mut endpoint, "filter_code_repo_name", &repo_name);
        }
        if let Some(team_id) = input.filter_team_id {
            append_query_param(&mut endpoint, "filter_team_id", &team_id.to_string());
        }
        if let Some(issue_type) = input.filter_issue_type {
            append_query_param(&mut endpoint, "filter_issue_type", &issue_type);
        }

        let groups: Value = self
            .state
            .client
            .get(&endpoint)
            .await
            .map_err(aikido_error_to_mcp)?;
        let json = serde_json::to_string_pretty(&groups).unwrap_or_default();
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(
        description = "Get detailed information about a specific issue group including all individual issues with affected files, packages, repos, and remediation guidance."
    )]
    pub async fn aikido_issues_group_get(
        &self,
        Parameters(input): Parameters<AikidoIssueGroupGetInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let (group, issues) = tokio::try_join!(
            self.state.client.get_issue_group(input.group_id),
            self.state.client.get_issue_group_issues(input.group_id),
        )
        .map_err(aikido_error_to_mcp)?;
        let mut result = serde_json::to_value(&group).unwrap_or_default();
        result["issues"] = serde_json::to_value(&issues).unwrap_or_default();
        let json = serde_json::to_string_pretty(&result).unwrap_or_default();
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Get detailed information about a specific issue by ID.")]
    pub async fn aikido_issues_get(
        &self,
        Parameters(input): Parameters<AikidoIssueGetInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let issue = self
            .state
            .client
            .get_issue(input.issue_id)
            .await
            .map_err(aikido_error_to_mcp)?;
        let json = serde_json::to_string_pretty(&issue).unwrap_or_default();
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Get issue counts aggregated by severity and type.")]
    pub async fn aikido_issues_counts(
        &self,
        Parameters(_input): Parameters<AikidoIssueCountsInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let counts = self
            .state
            .client
            .get_issue_counts()
            .await
            .map_err(aikido_error_to_mcp)?;
        let json = serde_json::to_string_pretty(&counts).unwrap_or_default();
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Export all issues as JSON.")]
    pub async fn aikido_issues_export(
        &self,
        Parameters(_input): Parameters<AikidoIssueExportInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let data = self
            .state
            .client
            .export_issues()
            .await
            .map_err(aikido_error_to_mcp)?;
        let json = serde_json::to_string_pretty(&data).unwrap_or_default();
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Snooze an issue until a specified date.")]
    pub async fn aikido_issues_snooze(
        &self,
        Parameters(input): Parameters<AikidoIssueSnoozeInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let body = SnoozeRequest {
            snooze_until: input.snooze_until,
            reason: input.reason,
        };
        self.state
            .client
            .snooze_issue(input.issue_id, &body)
            .await
            .map_err(aikido_error_to_mcp)?;
        Ok(CallToolResult::success(vec![Content::text(
            "Issue snoozed",
        )]))
    }

    #[tool(description = "Unsnooze a previously snoozed issue.")]
    pub async fn aikido_issues_unsnooze(
        &self,
        Parameters(input): Parameters<AikidoIssueUnsnoozeInput>,
    ) -> Result<CallToolResult, ErrorData> {
        self.state
            .client
            .unsnooze_issue(input.issue_id)
            .await
            .map_err(aikido_error_to_mcp)?;
        Ok(CallToolResult::success(vec![Content::text(
            "Issue unsnoozed",
        )]))
    }

    #[tool(description = "Ignore an issue with an optional reason.")]
    pub async fn aikido_issues_ignore(
        &self,
        Parameters(input): Parameters<AikidoIssueIgnoreInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let body = IgnoreRequest {
            reason: input.reason,
        };
        self.state
            .client
            .ignore_issue(input.issue_id, &body)
            .await
            .map_err(aikido_error_to_mcp)?;
        Ok(CallToolResult::success(vec![Content::text(
            "Issue ignored",
        )]))
    }

    #[tool(description = "Unignore a previously ignored issue.")]
    pub async fn aikido_issues_unignore(
        &self,
        Parameters(input): Parameters<AikidoIssueUnignoreInput>,
    ) -> Result<CallToolResult, ErrorData> {
        self.state
            .client
            .unignore_issue(input.issue_id)
            .await
            .map_err(aikido_error_to_mcp)?;
        Ok(CallToolResult::success(vec![Content::text(
            "Issue unignored",
        )]))
    }

    #[tool(description = "Snooze an issue group until a specified date.")]
    pub async fn aikido_issues_group_snooze(
        &self,
        Parameters(input): Parameters<AikidoIssueGroupSnoozeInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let body = SnoozeRequest {
            snooze_until: input.snooze_until,
            reason: input.reason,
        };
        self.state
            .client
            .snooze_issue_group(input.group_id, &body)
            .await
            .map_err(aikido_error_to_mcp)?;
        Ok(CallToolResult::success(vec![Content::text(
            "Issue group snoozed",
        )]))
    }

    #[tool(description = "Unsnooze a previously snoozed issue group.")]
    pub async fn aikido_issues_group_unsnooze(
        &self,
        Parameters(input): Parameters<AikidoIssueGroupUnsnoozeInput>,
    ) -> Result<CallToolResult, ErrorData> {
        self.state
            .client
            .unsnooze_issue_group(input.group_id)
            .await
            .map_err(aikido_error_to_mcp)?;
        Ok(CallToolResult::success(vec![Content::text(
            "Issue group unsnoozed",
        )]))
    }

    #[tool(description = "Ignore an issue group with an optional reason.")]
    pub async fn aikido_issues_group_ignore(
        &self,
        Parameters(input): Parameters<AikidoIssueGroupIgnoreInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let body = IgnoreRequest {
            reason: input.reason,
        };
        self.state
            .client
            .ignore_issue_group(input.group_id, &body)
            .await
            .map_err(aikido_error_to_mcp)?;
        Ok(CallToolResult::success(vec![Content::text(
            "Issue group ignored",
        )]))
    }

    #[tool(description = "Unignore a previously ignored issue group.")]
    pub async fn aikido_issues_group_unignore(
        &self,
        Parameters(input): Parameters<AikidoIssueGroupUnignoreInput>,
    ) -> Result<CallToolResult, ErrorData> {
        self.state
            .client
            .unignore_issue_group(input.group_id)
            .await
            .map_err(aikido_error_to_mcp)?;
        Ok(CallToolResult::success(vec![Content::text(
            "Issue group unignored",
        )]))
    }

    #[tool(description = "Add a note/comment to an issue group.")]
    pub async fn aikido_issues_group_notes_add(
        &self,
        Parameters(input): Parameters<AikidoIssueGroupNotesAddInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let body = NoteRequest {
            comment: input.comment,
        };
        self.state
            .client
            .add_note_to_issue_group(input.group_id, &body)
            .await
            .map_err(aikido_error_to_mcp)?;
        Ok(CallToolResult::success(vec![Content::text("Note added")]))
    }

    #[tool(description = "Get tasks linked to an issue group.")]
    pub async fn aikido_issues_group_tasks(
        &self,
        Parameters(input): Parameters<AikidoIssueGroupTasksInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let tasks = self
            .state
            .client
            .get_issue_group_tasks(input.group_id)
            .await
            .map_err(aikido_error_to_mcp)?;
        let json = serde_json::to_string_pretty(&tasks).unwrap_or_default();
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(
        description = "List all code repositories with name, provider, branch, and scan status."
    )]
    pub async fn aikido_repos_list(
        &self,
        Parameters(input): Parameters<AikidoReposListInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let mut endpoint = "/repositories/code".to_string();
        if let Some(page) = input.page {
            append_query_param(&mut endpoint, "page", &page.to_string());
        }
        append_query_param(
            &mut endpoint,
            "per_page",
            &resolve_per_page(input.per_page, self.state.max_results).to_string(),
        );
        if let Some(include_inactive) = input.include_inactive {
            append_query_param(
                &mut endpoint,
                "include_inactive",
                if include_inactive { "true" } else { "false" },
            );
        }
        if let Some(filter_name) = input.filter_name {
            append_query_param(&mut endpoint, "filter_name", &filter_name);
        }
        if let Some(filter_branch) = input.filter_branch {
            append_query_param(&mut endpoint, "filter_branch", &filter_branch);
        }

        let repos: Value = self
            .state
            .client
            .get(&endpoint)
            .await
            .map_err(aikido_error_to_mcp)?;
        let json = serde_json::to_string_pretty(&repos).unwrap_or_default();
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Get detailed information about a code repository.")]
    pub async fn aikido_repos_get(
        &self,
        Parameters(input): Parameters<AikidoRepoGetInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let repo = self
            .state
            .client
            .get_code_repo(input.repo_id)
            .await
            .map_err(aikido_error_to_mcp)?;
        let json = serde_json::to_string_pretty(&repo).unwrap_or_default();
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Trigger a scan for a code repository.")]
    pub async fn aikido_repos_scan(
        &self,
        Parameters(input): Parameters<AikidoRepoScanInput>,
    ) -> Result<CallToolResult, ErrorData> {
        self.state
            .client
            .scan_code_repo(input.repo_id)
            .await
            .map_err(aikido_error_to_mcp)?;
        Ok(CallToolResult::success(vec![Content::text(
            "Scan triggered",
        )]))
    }

    #[tool(description = "List SAST (Static Application Security Testing) rules.")]
    pub async fn aikido_repos_sast_rules(
        &self,
        Parameters(_input): Parameters<AikidoSastRulesListInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let rules = self
            .state
            .client
            .list_sast_rules()
            .await
            .map_err(aikido_error_to_mcp)?;
        let json = serde_json::to_string_pretty(&rules).unwrap_or_default();
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "List IaC (Infrastructure as Code) scanning rules.")]
    pub async fn aikido_repos_iac_rules(
        &self,
        Parameters(_input): Parameters<AikidoIacRulesListInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let rules = self
            .state
            .client
            .list_iac_rules()
            .await
            .map_err(aikido_error_to_mcp)?;
        let json = serde_json::to_string_pretty(&rules).unwrap_or_default();
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "List custom SAST rules.")]
    pub async fn aikido_repos_custom_rules(
        &self,
        Parameters(_input): Parameters<AikidoCustomRulesListInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let rules = self
            .state
            .client
            .list_custom_rules()
            .await
            .map_err(aikido_error_to_mcp)?;
        let json = serde_json::to_string_pretty(&rules).unwrap_or_default();
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "List container repositories with name, provider, tag, and scan status.")]
    pub async fn aikido_containers_list(
        &self,
        Parameters(input): Parameters<AikidoContainersListInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let mut endpoint = "/containers".to_string();
        if let Some(page) = input.page {
            append_query_param(&mut endpoint, "page", &page.to_string());
        }
        append_query_param(
            &mut endpoint,
            "per_page",
            &resolve_per_page(input.per_page, self.state.max_results).to_string(),
        );
        if let Some(team_id) = input.filter_team_id {
            append_query_param(&mut endpoint, "filter_team_id", &team_id.to_string());
        }
        if let Some(cloud_id) = input.filter_cloud_id {
            append_query_param(&mut endpoint, "filter_cloud_id", &cloud_id.to_string());
        }
        if let Some(include_inactive) = input.include_inactive {
            append_query_param(
                &mut endpoint,
                "include_inactive",
                if include_inactive { "true" } else { "false" },
            );
        }

        let containers: Value = self
            .state
            .client
            .get(&endpoint)
            .await
            .map_err(aikido_error_to_mcp)?;
        let json = serde_json::to_string_pretty(&containers).unwrap_or_default();
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Get detailed information about a container repository.")]
    pub async fn aikido_containers_get(
        &self,
        Parameters(input): Parameters<AikidoContainerGetInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let container = self
            .state
            .client
            .get_container(input.container_id)
            .await
            .map_err(aikido_error_to_mcp)?;
        let json = serde_json::to_string_pretty(&container).unwrap_or_default();
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Trigger a scan for a container repository.")]
    pub async fn aikido_containers_scan(
        &self,
        Parameters(input): Parameters<AikidoContainerScanInput>,
    ) -> Result<CallToolResult, ErrorData> {
        self.state
            .client
            .scan_container(input.container_id)
            .await
            .map_err(aikido_error_to_mcp)?;
        Ok(CallToolResult::success(vec![Content::text(
            "Scan triggered",
        )]))
    }

    #[tool(description = "List connected cloud environments (AWS, GCP, Azure).")]
    pub async fn aikido_clouds_list(
        &self,
        Parameters(input): Parameters<AikidoCloudsListInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let mut endpoint = "/clouds".to_string();
        if let Some(page) = input.page {
            append_query_param(&mut endpoint, "page", &page.to_string());
        }
        append_query_param(
            &mut endpoint,
            "per_page",
            &resolve_per_page(input.per_page, self.state.max_results).to_string(),
        );

        let clouds: Value = self
            .state
            .client
            .get(&endpoint)
            .await
            .map_err(aikido_error_to_mcp)?;
        let json = serde_json::to_string_pretty(&clouds).unwrap_or_default();
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "List domains configured for surface monitoring.")]
    pub async fn aikido_domains_list(
        &self,
        Parameters(input): Parameters<AikidoDomainsListInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let mut endpoint = "/domains".to_string();
        if let Some(page) = input.page {
            append_query_param(&mut endpoint, "page", &page.to_string());
        }
        append_query_param(
            &mut endpoint,
            "per_page",
            &resolve_per_page(input.per_page, self.state.max_results).to_string(),
        );
        if let Some(include_inactive) = input.include_inactive {
            append_query_param(
                &mut endpoint,
                "include_inactive",
                if include_inactive { "true" } else { "false" },
            );
        }

        let domains: Value = self
            .state
            .client
            .get(&endpoint)
            .await
            .map_err(aikido_error_to_mcp)?;
        let json = serde_json::to_string_pretty(&domains).unwrap_or_default();
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Start a scan for a domain.")]
    pub async fn aikido_domains_scan(
        &self,
        Parameters(input): Parameters<AikidoDomainScanInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let body = StartDomainScanRequest {
            domain_id: input.domain_id,
        };
        self.state
            .client
            .start_domain_scan(&body)
            .await
            .map_err(aikido_error_to_mcp)?;
        Ok(CallToolResult::success(vec![Content::text(
            "Domain scan started",
        )]))
    }

    #[tool(description = "List all teams with their responsibilities.")]
    pub async fn aikido_teams_list(
        &self,
        Parameters(input): Parameters<AikidoTeamsListInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let mut endpoint = "/teams".to_string();
        if let Some(page) = input.page {
            append_query_param(&mut endpoint, "page", &page.to_string());
        }
        append_query_param(
            &mut endpoint,
            "per_page",
            &resolve_per_page(input.per_page, self.state.max_results).to_string(),
        );
        if let Some(include_inactive) = input.include_inactive {
            append_query_param(
                &mut endpoint,
                "include_inactive",
                if include_inactive { "true" } else { "false" },
            );
        }

        let teams: Value = self
            .state
            .client
            .get(&endpoint)
            .await
            .map_err(aikido_error_to_mcp)?;
        let json = serde_json::to_string_pretty(&teams).unwrap_or_default();
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Create a new team.")]
    pub async fn aikido_teams_create(
        &self,
        Parameters(input): Parameters<AikidoTeamCreateInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let body = CreateTeamRequest { name: input.name };
        let result = self
            .state
            .client
            .create_team(&body)
            .await
            .map_err(aikido_error_to_mcp)?;
        let json = serde_json::to_string_pretty(&result).unwrap_or_default();
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "List all users in the workspace.")]
    pub async fn aikido_users_list(
        &self,
        Parameters(input): Parameters<AikidoUsersListInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let mut endpoint = "/users".to_string();
        if let Some(page) = input.page {
            append_query_param(&mut endpoint, "page", &page.to_string());
        }
        append_query_param(
            &mut endpoint,
            "per_page",
            &resolve_per_page(input.per_page, self.state.max_results).to_string(),
        );

        let users: Value = self
            .state
            .client
            .get(&endpoint)
            .await
            .map_err(aikido_error_to_mcp)?;
        let json = serde_json::to_string_pretty(&users).unwrap_or_default();
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Get detailed information about a user.")]
    pub async fn aikido_users_get(
        &self,
        Parameters(input): Parameters<AikidoUserGetInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let user = self
            .state
            .client
            .get_user(input.user_id)
            .await
            .map_err(aikido_error_to_mcp)?;
        let json = serde_json::to_string_pretty(&user).unwrap_or_default();
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "List Zen/firewall apps.")]
    pub async fn aikido_firewall_apps_list(
        &self,
        Parameters(input): Parameters<AikidoFirewallAppsListInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let mut endpoint = "/firewall/apps".to_string();
        if let Some(page) = input.page {
            append_query_param(&mut endpoint, "page", &page.to_string());
        }
        append_query_param(
            &mut endpoint,
            "per_page",
            &resolve_per_page(input.per_page, self.state.max_results).to_string(),
        );

        let apps: Value = self
            .state
            .client
            .get(&endpoint)
            .await
            .map_err(aikido_error_to_mcp)?;
        let json = serde_json::to_string_pretty(&apps).unwrap_or_default();
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Get detailed information about a firewall app.")]
    pub async fn aikido_firewall_app_get(
        &self,
        Parameters(input): Parameters<AikidoFirewallAppGetInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let app = self
            .state
            .client
            .get_firewall_app(input.app_id)
            .await
            .map_err(aikido_error_to_mcp)?;
        let json = serde_json::to_string_pretty(&app).unwrap_or_default();
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Get ISO 27001 compliance overview.")]
    pub async fn aikido_compliance_iso(
        &self,
        Parameters(_input): Parameters<AikidoComplianceIsoInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let overview = self
            .state
            .client
            .get_iso_compliance_overview()
            .await
            .map_err(aikido_error_to_mcp)?;
        let json = serde_json::to_string_pretty(&overview).unwrap_or_default();
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Get SOC2 compliance overview.")]
    pub async fn aikido_compliance_soc2(
        &self,
        Parameters(_input): Parameters<AikidoComplianceSoc2Input>,
    ) -> Result<CallToolResult, ErrorData> {
        let overview = self
            .state
            .client
            .get_soc2_compliance_overview()
            .await
            .map_err(aikido_error_to_mcp)?;
        let json = serde_json::to_string_pretty(&overview).unwrap_or_default();
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Get NIS2 compliance overview.")]
    pub async fn aikido_compliance_nis2(
        &self,
        Parameters(_input): Parameters<AikidoComplianceNis2Input>,
    ) -> Result<CallToolResult, ErrorData> {
        let overview = self
            .state
            .client
            .get_nis2_compliance_overview()
            .await
            .map_err(aikido_error_to_mcp)?;
        let json = serde_json::to_string_pretty(&overview).unwrap_or_default();
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Get the activity log.")]
    pub async fn aikido_activity_log(
        &self,
        Parameters(input): Parameters<AikidoActivityLogInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let mut endpoint = "/report/activityLog".to_string();
        if let Some(page) = input.page {
            append_query_param(&mut endpoint, "page", &page.to_string());
        }
        append_query_param(
            &mut endpoint,
            "per_page",
            &resolve_per_page(input.per_page, self.state.max_results).to_string(),
        );

        let log: Value = self
            .state
            .client
            .get(&endpoint)
            .await
            .map_err(aikido_error_to_mcp)?;
        let json = serde_json::to_string_pretty(&log).unwrap_or_default();
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "List CI scan results.")]
    pub async fn aikido_ci_scans(
        &self,
        Parameters(input): Parameters<AikidoCiScansInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let mut endpoint = "/report/ciScans".to_string();
        if let Some(page) = input.page {
            append_query_param(&mut endpoint, "page", &page.to_string());
        }
        append_query_param(
            &mut endpoint,
            "per_page",
            &resolve_per_page(input.per_page, self.state.max_results).to_string(),
        );

        let scans: Value = self
            .state
            .client
            .get(&endpoint)
            .await
            .map_err(aikido_error_to_mcp)?;
        let json = serde_json::to_string_pretty(&scans).unwrap_or_default();
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Raw API GET passthrough for allowed public endpoints.")]
    pub async fn aikido_api_get(
        &self,
        Parameters(input): Parameters<AikidoApiGetInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let endpoint = validate_raw_api_endpoint(&input.endpoint)?;
        let data: Value = self
            .state
            .client
            .get(&endpoint)
            .await
            .map_err(aikido_error_to_mcp)?;
        let json = serde_json::to_string_pretty(&data).unwrap_or_default();
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "List all bundled OpenAPI operations with method and path.")]
    pub async fn aikido_api_operations_list(
        &self,
        Parameters(_input): Parameters<AikidoApiOperationsListInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let operations = aikido::openapi::list_operations()
            .into_iter()
            .map(|op| {
                serde_json::json!({
                    "operation_id": op.operation_id,
                    "method": op.method,
                    "path": op.path,
                })
            })
            .collect::<Vec<_>>();
        let json = serde_json::to_string_pretty(&serde_json::json!({
            "total": operations.len(),
            "operations": operations,
        }))
        .unwrap_or_default();
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(
        description = "Execute API call by OpenAPI operationId. Mutating operations require confirm_mutation=true and MCP_ALLOW_RAW_API_MUTATIONS=true."
    )]
    pub async fn aikido_api_exec(
        &self,
        Parameters(input): Parameters<AikidoApiExecInput>,
    ) -> Result<CallToolResult, ErrorData> {
        let Some(op) = aikido::openapi::find_operation(&input.operation_id) else {
            return Err(raw_api_invalid_params(format!(
                "Unknown operationId: {}",
                input.operation_id
            )));
        };

        let path_params = input.path_params.unwrap_or_default();
        let path_pairs = path_params.into_iter().collect::<Vec<_>>();
        let rendered = aikido::openapi::render_path(&op.path, &path_pairs);
        let mut endpoint = normalize_endpoint(&rendered);
        if let Some(query) = &input.query_params {
            append_query_map(&mut endpoint, query);
        }
        let endpoint = validate_raw_api_endpoint(&endpoint)?;

        if matches!(op.method.as_str(), "post" | "put" | "delete") {
            self.ensure_raw_api_mutation_allowed(input.confirm_mutation)?;
        }

        let result = match op.method.as_str() {
            "get" => self
                .state
                .client
                .get::<Value>(&endpoint)
                .await
                .map_err(aikido_error_to_mcp)?,
            "post" => {
                let body = input
                    .body
                    .unwrap_or_else(|| Value::Object(serde_json::Map::new()));
                self.state
                    .client
                    .post::<Value, _>(&endpoint, &body)
                    .await
                    .map_err(aikido_error_to_mcp)?
            }
            "put" => {
                let body = input
                    .body
                    .unwrap_or_else(|| Value::Object(serde_json::Map::new()));
                self.state
                    .client
                    .put::<Value, _>(&endpoint, &body)
                    .await
                    .map_err(aikido_error_to_mcp)?
            }
            "delete" => {
                self.state
                    .client
                    .delete(&endpoint)
                    .await
                    .map_err(aikido_error_to_mcp)?;
                serde_json::json!({"success": true})
            }
            other => return Err(raw_api_invalid_params(format!("Unsupported method in spec: {other}"))),
        };

        let json = serde_json::to_string_pretty(&serde_json::json!({
            "operation": {
                "operation_id": op.operation_id,
                "method": op.method,
                "endpoint": endpoint,
            },
            "result": result,
        }))
        .unwrap_or_default();
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(
        description = "Raw API POST passthrough for allowed public endpoints. Requires confirm_mutation=true and MCP_ALLOW_RAW_API_MUTATIONS=true."
    )]
    pub async fn aikido_api_post(
        &self,
        Parameters(input): Parameters<AikidoApiPostInput>,
    ) -> Result<CallToolResult, ErrorData> {
        self.ensure_raw_api_mutation_allowed(input.confirm_mutation)?;
        let endpoint = validate_raw_api_endpoint(&input.endpoint)?;
        let body = input
            .body
            .unwrap_or_else(|| Value::Object(serde_json::Map::new()));
        let data: Value = self
            .state
            .client
            .post(&endpoint, &body)
            .await
            .map_err(aikido_error_to_mcp)?;
        let json = serde_json::to_string_pretty(&data).unwrap_or_default();
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(
        description = "Raw API PUT passthrough for allowed public endpoints. Requires confirm_mutation=true and MCP_ALLOW_RAW_API_MUTATIONS=true."
    )]
    pub async fn aikido_api_put(
        &self,
        Parameters(input): Parameters<AikidoApiPutInput>,
    ) -> Result<CallToolResult, ErrorData> {
        self.ensure_raw_api_mutation_allowed(input.confirm_mutation)?;
        let endpoint = validate_raw_api_endpoint(&input.endpoint)?;
        let body = input
            .body
            .unwrap_or_else(|| Value::Object(serde_json::Map::new()));
        let data: Value = self
            .state
            .client
            .put(&endpoint, &body)
            .await
            .map_err(aikido_error_to_mcp)?;
        let json = serde_json::to_string_pretty(&data).unwrap_or_default();
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(
        description = "Raw API DELETE passthrough for allowed public endpoints. Requires confirm_mutation=true and MCP_ALLOW_RAW_API_MUTATIONS=true."
    )]
    pub async fn aikido_api_delete(
        &self,
        Parameters(input): Parameters<AikidoApiDeleteInput>,
    ) -> Result<CallToolResult, ErrorData> {
        self.ensure_raw_api_mutation_allowed(input.confirm_mutation)?;
        let endpoint = validate_raw_api_endpoint(&input.endpoint)?;
        self.state
            .client
            .delete(&endpoint)
            .await
            .map_err(aikido_error_to_mcp)?;
        let json = serde_json::to_string_pretty(&serde_json::json!({ "success": true }))
            .unwrap_or_else(|_| "{\"success\":true}".to_string());
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for AikidoMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_protocol_version(ProtocolVersion::V_2024_11_05)
            .with_server_info(Implementation::from_build_env())
            .with_instructions(
                "Aikido Security MCP server. Provides tools for managing security vulnerabilities, \
                 code repositories, containers, clouds, domains, teams, and compliance reports.",
            )
    }
}
