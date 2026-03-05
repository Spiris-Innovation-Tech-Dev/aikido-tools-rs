use crate::client::AikidoClient;
use crate::error::Result;
use crate::models::*;
use serde_json::Value;

impl AikidoClient {
    pub async fn list_virtual_machines(&self) -> Result<Vec<VirtualMachine>> {
        self.get("/virtual-machines").await
    }

    pub async fn export_virtual_machine_sbom(&self, vm_id: i64, format: &str) -> Result<Value> {
        let format = crate::client::encode_path_segment(format, "format")?;
        self.get(&format!("/virtual-machines/{vm_id}/export/{format}"))
            .await
    }
}
