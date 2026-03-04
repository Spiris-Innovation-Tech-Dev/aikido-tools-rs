mod config;
mod error;
mod server;
mod state;
mod tools;

use config::McpConfig;
use rmcp::{transport::stdio, ServiceExt};
use server::AikidoMcpServer;
use state::ServerState;
use tracing_subscriber::{self, EnvFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("Starting Aikido MCP Server");

    let config = McpConfig::from_env()?;
    tracing::info!("Configuration loaded (region: {})", config.region);

    let state = ServerState::new(config);
    tracing::info!("Server state initialized");

    match state.test_connection().await {
        Ok(_) => tracing::info!("Successfully connected to Aikido"),
        Err(e) => tracing::warn!(
            "Could not verify connection: {}. Server will start anyway.",
            e
        ),
    }

    let server = AikidoMcpServer::new(state);
    tracing::info!("MCP server created");

    let service = server.serve(stdio()).await.inspect_err(|e| {
        tracing::error!("serving error: {:?}", e);
    })?;

    tracing::info!("MCP server running, waiting for requests");
    service.waiting().await?;

    Ok(())
}
