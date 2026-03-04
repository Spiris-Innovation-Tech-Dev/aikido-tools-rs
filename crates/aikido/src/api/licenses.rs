use crate::client::AikidoClient;
use crate::error::Result;
use crate::models::*;
use serde_json::Value;

impl AikidoClient {
    pub async fn overwrite_license(&self, body: &OverwriteLicenseRequest) -> Result<Value> {
        self.post("/licenses/overwrite", body).await
    }
}
