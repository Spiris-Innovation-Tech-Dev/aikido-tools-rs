use aikido::AikidoClient;
use anyhow::Result;
use clap::Args;

use crate::commands::Command;
use aikido::models::IgnoreRequest;

use crate::models::output::{
    IssueGroupDetailOutput, IssueGroupRow, IssueGroupsOutput, IssueRow, JsonOutput, LocationRow,
    MessageOutput,
};

#[derive(Debug, Args)]
pub struct IssueGroupsListArgs {
    /// Show all findings (including ignored/snoozed/closed), not just open
    #[arg(long)]
    pub all: bool,
}

impl Command for IssueGroupsListArgs {
    type Output = IssueGroupsOutput;

    async fn execute(&self, client: &AikidoClient) -> Result<Self::Output> {
        if self.all {
            self.execute_all(client).await
        } else {
            self.execute_open(client).await
        }
    }
}

impl IssueGroupsListArgs {
    async fn execute_open(&self, client: &AikidoClient) -> Result<IssueGroupsOutput> {
        let groups = client.list_open_issue_groups().await?;
        let total = groups.len();
        let rows = groups
            .into_iter()
            .map(|g| {
                let locations = g
                    .locations
                    .into_iter()
                    .map(|l| LocationRow {
                        id: l.id,
                        name: l.name,
                        location_type: l.location_type,
                    })
                    .collect();
                IssueGroupRow {
                    id: g.id,
                    title: g.title,
                    severity: g.severity.to_string(),
                    severity_score: g.severity_score,
                    issue_type: format!("{:?}", g.issue_type),
                    status: format!("{:?}", g.group_status),
                    locations,
                }
            })
            .collect();
        Ok(IssueGroupsOutput {
            groups: rows,
            total,
        })
    }

    async fn execute_all(&self, client: &AikidoClient) -> Result<IssueGroupsOutput> {
        use std::collections::HashMap;

        let issues: Vec<aikido::models::Issue> = client.get("/issues/export").await?;
        let mut groups_map: HashMap<i64, IssueGroupRow> = HashMap::new();

        for issue in &issues {
            groups_map
                .entry(issue.group_id)
                .and_modify(|row| {
                    // Keep the highest severity score
                    if issue.severity_score > row.severity_score {
                        row.severity_score = issue.severity_score;
                        row.severity = issue.severity.to_string();
                    }
                })
                .or_insert_with(|| {
                    let location = issue
                        .code_repo_name
                        .as_deref()
                        .or(issue.container_repo_name.as_deref())
                        .or(issue.cloud_name.as_deref())
                        .or(issue.domain_name.as_deref());
                    let locations = location
                        .map(|name| {
                            vec![LocationRow {
                                id: 0,
                                name: name.to_string(),
                                location_type: String::new(),
                            }]
                        })
                        .unwrap_or_default();
                    IssueGroupRow {
                        id: issue.group_id,
                        title: issue
                            .affected_package
                            .clone()
                            .or(issue.affected_file.clone())
                            .or(issue.rule.clone())
                            .unwrap_or_else(|| format!("Issue group {}", issue.group_id)),
                        severity: issue.severity.to_string(),
                        severity_score: issue.severity_score,
                        issue_type: format!("{:?}", issue.issue_type),
                        status: format!("{:?}", issue.status),
                        locations,
                    }
                });
        }

        let mut rows: Vec<IssueGroupRow> = groups_map.into_values().collect();
        rows.sort_by(|a, b| b.severity_score.cmp(&a.severity_score));
        let total = rows.len();
        Ok(IssueGroupsOutput {
            groups: rows,
            total,
        })
    }
}

#[derive(Debug, Args)]
pub struct IssueGroupGetArgs {
    pub group_id: i64,
}

impl Command for IssueGroupGetArgs {
    type Output = IssueGroupDetailOutput;

    async fn execute(&self, client: &AikidoClient) -> Result<Self::Output> {
        let (g, issues) = tokio::try_join!(
            client.get_issue_group(self.group_id),
            client.get_issue_group_issues(self.group_id),
        )?;
        let issue_rows = issues
            .into_iter()
            .map(|i| IssueRow {
                id: i.id,
                severity: i.severity.to_string(),
                severity_score: i.severity_score,
                status: format!("{:?}", i.status),
                affected_package: i.affected_package,
                affected_file: i.affected_file,
                start_line: i.start_line,
                end_line: i.end_line,
                cve_id: i.cve_id,
                code_repo_name: i.code_repo_name,
                container_repo_name: i.container_repo_name,
                cloud_name: i.cloud_name,
                domain_name: i.domain_name,
                programming_language: i.programming_language,
                installed_version: i.installed_version,
                patched_versions: i.patched_versions,
                cwe_classes: i.cwe_classes,
            })
            .collect();
        Ok(IssueGroupDetailOutput {
            id: g.id,
            title: g.title,
            description: g.description,
            severity: g.severity.to_string(),
            severity_score: g.severity_score,
            issue_type: format!("{:?}", g.issue_type),
            status: format!("{:?}", g.group_status),
            time_to_fix_minutes: g.time_to_fix_minutes,
            locations: g
                .locations
                .into_iter()
                .map(|l| LocationRow {
                    id: l.id,
                    name: l.name,
                    location_type: l.location_type,
                })
                .collect(),
            how_to_fix: g.how_to_fix,
            related_cve_ids: g.related_cve_ids,
            issues: issue_rows,
        })
    }
}

#[derive(Debug, Args)]
pub struct IssueIgnoreArgs {
    pub group_id: i64,
    /// Reason for ignoring
    #[arg(long)]
    pub reason: Option<String>,
}

impl Command for IssueIgnoreArgs {
    type Output = MessageOutput;

    async fn execute(&self, client: &AikidoClient) -> Result<Self::Output> {
        let body = IgnoreRequest {
            reason: self.reason.clone(),
        };
        client.ignore_issue_group(self.group_id, &body).await?;
        Ok(MessageOutput {
            message: format!("Issue group {} ignored", self.group_id),
        })
    }
}

#[derive(Debug, Args)]
pub struct IssueUnignoreArgs {
    pub group_id: i64,
}

impl Command for IssueUnignoreArgs {
    type Output = MessageOutput;

    async fn execute(&self, client: &AikidoClient) -> Result<Self::Output> {
        client.unignore_issue_group(self.group_id).await?;
        Ok(MessageOutput {
            message: format!("Issue group {} unignored", self.group_id),
        })
    }
}

#[derive(Debug, Args)]
pub struct IssueCountsArgs;

impl Command for IssueCountsArgs {
    type Output = JsonOutput;

    async fn execute(&self, client: &AikidoClient) -> Result<Self::Output> {
        let counts = client.get_issue_counts().await?;
        Ok(JsonOutput {
            data: serde_json::to_value(counts)?,
        })
    }
}

#[derive(Debug, Args)]
pub struct IssueExportArgs;

impl Command for IssueExportArgs {
    type Output = JsonOutput;

    async fn execute(&self, client: &AikidoClient) -> Result<Self::Output> {
        let data = client.export_issues().await?;
        Ok(JsonOutput { data })
    }
}
