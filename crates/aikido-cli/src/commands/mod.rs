use crate::output::Formattable;
use aikido::AikidoClient;
use anyhow::Result;

pub mod issues;
pub mod api;
pub mod repositories;
pub mod containers;
pub mod clouds;
pub mod domains;
pub mod teams;
pub mod users;
pub mod firewall;
pub mod reports;
pub mod workspace;

pub trait Command {
    type Output: Formattable;
    fn execute(
        &self,
        client: &AikidoClient,
    ) -> impl std::future::Future<Output = Result<Self::Output>> + Send;
}
