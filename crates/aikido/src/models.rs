use serde::{Deserialize, Serialize};
use serde_json::Value;

// ========== Shared Enums ==========

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Critical => write!(f, "critical"),
            Severity::High => write!(f, "high"),
            Severity::Medium => write!(f, "medium"),
            Severity::Low => write!(f, "low"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssueType {
    OpenSource,
    LeakedSecret,
    Cloud,
    Iac,
    Sast,
    SurfaceMonitoring,
    Malware,
    Eol,
    ScmSecurity,
    AiPentest,
    License,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssueStatus {
    Open,
    Ignored,
    Snoozed,
    Closed,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GroupStatus {
    New,
    Todo,
    TaskOpen,
    TaskClosed,
    PullRequestOpen,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttackSurface {
    Frontend,
    Backend,
    DockerContainer,
    Cloud,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CloudProvider {
    Aws,
    Gcp,
    Azure,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CloudEnvironment {
    Production,
    Staging,
    Development,
    Mixed,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitProvider {
    Github,
    Gitlab,
    #[serde(rename = "gitlab-server")]
    GitlabServer,
    Bitbucket,
    AzureDevops,
    Selfscan,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceType {
    CodeRepository,
    ContainerRepository,
    Cloud,
    Domain,
    #[serde(other)]
    Unknown,
}

// ========== Workspace ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceInfo {
    pub id: i64,
    pub name: String,
    pub linked_provider: Option<String>,
    pub linked_provider_org_name: Option<String>,
    pub git_base_url: Option<String>,
}

// ========== Issues ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub id: i64,
    pub group_id: i64,
    #[serde(rename = "type")]
    pub issue_type: IssueType,
    pub rule: Option<String>,
    pub attack_surface: AttackSurface,
    pub severity_score: i32,
    pub severity: Severity,
    pub status: IssueStatus,
    pub affected_package: Option<String>,
    pub affected_file: Option<String>,
    pub cve_id: Option<String>,
    pub first_detected_at: Option<i64>,
    pub last_detected_at: Option<i64>,
    pub auto_closed_at: Option<i64>,
    pub auto_closed_reason: Option<String>,
    pub code_repo_id: Option<i64>,
    pub code_repo_name: Option<String>,
    pub cloud_id: Option<i64>,
    pub cloud_name: Option<String>,
    pub container_repo_id: Option<i64>,
    pub container_repo_name: Option<String>,
    pub domain_id: Option<i64>,
    pub domain_name: Option<String>,
    pub how_to_fix: Option<String>,
    pub programming_language: Option<String>,
    pub reachability_status: Option<String>,
    pub start_line: Option<i64>,
    pub end_line: Option<i64>,
    #[serde(default)]
    pub cwe_classes: Vec<String>,
    pub installed_version: Option<String>,
    #[serde(default)]
    pub patched_versions: Vec<String>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueReachability {
    pub reachable: Option<bool>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueCounts {
    #[serde(flatten)]
    pub counts: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueExport {
    #[serde(flatten)]
    pub data: Value,
}

// ========== Issue Groups ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueGroup {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub issue_type: IssueType,
    pub group_status: GroupStatus,
    pub severity_score: i32,
    pub severity: Severity,
    pub time_to_fix_minutes: Option<i32>,
    #[serde(default)]
    pub locations: Vec<IssueLocation>,
    pub how_to_fix: Option<String>,
    #[serde(default)]
    pub related_cve_ids: Vec<String>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueLocation {
    pub id: i64,
    pub name: String,
    #[serde(rename = "type")]
    pub location_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueGroupTask {
    #[serde(flatten)]
    pub data: Value,
}

// ========== Code Repositories ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeRepo {
    pub id: i64,
    pub name: String,
    pub provider: GitProvider,
    pub external_repo_id: Option<String>,
    pub url: Option<String>,
    pub branch: Option<String>,
    pub last_scanned_at: Option<i64>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeRepoDetail {
    pub id: i64,
    pub name: String,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SastRule {
    pub id: Option<Value>,
    pub name: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IacRule {
    pub id: Option<Value>,
    pub name: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileRule {
    #[serde(flatten)]
    pub data: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomRule {
    pub id: Option<i64>,
    pub name: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

// ========== Containers ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerRepo {
    pub id: i64,
    pub name: String,
    pub provider: Option<String>,
    pub cloud_id: Option<i64>,
    pub registry_id: Option<i64>,
    pub registry_name: Option<String>,
    pub tag: Option<String>,
    pub distro: Option<String>,
    pub distro_version: Option<String>,
    pub last_scanned_at: Option<i64>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerRegistry {
    pub id: Option<i64>,
    pub name: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

// ========== Clouds ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cloud {
    pub id: i64,
    pub name: String,
    pub provider: CloudProvider,
    pub environment: CloudEnvironment,
    pub external_id: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

// ========== Domains ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Domain {
    pub id: i64,
    pub name: Option<String>,
    pub url: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

// ========== Teams ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    pub id: i64,
    pub name: String,
    pub external_source: Option<String>,
    pub external_source_id: Option<String>,
    #[serde(default)]
    pub responsibilities: Vec<TeamResponsibility>,
    pub active: Option<bool>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamResponsibility {
    pub id: i64,
    #[serde(rename = "type")]
    pub resource_type: ResourceType,
    pub included_paths: Option<Vec<String>>,
    pub excluded_paths: Option<Vec<String>>,
}

// ========== Users ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub full_name: Option<String>,
    pub email: Option<String>,
    pub active: Option<i32>,
    pub last_login_timestamp: Option<i64>,
    pub role: Option<String>,
    pub auth_type: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

// ========== Firewall / Zen ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirewallApp {
    pub id: Option<i64>,
    pub name: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirewallEvent {
    #[serde(flatten)]
    pub data: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotLists {
    #[serde(flatten)]
    pub data: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Countries {
    #[serde(flatten)]
    pub data: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpLists {
    #[serde(flatten)]
    pub data: Value,
}

// ========== Reports ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityLogEntry {
    #[serde(flatten)]
    pub data: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CiScan {
    #[serde(flatten)]
    pub data: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceOverview {
    #[serde(flatten)]
    pub data: Value,
}

// ========== Task Tracking ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskTrackingProject {
    #[serde(flatten)]
    pub data: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskTrackingTask {
    #[serde(flatten)]
    pub data: Value,
}

// ========== Pentests ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PentestAssessment {
    #[serde(flatten)]
    pub data: Value,
}

// ========== Virtual Machines ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualMachine {
    #[serde(flatten)]
    pub data: Value,
}

// ========== Changelog ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangelogSummary {
    #[serde(flatten)]
    pub data: Value,
}

// ========== Local Scanner ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalScanInfo {
    #[serde(flatten)]
    pub data: Value,
}

// ========== Generic Response ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse {
    #[serde(flatten)]
    pub data: Value,
}

// ========== Request Bodies ==========

#[derive(Debug, Clone, Serialize)]
pub struct SnoozeRequest {
    pub snooze_until: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct NoteRequest {
    pub comment: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AdjustSeverityRequest {
    pub adjusted_severity: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct IgnoreRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateDomainRequest {
    pub domain: String,
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zen_service_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openapi_spec_base64: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openapi_spec_url: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateTeamRequest {
    pub name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateTeamRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AddUserToTeamRequest {
    pub user_id: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct LinkResourceToTeamRequest {
    pub resource_id: i64,
    pub resource_type: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ConnectAwsCloudRequest {
    pub name: String,
    pub environment: String,
    pub aws_account_id: String,
    pub aws_role_arn: String,
    pub aws_external_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ConnectAzureCloudRequest {
    pub name: String,
    pub environment: String,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct ConnectGcpCloudRequest {
    pub name: String,
    pub environment: String,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateKubernetesCloudRequest {
    pub name: String,
    pub environment: String,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateBlockingRequest {
    pub block: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_minimum_wait_check: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateAppRequest {
    pub name: String,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateAppRequest {
    #[serde(flatten)]
    pub data: Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateCustomRuleRequest {
    pub name: String,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct EditCustomRuleRequest {
    #[serde(flatten)]
    pub data: Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct OverwriteLicenseRequest {
    pub package_name: String,
    pub license: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreatePentestDraftRequest {
    #[serde(flatten)]
    pub data: Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateUserRightsRequest {
    pub role: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ActivateCodeRepoRequest {
    pub repo_id: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct DeactivateCodeRepoRequest {
    pub repo_id: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct CloneCodeRepoRequest {
    pub repo_id: i64,
    pub branch: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ActivateContainerRequest {
    pub container_repo_id: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct DeactivateContainerRequest {
    pub container_repo_id: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct CloneContainerRequest {
    pub container_repo_id: i64,
    pub tag: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct LinkCodeRepoToContainerRequest {
    pub container_repo_id: i64,
    pub code_repo_id: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateSensitivityRequest {
    pub sensitivity: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateInternetConnectionRequest {
    pub has_internet_connection: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct ScanRequest {}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateTagFilterRequest {
    pub container_repo_id: i64,
    pub tag_filter: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct LinkTaskToIssueGroupRequest {
    pub issue_group_id: i64,
    pub task_id: String,
    pub provider: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct MapReposToProjectRequest {
    #[serde(flatten)]
    pub data: Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateDomainHeadersRequest {
    #[serde(flatten)]
    pub data: Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateDomainOpenApiSpecRequest {
    #[serde(flatten)]
    pub data: Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct AddPublicContainerRequest {
    pub name: String,
    pub tag: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UploadContainerSbomRequest {
    #[serde(flatten)]
    pub data: Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct GenerateContainerSbomRequest {
    pub container_repo_ids: Vec<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AddPrivateRegistryRequest {
    #[serde(flatten)]
    pub data: Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateDevDepScanRequest {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct StartDomainScanRequest {
    pub domain_id: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateAzureCloudCredentialsRequest {
    #[serde(flatten)]
    pub data: Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateUserRequest {
    #[serde(flatten)]
    pub data: Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct AddBugBountyReportRequest {
    #[serde(flatten)]
    pub data: Value,
}

#[cfg(test)]
mod request_body_tests {
    use super::*;

    #[test]
    fn adjust_severity_serializes_spec_keys() {
        let body = AdjustSeverityRequest {
            adjusted_severity: "medium".to_string(),
            reason: "Validated false positive impact".to_string(),
        };
        let json = serde_json::to_value(body).unwrap();

        assert!(json.get("adjusted_severity").is_some());
        assert!(json.get("reason").is_some());
        assert!(json.get("severity").is_none());
    }

    #[test]
    fn update_blocking_serializes_spec_keys() {
        let body = UpdateBlockingRequest {
            block: true,
            disable_minimum_wait_check: Some(false),
        };
        let json = serde_json::to_value(body).unwrap();

        assert!(json.get("block").is_some());
        assert!(json.get("disable_minimum_wait_check").is_some());
        assert!(json.get("blocking_enabled").is_none());
    }

    #[test]
    fn create_domain_serializes_spec_keys() {
        let body = CreateDomainRequest {
            domain: "https://www.example.com".to_string(),
            kind: "front_end".to_string(),
            zen_service_id: None,
            openapi_spec_base64: None,
            openapi_spec_url: None,
        };
        let json = serde_json::to_value(body).unwrap();

        assert!(json.get("domain").is_some());
        assert!(json.get("kind").is_some());
        assert!(json.get("name").is_none());
    }
}
