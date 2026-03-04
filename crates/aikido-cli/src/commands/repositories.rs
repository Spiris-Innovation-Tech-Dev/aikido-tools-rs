use aikido::AikidoClient;
use anyhow::Result;
use clap::Args;

use crate::commands::Command;
use crate::models::output::{CodeRepoRow, CodeReposOutput, JsonOutput};

#[derive(Debug, Args)]
pub struct ReposListArgs;

impl Command for ReposListArgs {
    type Output = CodeReposOutput;

    async fn execute(&self, client: &AikidoClient) -> Result<Self::Output> {
        let repos = client.list_code_repos().await?;
        let total = repos.len();
        let rows = repos
            .into_iter()
            .map(|r| CodeRepoRow {
                id: r.id,
                name: r.name,
                provider: format!("{:?}", r.provider),
                branch: r.branch,
            })
            .collect();
        Ok(CodeReposOutput { repos: rows, total })
    }
}

#[derive(Debug, Args)]
pub struct RepoGetArgs {
    pub repo_id: i64,
}

impl Command for RepoGetArgs {
    type Output = JsonOutput;

    async fn execute(&self, client: &AikidoClient) -> Result<Self::Output> {
        let repo = client.get_code_repo(self.repo_id).await?;
        Ok(JsonOutput {
            data: serde_json::to_value(repo)?,
        })
    }
}
