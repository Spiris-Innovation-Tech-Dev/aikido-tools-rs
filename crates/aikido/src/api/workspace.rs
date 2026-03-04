use crate::client::AikidoClient;
use crate::error::Result;
use crate::models::WorkspaceInfo;

impl AikidoClient {
    pub async fn get_workspace_info(&self) -> Result<WorkspaceInfo> {
        self.get("/workspace").await
    }
}
