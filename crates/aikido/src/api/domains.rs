use crate::client::AikidoClient;
use crate::error::Result;
use crate::models::*;
use serde_json::Value;

impl AikidoClient {
    pub async fn list_domains(&self) -> Result<Vec<Domain>> {
        self.get("/domains").await
    }

    pub async fn create_domain(&self, body: &CreateDomainRequest) -> Result<Value> {
        self.post("/domains", body).await
    }

    pub async fn remove_domain(&self, domain_id: i64) -> Result<()> {
        self.delete(&format!("/domains/{domain_id}")).await
    }

    pub async fn start_domain_scan(&self, body: &StartDomainScanRequest) -> Result<Value> {
        self.post("/domains/scan", body).await
    }

    pub async fn update_domain_auth_headers(
        &self,
        domain_id: i64,
        body: &UpdateDomainHeadersRequest,
    ) -> Result<Value> {
        self.post(&format!("/domains/{domain_id}/headers"), body)
            .await
    }

    pub async fn update_domain_openapi_spec(
        &self,
        domain_id: i64,
        body: &UpdateDomainOpenApiSpecRequest,
    ) -> Result<Value> {
        self.put(
            &format!("/domains/{domain_id}/update/openapi-spec"),
            body,
        )
        .await
    }
}
