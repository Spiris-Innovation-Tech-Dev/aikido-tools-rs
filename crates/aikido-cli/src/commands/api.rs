use aikido::AikidoClient;
use anyhow::{anyhow, Result};
use clap::{Args, Subcommand};
use serde_json::Value;

use crate::commands::Command;
use crate::models::output::JsonOutput;

fn normalize_endpoint(endpoint: &str) -> String {
    if endpoint.starts_with('/') {
        endpoint.to_string()
    } else {
        format!("/{endpoint}")
    }
}

fn parse_body(body: &Option<String>) -> Result<Value> {
    match body {
        Some(raw) => serde_json::from_str(raw).map_err(|e| anyhow!("Invalid JSON body: {e}")),
        None => Ok(Value::Object(serde_json::Map::new())),
    }
}

#[derive(Debug, Subcommand)]
pub enum ApiCommands {
    /// Perform a raw GET request on any public API endpoint
    Get(ApiGetArgs),
    /// Perform a raw POST request on any public API endpoint
    Post(ApiPostArgs),
    /// Perform a raw PUT request on any public API endpoint
    Put(ApiPutArgs),
    /// Perform a raw DELETE request on any public API endpoint
    Delete(ApiDeleteArgs),
}

#[derive(Debug, Args)]
pub struct ApiGetArgs {
    /// Endpoint path, e.g. /repositories/code?page=0&per_page=20
    pub endpoint: String,
}

impl Command for ApiGetArgs {
    type Output = JsonOutput;

    async fn execute(&self, client: &AikidoClient) -> Result<Self::Output> {
        let endpoint = normalize_endpoint(&self.endpoint);
        let data: Value = client.get(&endpoint).await?;
        Ok(JsonOutput { data })
    }
}

#[derive(Debug, Args)]
pub struct ApiPostArgs {
    /// Endpoint path, e.g. /domains
    pub endpoint: String,
    /// JSON request body
    #[arg(long)]
    pub body: Option<String>,
}

impl Command for ApiPostArgs {
    type Output = JsonOutput;

    async fn execute(&self, client: &AikidoClient) -> Result<Self::Output> {
        let endpoint = normalize_endpoint(&self.endpoint);
        let body = parse_body(&self.body)?;
        let data: Value = client.post(&endpoint, &body).await?;
        Ok(JsonOutput { data })
    }
}

#[derive(Debug, Args)]
pub struct ApiPutArgs {
    /// Endpoint path, e.g. /issues/123/ignore
    pub endpoint: String,
    /// JSON request body
    #[arg(long)]
    pub body: Option<String>,
}

impl Command for ApiPutArgs {
    type Output = JsonOutput;

    async fn execute(&self, client: &AikidoClient) -> Result<Self::Output> {
        let endpoint = normalize_endpoint(&self.endpoint);
        let body = parse_body(&self.body)?;
        let data: Value = client.put(&endpoint, &body).await?;
        Ok(JsonOutput { data })
    }
}

#[derive(Debug, Args)]
pub struct ApiDeleteArgs {
    /// Endpoint path, e.g. /domains/42
    pub endpoint: String,
}

impl Command for ApiDeleteArgs {
    type Output = JsonOutput;

    async fn execute(&self, client: &AikidoClient) -> Result<Self::Output> {
        let endpoint = normalize_endpoint(&self.endpoint);
        client.delete(&endpoint).await?;
        Ok(JsonOutput {
            data: serde_json::json!({ "success": true }),
        })
    }
}
