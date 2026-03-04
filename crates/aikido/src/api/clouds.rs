use crate::client::AikidoClient;
use crate::error::Result;
use crate::models::*;
use serde_json::Value;

impl AikidoClient {
    pub async fn list_clouds(&self) -> Result<Vec<Cloud>> {
        self.get("/clouds").await
    }

    pub async fn connect_aws_cloud(&self, body: &ConnectAwsCloudRequest) -> Result<Value> {
        self.post("/clouds/aws", body).await
    }

    pub async fn connect_azure_cloud(&self, body: &ConnectAzureCloudRequest) -> Result<Value> {
        self.post("/clouds/azure", body).await
    }

    pub async fn update_azure_cloud_credentials(
        &self,
        cloud_id: i64,
        body: &UpdateAzureCloudCredentialsRequest,
    ) -> Result<Value> {
        self.put(&format!("/clouds/azure/{cloud_id}/credentials"), body)
            .await
    }

    pub async fn connect_gcp_cloud(&self, body: &ConnectGcpCloudRequest) -> Result<Value> {
        self.post("/clouds/gcp", body).await
    }

    pub async fn create_kubernetes_cloud(
        &self,
        body: &CreateKubernetesCloudRequest,
    ) -> Result<Value> {
        self.post("/clouds/kubernetes", body).await
    }

    pub async fn remove_cloud(&self, cloud_id: i64) -> Result<()> {
        self.delete(&format!("/clouds/{cloud_id}")).await
    }
}
