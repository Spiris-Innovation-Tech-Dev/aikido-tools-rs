use crate::client::AikidoClient;
use crate::error::Result;
use crate::models::*;
use serde_json::Value;

impl AikidoClient {
    pub async fn list_containers(&self) -> Result<Vec<ContainerRepo>> {
        self.get("/containers").await
    }

    pub async fn get_container(&self, container_id: i64) -> Result<ContainerRepo> {
        self.get(&format!("/containers/{container_id}")).await
    }

    pub async fn delete_container(&self, container_id: i64) -> Result<()> {
        self.delete(&format!("/containers/{container_id}")).await
    }

    pub async fn activate_container(&self, body: &ActivateContainerRequest) -> Result<Value> {
        self.post("/containers/activate", body).await
    }

    pub async fn deactivate_container(&self, body: &DeactivateContainerRequest) -> Result<Value> {
        self.post("/containers/deactivate", body).await
    }

    pub async fn clone_container(&self, body: &CloneContainerRequest) -> Result<Value> {
        self.post("/containers/clone", body).await
    }

    pub async fn add_public_container(&self, body: &AddPublicContainerRequest) -> Result<Value> {
        self.post("/containers/public", body).await
    }

    pub async fn link_code_repo_to_container(
        &self,
        body: &LinkCodeRepoToContainerRequest,
    ) -> Result<Value> {
        self.post("/containers/linkCodeRepo", body).await
    }

    pub async fn scan_container(&self, container_id: i64) -> Result<Value> {
        self.post(
            &format!("/containers/{container_id}/scan"),
            &ScanRequest {},
        )
        .await
    }

    pub async fn update_container_sensitivity(
        &self,
        container_id: i64,
        body: &UpdateSensitivityRequest,
    ) -> Result<Value> {
        self.put(
            &format!("/containers/{container_id}/sensitivity"),
            body,
        )
        .await
    }

    pub async fn update_container_internet_connection(
        &self,
        container_id: i64,
        body: &UpdateInternetConnectionRequest,
    ) -> Result<Value> {
        self.put(
            &format!("/containers/{container_id}/internetConnection"),
            body,
        )
        .await
    }

    pub async fn export_container_licenses(&self, container_id: i64) -> Result<Value> {
        self.get(&format!(
            "/containers/{container_id}/licenses/export"
        ))
        .await
    }

    pub async fn update_tag_filter(&self, body: &UpdateTagFilterRequest) -> Result<Value> {
        self.post("/containers/updateTagFilter", body).await
    }

    pub async fn upload_container_sbom(&self, body: &UploadContainerSbomRequest) -> Result<Value> {
        self.post("/containers/sbom", body).await
    }

    pub async fn generate_container_sbom(
        &self,
        body: &GenerateContainerSbomRequest,
    ) -> Result<Value> {
        self.post("/containers/sbom/generate", body).await
    }

    pub async fn get_container_registry(&self, registry_id: i64) -> Result<ContainerRegistry> {
        self.get(&format!("/containers/registries/{registry_id}"))
            .await
    }
}
