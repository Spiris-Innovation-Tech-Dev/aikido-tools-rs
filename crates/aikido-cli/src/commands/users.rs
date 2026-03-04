use aikido::AikidoClient;
use anyhow::Result;
use clap::Args;

use crate::commands::Command;
use crate::models::output::{UserRow, UsersOutput};

#[derive(Debug, Args)]
pub struct UsersListArgs;

impl Command for UsersListArgs {
    type Output = UsersOutput;

    async fn execute(&self, client: &AikidoClient) -> Result<Self::Output> {
        let users = client.list_users().await?;
        let total = users.len();
        let rows = users
            .into_iter()
            .map(|u| UserRow {
                id: u.id,
                name: u.full_name.unwrap_or_default(),
                email: u.email,
                role: u.role,
                active: u.active.map(|a| a == 1).unwrap_or(false),
            })
            .collect();
        Ok(UsersOutput { users: rows, total })
    }
}
