use crate::client::AikidoClient;
use crate::error::Result;
use crate::models::*;

impl AikidoClient {
    pub async fn get_changelog_summary(&self) -> Result<ChangelogSummary> {
        self.get("/changelog-summary").await
    }

    pub async fn get_latest_local_scan_info(&self) -> Result<LocalScanInfo> {
        self.get("/localscan/latest").await
    }
}
