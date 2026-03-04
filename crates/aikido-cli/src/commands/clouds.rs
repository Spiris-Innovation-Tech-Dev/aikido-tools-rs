use aikido::AikidoClient;
use anyhow::Result;
use clap::Args;

use crate::commands::Command;
use crate::models::output::{CloudRow, CloudsOutput};

#[derive(Debug, Args)]
pub struct CloudsListArgs;

impl Command for CloudsListArgs {
    type Output = CloudsOutput;

    async fn execute(&self, client: &AikidoClient) -> Result<Self::Output> {
        let clouds = client.list_clouds().await?;
        let total = clouds.len();
        let rows = clouds
            .into_iter()
            .map(|c| CloudRow {
                id: c.id,
                name: c.name,
                provider: format!("{:?}", c.provider),
                environment: format!("{:?}", c.environment),
            })
            .collect();
        Ok(CloudsOutput {
            clouds: rows,
            total,
        })
    }
}
