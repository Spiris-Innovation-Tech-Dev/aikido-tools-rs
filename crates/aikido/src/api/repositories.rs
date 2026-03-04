use crate::client::AikidoClient;
use crate::error::Result;
use crate::models::*;
use serde_json::Value;

impl AikidoClient {
    pub async fn list_code_repos(&self) -> Result<Vec<CodeRepo>> {
        self.get("/repositories/code").await
    }

    pub async fn get_code_repo(&self, repo_id: i64) -> Result<CodeRepoDetail> {
        self.get(&format!("/repositories/code/{repo_id}")).await
    }

    pub async fn activate_code_repo(&self, body: &ActivateCodeRepoRequest) -> Result<Value> {
        self.post("/repositories/code/activate", body).await
    }

    pub async fn deactivate_code_repo(&self, body: &DeactivateCodeRepoRequest) -> Result<Value> {
        self.post("/repositories/code/deactivate", body).await
    }

    pub async fn clone_code_repo(&self, body: &CloneCodeRepoRequest) -> Result<Value> {
        self.post("/repositories/code/clone", body).await
    }

    pub async fn scan_code_repo(&self, repo_id: i64) -> Result<Value> {
        self.post(&format!("/repositories/code/{repo_id}/scan"), &ScanRequest {})
            .await
    }

    pub async fn update_code_repo_sensitivity(
        &self,
        repo_id: i64,
        body: &UpdateSensitivityRequest,
    ) -> Result<Value> {
        self.put(&format!("/repositories/code/{repo_id}/sensitivity"), body)
            .await
    }

    pub async fn update_code_repo_dev_dep_scan(
        &self,
        repo_id: i64,
        body: &UpdateDevDepScanRequest,
    ) -> Result<Value> {
        self.put(
            &format!("/repositories/code/{repo_id}/devdep-scan"),
            body,
        )
        .await
    }

    pub async fn export_code_repo_licenses(&self, repo_id: i64) -> Result<Value> {
        self.get(&format!("/repositories/code/{repo_id}/licenses/export"))
            .await
    }

    pub async fn export_code_repo_licenses_for_team(&self, team_id: i64) -> Result<Value> {
        self.get(&format!("/repositories/code/team/{team_id}/licenses/export"))
            .await
    }

    pub async fn import_repositories(&self) -> Result<Value> {
        self.post("/repositories/import", &serde_json::json!({}))
            .await
    }

    pub async fn add_private_registry(&self, body: &AddPrivateRegistryRequest) -> Result<Value> {
        self.post("/repositories/code/private-registries", body)
            .await
    }

    // SAST rules
    pub async fn list_sast_rules(&self) -> Result<Vec<SastRule>> {
        self.get("/repositories/code/sast/rules").await
    }

    // IaC rules
    pub async fn list_iac_rules(&self) -> Result<Vec<IacRule>> {
        self.get("/repositories/code/iac/rules").await
    }

    // Mobile rules
    pub async fn list_mobile_rules(&self) -> Result<Vec<MobileRule>> {
        self.get("/repositories/code/mobile/rules").await
    }

    // Custom rules
    pub async fn list_custom_rules(&self) -> Result<Vec<CustomRule>> {
        self.get("/repositories/sast/custom-rules").await
    }

    pub async fn get_custom_rule(&self, rule_id: i64) -> Result<CustomRule> {
        self.get(&format!("/repositories/sast/custom-rules/{rule_id}"))
            .await
    }

    pub async fn create_custom_rule(&self, body: &CreateCustomRuleRequest) -> Result<CustomRule> {
        self.post("/repositories/sast/custom-rules", body).await
    }

    pub async fn edit_custom_rule(
        &self,
        rule_id: i64,
        body: &EditCustomRuleRequest,
    ) -> Result<Value> {
        self.put(&format!("/repositories/sast/custom-rules/{rule_id}"), body)
            .await
    }

    pub async fn remove_custom_rule(&self, rule_id: i64) -> Result<()> {
        self.delete(&format!("/repositories/sast/custom-rules/{rule_id}"))
            .await
    }
}
