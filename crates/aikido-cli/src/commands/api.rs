use aikido::openapi::{
    append_query, find_operation, list_operations, render_path, OperationExecuteInfo,
};
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

fn parse_kv_list(values: &[String]) -> Result<Vec<(String, String)>> {
    values
        .iter()
        .map(|item| {
            let mut parts = item.splitn(2, '=');
            let key = parts
                .next()
                .ok_or_else(|| anyhow!("Invalid key=value pair: {item}"))?;
            let value = parts
                .next()
                .ok_or_else(|| anyhow!("Invalid key=value pair: {item}"))?;
            if key.is_empty() {
                return Err(anyhow!("Empty key in key=value pair: {item}"));
            }
            Ok((key.to_string(), value.to_string()))
        })
        .collect()
}

#[derive(Debug, Subcommand)]
pub enum ApiCommands {
    /// List all OpenAPI operations bundled with this CLI
    Ops(ApiOpsArgs),
    /// Execute an operation by OpenAPI operationId
    Exec(ApiExecArgs),
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
pub struct ApiOpsArgs;

impl Command for ApiOpsArgs {
    type Output = JsonOutput;

    async fn execute(&self, _client: &AikidoClient) -> Result<Self::Output> {
        let operations = list_operations()
            .into_iter()
            .map(|op| {
                serde_json::json!({
                    "operation_id": op.operation_id,
                    "method": op.method,
                    "path": op.path,
                })
            })
            .collect::<Vec<_>>();
        Ok(JsonOutput {
            data: serde_json::json!({
                "total": operations.len(),
                "operations": operations,
            }),
        })
    }
}

#[derive(Debug, Args)]
pub struct ApiExecArgs {
    /// OpenAPI operationId, e.g. listCodeRepos
    pub operation_id: String,
    /// Path parameter in key=value form; repeat for multiple params
    #[arg(long = "path-param")]
    pub path_params: Vec<String>,
    /// Query parameter in key=value form; repeat for multiple params
    #[arg(long = "query")]
    pub query_params: Vec<String>,
    /// JSON request body for POST/PUT
    #[arg(long)]
    pub body: Option<String>,
}

impl Command for ApiExecArgs {
    type Output = JsonOutput;

    async fn execute(&self, client: &AikidoClient) -> Result<Self::Output> {
        let Some(op) = find_operation(&self.operation_id) else {
            return Err(anyhow!("Unknown operationId: {}", self.operation_id));
        };

        let path_params = parse_kv_list(&self.path_params)?;
        let query_params = parse_kv_list(&self.query_params)?;

        let rendered = render_path(&op.path, &path_params);
        let endpoint = append_query(&rendered, &query_params);

        let result = match op.method.as_str() {
            "get" => client.get::<Value>(&endpoint).await?,
            "post" => {
                let body = parse_body(&self.body)?;
                client.post::<Value, _>(&endpoint, &body).await?
            }
            "put" => {
                let body = parse_body(&self.body)?;
                client.put::<Value, _>(&endpoint, &body).await?
            }
            "delete" => {
                client.delete(&endpoint).await?;
                serde_json::json!({"success": true})
            }
            other => return Err(anyhow!("Unsupported method in spec: {other}")),
        };

        let meta = OperationExecuteInfo {
            operation_id: op.operation_id,
            method: op.method,
            endpoint,
        };

        Ok(JsonOutput {
            data: serde_json::json!({
                "operation": meta,
                "result": result,
            }),
        })
    }
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
