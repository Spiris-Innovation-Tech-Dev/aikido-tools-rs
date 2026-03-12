use aikido::config::{Config, ConfigOverrides};
use aikido::AikidoClient;
use anyhow::Result;
use clap::Args;

use crate::commands::Command;
use crate::models::output::{
    MessageOutput, WorkspaceAliasRow, WorkspaceAliasesOutput, WorkspaceCommandOutput,
    WorkspaceOutput,
};

#[derive(Debug, Args)]
pub struct WorkspaceArgs {
    /// Show configured workspace aliases from config
    #[arg(long, conflicts_with_all = ["use_workspace", "clear_active"])]
    pub list: bool,

    /// Set the default active workspace alias
    #[arg(long, value_name = "ALIAS", conflicts_with_all = ["list", "clear_active"])]
    pub use_workspace: Option<String>,

    /// Clear active workspace alias
    #[arg(long, conflicts_with_all = ["list", "use_workspace"])]
    pub clear_active: bool,
}

impl WorkspaceArgs {
    pub fn requires_client(&self) -> bool {
        !(self.list || self.use_workspace.is_some() || self.clear_active)
    }

    pub fn execute_local(
        &self,
        workspace_override: Option<&str>,
    ) -> Result<WorkspaceCommandOutput> {
        if let Some(alias) = &self.use_workspace {
            return self.set_active_workspace(alias);
        }
        if self.clear_active {
            return self.clear_active_workspace();
        }

        let config = Config::load(ConfigOverrides {
            workspace: workspace_override.map(ToOwned::to_owned),
            ..Default::default()
        })?;
        Ok(WorkspaceCommandOutput::List {
            configured: build_aliases_output(&config),
        })
    }

    fn set_active_workspace(&self, alias: &str) -> Result<WorkspaceCommandOutput> {
        let mut config = Config::load_global_file()?;
        config.set_active_workspace(Some(alias.to_string()))?;
        config.save_global()?;
        Ok(WorkspaceCommandOutput::Message {
            result: MessageOutput {
                message: format!("Active workspace set to '{alias}'"),
            },
        })
    }

    fn clear_active_workspace(&self) -> Result<WorkspaceCommandOutput> {
        let mut config = Config::load_global_file()?;
        config.set_active_workspace(None)?;
        config.save_global()?;
        Ok(WorkspaceCommandOutput::Message {
            result: MessageOutput {
                message: "Active workspace cleared".to_string(),
            },
        })
    }
}

impl Command for WorkspaceArgs {
    type Output = WorkspaceCommandOutput;

    async fn execute(&self, client: &AikidoClient) -> Result<Self::Output> {
        let info = client.get_workspace_info().await?;
        Ok(WorkspaceCommandOutput::Info {
            workspace: WorkspaceOutput {
                id: info.id,
                name: info.name,
                provider: info.linked_provider,
                org_name: info.linked_provider_org_name,
            },
        })
    }
}

fn build_aliases_output(config: &Config) -> WorkspaceAliasesOutput {
    let selected = config.selected_workspace_name().map(ToOwned::to_owned);
    let active = config.active_workspace_name().map(ToOwned::to_owned);

    let workspaces = config
        .workspaces
        .iter()
        .map(|(alias, ws)| WorkspaceAliasRow {
            alias: alias.clone(),
            region: ws.region.map(|r| r.to_string()),
            has_client_id: ws.client_id.as_ref().is_some_and(|v| !v.is_empty()),
            has_client_secret: ws.client_secret.as_ref().is_some_and(|v| !v.is_empty()),
            is_selected: selected.as_deref() == Some(alias.as_str()),
            is_active: active.as_deref() == Some(alias.as_str()),
        })
        .collect();

    WorkspaceAliasesOutput {
        active_workspace: active,
        selected_workspace: selected,
        workspaces,
    }
}
