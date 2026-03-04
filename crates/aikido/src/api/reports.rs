use crate::client::AikidoClient;
use crate::error::Result;
use crate::models::*;
use serde_json::Value;

impl AikidoClient {
    pub async fn export_report_pdf(&self) -> Result<Vec<u8>> {
        self.get_bytes("/report/export/pdf").await
    }

    pub async fn list_activity_log(&self) -> Result<Value> {
        self.get("/report/activityLog").await
    }

    pub async fn list_ci_scans(&self) -> Result<Value> {
        self.get("/report/ciScans").await
    }

    pub async fn get_iso_compliance_overview(&self) -> Result<ComplianceOverview> {
        self.get("/report/iso/overview").await
    }

    pub async fn get_nis2_compliance_overview(&self) -> Result<ComplianceOverview> {
        self.get("/report/nis2/overview").await
    }

    pub async fn get_soc2_compliance_overview(&self) -> Result<ComplianceOverview> {
        self.get("/report/soc2/overview").await
    }
}
