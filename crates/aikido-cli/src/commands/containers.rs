use aikido::AikidoClient;
use anyhow::Result;
use clap::Args;

use crate::commands::Command;
use crate::models::output::{ContainerRow, ContainersOutput};

#[derive(Debug, Args)]
pub struct ContainersListArgs;

impl Command for ContainersListArgs {
    type Output = ContainersOutput;

    async fn execute(&self, client: &AikidoClient) -> Result<Self::Output> {
        let containers = client.list_containers().await?;
        let total = containers.len();
        let rows = containers
            .into_iter()
            .map(|c| ContainerRow {
                id: c.id,
                name: c.name,
                provider: c.provider.unwrap_or_default(),
                tag: c.tag,
            })
            .collect();
        Ok(ContainersOutput {
            containers: rows,
            total,
        })
    }
}
