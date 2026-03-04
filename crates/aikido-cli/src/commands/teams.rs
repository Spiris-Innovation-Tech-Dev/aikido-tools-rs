use aikido::AikidoClient;
use anyhow::Result;
use clap::Args;

use crate::commands::Command;
use crate::models::output::{TeamRow, TeamsOutput};

#[derive(Debug, Args)]
pub struct TeamsListArgs;

impl Command for TeamsListArgs {
    type Output = TeamsOutput;

    async fn execute(&self, client: &AikidoClient) -> Result<Self::Output> {
        let teams = client.list_teams().await?;
        let total = teams.len();
        let rows = teams
            .into_iter()
            .map(|t| TeamRow {
                id: t.id,
                name: t.name,
                responsibilities_count: t.responsibilities.len(),
            })
            .collect();
        Ok(TeamsOutput { teams: rows, total })
    }
}
