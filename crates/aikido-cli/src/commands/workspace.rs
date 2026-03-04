use aikido::AikidoClient;
use anyhow::Result;
use clap::Args;

use crate::commands::Command;
use crate::models::output::WorkspaceOutput;

#[derive(Debug, Args)]
pub struct WorkspaceInfoArgs;

impl Command for WorkspaceInfoArgs {
    type Output = WorkspaceOutput;

    async fn execute(&self, client: &AikidoClient) -> Result<Self::Output> {
        let info = client.get_workspace_info().await?;
        Ok(WorkspaceOutput {
            id: info.id,
            name: info.name,
            provider: info.linked_provider,
            org_name: info.linked_provider_org_name,
        })
    }
}
