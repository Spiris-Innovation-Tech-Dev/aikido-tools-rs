use aikido::AikidoClient;
use anyhow::Result;
use clap::Args;

use crate::commands::Command;
use crate::models::output::{DomainRow, DomainsOutput};

#[derive(Debug, Args)]
pub struct DomainsListArgs;

impl Command for DomainsListArgs {
    type Output = DomainsOutput;

    async fn execute(&self, client: &AikidoClient) -> Result<Self::Output> {
        let domains = client.list_domains().await?;
        let total = domains.len();
        let rows = domains
            .into_iter()
            .map(|d| DomainRow {
                id: d.id,
                name: d.name.or(d.url).unwrap_or_default(),
            })
            .collect();
        Ok(DomainsOutput {
            domains: rows,
            total,
        })
    }
}
