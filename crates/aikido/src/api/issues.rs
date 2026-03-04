use crate::client::AikidoClient;
use crate::error::Result;
use crate::models::*;
use serde_json::Value;

impl AikidoClient {
    pub async fn get_issue(&self, issue_id: i64) -> Result<Issue> {
        self.get(&format!("/issues/{issue_id}")).await
    }

    pub async fn get_issue_reachability(&self, issue_id: i64) -> Result<IssueReachability> {
        self.get(&format!("/issues/{issue_id}/reachability")).await
    }

    pub async fn get_issue_counts(&self) -> Result<IssueCounts> {
        self.get("/issues/counts").await
    }

    pub async fn get_issue_details_bulk(&self, issue_ids: &[i64]) -> Result<Value> {
        let ids_param = issue_ids
            .iter()
            .map(|id| format!("id={id}"))
            .collect::<Vec<_>>()
            .join("&");
        self.get(&format!("/issues/detail/bulk?{ids_param}")).await
    }

    pub async fn export_issues(&self) -> Result<Value> {
        self.get("/issues/export").await
    }

    pub async fn ignore_issue(&self, issue_id: i64, body: &IgnoreRequest) -> Result<Value> {
        self.put(&format!("/issues/{issue_id}/ignore"), body).await
    }

    pub async fn unignore_issue(&self, issue_id: i64) -> Result<Value> {
        self.put_no_content(&format!("/issues/{issue_id}/unignore"), &serde_json::json!({}))
            .await
            .map(|_| Value::Null)
    }

    pub async fn snooze_issue(&self, issue_id: i64, body: &SnoozeRequest) -> Result<Value> {
        self.put(&format!("/issues/{issue_id}/snooze"), body).await
    }

    pub async fn unsnooze_issue(&self, issue_id: i64) -> Result<Value> {
        self.put_no_content(&format!("/issues/{issue_id}/unsnooze"), &serde_json::json!({}))
            .await
            .map(|_| Value::Null)
    }

    pub async fn adjust_severity(
        &self,
        issue_id: i64,
        body: &AdjustSeverityRequest,
    ) -> Result<Value> {
        self.post(&format!("/issues/{issue_id}/severity/adjust"), body)
            .await
    }
}
