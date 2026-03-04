use aikido::AikidoClient;
use anyhow::Result;
use clap::Args;

use crate::commands::Command;
use crate::models::output::JsonOutput;

#[derive(Debug, Args)]
pub struct FirewallAppsListArgs;

impl Command for FirewallAppsListArgs {
    type Output = JsonOutput;

    async fn execute(&self, client: &AikidoClient) -> Result<Self::Output> {
        let apps = client.list_firewall_apps().await?;
        Ok(JsonOutput {
            data: serde_json::to_value(apps)?,
        })
    }
}
