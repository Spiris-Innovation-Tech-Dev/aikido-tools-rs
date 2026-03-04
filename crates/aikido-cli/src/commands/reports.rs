use aikido::AikidoClient;
use anyhow::Result;
use clap::Args;

use crate::commands::Command;
use crate::models::output::JsonOutput;

#[derive(Debug, Args)]
pub struct ComplianceIsoArgs;

impl Command for ComplianceIsoArgs {
    type Output = JsonOutput;

    async fn execute(&self, client: &AikidoClient) -> Result<Self::Output> {
        let overview = client.get_iso_compliance_overview().await?;
        Ok(JsonOutput {
            data: serde_json::to_value(overview)?,
        })
    }
}

#[derive(Debug, Args)]
pub struct ComplianceSoc2Args;

impl Command for ComplianceSoc2Args {
    type Output = JsonOutput;

    async fn execute(&self, client: &AikidoClient) -> Result<Self::Output> {
        let overview = client.get_soc2_compliance_overview().await?;
        Ok(JsonOutput {
            data: serde_json::to_value(overview)?,
        })
    }
}

#[derive(Debug, Args)]
pub struct ComplianceNis2Args;

impl Command for ComplianceNis2Args {
    type Output = JsonOutput;

    async fn execute(&self, client: &AikidoClient) -> Result<Self::Output> {
        let overview = client.get_nis2_compliance_overview().await?;
        Ok(JsonOutput {
            data: serde_json::to_value(overview)?,
        })
    }
}

#[derive(Debug, Args)]
pub struct ActivityLogArgs;

impl Command for ActivityLogArgs {
    type Output = JsonOutput;

    async fn execute(&self, client: &AikidoClient) -> Result<Self::Output> {
        let log = client.list_activity_log().await?;
        Ok(JsonOutput { data: log })
    }
}

#[derive(Debug, Args)]
pub struct CiScansArgs;

impl Command for CiScansArgs {
    type Output = JsonOutput;

    async fn execute(&self, client: &AikidoClient) -> Result<Self::Output> {
        let scans = client.list_ci_scans().await?;
        Ok(JsonOutput { data: scans })
    }
}
